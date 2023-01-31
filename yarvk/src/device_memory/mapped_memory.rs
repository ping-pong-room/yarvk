use crate::device_memory::DeviceMemory;
use ash::vk::DeviceSize;
use derive_more::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
pub struct MappedMemory<'a> {
    pub(crate) device_memory: &'a mut DeviceMemory,
    #[deref]
    #[deref_mut]
    data: &'a mut [u8],
}

impl<'a> Drop for MappedMemory<'a> {
    fn drop(&mut self) {
        unsafe {
            self.device_memory
                .device
                .ash_device
                .unmap_memory(self.device_memory.vk_device_memory)
        }
    }
}

impl<'a> MappedMemory<'a> {
    pub fn unmap_memory(self) {
        drop(self);
    }
}

impl DeviceMemory {
    // DONE VUID-vkMapMemory-memory-00678
    pub fn map_memory(
        &mut self,
        offset: DeviceSize,
        mut size: DeviceSize,
    ) -> Result<MappedMemory, ash::vk::Result> {
        let end = offset.overflowing_add(size);
        if end.1 || end.0 > self.size {
            size = self.size - offset;
        }
        let mut real_offset = offset;
        let mut real_size = size;
        if !self
            .memory_type
            .property_flags
            .contains(ash::vk::MemoryPropertyFlags::HOST_COHERENT)
        {
            // DONE VUID-VkMappedMemoryRange-size-01389
            let non_coherent_atom_size = self
                .device
                .physical_device
                .get_physical_device_properties()
                .limits
                .non_coherent_atom_size;
            let edge = offset + size + non_coherent_atom_size - (size % non_coherent_atom_size);
            let edge = std::cmp::min(edge, self.size);
            real_offset = offset - offset % non_coherent_atom_size;
            real_size = edge - real_offset;
        }
        let ptr = {
            unsafe {
                self.device.ash_device.map_memory(
                    self.vk_device_memory,
                    real_offset,
                    real_size,
                    ash::vk::MemoryMapFlags::empty(),
                )?
            }
        };
        let data = unsafe {
            std::slice::from_raw_parts_mut(ptr.add((offset - real_offset) as _) as _, size as _)
        };
        Ok(MappedMemory {
            device_memory: self,
            data,
        })
    }
}
