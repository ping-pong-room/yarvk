use crate::device::Device;
use std::sync::Arc;
use crate::device_memory::DeviceMemory;

pub struct MappedRanges<'a> {
    device: Arc<Device>,
    _device_memories: Vec<&'a DeviceMemory>,
    ash_vk_mapped_ranges: Vec<ash::vk::MappedMemoryRange>,
}

impl<'a> MappedRanges<'a> {
    pub fn new(device: &Arc<Device>) -> Self {
        Self {
            device: device.clone(),
            _device_memories: vec![],
            ash_vk_mapped_ranges: vec![],
        }
    }
    pub fn clear(&mut self) {
        self._device_memories.clear();
        self.ash_vk_mapped_ranges.clear();
    }
    pub fn add_range(
        &mut self,
        device_memory: &'a DeviceMemory,
        mut offset: ash::vk::DeviceSize,
        mut size: ash::vk::DeviceSize,
    ) {
        if device_memory
            .memory_type
            .property_flags
            .contains(ash::vk::MemoryPropertyFlags::HOST_COHERENT)
        {
            return;
        }
        let end = offset.overflowing_add(size);
        if end.1 || end.0 > device_memory.size {
            size = device_memory.size - offset;
        }
        let non_coherent_atom_size = self
            .device
            .physical_device
            .get_physical_device_properties()
            .limits
            .non_coherent_atom_size;
        // DONE VUID-VkMappedMemoryRange-offset-00687
        offset = offset - offset % non_coherent_atom_size;
        // DONE VUID-VkMappedMemoryRange-size-01390
        let edge = offset + size + non_coherent_atom_size - size % non_coherent_atom_size;
        let edge = std::cmp::min(edge, device_memory.size);
        size = edge - offset;

        self._device_memories.push(device_memory);
        self.ash_vk_mapped_ranges.push(
            ash::vk::MappedMemoryRange::builder()
                .memory(device_memory.vk_device_memory)
                .offset(offset)
                .size(size)
                .build(),
        );
    }
    pub fn flush(&self) -> Result<(), ash::vk::Result> {
        if self.ash_vk_mapped_ranges.is_empty() {
            return Ok(());
        }
        unsafe {
            self.device
                .ash_device
                .flush_mapped_memory_ranges(self.ash_vk_mapped_ranges.as_slice())
        }
    }
    pub fn invalidate(&self) -> Result<(), ash::vk::Result> {
        if self.ash_vk_mapped_ranges.is_empty() {
            return Ok(());
        }
        unsafe {
            self.device
                .ash_device
                .invalidate_mapped_memory_ranges(self.ash_vk_mapped_ranges.as_slice())
        }
    }
}
