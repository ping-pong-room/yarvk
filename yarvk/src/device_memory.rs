use crate::device::Device;
use crate::device_memory::dedicated_memory::MemoryDedicatedAllocateInfo;
use crate::physical_device::memory_properties::MemoryType;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub mod dedicated_memory;

pub struct DeviceMemory {
    pub device: Arc<Device>,
    pub(crate) vk_device_memory: ash::vk::DeviceMemory,
    pub size: ash::vk::DeviceSize,
}

impl Drop for DeviceMemory {
    fn drop(&mut self) {
        unsafe {
            // Host Synchronization: memory
            self.device
                .ash_device
                .free_memory(self.vk_device_memory, None);
            self.device.allocations.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

impl DeviceMemory {
    pub fn builder<'a>(memory_type: MemoryType, device: Arc<Device>) -> DeviceMemoryBuilder<'a> {
        DeviceMemoryBuilder {
            device,
            allocation_size: 0,
            memory_type,
            dedicated_allocate_info: None,
        }
    }
    pub fn map_memory<F: FnOnce(&mut [u8])>(
        &mut self,
        offset: ash::vk::DeviceSize,
        size: ash::vk::DeviceSize,
        f: F,
    ) -> Result<(), ash::vk::Result> {
        // DONE VUID-vkMapMemory-memory-00678
        // Host Synchronization: memory
        unsafe {
            let ptr = self.device.ash_device.map_memory(
                self.vk_device_memory,
                offset,
                size,
                ash::vk::MemoryMapFlags::empty(),
            )?;
            let mapped_memory = std::slice::from_raw_parts_mut(ptr as _, size as _);
            f(mapped_memory);
            self.device.ash_device.unmap_memory(self.vk_device_memory);
            Ok(())
        }
    }
}

pub struct DeviceMemoryBuilder<'a> {
    device: Arc<Device>,
    allocation_size: ash::vk::DeviceSize,
    memory_type: MemoryType,
    dedicated_allocate_info: Option<MemoryDedicatedAllocateInfo<'a>>,
}

impl<'a> DeviceMemoryBuilder<'a> {
    pub fn allocation_size(mut self, allocation_size: ash::vk::DeviceSize) -> Self {
        self.allocation_size = allocation_size;
        self
    }
    pub fn dedicated_info(mut self, dedicated_memory_info: MemoryDedicatedAllocateInfo<'a>) -> Self {
        self.dedicated_allocate_info = Some(dedicated_memory_info);
        self
    }
    pub fn build(self) -> Result<DeviceMemory, ash::vk::Result> {
        self.device.allocations.fetch_add(1, Ordering::Relaxed);
        let mut allocate_info_builder = ash::vk::MemoryAllocateInfo::builder()
            .memory_type_index(self.memory_type.index)
            .allocation_size(self.allocation_size);
        let mut vk_dedicated_memory_info;
        if let Some(dedicated_memory_info) = self.dedicated_allocate_info {
            vk_dedicated_memory_info = dedicated_memory_info.ash_builder().build();
            allocate_info_builder =
                allocate_info_builder.push_next(&mut vk_dedicated_memory_info);
        }
        let vk_allocate_info = allocate_info_builder.build();
        // Host Synchronization: none
        let vk_device_memory = unsafe {
            self.device
                .ash_device
                .allocate_memory(&vk_allocate_info, None)?
        };

        Ok(DeviceMemory {
            device: self.device,
            vk_device_memory,
            size: self.allocation_size,
        })
    }
}
