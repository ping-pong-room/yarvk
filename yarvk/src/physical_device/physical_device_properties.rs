use crate::physical_device::PhysicalDevice;
use ash::vk::ExtendsPhysicalDeviceProperties2;

impl PhysicalDevice {
    pub fn get_physical_device_properties(&self) -> &ash::vk::PhysicalDeviceProperties {
        &self.physical_device_properties
    }

    pub fn get_physical_device_properties2<T: ExtendsPhysicalDeviceProperties2 + Default>(
        &self,
    ) -> T {
        let mut t = T::default();
        let mut prop2 = ash::vk::PhysicalDeviceProperties2::builder()
            .push_next(&mut t)
            .build();
        unsafe {
            // Host Synchronization: none
            self.instance
                .ash_instance
                .get_physical_device_properties2(self.vk_physical_device, &mut prop2);
        }
        t
    }
}
