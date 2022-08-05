use crate::command::command_buffer::CommandBuffer;
use crate::command::command_buffer::Level::PRIMARY;
use crate::command::command_buffer::RenderPassScope::OUTSIDE;
use crate::command::command_buffer::State::{EXECUTABLE, INVALID};
use crate::device::Device;
use crate::fence::{SignalingFence, UnsignaledFence};

use crate::pipeline::pipeline_stage_flags::PipelineStageFlags;
use crate::semaphore::Semaphore;
use std::sync::Arc;

#[derive(Default)]
pub struct SubmitInfo<'a> {
    wait_semaphores: Vec<(&'a Semaphore, PipelineStageFlags)>,
    signal_semaphores: Vec<&'a Semaphore>,
    command_buffers: Vec<CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>>,
    onetime_submit_command_buffers: Vec<CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>>,
    invalid_buffers: Vec<CommandBuffer<{ PRIMARY }, { INVALID }, { OUTSIDE }>>,

    ash_vk_wait_semaphores: Vec<ash::vk::Semaphore>,
    ash_vk_wait_dst_stage_masks: Vec<ash::vk::PipelineStageFlags>,
    ash_vk_signal_semaphores: Vec<ash::vk::Semaphore>,
    ash_vk_command_buffers: Vec<ash::vk::CommandBuffer>,
}

impl<'a> SubmitInfo<'a> {
    pub fn new() -> SubmitInfo<'a> {
        SubmitInfo::default()
    }

    pub fn take_invalid_buffers(
        &mut self,
    ) -> Vec<CommandBuffer<{ PRIMARY }, { INVALID }, { OUTSIDE }>> {
        std::mem::take(&mut self.invalid_buffers)
    }

    pub fn take_executable_buffers(
        &mut self,
    ) -> Vec<CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>> {
        std::mem::take(&mut self.command_buffers)
    }

    pub fn clear<'b>(mut self) {
        self.wait_semaphores.clear();
        self.signal_semaphores.clear();
        self.command_buffers.clear();
        self.onetime_submit_command_buffers.clear();
        self.invalid_buffers.clear();

        self.ash_vk_wait_semaphores.clear();
        self.ash_vk_wait_dst_stage_masks.clear();
        self.ash_vk_signal_semaphores.clear();
        self.ash_vk_command_buffers.clear();
    }
    pub fn add_wait_semaphore(
        &mut self,
        wait_semaphore: &'a Semaphore,
        wait_mask: PipelineStageFlags,
    ) {
        self.wait_semaphores.push((wait_semaphore, wait_mask));
    }
    pub fn add_command_buffer(
        &mut self,
        command_buffer: CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>,
    ) {
        // DONE VUID-VkSubmitInfo-pCommandBuffers-00075
        if command_buffer.one_time_submit {
            self.onetime_submit_command_buffers.push(command_buffer);
        } else {
            self.command_buffers.push(command_buffer);
        }
    }
    pub fn add_signal_semaphore(&mut self, signal_semaphore: &'a Semaphore) {
        self.signal_semaphores.push(signal_semaphore);
    }

    pub(crate) fn ash_builder(&mut self) -> ash::vk::SubmitInfoBuilder {
        for (semaphore, masks) in &self.wait_semaphores {
            self.ash_vk_wait_semaphores.push(semaphore.ash_vk_semaphore);
            self.ash_vk_wait_dst_stage_masks.push(masks.to_ash());
        }
        for buffer in &self.onetime_submit_command_buffers {
            self.ash_vk_command_buffers.push(buffer.vk_command_buffer);
        }
        for buffer in &self.command_buffers {
            self.ash_vk_command_buffers.push(buffer.vk_command_buffer);
        }
        for semaphore in &self.signal_semaphores {
            self.ash_vk_signal_semaphores.push(semaphore.ash_vk_semaphore);
        }
        ash::vk::SubmitInfo::builder()
            .wait_semaphores(self.ash_vk_wait_semaphores.as_slice())
            .wait_dst_stage_mask(self.ash_vk_wait_dst_stage_masks.as_slice())
            .command_buffers(self.ash_vk_command_buffers.as_slice())
            .signal_semaphores(self.ash_vk_signal_semaphores.as_slice())
    }
}

// fn is_stage_supported(stage_flags: ash::vk::PipelineStageFlags, queue_flags: ash::vk::QueueFlags) -> bool {
//     if (stage_flags.contains(ash::vk::PipelineStageFlags::VERTEX_INPUT)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::VERTEX_SHADER)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::GEOMETRY_SHADER)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::FRAGMENT_SHADER)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::LATE_FRAGMENT_TESTS)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::ALL_GRAPHICS)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::TRANSFORM_FEEDBACK_EXT)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::TASK_SHADER_NV)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::MESH_SHADER_NV) || stage_flags.contains(ash::vk::PipelineStageFlags::FRAGMENT_DENSITY_PROCESS_EXT)) && !queue_flags.contains(ash::vk::QueueFlags::GRAPHICS) {
//         return false;
//     }
//
//     if (stage_flags.contains(ash::vk::PipelineStageFlags::COMPUTE_SHADER)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR) || stage_flags.contains(ash::vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR)) && !queue_flags.contains(ash::vk::QueueFlags::COMPUTE) {
//         return false;
//     }
//
//     if (stage_flags.contains(ash::vk::PipelineStageFlags::DRAW_INDIRECT)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::CONDITIONAL_RENDERING_EXT) || stage_flags.contains(ash::vk::PipelineStageFlags::COMMAND_PREPROCESS_NV)) && !queue_flags.contains(ash::vk::QueueFlags::GRAPHICS) && !queue_flags.contains(ash::vk::QueueFlags::COMPUTE) {
//         return false;
//     }
//
//     if (stage_flags.contains(ash::vk::PipelineStageFlags::TRANSFER)
//         || stage_flags.contains(ash::vk::PipelineStageFlags::CONDITIONAL_RENDERING_EXT) || stage_flags.contains(ash::vk::PipelineStageFlags::COMMAND_PREPROCESS_NV)) && !queue_flags.contains(ash::vk::QueueFlags::GRAPHICS)
//         && !queue_flags.contains(ash::vk::QueueFlags::COMPUTE) && !queue_flags.contains(ash::vk::QueueFlags::TRANSFER) {
//         return false;
//     }
//
//     true
// }

pub struct Queue {
    pub(crate) device: Arc<Device>,
    pub(crate) vk_queue: ash::vk::Queue,
    // pub(crate) queue_family_property: ash::vk::QueueFamilyProperties,
}

impl Queue {
    pub fn submit<'a>(
        &'a mut self,
        fence: UnsignaledFence,
        submit_infos: &'a mut [&'a mut SubmitInfo<'a>],
    ) -> Result<SignalingFence<&'a mut [&'a mut SubmitInfo<'a>]>, ash::vk::Result> {
        // Host Synchronization: queue fence
        // DONE VUID-vkQueueSubmit-fence-00063
        // DONE VUID-vkQueueSubmit-fence-00064
        // DONE VUID-vkQueueSubmit-pWaitSemaphores-00068
        // DONE VUID-vkQueueSubmit-pSignalSemaphores-00067

        let vk_submit_infos = submit_infos
            .iter_mut()
            .map(|info| info.ash_builder().build())
            .collect::<Vec<_>>();

        unsafe {
            // Host Synchronization: queue fence
            self.device.ash_device.queue_submit(
                self.vk_queue,
                vk_submit_infos.as_slice(),
                fence.vk_fence,
            )?
        };
        // move all onetime submit buffers to invalid_buffers vector.
        submit_infos.iter_mut().for_each(|submit_info|{
            let onetime_submit_command_buffers =
                std::mem::take(&mut submit_info.onetime_submit_command_buffers);
            let onetime_submit_command_buffers: Vec<
                CommandBuffer<{ PRIMARY }, { INVALID }, { OUTSIDE }>,
            > = unsafe { std::mem::transmute(onetime_submit_command_buffers) };
            submit_info.invalid_buffers = onetime_submit_command_buffers;
        });

        Ok(SignalingFence {
            inner: fence.0,
            t: submit_infos,
        })
    }

    pub fn wait_idle(&mut self) -> Result<(), ash::vk::Result> {
        unsafe {
            // Host Synchronization: queue
            self.device.ash_device.queue_wait_idle(self.vk_queue)?;
        }
        Ok(())
    }
}
