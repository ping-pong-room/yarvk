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
    pub render_pass: Arc<RenderPass>,
    pub framebuffer: Arc<Framebuffer>,
    pub render_area: Rect2D,
    // DONE VUID-VkRenderPassBeginInfo-clearValueCount-04962
    pub clear_values: Vec<ClearValue>,
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
    pub fn cmd_begin_render_pass(
        mut self,
        create_info: Arc<RenderPassBeginInfo>,
        contents: ash::vk::SubpassContents,
    ) -> CommandBuffer<{ PRIMARY }, { RECORDING }, { INSIDE }> {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_begin_render_pass(
                self.vk_command_buffer,
                &create_info.ash_builder().build(),
                contents,
            );
        }
        self.holding_resources.render_pass = Some(create_info.render_pass.clone());
        self.holding_resources.framebuffers = Some(create_info.framebuffer.clone());
        unsafe { std::mem::transmute(self) }
    }
    pub fn cmd_during_render_pass<F>(
        &mut self,
        create_info: Arc<RenderPassBeginInfo>,
        contents: ash::vk::SubpassContents,
        f: F,
    ) -> Result<(), ash::vk::Result>
    where
        F: FnOnce(
            &mut CommandBuffer<{ PRIMARY }, { RECORDING }, { INSIDE }>,
        ) -> Result<(), ash::vk::Result>,
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
        })?;
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device
                .ash_device
                .cmd_end_render_pass(self.vk_command_buffer);
        }
        self.holding_resources.render_pass = None;
        self.holding_resources.framebuffers = None;
        Ok(())
    }
}

impl CommandBuffer<{ PRIMARY }, { RECORDING }, { INSIDE }> {
    pub fn cmd_next_subpass(&mut self, contents: ash::vk::SubpassContents) {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device
                .ash_device
                .cmd_next_subpass(self.vk_command_buffer, contents)
        }
    }
    pub fn cmd_end_render_pass(mut self) -> CommandBuffer<{ PRIMARY }, { RECORDING }, { OUTSIDE }> {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device
                .ash_device
                .cmd_end_render_pass(self.vk_command_buffer);
        }
        self.holding_resources.render_pass = None;
        self.holding_resources.framebuffers = None;
        unsafe { std::mem::transmute(self) }
    }
}
