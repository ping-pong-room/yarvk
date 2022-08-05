use crate::command::command_buffer::CommandBuffer;
use crate::command::command_buffer::Level::PRIMARY;
use crate::command::command_buffer::RenderPassScope::{INSIDE, OUTSIDE};
use crate::command::command_buffer::State::RECORDING;
use crate::frame_buffer::Framebuffer;
use crate::render_pass::RenderPass;
use ash::vk::{ClearValue, Rect2D};
use std::sync::Arc;

pub struct RenderPassBeginInfoBuilder {
    inner: RenderPassBeginInfo,
}

impl RenderPassBeginInfoBuilder {
    pub fn render_area(mut self, render_area: Rect2D) -> Self {
        self.inner.render_area = render_area;
        self
    }
    pub fn add_clear_value(mut self, clear_value: ClearValue) -> Self {
        self.inner.clear_values.push(clear_value);
        self
    }
    pub fn build(self) -> RenderPassBeginInfo {
        self.inner
    }
}

pub struct RenderPassBeginInfo {
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    render_area: Rect2D,
    // DONE VUID-VkRenderPassBeginInfo-clearValueCount-04962
    clear_values: Vec<ClearValue>,
}

impl RenderPassBeginInfo {
    pub fn builder(
        render_pass: Arc<RenderPass>,
        framebuffer: Arc<Framebuffer>,
    ) -> RenderPassBeginInfoBuilder {
        RenderPassBeginInfoBuilder {
            inner: RenderPassBeginInfo {
                render_pass,
                framebuffer,
                render_area: Default::default(),
                clear_values: vec![],
            },
        }
    }
    pub(crate) fn ash_builder(&self) -> ash::vk::RenderPassBeginInfoBuilder {
        ash::vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass.ash_vk_renderpass)
            .framebuffer(self.framebuffer.ash_vk_framebuffer)
            .render_area(self.render_area)
            .clear_values(self.clear_values.as_slice())
    }
}

impl CommandBuffer<{ PRIMARY }, { RECORDING }, { OUTSIDE }> {
    // DONE VUID-vkCmdBeginRenderPass-commandBuffer-recording
    // DONE VUID-vkCmdEndRenderPass-commandBuffer-recording
    pub fn cmd_begin_render_pass<F>(
        &mut self,
        create_info: &RenderPassBeginInfo,
        contents: ash::vk::SubpassContents,
        f: F,
    ) where
        F: FnOnce(&mut CommandBuffer<{ PRIMARY }, { RECORDING }, { INSIDE }>),
    {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_begin_render_pass(
                self.vk_command_buffer,
                &create_info.ash_builder().build(),
                contents,
            );
        }
        f(unsafe {
            &mut *(self as *mut Self as *mut CommandBuffer<{ PRIMARY }, { RECORDING }, { INSIDE }>)
        });
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device
                .ash_device
                .cmd_end_render_pass(self.vk_command_buffer);
        }
    }
}
