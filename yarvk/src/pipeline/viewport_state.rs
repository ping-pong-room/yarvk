use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceFeatures::MultiViewport;

pub struct PipelineViewportStateCreateInfo {
    viewports: Vec<ash::vk::Viewport>,
    dynamic_viewports: u32,
    scissors: Vec<ash::vk::Rect2D>,
    dynamic_scissors: u32,
}

impl Default for PipelineViewportStateCreateInfo {
    fn default() -> Self {
        PipelineViewportStateCreateInfo {
            viewports: Default::default(),
            dynamic_viewports: 1,
            scissors: Default::default(),
            dynamic_scissors: 1
        }
    }
}

impl PipelineViewportStateCreateInfo {
    pub fn builder() -> PipelineViewportStateCreateInfoBuilder {
        PipelineViewportStateCreateInfoBuilder {
            inner: PipelineViewportStateCreateInfo::default(),
        }
    }
    pub(crate) fn ash_builder(&self) -> ash::vk::PipelineViewportStateCreateInfoBuilder {
        let mut ash_vk_viewport_state = ash::vk::PipelineViewportStateCreateInfo::builder();
        if !self.viewports.is_empty() {
            ash_vk_viewport_state = ash_vk_viewport_state.viewports(self.viewports.as_slice());
        } else {
            ash_vk_viewport_state = ash_vk_viewport_state.viewport_count(self.dynamic_viewports);
        }
        if !self.scissors.is_empty() {
            ash_vk_viewport_state = ash_vk_viewport_state.scissors(self.scissors.as_slice());
        } else {
            ash_vk_viewport_state = ash_vk_viewport_state.scissor_count(self.dynamic_scissors);
        }
        ash_vk_viewport_state
    }
}

pub struct PipelineViewportStateCreateInfoBuilder {
    inner: PipelineViewportStateCreateInfo,
}

impl PipelineViewportStateCreateInfoBuilder {
    pub fn viewport(mut self, viewport: ash::vk::Viewport) -> Self {
        if self.inner.viewports.is_empty() {
            self.inner.viewports.push(viewport);
        } else {
            unsafe { *self.inner.viewports.get_unchecked_mut(0) = viewport };
        }
        self
    }
    // DONE VUID-VkPipelineViewportStateCreateInfo-viewportCount-01216
    pub fn add_viewport(
        mut self,
        viewport: ash::vk::Viewport,
        _feature: Feature<{ MultiViewport.into() }>,
    ) -> Self {
        self.inner.viewports.push(viewport);
        self
    }
    pub fn dynamic_viewports(
        mut self,
        counts: u32,
        _feature: Feature<{ MultiViewport.into() }>,
    ) -> Self {
        self.inner.dynamic_viewports = counts;
        self
    }
    pub fn scissor(mut self, scissor: ash::vk::Rect2D) -> Self {
        if self.inner.scissors.is_empty() {
            self.inner.scissors.push(scissor);
        } else {
            unsafe { *self.inner.scissors.get_unchecked_mut(0) = scissor };
        }
        self
    }
    // DONE VUID-VkPipelineViewportStateCreateInfo-scissorCount-01217
    pub fn add_scissor(
        mut self,
        scissor: ash::vk::Rect2D,
        _feature: Feature<{ MultiViewport.into() }>,
    ) -> Self {
        self.inner.scissors.push(scissor);
        self
    }
    pub fn dynamic_scissors(
        mut self,
        counts: u32,
        _feature: Feature<{ MultiViewport.into() }>,
    ) -> Self {
        self.inner.dynamic_scissors = counts;
        self
    }
    pub fn build(self) -> PipelineViewportStateCreateInfo {
        self.inner
    }
}

impl<const LEVEL: Level, const SCOPE: RenderPassScope>
    CommandBuffer<LEVEL, { RECORDING }, SCOPE>
{
    // DONE VUID-vkCmdSetViewport-commandBuffer-recording
    pub fn cmd_set_viewport(&mut self, viewports: &ash::vk::Viewport) {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device
                .ash_device
                .cmd_set_viewport(self.vk_command_buffer, 0, &[*viewports]);
        }
    }

    pub fn cmd_set_viewports(
        &mut self,
        first_viewport: u32,
        viewports: &[ash::vk::Viewport],
        _feature: Feature<{ MultiViewport.into() }>,
    ) {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_set_viewport(
                self.vk_command_buffer,
                first_viewport,
                viewports,
            );
        }
    }

    // DONE VUID-vkCmdSetScissor-commandBuffer-recording
    pub fn cmd_set_scissor(&mut self, scissor: &ash::vk::Rect2D) {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device
                .ash_device
                .cmd_set_scissor(self.vk_command_buffer, 0, &[*scissor]);
        }
    }
    // DONE VUID-vkCmdSetScissor-firstScissor-00593
    // DONE VUID-vkCmdSetScissor-scissorCount-00594
    pub fn cmd_set_scissors(
        &mut self,
        first_scissor: u32,
        scissors: &[ash::vk::Rect2D],
        _feature: Feature<{ MultiViewport.into() }>,
    ) {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device
                .ash_device
                .cmd_set_scissor(self.vk_command_buffer, first_scissor, scissors);
        }
    }
}
