use crate::device::Device;
use std::sync::Arc;

pub struct Semaphore {
    pub device: Arc<Device>,
    pub(crate) ash_vk_semaphore: ash::vk::Semaphore,
}

impl Semaphore {
    pub fn new(device: &Arc<Device>) -> Result<Semaphore, ash::vk::Result> {
        let create_info = ash::vk::SemaphoreCreateInfo::builder().build();
        let ash_vk_semaphore = unsafe {
            // Host Synchronization: none
            device.ash_device.create_semaphore(&create_info, None)?
        };
        Ok(Semaphore {
            device: device.clone(),
            ash_vk_semaphore,
        })
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            // TODO VUID-vkDestroySemaphore-semaphore-01137
            // Host Synchronization: semaphore
            self.device
                .ash_device
                .destroy_semaphore(self.ash_vk_semaphore, None);
        }
    }
}
