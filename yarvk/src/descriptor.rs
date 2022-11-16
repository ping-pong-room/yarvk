use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::descriptor::descriptor_pool::DescriptorSet;
use crate::pipeline::PipelineLayout;
use std::sync::Arc;
use ash::vk::Handle;

pub mod descriptor_pool;
pub mod descriptor_set_layout;
pub mod write_descriptor_sets;

impl<const LEVEL: Level, const SCOPE: RenderPassScope, const ONE_TIME_SUBMIT: bool>
    CommandBuffer<LEVEL, { RECORDING }, SCOPE, ONE_TIME_SUBMIT>
{
    // DONE VUID-vkCmdBindDescriptorSets-commandBuffer-recording
    pub fn cmd_bind_descriptor_sets<It: IntoIterator<Item = Arc<DescriptorSet>>>(
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
                let ash_vk_descriptor_set = ds.ash_vk_descriptor_set;
                self.holding_resources
                    .descriptor_sets
                    .insert(ds.ash_vk_descriptor_set.as_raw(), ds.clone());
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
