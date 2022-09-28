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
            // TODO Allow individually free for now
            flags: ash::vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,
            max_sets: 1,
            descriptor_pool_sizes: Default::default(),
        }
    }

    pub fn allocate_descriptor_sets(
        self: &Arc<Self>,
        layouts: &[Arc<DescriptorSetLayout>],
    ) -> Result<Vec<DescriptorSet>, ash::vk::Result> {
        let vk_layouts = layouts
            .iter()
            .map(|layout| layout.ash_vk_descriptor_set_layout)
            .collect::<Vec<_>>();
        let ash_vk_descriptor_pool = self.ash_vk_descriptor_pool.write();
        let create_info = ash::vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(*ash_vk_descriptor_pool)
            .set_layouts(vk_layouts.as_slice())
            .build();
        let ash_vk_descriptor_sets = unsafe {
            // Host Synchronization: pAllocateInfo->descriptorPool
            self.device
                .ash_device
                .allocate_descriptor_sets(&create_info)?
        };
        drop(ash_vk_descriptor_pool);
        Ok(ash_vk_descriptor_sets
            .into_iter()
            .enumerate()
            .map(|(i, ash_vk_descriptor_set)| DescriptorSet {
                descriptor_pool: self.clone(),
                ash_vk_descriptor_set,
                descriptor_set_layout: layouts[i].clone(),
            })
            .collect())
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
    pub(crate) ash_vk_descriptor_set: ash::vk::DescriptorSet,
}

impl DescriptorSet {
    pub fn change(&mut self) -> ChangedDescriptorSet {
        ChangedDescriptorSet {
            descriptor_set: self,
            bindings: Default::default(),
            images_counts: 0,
            buffers_counts: 0,
            buffer_views_counts: 0,
        }
    }
}

// TODO VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT
impl Drop for DescriptorSet {
    fn drop(&mut self) {
        unsafe {
            // TODO VUID-vkFreeDescriptorSets-pDescriptorSets-00309
            // Host Synchronization descriptorPool, pDescriptorSets
            let ash_vk_descriptor_pool = self.descriptor_pool.ash_vk_descriptor_pool.write();
            self.descriptor_pool
                .device
                .ash_device
                .free_descriptor_sets(*ash_vk_descriptor_pool, &[self.ash_vk_descriptor_set])
                .unwrap();
        }
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
    pub buffer: Arc<dyn Buffer>,
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

struct WriteImage<'a> {
    dst_array_element: u32,
    image_info: &'a [DescriptorImageInfo],
}

struct WriteBuffer<'a> {
    dst_array_element: u32,
    buffer_info: &'a [DescriptorBufferInfo],
}

struct WriteTexelBufferView<'a> {
    dst_array_element: u32,
    texel_buffer_view: &'a [BufferView],
}

enum DescriptorInfoType<'a> {
    Image(WriteImage<'a>),
    Buffer(WriteBuffer<'a>),
    TexelBufferView(WriteTexelBufferView<'a>),
}

// TODO CopyDescriptorSet
pub struct ChangedDescriptorSet<'a> {
    descriptor_set: &'a mut DescriptorSet,
    bindings: FxHashMap<u32, DescriptorInfoType<'a>>,
    pub(crate) images_counts: usize,
    pub(crate) buffers_counts: usize,
    pub(crate) buffer_views_counts: usize,
}

// used for device.update_descriptor_sets for performance reason
pub(crate) struct UpdateHolder {
    pub vk_image_infos: Vec<Vec<ash::vk::DescriptorImageInfo>>,
    pub vk_buffer_infos: Vec<Vec<ash::vk::DescriptorBufferInfo>>,
    pub vk_texel_buffer_views: Vec<Vec<ash::vk::BufferView>>,
    pub vk_write_descriptor_sets: Vec<ash::vk::WriteDescriptorSet>,
}

impl<'a> ChangedDescriptorSet<'a> {
    pub fn update_image(
        &mut self,
        binding: u32,
        dst_array_element: u32,
        image_info: &'a [DescriptorImageInfo],
    ) {
        if self
            .bindings
            .insert(
                binding,
                DescriptorInfoType::Image(WriteImage {
                    dst_array_element,
                    image_info,
                }),
            )
            .is_none()
        {
            self.images_counts += 1;
        }
    }

    pub fn update_buffer(
        &mut self,
        binding: u32,
        dst_array_element: u32,
        buffer_info: &'a [DescriptorBufferInfo],
    ) {
        if self
            .bindings
            .insert(
                binding,
                DescriptorInfoType::Buffer(WriteBuffer {
                    dst_array_element,
                    buffer_info,
                }),
            )
            .is_none()
        {
            self.buffers_counts += 1;
        }
    }

    pub fn update_texel_buffer_view(
        &mut self,
        binding: u32,
        dst_array_element: u32,
        texel_buffer_view: &'a [BufferView],
    ) {
        if self
            .bindings
            .insert(
                binding,
                DescriptorInfoType::TexelBufferView(WriteTexelBufferView {
                    dst_array_element,
                    texel_buffer_view,
                }),
            )
            .is_none()
        {
            self.buffer_views_counts += 1;
        }
    }

    pub(crate) fn to_ash(&self, holder: &mut UpdateHolder) {
        let write_descriptors = self
            .bindings
            .iter()
            .map(|(binding, descriptor_info_type)| {
                let mut builder = ash::vk::WriteDescriptorSet::builder()
                    .dst_set(self.descriptor_set.ash_vk_descriptor_set)
                    .dst_binding(*binding)
                    .descriptor_type(
                        self.descriptor_set.descriptor_set_layout.bindings[&binding]
                            .descriptor_type,
                    );
                match descriptor_info_type {
                    DescriptorInfoType::Image(image_info) => {
                        builder = builder.dst_array_element(image_info.dst_array_element);
                        holder.vk_image_infos.push(
                            image_info
                                .image_info
                                .iter()
                                .map(|info| info.ash_builder().build())
                                .collect(),
                        );
                        let slice = holder.vk_image_infos.last().unwrap().as_slice();
                        builder = builder.image_info(slice);
                    }
                    DescriptorInfoType::Buffer(buffer_info) => {
                        builder = builder.dst_array_element(buffer_info.dst_array_element);
                        holder.vk_buffer_infos.push(
                            buffer_info
                                .buffer_info
                                .iter()
                                .map(|info| info.ash_builder().build())
                                .collect(),
                        );
                        let slice = holder.vk_buffer_infos.last().unwrap();
                        builder = builder.buffer_info(slice);
                    }
                    DescriptorInfoType::TexelBufferView(ash_vk_buffer_views) => {
                        builder = builder.dst_array_element(ash_vk_buffer_views.dst_array_element);
                        holder.vk_texel_buffer_views.push(
                            ash_vk_buffer_views
                                .texel_buffer_view
                                .iter()
                                .map(|info| info.ash_vk_buffer_view)
                                .collect(),
                        );
                        let slice = holder.vk_texel_buffer_views.last().unwrap();
                        builder = builder.texel_buffer_view(slice);
                    }
                }
                builder.build()
            })
            .collect::<Vec<_>>();
        holder.vk_write_descriptor_sets = write_descriptors;
    }
    // TODO implement copy
    pub fn update(self) {
        let mut holder = UpdateHolder {
            vk_image_infos: Vec::with_capacity(self.images_counts),
            vk_buffer_infos: Vec::with_capacity(self.buffers_counts),
            vk_texel_buffer_views: Vec::with_capacity(self.buffer_views_counts),
            vk_write_descriptor_sets: Vec::with_capacity(self.bindings.len()),
        };
        self.to_ash(&mut holder);
        unsafe {
            // Host Synchronization: VUID-vkUpdateDescriptorSets-pDescriptorWrites-06993
            //  pDescriptorWrites[i].dstSet pDescriptorCopies[i].dstSet
            self.descriptor_set
                .descriptor_pool
                .device
                .ash_device
                .update_descriptor_sets(holder.vk_write_descriptor_sets.as_slice(), &[]);
        }
    }
}

impl<const LEVEL: Level, const SCOPE: RenderPassScope, const ONE_TIME_SUBMIT: bool>
    CommandBuffer<LEVEL, { RECORDING }, SCOPE, ONE_TIME_SUBMIT>
{
    // DONE VUID-vkCmdBindDescriptorSets-commandBuffer-recording
    pub fn cmd_bind_descriptor_sets(
        &mut self,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        layout: &PipelineLayout,
        first_set: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        let vk_descriptor_sets: Vec<_> = descriptor_sets
            .iter()
            .map(|ds| ds.ash_vk_descriptor_set)
            .collect();

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
