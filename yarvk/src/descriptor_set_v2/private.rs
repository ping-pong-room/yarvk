use crate::descriptor_set_v2::descriptor_set::{
    ConstDescriptorSetValue1, ConstDescriptorSetValue2, DescriptorSetValue, DescriptorSetValue1,
    DescriptorSetValue2,
};
use crate::descriptor_set_v2::descriptor_set_layout::DescriptorSetLayout;
use crate::descriptor_set_v2::descriptor_type::{
    DescriptorKind, DescriptorType, DescriptorTypeDetail,
};
use crate::descriptor_set_v2::diff;
use crate::device::Device;
use crate::image_view::ImageView;
use crate::sampler::Sampler;
use crate::{Buffer, BufferView};
use ash::vk::DescriptorSet;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::marker::PhantomData;
use std::sync::Arc;

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

trait DescriptorBuilderUpdate<'a> {
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

impl FnVkUpdateInfo for (Arc<dyn Buffer>, ash::vk::DeviceSize, ash::vk::DeviceSize) {
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
        &self,
        device: &Arc<Device>,
        flags: ash::vk::DescriptorSetLayoutCreateFlags,
    ) -> Result<Arc<DescriptorSetLayout<T>>, ash::vk::Result>;
}

impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
        const BINDING_1: u32,
        const DESCRIPTOR_KIND_1: DescriptorKind,
        const DESCRIPTOR_COUNTS_1: usize,
    >
    PrivateConstDescriptorSetValue<
        DescriptorSetValue2<
            BINDING_0,
            DESCRIPTOR_KIND_0,
            DESCRIPTOR_COUNTS_0,
            BINDING_1,
            DESCRIPTOR_KIND_1,
            DESCRIPTOR_COUNTS_1,
        >,
    >
    for ConstDescriptorSetValue2<
        BINDING_0,
        DESCRIPTOR_KIND_0,
        DESCRIPTOR_COUNTS_0,
        BINDING_1,
        DESCRIPTOR_KIND_1,
        DESCRIPTOR_COUNTS_1,
    >
{
    fn new_descriptor_layout(
        &self,
        device: &Arc<Device>,
        flags: ash::vk::DescriptorSetLayoutCreateFlags,
    ) -> Result<
        Arc<
            DescriptorSetLayout<
                DescriptorSetValue2<
                    BINDING_0,
                    DESCRIPTOR_KIND_0,
                    DESCRIPTOR_COUNTS_0,
                    BINDING_1,
                    DESCRIPTOR_KIND_1,
                    DESCRIPTOR_COUNTS_1,
                >,
            >,
        >,
        ash::vk::Result,
    > {
        let mut ash_vk_bindings = Vec::with_capacity(1);

        let immutable_samplers_0 = self.t0.0.try_get_immutable_samplers();
        let mut builder_0 = ash::vk::DescriptorSetLayoutBinding::builder()
            .binding(BINDING_0)
            .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
            .descriptor_count(DESCRIPTOR_COUNTS_0 as _)
            .stage_flags(self.t0.1.to_ash());
        if !immutable_samplers_0.is_empty() {
            builder_0 = builder_0.immutable_samplers(immutable_samplers_0.as_slice());
        };
        ash_vk_bindings.push(builder_0.build());

        let immutable_samplers_1 = self.t1.0.try_get_immutable_samplers();
        let mut builder_1 = ash::vk::DescriptorSetLayoutBinding::builder()
            .binding(BINDING_1)
            .descriptor_type(DESCRIPTOR_KIND_1.to_ash())
            .descriptor_count(DESCRIPTOR_COUNTS_1 as _)
            .stage_flags(self.t0.1.to_ash());
        if !immutable_samplers_1.is_empty() {
            builder_1 = builder_1.immutable_samplers(immutable_samplers_1.as_slice());
        };
        ash_vk_bindings.push(builder_1.build());

        let create_info = ash::vk::DescriptorSetLayoutCreateInfo::builder()
            .flags(flags)
            .bindings(ash_vk_bindings.as_slice())
            .build();
        unsafe {
            // Host Synchronization: none
            let ash_vk_descriptor_set_layout = device
                .ash_device
                .create_descriptor_set_layout(&create_info, None)?;
            Ok(Arc::new(DescriptorSetLayout {
                device: device.clone(),
                ash_vk_descriptor_set_layout,
                _phantom_data: Default::default(),
            }))
        }
    }
}

impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
    >
    PrivateConstDescriptorSetValue<
        DescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>,
    > for ConstDescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>
{
    fn new_descriptor_layout(
        &self,
        device: &Arc<Device>,
        flags: ash::vk::DescriptorSetLayoutCreateFlags,
    ) -> Result<
        Arc<
            DescriptorSetLayout<
                DescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>,
            >,
        >,
        ash::vk::Result,
    > {
        let mut ash_vk_bindings = Vec::with_capacity(1);
        let immutable_samplers_0 = self.t0.0.try_get_immutable_samplers();
        let mut builder_0 = ash::vk::DescriptorSetLayoutBinding::builder()
            .binding(BINDING_0)
            .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
            .descriptor_count(DESCRIPTOR_COUNTS_0 as _)
            .stage_flags(self.t0.1.to_ash());
        if !immutable_samplers_0.is_empty() {
            builder_0 = builder_0.immutable_samplers(immutable_samplers_0.as_slice());
        };
        ash_vk_bindings.push(builder_0.build());

        let create_info = ash::vk::DescriptorSetLayoutCreateInfo::builder()
            .flags(flags)
            .bindings(ash_vk_bindings.as_slice())
            .build();
        unsafe {
            // Host Synchronization: none
            let ash_vk_descriptor_set_layout = device
                .ash_device
                .create_descriptor_set_layout(&create_info, None)?;
            Ok(Arc::new(DescriptorSetLayout {
                device: device.clone(),
                ash_vk_descriptor_set_layout,
                _phantom_data: Default::default(),
            }))
        }
    }
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

pub struct VkUpdateInfoCollection1<
    const BINDING_0: u32,
    const DESCRIPTOR_KIND_0: DescriptorKind,
    const DESCRIPTOR_COUNTS_0: usize,
> {
    pub(crate) vk_t0: [<<DescriptorTypeDetail<{ DESCRIPTOR_KIND_0 }> as DescriptorType>::ValueType as FnVkUpdateInfo>::VkValueType;
        DESCRIPTOR_COUNTS_0],
}

impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
    > PrivateDescriptorSetValue
    for DescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>
{
    type VkType = VkUpdateInfoCollection1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>;

    fn to_vk_ty(&self) -> Self::VkType {
        VkUpdateInfoCollection1 {
            vk_t0: std::array::from_fn(|i| self.t0[i].to_vk_update_ty()),
        }
    }

    fn push_to_update(
        &self,
        new_value: &Self,
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: ash::vk::DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    ) {
        diff(&self.t0, &new_value.t0, |dst_array_element, counts| {
            let slice = &vk_value.vk_t0[dst_array_element..counts];
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_0)
                .dst_array_element(dst_array_element as _)
                .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        });

        vec.focus_next();
    }

    fn push_to_init(
        &self,
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    ) {
        let slice = &vk_value.vk_t0;
        let builder = ash::vk::WriteDescriptorSet::builder()
            .dst_set(ash_vk_descriptor_set)
            .dst_binding(BINDING_0)
            .dst_array_element(0)
            .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
            .update_info(slice);
        vec.fix_push(builder);
    }

    fn put_pool_size(vec: &mut Vec<ash::vk::DescriptorPoolSize>) {
        vec.push(ash::vk::DescriptorPoolSize {
            ty: DESCRIPTOR_KIND_0.to_ash(),
            descriptor_count: DESCRIPTOR_COUNTS_0 as u32,
        })
    }
}

pub struct VkUpdateInfoCollection2<
    const BINDING_0: u32,
    const DESCRIPTOR_KIND_0: DescriptorKind,
    const DESCRIPTOR_COUNTS_0: usize,
    const BINDING_1: u32,
    const DESCRIPTOR_KIND_1: DescriptorKind,
    const DESCRIPTOR_COUNTS_1: usize,
> {
    pub(crate) vk_t0: [<<DescriptorTypeDetail<{ DESCRIPTOR_KIND_0 }> as DescriptorType>::ValueType as FnVkUpdateInfo>::VkValueType;
        DESCRIPTOR_COUNTS_0],
    pub(crate) vk_t1: [<<DescriptorTypeDetail<{ DESCRIPTOR_KIND_1 }> as DescriptorType>::ValueType as FnVkUpdateInfo>::VkValueType;
        DESCRIPTOR_COUNTS_1],
}

impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
        const BINDING_1: u32,
        const DESCRIPTOR_KIND_1: DescriptorKind,
        const DESCRIPTOR_COUNTS_1: usize,
    > PrivateDescriptorSetValue
    for DescriptorSetValue2<
        BINDING_0,
        DESCRIPTOR_KIND_0,
        DESCRIPTOR_COUNTS_0,
        BINDING_1,
        DESCRIPTOR_KIND_1,
        DESCRIPTOR_COUNTS_1,
    >
{
    type VkType = VkUpdateInfoCollection2<
        BINDING_0,
        DESCRIPTOR_KIND_0,
        DESCRIPTOR_COUNTS_0,
        BINDING_1,
        DESCRIPTOR_KIND_1,
        DESCRIPTOR_COUNTS_1,
    >;

    fn to_vk_ty(&self) -> Self::VkType {
        VkUpdateInfoCollection2 {
            vk_t0: std::array::from_fn(|i| self.t0[i].to_vk_update_ty()),
            vk_t1: std::array::from_fn(|i| self.t1[i].to_vk_update_ty()),
        }
    }

    fn push_to_update(
        &self,
        new_value: &Self,
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: ash::vk::DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    ) {
        diff(&self.t0, &new_value.t0, |dst_array_element, counts| {
            let slice = &vk_value.vk_t0[dst_array_element..counts];
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_0)
                .dst_array_element(dst_array_element as _)
                .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        });

        diff(&self.t1, &new_value.t1, |dst_array_element, counts| {
            let slice = &vk_value.vk_t1[dst_array_element..counts];
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_1)
                .dst_array_element(dst_array_element as _)
                .descriptor_type(DESCRIPTOR_KIND_1.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        });

        vec.focus_next();
    }

    fn push_to_init(
        &self,
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    ) {
        {
            let slice = &vk_value.vk_t0;
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_0)
                .dst_array_element(0)
                .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        }
        {
            let slice = &vk_value.vk_t1;
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_1)
                .dst_array_element(0)
                .descriptor_type(DESCRIPTOR_KIND_1.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        }
    }

    fn put_pool_size(vec: &mut Vec<ash::vk::DescriptorPoolSize>) {
        vec.push(ash::vk::DescriptorPoolSize {
            ty: DESCRIPTOR_KIND_0.to_ash(),
            descriptor_count: DESCRIPTOR_COUNTS_0 as u32,
        });

        vec.push(ash::vk::DescriptorPoolSize {
            ty: DESCRIPTOR_KIND_1.to_ash(),
            descriptor_count: DESCRIPTOR_COUNTS_1 as u32,
        });
    }
}
