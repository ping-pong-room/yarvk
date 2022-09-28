use crate::physical_device::PhysicalDevice;
use crate::Handle;

#[derive(Clone)]
pub struct MemoryType {
    pub(crate) index: u32,
    pub property_flags: ash::vk::MemoryPropertyFlags,
    pub heap: ash::vk::MemoryHeap,
}

impl Handle for MemoryType {
    fn handle(&self) -> u64 {
        self.index as u64
    }
}

pub struct PhysicalDeviceMemoryProperties {
    pub memory_types: Vec<MemoryType>,
}

impl PhysicalDevice {
    pub(super) fn memory_properties_inner(
        instance: &ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
    ) -> PhysicalDeviceMemoryProperties {
        let vk_physical_device_memory_properties = unsafe {
            // Host Synchronization: none
            instance.get_physical_device_memory_properties(physical_device)
        };
        let mut memory_properties =
            Vec::with_capacity(vk_physical_device_memory_properties.memory_type_count as _);
        vk_physical_device_memory_properties.memory_types
            [..vk_physical_device_memory_properties.memory_type_count as _]
            .iter()
            .enumerate()
            .for_each(|(index, vk_memory_type)| {
                memory_properties.push(MemoryType {
                    index: index as u32,
                    property_flags: vk_memory_type.property_flags,
                    heap: vk_physical_device_memory_properties.memory_heaps
                        [vk_memory_type.heap_index as usize],
                })
            });
        PhysicalDeviceMemoryProperties {
            memory_types: memory_properties,
        }
    }

    pub fn memory_properties(&self) -> &PhysicalDeviceMemoryProperties {
        &self.memory_properties
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
