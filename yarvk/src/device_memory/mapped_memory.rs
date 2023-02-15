use crate::device_memory::DeviceMemory;
use ash::vk::DeviceSize;
use std::ffi::c_void;
use std::ops::{Deref, DerefMut};

pub struct MappedMemory {
    pub device_memory: DeviceMemory,
    size: DeviceSize,
    ptr: *mut c_void,
}

unsafe impl Sync for MappedMemory {}
unsafe impl Send for MappedMemory {}

impl Deref for MappedMemory {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr as _, self.size as _) }
    }
}

impl DerefMut for MappedMemory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr as _, self.size as _) }
    }
}

impl MappedMemory {
    pub fn unmap_memory(self) -> DeviceMemory {
        unsafe {
            self.device_memory
                .device
                .ash_device
                .unmap_memory(self.device_memory.vk_device_memory)
        }
        self.device_memory
    }
}

impl DeviceMemory {
    // DONE VUID-vkMapMemory-memory-00678
    pub fn map_memory(
        self,
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
                // Host Synchronization
                // Host access to memory must be externally synchronized
                self.device.ash_device.map_memory(
                    self.vk_device_memory,
                    real_offset,
                    real_size,
                    ash::vk::MemoryMapFlags::empty(),
                )?
            }
        };
        Ok(MappedMemory {
            device_memory: self,
            size,
            ptr: unsafe { ptr.add((offset - real_offset) as _) },
        })
    }
}
