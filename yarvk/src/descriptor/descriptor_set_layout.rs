use crate::device::Device;
use crate::pipeline::shader_stage::ShaderStage;
use rustc_hash::FxHashMap;
use std::sync::Arc;
use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceAccelerationStructureFeaturesKHR::AccelerationStructure;
use crate::extensions::DeviceExtension;
use crate::extensions::PhysicalDeviceExtensionType::NvRayTracing;
use crate::sampler::Sampler;

#[derive(Default)]
pub enum DescriptorType {
    #[default]
    Sampler,
    SamplerImmutable(Vec<Arc<Sampler>>),
    CombinedImageSampler,
    CombinedImageSamplerImmutable(Vec<Arc<Sampler>>),
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
    InlineUniformBlock,
    AccelerationStructureKhr(Feature<{ AccelerationStructure.into() }>),
    AccelerationStructureNv(DeviceExtension<{ NvRayTracing.into() }>),
    MutableValve,
}

impl DescriptorType {
    pub(crate) fn to_ash(&self) -> ash::vk::DescriptorType {
        match self {
            DescriptorType::Sampler => ash::vk::DescriptorType::SAMPLER,
            DescriptorType::SamplerImmutable(_) => ash::vk::DescriptorType::SAMPLER,
            DescriptorType::CombinedImageSampler => ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            DescriptorType::CombinedImageSamplerImmutable(_) => {
                ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER
            }
            DescriptorType::SampledImage => ash::vk::DescriptorType::SAMPLED_IMAGE,
            DescriptorType::StorageImage => ash::vk::DescriptorType::STORAGE_IMAGE,
            DescriptorType::UniformTexelBuffer => ash::vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
            DescriptorType::StorageTexelBuffer => ash::vk::DescriptorType::STORAGE_TEXEL_BUFFER,
            DescriptorType::UniformBuffer => ash::vk::DescriptorType::UNIFORM_BUFFER,
            DescriptorType::StorageBuffer => ash::vk::DescriptorType::STORAGE_BUFFER,
            DescriptorType::UniformBufferDynamic => ash::vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            DescriptorType::StorageBufferDynamic => ash::vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            DescriptorType::InputAttachment => ash::vk::DescriptorType::INPUT_ATTACHMENT,
            DescriptorType::InlineUniformBlock => ash::vk::DescriptorType::INLINE_UNIFORM_BLOCK,
            DescriptorType::AccelerationStructureKhr(_) => {
                ash::vk::DescriptorType::ACCELERATION_STRUCTURE_KHR
            }
            DescriptorType::AccelerationStructureNv(_) => {
                ash::vk::DescriptorType::ACCELERATION_STRUCTURE_NV
            }
            DescriptorType::MutableValve => ash::vk::DescriptorType::MUTABLE_VALVE,
        }
    }
}

#[derive(Default)]
pub struct DescriptorSetLayoutBinding {
    binding: u32,
    descriptor_type: DescriptorType,
    descriptor_count: u32,
    stage_flags: ash::vk::ShaderStageFlags,
}

impl DescriptorSetLayoutBinding {
    pub fn builder() -> DescriptorSetLayoutBindingBuilder {
        DescriptorSetLayoutBindingBuilder {
            inner: Default::default(),
        }
    }
    fn ash_builder<'a>(
        &self,
        ash_vk_sampler_caches: &'a mut Vec<Vec<ash::vk::Sampler>>,
    ) -> ash::vk::DescriptorSetLayoutBindingBuilder<'a> {
        let mut builder = ash::vk::DescriptorSetLayoutBinding::builder()
            .binding(self.binding)
            .descriptor_type(self.descriptor_type.to_ash())
            .descriptor_count(self.descriptor_count)
            .stage_flags(self.stage_flags);
        match &self.descriptor_type {
            DescriptorType::SamplerImmutable(vec) => {
                let ash_vk_samplers = vec.iter().map(|sampler| sampler.ash_vk_sampler).collect();
                ash_vk_sampler_caches.push(ash_vk_samplers);
                let ash_vk_samplers = ash_vk_sampler_caches.last().unwrap();
                builder = builder.immutable_samplers(ash_vk_samplers.as_slice())
            }
            DescriptorType::CombinedImageSamplerImmutable(vec) => {
                let ash_vk_samplers = vec.iter().map(|sampler| sampler.ash_vk_sampler).collect();
                ash_vk_sampler_caches.push(ash_vk_samplers);
                let ash_vk_samplers = ash_vk_sampler_caches.last().unwrap();
                builder = builder.immutable_samplers(ash_vk_samplers.as_slice())
            }
            _ => {}
        }
        builder
    }
    pub fn binding(&self) -> u32 {
        self.binding
    }
    pub fn descriptor_type(&self) -> &DescriptorType {
        &self.descriptor_type
    }
    pub fn descriptor_count(&self) -> u32 {
        self.descriptor_count
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
    pub fn descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
        match &descriptor_type {
            DescriptorType::SamplerImmutable(samplers) => {
                self.inner.descriptor_count = samplers.len() as _;
            }
            DescriptorType::CombinedImageSamplerImmutable(samplers) => {
                self.inner.descriptor_count = samplers.len() as _;
            }
            _ => {
                self.inner.descriptor_count = 1;
            }
        }
        self.inner.descriptor_type = descriptor_type;
        self
    }
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
    pub fn add_stage_flag(mut self, stage_flag: ShaderStage) -> Self {
        self.inner.stage_flags |= stage_flag.to_ash();
        self
    }
    pub fn build(self) -> DescriptorSetLayoutBinding {
        self.inner
    }
}

pub struct DescriptorSetLayout {
    pub device: Arc<Device>,
    pub(crate) ash_vk_descriptor_set_layout: ash::vk::DescriptorSetLayout,
    pub bindings: FxHashMap<u32, DescriptorSetLayoutBinding>,
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
        let mut ash_cache = Vec::new();
        let bindings = self
            .bindings
            .iter()
            .map(|(_, binding)| binding.ash_builder(&mut ash_cache).build())
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
