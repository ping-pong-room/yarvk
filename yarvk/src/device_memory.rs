use crate::device::Device;
use crate::device_memory::dedicated_memory::MemoryDedicatedAllocateInfo;
use crate::physical_device::memory_properties::MemoryType;
use ash::vk::Handle;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub mod dedicated_memory;
pub mod mapped_memory;
pub mod mapped_ranges;

pub trait IMemoryRequirements {
    fn get_memory_requirements(&self) -> &ash::vk::MemoryRequirements;
    fn get_memory_requirements2<T: ash::vk::ExtendsMemoryRequirements2 + Default>(&self) -> T;
}

pub trait UnBoundMemory: IMemoryRequirements {
    type BoundType;
    fn device(&self) -> &Arc<Device>;
    fn bind_memory(
        self,
        memory: &DeviceMemory,
        memory_offset: ash::vk::DeviceSize,
    ) -> Result<Self::BoundType, ash::vk::Result>;
}

#[derive(PartialEq, Eq)]
pub enum State {
    Unbound,
    Bound,
}

pub struct DeviceMemory {
    pub device: Arc<Device>,
    pub(crate) vk_device_memory: ash::vk::DeviceMemory,
    pub size: ash::vk::DeviceSize,
    pub memory_type: MemoryType,
}

unsafe impl Sync for DeviceMemory {}
unsafe impl Send for DeviceMemory {}

impl Drop for DeviceMemory {
    fn drop(&mut self) {
        unsafe {
            // Host Synchronization: memory
            self.device
                .ash_device
                .free_memory(self.vk_device_memory, None);
            #[cfg(feature = "max_memory_allocation_count_check")]
            self.device.allocations.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

impl crate::Handle for DeviceMemory {
    fn handle(&self) -> u64 {
        self.vk_device_memory.as_raw()
    }
}

impl DeviceMemory {
    pub fn builder(memory_type: &MemoryType, device: Arc<Device>) -> DeviceMemoryBuilder {
        DeviceMemoryBuilder {
            device,
            allocation_size: 0,
            memory_type,
            dedicated_allocate_info: None,
        }
    }
}

pub struct DeviceMemoryBuilder<'a> {
    device: Arc<Device>,
    allocation_size: ash::vk::DeviceSize,
    memory_type: &'a MemoryType,
    dedicated_allocate_info: Option<MemoryDedicatedAllocateInfo<'a>>,
}

impl<'a> DeviceMemoryBuilder<'a> {
    pub fn allocation_size(mut self, allocation_size: ash::vk::DeviceSize) -> Self {
        self.allocation_size = allocation_size;
        self
    }
    pub fn dedicated_info(
        mut self,
        dedicated_memory_info: MemoryDedicatedAllocateInfo<'a>,
    ) -> Self {
        self.dedicated_allocate_info = Some(dedicated_memory_info);
        self
    }
    pub fn build(self) -> Result<DeviceMemory, ash::vk::Result> {
        let mut allocate_info_builder = ash::vk::MemoryAllocateInfo::builder()
            .memory_type_index(self.memory_type.index)
            .allocation_size(self.allocation_size);
        let mut vk_dedicated_memory_info;
        if let Some(dedicated_memory_info) = self.dedicated_allocate_info {
            vk_dedicated_memory_info = dedicated_memory_info.ash_builder().build();
            allocate_info_builder = allocate_info_builder.push_next(&mut vk_dedicated_memory_info);
        }
        let vk_allocate_info = allocate_info_builder.build();
        // Host Synchronization: none
        let vk_device_memory = unsafe {
            self.device
                .ash_device
                .allocate_memory(&vk_allocate_info, None)?
        };
        #[cfg(feature = "max_memory_allocation_count_check")]
        {
            // codes below has bugs, it should use mutex other than atomic if to make the checking
            // 100% accurate. But for performance reasons, it is acceptable to have some deviation.
            let max_memory_allocation_count = self
                .device
                .physical_device
                .get_physical_device_properties()
                .limits
                .max_memory_allocation_count;

            let result = self.device.allocations.fetch_add(1, Ordering::Relaxed);
            if result >= max_memory_allocation_count {
                // DONE VUID-vkAllocateMemory-maxMemoryAllocationCount-04101
                self.device.allocations.fetch_sub(1, Ordering::Relaxed);
                return Err(ash::vk::Result::ERROR_TOO_MANY_OBJECTS);
            }
        }
        Ok(DeviceMemory {
            device: self.device,
            vk_device_memory,
            size: self.allocation_size,
            memory_type: self.memory_type.clone(),
        })
    }
}
