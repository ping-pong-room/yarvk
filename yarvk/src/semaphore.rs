use crate::device::Device;


use std::sync::Arc;
use parking_lot::RwLock;

pub struct Semaphore {
    pub device: Arc<Device>,
    pub(crate) ash_vk_semaphore: RwLock<ash::vk::Semaphore>,
}

impl Semaphore {
    pub fn new(device: Arc<Device>) -> Result<Arc<Semaphore>, ash::vk::Result> {
        let create_info = ash::vk::SemaphoreCreateInfo::builder().build();
        let ash_vk_semaphore = unsafe {
            // Host Synchronization: none
            device.ash_device.create_semaphore(&create_info, None)?
        };
        Ok(Arc::new(Semaphore {
            device,
            ash_vk_semaphore: RwLock::new(ash_vk_semaphore),
        }))
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            // TODO VUID-vkDestroySemaphore-semaphore-01137
            // Host Synchronization: semaphore
            self.device
                .ash_device
                .destroy_semaphore(*self.ash_vk_semaphore.get_mut(), None);
        }
    }
}
