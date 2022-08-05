use crate::buffer::{Buffer, BufferView};
use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::device::Device;
use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE::MutableDescriptorType;
use crate::image::image_view::ImageView;
use crate::pipeline::shader_stage::ShaderStageFlags;
use crate::pipeline::PipelineLayout;
use crate::sampler::Sampler;

use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use std::sync::Arc;

pub enum DescriptorPoolCreateFlags {
    DescriptorPoolCreateFreeDescriptorSet,
    VkDescriptorPoolCreateUpdateAfterBind,
    // DONE VUID-VkDescriptorPoolCreateInfo-flags-04609
    VkDescriptorPoolCreateHostOnlyValve(Feature<{ MutableDescriptorType.into() }>),
}

impl DescriptorPoolCreateFlags {
    pub fn to_ash(&self) -> ash::vk::DescriptorPoolCreateFlags {
        match self {
            DescriptorPoolCreateFlags::DescriptorPoolCreateFreeDescriptorSet => {
                ash::vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET
            }
            DescriptorPoolCreateFlags::VkDescriptorPoolCreateUpdateAfterBind => {
                ash::vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND
            }
            DescriptorPoolCreateFlags::VkDescriptorPoolCreateHostOnlyValve(_) => {
                ash::vk::DescriptorPoolCreateFlags::HOST_ONLY_VALVE
            }
        }
    }
}

pub struct DescriptorPool {
    pub device: Arc<Device>,
    pub(crate) ash_vk_descriptor_pool: RwLock<ash::vk::DescriptorPool>,
}

impl DescriptorPool {
    pub fn builder(device: Arc<Device>) -> DescriptorPoolBuilder {
        DescriptorPoolBuilder {
            device,
            flags: Default::default(),
            max_sets: 1,
            descriptor_pool_sizes: Default::default(),
        }
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        unsafe {
            // DONE VUID-vkDestroyDescriptorPool-descriptorPool-00303
            // Host Synchronization: descriptorPool
            let ash_vk_descriptor_pool = self.ash_vk_descriptor_pool.write();
            self.device
                .ash_device
                .destroy_descriptor_pool(*ash_vk_descriptor_pool, None);
        }
    }
}

pub struct DescriptorPoolBuilder {
    device: Arc<Device>,
    flags: ash::vk::DescriptorPoolCreateFlags,
    max_sets: u32,
    descriptor_pool_sizes: Vec<ash::vk::DescriptorPoolSize>,
}

impl DescriptorPoolBuilder {
    pub fn add_flag(mut self, flag: DescriptorPoolCreateFlags) -> Self {
        self.flags |= flag.to_ash();
        self
    }
    pub fn max_sets(mut self, max_sets: u32) -> Self {
        self.max_sets = max_sets;
        self
    }
    pub fn add_descriptor_pool_size(
        mut self,
        descriptor_pool_size: ash::vk::DescriptorPoolSize,
    ) -> Self {
        self.descriptor_pool_sizes.push(descriptor_pool_size);
        self
    }

    pub fn build(self) -> Result<Arc<DescriptorPool>, ash::vk::Result> {
        let info = ash::vk::DescriptorPoolCreateInfo::builder()
            .flags(self.flags)
            .max_sets(self.max_sets)
            .pool_sizes(self.descriptor_pool_sizes.as_slice())
            .build();
        unsafe {
            // Host Synchronization: none
            let ash_vk_descriptor_pool =
                self.device.ash_device.create_descriptor_pool(&info, None)?;
            Ok(Arc::new(DescriptorPool {
                device: self.device,
                ash_vk_descriptor_pool: RwLock::new(ash_vk_descriptor_pool),
            }))
        }
    }
}

#[derive(Default)]
pub struct DescriptorSetLayoutBinding {
    binding: u32,
    descriptor_type: ash::vk::DescriptorType,
    descriptor_count: u32,
    stage_flags: ash::vk::ShaderStageFlags,
    p_immutable_samplers: Vec<Arc<Sampler>>,
    ash_vk_samplers: Vec<ash::vk::Sampler>,
}

impl DescriptorSetLayoutBinding {
    pub fn builder() -> DescriptorSetLayoutBindingBuilder {
        DescriptorSetLayoutBindingBuilder {
            inner: Default::default(),
        }
    }
    fn ash_builder(&self) -> ash::vk::DescriptorSetLayoutBindingBuilder {
        let builder = ash::vk::DescriptorSetLayoutBinding::builder()
            .binding(self.binding)
            .descriptor_type(self.descriptor_type)
            .descriptor_count(self.descriptor_count)
            .stage_flags(self.stage_flags);
        return if self.descriptor_type == ash::vk::DescriptorType::SAMPLER
            || self.descriptor_type == ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER
                && !self.p_immutable_samplers.is_empty()
        {
            builder.immutable_samplers(self.ash_vk_samplers.as_slice())
        } else {
            builder
        };
    }
}

pub struct DescriptorSetLayoutBindingBuilder {
    inner: DescriptorSetLayoutBinding,
}

impl DescriptorSetLayoutBindingBuilder {
    pub fn binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: ash::vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
    pub fn add_stage_flag(mut self, stage_flag: ShaderStageFlags) -> Self {
        self.inner.stage_flags |= stage_flag.to_ash();
        self
    }
    pub fn immutable_sampler(mut self, immutable_samplers: &[Arc<Sampler>]) -> Self {
        self.inner.descriptor_count = immutable_samplers.len() as _;
        self.inner
            .p_immutable_samplers
            .extend_from_slice(immutable_samplers);
        self
    }
    pub fn build(self) -> DescriptorSetLayoutBinding {
        self.inner
    }
}

pub struct DescriptorSetLayout {
    pub device: Arc<Device>,
    pub(crate) ash_vk_descriptor_set_layout: ash::vk::DescriptorSetLayout,
    bindings: FxHashMap<u32, DescriptorSetLayoutBinding>,
}

impl DescriptorSetLayout {
    pub fn builder(device: Arc<Device>) -> DescriptorSetLayoutBuilder {
        DescriptorSetLayoutBuilder {
            device,
            flags: Default::default(),
            bindings: Default::default(),
        }
    }
}

pub struct DescriptorSetLayoutBuilder {
    device: Arc<Device>,
    flags: ash::vk::DescriptorSetLayoutCreateFlags,
    bindings: FxHashMap<u32, DescriptorSetLayoutBinding>,
}

impl DescriptorSetLayoutBuilder {
    pub fn flags(mut self, flags: ash::vk::DescriptorSetLayoutCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn add_binding(mut self, binding: DescriptorSetLayoutBinding) -> Self {
        let result = self.bindings.insert(binding.binding, binding);
        // Must VUID-VkDescriptorSetLayoutCreateInfo-binding-00279
        if result.is_some() {
            panic!("VUID-VkDescriptorSetLayoutCreateInfo-binding-00279")
        }
        self
    }
    pub fn build(self) -> Result<Arc<DescriptorSetLayout>, ash::vk::Result> {
        let bindings = self
            .bindings
            .iter()
            .map(|(_, binding)| binding.ash_builder().build())
            .collect::<Vec<_>>();
        let create_info = ash::vk::DescriptorSetLayoutCreateInfo::builder()
            .flags(self.flags)
            .bindings(bindings.as_slice())
            .build();
        unsafe {
            // Host Synchronization: none
            let ash_vk_descriptor_set_layout = self
                .device
                .ash_device
                .create_descriptor_set_layout(&create_info, None)?;
            Ok(Arc::new(DescriptorSetLayout {
                device: self.device,
                ash_vk_descriptor_set_layout,
                bindings: self.bindings,
            }))
        }
    }
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            // Host Synchronization: descriptorSetLayout
            self.device
                .ash_device
                .destroy_descriptor_set_layout(self.ash_vk_descriptor_set_layout, None);
        }
    }
}

pub struct DescriptorSet {
    pub descriptor_pool: Arc<DescriptorPool>,
    pub descriptor_set_layout: Arc<DescriptorSetLayout>,
    pub(crate) ash_vk_descriptor_set: RwLock<ash::vk::DescriptorSet>,
}

impl DescriptorSet {
    pub fn builder(descriptor_pool: Arc<DescriptorPool>) -> DescriptorSetBuilder {
        DescriptorSetBuilder {
            descriptor_pool,
            layouts: Default::default(),
        }
    }
}

// TODO need more sufficient design, current leak the resource now
impl Drop for DescriptorSet {
    fn drop(&mut self) {
        // unsafe {
        //     // TODO VUID-vkFreeDescriptorSets-pDescriptorSets-00309
        //     // Host Synchronization descriptorPool, pDescriptorSets
        //     let ash_vk_descriptor_pool = self.descriptor_pool.ash_vk_descriptor_pool.write();
        //     self.descriptor_pool.device.ash_device.free_descriptor_sets(*ash_vk_descriptor_pool, &[self.ash_vk_descriptor_set]);
        // }
    }
}

pub struct DescriptorSetBuilder {
    descriptor_pool: Arc<DescriptorPool>,
    layouts: Vec<Arc<DescriptorSetLayout>>,
}

impl DescriptorSetBuilder {
    pub fn add_set_layout(mut self, layout: Arc<DescriptorSetLayout>) -> Self {
        self.layouts.push(layout);
        self
    }
    pub fn build(self) -> Result<Vec<Arc<DescriptorSet>>, ash::vk::Result> {
        let layouts = self
            .layouts
            .iter()
            .map(|layout| layout.ash_vk_descriptor_set_layout)
            .collect::<Vec<_>>();
        let ash_vk_descriptor_pool = self.descriptor_pool.ash_vk_descriptor_pool.write();
        let create_info = ash::vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(*ash_vk_descriptor_pool)
            .set_layouts(layouts.as_slice())
            .build();
        let ash_vk_descriptor_sets = unsafe {
            // Host Synchronization: pAllocateInfo->descriptorPool
            self.descriptor_pool
                .device
                .ash_device
                .allocate_descriptor_sets(&create_info)?
        };
        drop(ash_vk_descriptor_pool);
        Ok(ash_vk_descriptor_sets
            .into_iter()
            .enumerate()
            .map(|(i, ash_vk_descriptor_set)| {
                Arc::new(DescriptorSet {
                    descriptor_pool: self.descriptor_pool.clone(),
                    ash_vk_descriptor_set: RwLock::new(ash_vk_descriptor_set),
                    descriptor_set_layout: self.layouts[i].clone(),
                })
            })
            .collect())
    }
}

#[derive(Clone)]
pub struct DescriptorImageInfo {
    pub sampler: Arc<Sampler>,
    pub image_view: Arc<ImageView>,
    pub image_layout: ash::vk::ImageLayout,
}

impl DescriptorImageInfo {
    pub(crate) fn ash_builder(&self) -> ash::vk::DescriptorImageInfoBuilder {
        ash::vk::DescriptorImageInfo::builder()
            .sampler(self.sampler.ash_vk_sampler)
            .image_view(self.image_view.ash_vk_image_view)
            .image_layout(self.image_layout)
    }
}

#[derive(Clone)]
pub struct DescriptorBufferInfo {
    pub buffer: Arc<Buffer>,
    pub offset: ash::vk::DeviceSize,
    pub range: ash::vk::DeviceSize,
}

impl DescriptorBufferInfo {
    pub(crate) fn ash_builder(&self) -> ash::vk::DescriptorBufferInfoBuilder {
        ash::vk::DescriptorBufferInfo::builder()
            .buffer(self.buffer.ash_vk_buffer)
            .offset(self.offset)
            .range(self.range)
    }
}

pub const DESCRIPTOR_INFO_TYPE_IMAGE: usize = 0;
pub const DESCRIPTOR_INFO_TYPE_BUFFER: usize = 1;
pub const DESCRIPTOR_INFO_TYPE_TEXEL_BUFFER_VIEW: usize = 2;

pub struct WriteDescriptorSet {
    pub(crate) dst_set: Arc<DescriptorSet>,
    dst_binding: u32,
    dst_array_element: u32,
    p_image_info: Vec<DescriptorImageInfo>,
    p_buffer_info: Vec<DescriptorBufferInfo>,
    p_texel_buffer_view: Vec<Arc<BufferView>>,
    ash_vk_image_infos: Vec<ash::vk::DescriptorImageInfo>,
    ash_vk_buffer_infos: Vec<ash::vk::DescriptorBufferInfo>,
    ash_vk_buffer_views: Vec<ash::vk::BufferView>,
    which_info: usize,
}

impl WriteDescriptorSet {
    pub fn builder<const INFO_TYPE: usize>(
        dst_set: Arc<DescriptorSet>,
    ) -> WriteDescriptorSetBuilder<INFO_TYPE> {
        WriteDescriptorSetBuilder {
            inner: WriteDescriptorSet {
                which_info: 0,
                dst_binding: 0,
                dst_array_element: 0,
                p_image_info: vec![],
                p_buffer_info: vec![],
                p_texel_buffer_view: vec![],
                ash_vk_image_infos: vec![],
                ash_vk_buffer_infos: vec![],
                ash_vk_buffer_views: vec![],
                dst_set,
            },
        }
    }
    pub(crate) fn ash_builder(&self) -> ash::vk::WriteDescriptorSetBuilder {
        // Must VUID-VkWriteDescriptorSet-dstBinding-00315
        let binding = self
            .dst_set
            .descriptor_set_layout
            .bindings
            .get(&self.dst_binding)
            .expect("VUID-VkWriteDescriptorSet-dstBinding-00315");
        let builder = ash::vk::WriteDescriptorSet::builder()
            .dst_set(*self.dst_set.ash_vk_descriptor_set.read())
            .dst_binding(self.dst_binding)
            .dst_array_element(self.dst_array_element)
            .descriptor_type(binding.descriptor_type);
        return match self.which_info {
            DESCRIPTOR_INFO_TYPE_IMAGE => builder.image_info(self.ash_vk_image_infos.as_slice()),
            DESCRIPTOR_INFO_TYPE_BUFFER => builder.buffer_info(self.ash_vk_buffer_infos.as_slice()),
            DESCRIPTOR_INFO_TYPE_TEXEL_BUFFER_VIEW => {
                builder.texel_buffer_view(self.ash_vk_buffer_views.as_slice())
            }
            _ => {
                panic!("unsupported descriptor info")
            }
        };
    }
}

pub struct WriteDescriptorSetBuilder<const INFO_TYPE: usize> {
    inner: WriteDescriptorSet,
}

impl<const INFO_TYPE: usize> WriteDescriptorSetBuilder<INFO_TYPE> {
    pub fn dst_set(mut self, dst_set: Arc<DescriptorSet>) -> Self {
        self.inner.dst_set = dst_set;
        self
    }
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.inner.dst_binding = dst_binding;
        self
    }
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.inner.dst_array_element = dst_array_element;
        self
    }
}

impl WriteDescriptorSetBuilder<DESCRIPTOR_INFO_TYPE_IMAGE> {
    pub fn add_image_info(mut self, image_info: DescriptorImageInfo) -> Self {
        self.inner.p_image_info.push(image_info);
        self
    }
    pub fn build(mut self) -> WriteDescriptorSet {
        self.inner.which_info = DESCRIPTOR_INFO_TYPE_IMAGE;
        self.inner.ash_vk_image_infos = self
            .inner
            .p_image_info
            .iter()
            .map(|b| b.ash_builder().build())
            .collect::<Vec<_>>();
        self.inner
    }
}

impl WriteDescriptorSetBuilder<DESCRIPTOR_INFO_TYPE_BUFFER> {
    pub fn add_buffer_info(mut self, buffer_info: DescriptorBufferInfo) -> Self {
        self.inner.p_buffer_info.push(buffer_info);
        self
    }
    pub fn build(mut self) -> WriteDescriptorSet {
        self.inner.which_info = DESCRIPTOR_INFO_TYPE_BUFFER;
        self.inner.ash_vk_buffer_infos = self
            .inner
            .p_buffer_info
            .iter()
            .map(|b| b.ash_builder().build())
            .collect::<Vec<_>>();
        self.inner
    }
}

impl WriteDescriptorSetBuilder<DESCRIPTOR_INFO_TYPE_TEXEL_BUFFER_VIEW> {
    pub fn add_texel_buffer_view(mut self, texel_buffer_view: Arc<BufferView>) -> Self {
        self.inner.p_texel_buffer_view.push(texel_buffer_view);
        self
    }
    pub fn build(mut self) -> WriteDescriptorSet {
        self.inner.which_info = DESCRIPTOR_INFO_TYPE_TEXEL_BUFFER_VIEW;
        self.inner.ash_vk_buffer_views = self
            .inner
            .p_texel_buffer_view
            .iter()
            .map(|b| b.ash_vk_buffer_view)
            .collect::<Vec<_>>();
        self.inner
    }
}

pub struct CopyDescriptorSet {
    pub(crate) src_set: Arc<DescriptorSet>,
    src_binding: u32,
    src_array_element: u32,
    pub(crate) dst_set: Arc<DescriptorSet>,
    dst_binding: u32,
    dst_array_element: u32,
    descriptor_count: u32,
}

impl CopyDescriptorSet {
    pub(crate) fn ash_builder(&self) -> ash::vk::CopyDescriptorSetBuilder {
        ash::vk::CopyDescriptorSet::builder()
            .src_set(*self.src_set.ash_vk_descriptor_set.read())
            .src_binding(self.src_binding)
            .src_array_element(self.src_array_element)
            .dst_set(*self.dst_set.ash_vk_descriptor_set.read())
            .dst_binding(self.dst_binding)
            .dst_array_element(self.dst_array_element)
            .descriptor_count(self.descriptor_count)
    }
}

pub struct CopyDescriptorSetBuilder {
    inner: CopyDescriptorSet,
}

impl CopyDescriptorSetBuilder {
    pub fn src_set(mut self, src_set: Arc<DescriptorSet>) -> Self {
        self.inner.src_set = src_set;
        self
    }
    pub fn src_binding(mut self, src_binding: u32) -> Self {
        self.inner.src_binding = src_binding;
        self
    }
    pub fn src_array_element(mut self, src_array_element: u32) -> Self {
        self.inner.src_array_element = src_array_element;
        self
    }
    pub fn dst_set(mut self, dst_set: Arc<DescriptorSet>) -> Self {
        self.inner.dst_set = dst_set;
        self
    }
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.inner.dst_binding = dst_binding;
        self
    }
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.inner.dst_array_element = dst_array_element;
        self
    }
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
}

impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { RECORDING }, SCOPE> {
    // DONE VUID-vkCmdBindDescriptorSets-commandBuffer-recording
    pub fn cmd_bind_descriptor_sets(
        &mut self,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        layout: &PipelineLayout,
        first_set: u32,
        descriptor_sets: &[Arc<DescriptorSet>],
        dynamic_offsets: &[u32],
    ) {
        // TODO performance (vec new)
        let mut locks = Vec::new();
        let mut vk_descriptor_sets = Vec::new();
        for set in descriptor_sets.iter() {
            let lock = set.ash_vk_descriptor_set.read();
            let vk_ds = *lock;
            locks.push(lock);
            vk_descriptor_sets.push(vk_ds);
        }

        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_bind_descriptor_sets(
                self.vk_command_buffer,
                pipeline_bind_point,
                layout.ash_vk_pipeline_layout,
                first_set,
                vk_descriptor_sets.as_slice(),
                dynamic_offsets,
            )
        }
    }
}
