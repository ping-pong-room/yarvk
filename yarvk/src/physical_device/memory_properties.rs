use crate::physical_device::PhysicalDevice;
use crate::Handler;
use std::sync::Arc;

#[derive(Clone)]
pub struct MemoryType {
    pub physical_device: Arc<PhysicalDevice>,
    pub(crate) index: u32,
    pub property_flags: ash::vk::MemoryPropertyFlags,
    pub heap: ash::vk::MemoryHeap,
}

impl Handler for MemoryType {
    fn handler(&self) -> u64 {
        self.index as u64
    }
}

pub struct PhysicalDeviceMemoryProperties {
    pub physical_device: Arc<PhysicalDevice>,
    pub memory_types: Vec<MemoryType>,
}

impl PhysicalDevice {
    pub fn memory_properties(self: &Arc<Self>) -> PhysicalDeviceMemoryProperties {
        let vk_physical_device_memory_properties = unsafe {
            // Host Synchronization: none
            self.instance
                .ash_instance
                .get_physical_device_memory_properties(self.vk_physical_device)
        };
        let mut memory_properties =
            Vec::with_capacity(vk_physical_device_memory_properties.memory_type_count as _);
        vk_physical_device_memory_properties.memory_types
            [..vk_physical_device_memory_properties.memory_type_count as _]
            .iter()
            .enumerate()
            .for_each(|(index, vk_memory_type)| {
                memory_properties.push(MemoryType {
                    physical_device: self.clone(),
                    index: index as u32,
                    property_flags: vk_memory_type.property_flags,
                    heap: vk_physical_device_memory_properties.memory_heaps
                        [vk_memory_type.heap_index as usize],
                })
            });
        PhysicalDeviceMemoryProperties {
            physical_device: self.clone(),
            memory_types: memory_properties,
        }
    }

    pub fn memory_properties2<T: ash::vk::ExtendsPhysicalDeviceMemoryProperties2 + Default>(
        &self,
    ) -> T {
        let mut t = T::default();
        let mut prop2 = ash::vk::PhysicalDeviceMemoryProperties2::builder()
            .push_next(&mut t)
            .build();
        unsafe {
            // Host Synchronization: none
            self.instance
                .ash_instance
                .get_physical_device_memory_properties2(self.vk_physical_device, &mut prop2);
        };
        t
    }
}
