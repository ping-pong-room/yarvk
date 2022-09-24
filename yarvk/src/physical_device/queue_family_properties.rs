use crate::physical_device::PhysicalDevice;
use std::hash::Hasher;
use std::sync::Arc;

#[derive(Clone)]
pub struct QueueFamilyProperties {
    pub physical_device: Arc<PhysicalDevice>,
    // Done VUID-vkGetPhysicalDeviceSurfaceSupportKHR-queueFamilyIndex-01269
    pub(crate) index: u32,
    pub queue_flags: ash::vk::QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: ash::vk::Extent3D,
}

impl PartialEq for QueueFamilyProperties {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for QueueFamilyProperties {}

impl std::hash::Hash for QueueFamilyProperties {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl PhysicalDevice {
    pub fn get_physical_device_queue_family_properties(
        self: &Arc<Self>,
    ) -> Vec<QueueFamilyProperties> {
        // Done VUID-vkGetPhysicalDeviceQueueFamilyProperties-physicalDevice-parameter
        // Done VUID-vkGetPhysicalDeviceQueueFamilyProperties-pQueueFamilyPropertyCount-parameter
        // Done VUID-vkGetPhysicalDeviceQueueFamilyProperties-pQueueFamilyProperties-parameter
        unsafe {
            // Host Synchronization: none
            self.instance
                .ash_instance
                .get_physical_device_queue_family_properties(self.vk_physical_device)
        }
        .into_iter()
        .enumerate()
        .map(|(index, property)| QueueFamilyProperties {
            physical_device: self.clone(),
            index: index as u32,
            queue_flags: property.queue_flags,
            queue_count: property.queue_count,
            timestamp_valid_bits: property.timestamp_valid_bits,
            min_image_transfer_granularity: property.min_image_transfer_granularity,
        })
        .collect()
    }
    // TODO get_physical_device_queue_family_properties2
}
