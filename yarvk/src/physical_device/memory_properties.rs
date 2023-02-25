use crate::physical_device::PhysicalDevice;
use crate::Handle;
use ash::vk::MemoryPropertyFlags;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct MemoryType {
    pub index: u32,
    pub property_flags: ash::vk::MemoryPropertyFlags,
    pub heap_index: u32,
    pub heap_size: u64,
}

impl Handle for MemoryType {
    fn handle(&self) -> u64 {
        self.index as u64
    }
}

pub struct MemoryHeap {
    pub size: ash::vk::DeviceSize,
    pub flags: ash::vk::MemoryHeapFlags,
    pub memory_types: Vec<MemoryType>,
}

pub struct PhysicalDeviceMemoryProperties {
    heaps: BTreeMap<u32, MemoryHeap>,
    memory_type_in_order: Vec<MemoryType>,
}

impl PhysicalDeviceMemoryProperties {
    pub fn memory_type_in_order(&self) -> &[MemoryType] {
        self.memory_type_in_order.as_slice()
    }
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
        let mut heaps = BTreeMap::new();
        let mut rank = BTreeMap::new();
        vk_physical_device_memory_properties.memory_types
            [..vk_physical_device_memory_properties.memory_type_count as _]
            .iter()
            .enumerate()
            .for_each(|(index, vk_memory_type)| {
                let memory_type = MemoryType {
                    index: index as _,
                    property_flags: vk_memory_type.property_flags,
                    heap_index: vk_memory_type.heap_index,
                    heap_size: vk_physical_device_memory_properties.memory_heaps
                        [vk_memory_type.heap_index as usize]
                        .size,
                };
                rank.insert(
                    vk_memory_type.property_flags.as_raw() & 0b1111,
                    memory_type.clone(),
                );
                match heaps.entry(vk_memory_type.heap_index) {
                    Entry::Vacant(entry) => {
                        let vk_memory_heap =
                            vk_physical_device_memory_properties.memory_heaps[index];
                        let memory_heap = MemoryHeap {
                            size: vk_memory_heap.size,
                            flags: vk_memory_heap.flags,
                            memory_types: vec![memory_type],
                        };
                        entry.insert(memory_heap);
                    }
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().memory_types.push(memory_type);
                    }
                }
            });
        let mut memory_type_in_order = Vec::with_capacity(rank.len());
        while let Some((_, memory_type)) = rank.pop_last() {
            memory_type_in_order.push(memory_type);
        }
        PhysicalDeviceMemoryProperties {
            heaps,
            memory_type_in_order,
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
