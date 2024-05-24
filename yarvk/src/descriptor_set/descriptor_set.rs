use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::descriptor_set::desccriptor_pool::DescriptorPool;
use crate::descriptor_set::private::{PrivateConstDescriptorSetValue, PrivateDescriptorSetValue};
use crate::device::Device;
use crate::pipeline::PipelineLayout;
use ash::vk::Handle;
use std::sync::Arc;

pub trait DescriptorSetValue: PrivateDescriptorSetValue + Send + Sync {
    type ConstDescriptorSetValue: PrivateConstDescriptorSetValue<Self>;
}

pub trait IDescriptorSet: Sync + Send {
    // TODO make this private
    fn raw(&self) -> ash::vk::DescriptorSet;
}

pub struct DescriptorSet<T: DescriptorSetValue> {
    pub device: Arc<Device>,
    pub(crate) ash_vk_descriptor_set: ash::vk::DescriptorSet,
    pub(crate) value: Option<T>,
    pub(crate) _descriptor_pool: Arc<DescriptorPool<T>>,
}

impl<T: DescriptorSetValue> IDescriptorSet for DescriptorSet<T> {
    fn raw(&self) -> ash::vk::DescriptorSet {
        self.ash_vk_descriptor_set
    }
}

impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { RECORDING }, SCOPE> {
    // DONE VUID-vkCmdBindDescriptorSets-commandBuffer-recording
    pub fn cmd_bind_descriptor_sets<It: IntoIterator<Item = Arc<dyn IDescriptorSet>>>(
        &mut self,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        layout: Arc<PipelineLayout>,
        first_set: u32,
        descriptor_sets: It,
        dynamic_offsets: &[u32],
    ) {
        self.holding_resources
            .pipeline_layouts
            .insert(layout.ash_vk_pipeline_layout.as_raw(), layout.clone());
        let vk_descriptor_sets: Vec<_> = descriptor_sets
            .into_iter()
            .map(|ds| {
                let ash_vk_descriptor_set = ds.raw();
                self.holding_resources
                    .descriptor_sets
                    .insert(ash_vk_descriptor_set.as_raw(), ds.clone());
                ash_vk_descriptor_set
            })
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
