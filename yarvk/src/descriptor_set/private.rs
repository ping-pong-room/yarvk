use crate::descriptor_set::descriptor_set_layout::DescriptorSetLayout;
use crate::device::Device;
use crate::image_view::ImageView;
use crate::sampler::Sampler;
use crate::{IBuffer, BufferView};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::marker::PhantomData;
use std::sync::Arc;
use crate::descriptor_set::descriptor_set::DescriptorSetValue;

pub trait VkDescriptorUpdateInfo {
    fn update_info<'a>(
        builder: ash::vk::WriteDescriptorSetBuilder<'a>,
        update_info: &'a [Self],
    ) -> ash::vk::WriteDescriptorSetBuilder<'a>
    where
        Self: Sized;
}

impl VkDescriptorUpdateInfo for ash::vk::DescriptorImageInfo {
    fn update_info<'a>(
        builder: ash::vk::WriteDescriptorSetBuilder<'a>,
        update_info: &'a [Self],
    ) -> ash::vk::WriteDescriptorSetBuilder<'a> {
        builder.image_info(update_info)
    }
}

impl VkDescriptorUpdateInfo for ash::vk::DescriptorBufferInfo {
    fn update_info<'a>(
        builder: ash::vk::WriteDescriptorSetBuilder<'a>,
        update_info: &'a [Self],
    ) -> ash::vk::WriteDescriptorSetBuilder<'a> {
        builder.buffer_info(update_info)
    }
}

impl VkDescriptorUpdateInfo for ash::vk::BufferView {
    fn update_info<'a>(
        builder: ash::vk::WriteDescriptorSetBuilder<'a>,
        update_info: &'a [Self],
    ) -> ash::vk::WriteDescriptorSetBuilder<'a> {
        builder.texel_buffer_view(update_info)
    }
}

pub(crate) trait DescriptorBuilderUpdate<'a> {
    fn update_info<T: VkDescriptorUpdateInfo>(self, update_info: &'a [T]) -> Self;
}

impl<'a> DescriptorBuilderUpdate<'a> for ash::vk::WriteDescriptorSetBuilder<'a> {
    fn update_info<T: VkDescriptorUpdateInfo>(self, update_info: &'a [T]) -> Self {
        T::update_info(self, update_info)
    }
}

pub trait FnVkUpdateInfo {
    type VkValueType: VkDescriptorUpdateInfo + Sync + Send;
    fn to_vk_update_ty(&self) -> Self::VkValueType;
}

impl FnVkUpdateInfo for Arc<BufferView> {
    type VkValueType = ash::vk::BufferView;

    fn to_vk_update_ty(&self) -> Self::VkValueType {
        self.ash_vk_buffer_view
    }
}

impl FnVkUpdateInfo for (Arc<IBuffer>, ash::vk::DeviceSize, ash::vk::DeviceSize) {
    type VkValueType = ash::vk::DescriptorBufferInfo;

    fn to_vk_update_ty(&self) -> Self::VkValueType {
        ash::vk::DescriptorBufferInfo::builder()
            .buffer(self.0.ash_vk_buffer)
            .offset(self.1)
            .range(self.2)
            .build()
    }
}

impl FnVkUpdateInfo for Arc<Sampler> {
    type VkValueType = ash::vk::DescriptorImageInfo;

    fn to_vk_update_ty(&self) -> Self::VkValueType {
        ash::vk::DescriptorImageInfo::builder()
            .sampler(self.ash_vk_sampler)
            .build()
    }
}

impl FnVkUpdateInfo for (Arc<ImageView>, ash::vk::ImageLayout) {
    type VkValueType = ash::vk::DescriptorImageInfo;

    fn to_vk_update_ty(&self) -> Self::VkValueType {
        ash::vk::DescriptorImageInfo::builder()
            .image_view(self.0.ash_vk_image_view)
            .image_layout(self.1)
            .build()
    }
}

impl FnVkUpdateInfo for (Arc<Sampler>, Arc<ImageView>, ash::vk::ImageLayout) {
    type VkValueType = ash::vk::DescriptorImageInfo;

    fn to_vk_update_ty(&self) -> Self::VkValueType {
        ash::vk::DescriptorImageInfo::builder()
            .sampler(self.0.ash_vk_sampler)
            .image_view(self.1.ash_vk_image_view)
            .image_layout(self.2)
            .build()
    }
}

impl FnVkUpdateInfo for () {
    type VkValueType = ash::vk::DescriptorImageInfo;

    fn to_vk_update_ty(&self) -> Self::VkValueType {
        ash::vk::DescriptorImageInfo::builder().build()
    }
}

pub trait TryGetImmutableSamplers {
    fn try_get_immutable_samplers(&self) -> Vec<ash::vk::Sampler>;
}

impl<const N: usize> TryGetImmutableSamplers for [Arc<Sampler>; N] {
    fn try_get_immutable_samplers(&self) -> Vec<ash::vk::Sampler> {
        self.iter().map(|sampler| sampler.ash_vk_sampler).collect()
    }
}

impl<T> TryGetImmutableSamplers for T {
    default fn try_get_immutable_samplers(&self) -> Vec<ash::vk::Sampler> {
        Vec::new()
    }
}

pub trait PrivateConstDescriptorSetValue<T: DescriptorSetValue> {
    fn new_descriptor_layout(
        self,
        device: &Arc<Device>,
        flags: ash::vk::DescriptorSetLayoutCreateFlags,
    ) -> Result<Arc<DescriptorSetLayout<T>>, ash::vk::Result>;
}


struct SyncWriteDescriptorSet(ash::vk::WriteDescriptorSet);
unsafe impl Sync for SyncWriteDescriptorSet {}

pub struct ParallelSplitWriteDescriptorSets<'a> {
    focused: usize,
    current_num_threads: usize,
    vectors: Vec<Vec<SyncWriteDescriptorSet>>,
    _phantom_data: PhantomData<&'a usize>,
}

impl<'a> ParallelSplitWriteDescriptorSets<'a> {
    pub fn new() -> Self {
        let current_num_threads = rayon::current_num_threads();
        let mut vectors = Vec::new();
        vectors.resize_with(rayon::current_num_threads(), || Vec::new());
        Self {
            focused: 0,
            current_num_threads,
            vectors,
            _phantom_data: Default::default(),
        }
    }

    // push the same vector in the last time.
    pub fn fix_push(&mut self, write_descriptor_set: ash::vk::WriteDescriptorSetBuilder) {
        let vector = &mut self.vectors[self.focused];
        vector.push(SyncWriteDescriptorSet(write_descriptor_set.build()));
    }

    pub fn focus_next(&mut self) {
        self.focused = (self.focused + 1) % self.current_num_threads;
    }

    pub fn update(self, device: &Device) {
        self.vectors.par_iter().for_each(|vec| {
            unsafe {
                // Host Synchronization: VUID-vkUpdateDescriptorSets-pDescriptorWrites-06993
                //  pDescriptorWrites[i].dstSet pDescriptorCopies[i].dstSet
                device
                    .ash_device
                    .update_descriptor_sets(std::mem::transmute(vec.as_slice()), &[]);
            }
        })
    }
}

pub trait PrivateDescriptorSetValue: Clone {
    type VkType: 'static;
    fn to_vk_ty(&self) -> Self::VkType;
    fn push_to_update(
        &self,
        new_value: &Self,
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: ash::vk::DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    );
    fn push_to_init(
        &self,
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: ash::vk::DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    );
    fn put_pool_size(vec: &mut Vec<ash::vk::DescriptorPoolSize>);
}
