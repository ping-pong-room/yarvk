use crate::command::command_buffer::CommandBuffer;
use crate::command::command_buffer::Level::PRIMARY;
use crate::command::command_buffer::RenderPassScope::OUTSIDE;
use crate::command::command_buffer::State::{EXECUTABLE, INVALID};
use crate::device::Device;
use crate::fence::{SignalingFence, UnsignaledFence};

use crate::pipeline::pipeline_stage_flags::PipelineStageFlags;
use crate::semaphore::Semaphore;
use parking_lot::RwLockReadGuard;
use std::cell::Cell;
use std::sync::Arc;

#[derive(Default)]
pub struct SubmitInfo {
    wait_semaphores: Vec<(Arc<Semaphore>, PipelineStageFlags)>,
    signal_semaphores: Vec<Arc<Semaphore>>,
    command_buffers: Vec<CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>>,
    onetime_submit_command_buffers: Vec<CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>>,
    invalid_buffers: Vec<CommandBuffer<{ PRIMARY }, { INVALID }, { OUTSIDE }>>,
}

impl SubmitInfo {
    pub fn new() -> SubmitInfo {
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

    pub fn clear(&mut self) {
        self.wait_semaphores.clear();
        self.signal_semaphores.clear();
        self.command_buffers.clear();
        self.onetime_submit_command_buffers.clear();
        self.invalid_buffers.clear();
    }
    pub fn add_wait_semaphore(
        &mut self,
        wait_semaphore: Arc<Semaphore>,
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
    pub fn add_signal_semaphore(&mut self, signal_semaphore: Arc<Semaphore>) {
        self.signal_semaphores.push(signal_semaphore);
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
    thread_local! {
        static SUBMIT_CACHES: Cell<(
            Vec<RwLockReadGuard<'static, ash::vk::Semaphore>>,
            Vec<ash::vk::SubmitInfo>,
            Vec<ash::vk::Semaphore>,
            Vec<ash::vk::PipelineStageFlags>,
            Vec<ash::vk::Semaphore>,
            Vec<ash::vk::CommandBuffer>,
        )> = Cell::new((Vec::new(), Vec::new(), Vec::new(), Vec::new(),Vec::new(), Vec::new()));
    }

    pub fn submit(
        &mut self,
        fence: UnsignaledFence,
        mut submit_infos: Vec<SubmitInfo>,
    ) -> Result<SignalingFence<Vec<SubmitInfo>>, ash::vk::Result> {
        // Host Synchronization: queue fence
        // DONE VUID-vkQueueSubmit-fence-00063
        // DONE VUID-vkQueueSubmit-fence-00064
        // DONE VUID-vkQueueSubmit-pWaitSemaphores-00068
        // DONE VUID-vkQueueSubmit-pSignalSemaphores-00067
        Self::SUBMIT_CACHES.with(move |local| {
            let (
                mut semaphore_locks,
                mut vk_submit_infos,
                mut ash_vk_wait_semaphores,
                mut ash_vk_wait_dst_stage_masks,
                mut ash_vk_signal_semaphores,
                mut ash_vk_command_buffers,
            ) = local.take();
            for submit_info in &submit_infos {
                for (semaphore, masks) in &submit_info.wait_semaphores {
                    let lock = semaphore.ash_vk_semaphore.read();
                    ash_vk_wait_semaphores.push(*lock);
                    ash_vk_wait_dst_stage_masks.push(masks.to_ash());
                    semaphore_locks.push(lock);
                }
                for buffer in &submit_info.onetime_submit_command_buffers {
                    ash_vk_command_buffers.push(buffer.vk_command_buffer);
                }
                for buffer in &submit_info.command_buffers {
                    ash_vk_command_buffers.push(buffer.vk_command_buffer);
                }
                for semaphore in &submit_info.signal_semaphores {
                    let lock = semaphore.ash_vk_semaphore.read();
                    ash_vk_signal_semaphores.push(*lock);
                    semaphore_locks.push(lock);
                }
                let submit_info = ash::vk::SubmitInfo::builder()
                    .wait_semaphores(ash_vk_wait_semaphores.as_slice())
                    .wait_dst_stage_mask(ash_vk_wait_dst_stage_masks.as_slice())
                    .command_buffers(ash_vk_command_buffers.as_slice())
                    .signal_semaphores(ash_vk_signal_semaphores.as_slice())
                    .build();
                vk_submit_infos.push(submit_info);
            }

            unsafe {
                // Host Synchronization: queue fence
                self.device.ash_device.queue_submit(
                    self.vk_queue,
                    vk_submit_infos.as_slice(),
                    fence.vk_fence,
                )?
            };
            semaphore_locks.clear();
            vk_submit_infos.clear();
            ash_vk_wait_semaphores.clear();
            ash_vk_wait_dst_stage_masks.clear();
            ash_vk_signal_semaphores.clear();
            ash_vk_command_buffers.clear();
            local.set(unsafe {(
                std::mem::transmute(semaphore_locks),
                std::mem::transmute(vk_submit_infos),
                std::mem::transmute(ash_vk_wait_semaphores),
                std::mem::transmute (ash_vk_wait_dst_stage_masks),
                std::mem::transmute (ash_vk_signal_semaphores),
                (ash_vk_command_buffers),
            )});
            // move all onetime submit buffers to invalid_buffers vector.
            for mut submit_info in &mut submit_infos {
                let onetime_submit_command_buffers =
                    std::mem::take(&mut submit_info.onetime_submit_command_buffers);
                let onetime_submit_command_buffers: Vec<
                    CommandBuffer<{ PRIMARY }, { INVALID }, { OUTSIDE }>,
                > = unsafe { std::mem::transmute(onetime_submit_command_buffers) };
                submit_info.invalid_buffers = onetime_submit_command_buffers;
            }

            Ok(SignalingFence {
                inner: fence.0,
                t: submit_infos,
            })
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
