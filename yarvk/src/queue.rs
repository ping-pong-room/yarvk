use std::sync::Arc;
use crate::device::Device;

pub mod submit_info;

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
    pub fn wait_idle(&mut self) -> Result<(), ash::vk::Result> {
        unsafe {
            // Host Synchronization: queue
            self.device.ash_device.queue_wait_idle(self.vk_queue)?;
        }
        Ok(())
    }
}
