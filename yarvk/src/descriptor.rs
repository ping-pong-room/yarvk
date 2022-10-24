use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::descriptor::descriptor_set::DescriptorSet;
use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceAccelerationStructureFeaturesKHR::AccelerationStructure;
use crate::extensions::DeviceExtension;
use crate::extensions::PhysicalDeviceExtensionType::NvRayTracing;
use crate::pipeline::PipelineLayout;
use crate::sampler::Sampler;
use std::sync::Arc;

pub mod descriptor_pool;
pub mod descriptor_set;
pub mod descriptor_set_layout;
pub mod write_descriptor_sets;

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
    fn to_ash(&self) -> ash::vk::DescriptorType {
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

impl<const LEVEL: Level, const SCOPE: RenderPassScope, const ONE_TIME_SUBMIT: bool>
    CommandBuffer<LEVEL, { RECORDING }, SCOPE, ONE_TIME_SUBMIT>
{
    // DONE VUID-vkCmdBindDescriptorSets-commandBuffer-recording
    pub fn cmd_bind_descriptor_sets(
        &mut self,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        layout: &PipelineLayout,
        first_set: u32,
        descriptor_sets: &[&DescriptorSet],
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
