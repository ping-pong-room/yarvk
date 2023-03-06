use crate::binding_resource::{BindMemoryInfo, BindingResource};
use crate::device::Device;
use crate::device_memory::dedicated_memory::MemoryDedicatedAllocateInfo;
use crate::physical_device::memory_properties::MemoryType;
use ash::vk::Handle;
use std::ffi::c_void;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub mod dedicated_memory;
pub mod mapped_ranges;

pub trait IMemoryRequirements: Send + Sync {
    fn get_memory_requirements(&self) -> &ash::vk::MemoryRequirements;
    fn get_memory_requirements2<T: ash::vk::ExtendsMemoryRequirements2 + Default>(&self) -> T;
}

pub trait UnboundResource: IMemoryRequirements + Send + Sync {
    type RawTy: IMemoryRequirements;
    type BoundType: IMemoryRequirements + BindingResource<RawTy = Self::RawTy> + Send + Sync;
    fn device(&self) -> &Arc<Device>;
    fn dedicated_info(&self) -> MemoryDedicatedAllocateInfo;
    fn bind_memory(
        self,
        memory: &DeviceMemory,
        memory_offset: ash::vk::DeviceSize,
    ) -> Result<Self::BoundType, ash::vk::Result>;
    fn bind_memories<'a, It: IntoIterator<Item = BindMemoryInfo<'a, Self>>>(
        device: &Arc<Device>,
        it: It,
    ) -> Result<Vec<Self::BoundType>, ash::vk::Result>
    where
        Self: Sized;
}

pub struct DeviceMemory {
    pub device: Arc<Device>,
    pub(crate) vk_device_memory: ash::vk::DeviceMemory,
    pub size: ash::vk::DeviceSize,
    pub memory_type: MemoryType,
    map_range: Option<(*mut c_void, ash::vk::DeviceSize, ash::vk::DeviceSize)>,
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
    pub fn builder<'a>(
        memory_type: &'a MemoryType,
        device: &Arc<Device>,
    ) -> DeviceMemoryBuilder<'a> {
        DeviceMemoryBuilder {
            device: device.clone(),
            allocation_size: 0,
            memory_type,
            dedicated_allocate_info: None,
        }
    }
    pub fn unmap_memory(&mut self) {
        // DONE VUID-vkUnmapMemory-memory-00689
        if self.map_range.is_none() {
            return;
        }
        unsafe {
            // Host Synchronization
            // Host access to memory must be externally synchronized
            self.device.ash_device.unmap_memory(self.vk_device_memory)
        }
    }
    fn map_exact_memory(
        &mut self,
        offset: ash::vk::DeviceSize,
        mut size: ash::vk::DeviceSize,
    ) -> Result<(), ash::vk::Result> {
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
        unsafe {
            let ptr = self.device.ash_device.map_memory(
                self.vk_device_memory,
                real_offset,
                real_size,
                ash::vk::MemoryMapFlags::empty(),
            )?;
            self.map_range = Some((ptr, real_offset, real_size));
            Ok(())
        }
    }
    fn get_memory_inner(
        &self,
        offset: ash::vk::DeviceSize,
        size: ash::vk::DeviceSize,
    ) -> Result<&mut [u8], ash::vk::Result> {
        if let Some((ptr, self_offset, self_size)) = &self.map_range {
            // required address is in the mapped address range
            if offset >= *self_offset
                && offset < self_offset + self_size
                && offset + size <= self_offset + self_size
            {
                unsafe {
                    let ptr = ptr.add((offset - self_offset) as usize);
                    return Ok(std::slice::from_raw_parts_mut(ptr as _, size as _));
                }
            }
        }
        Err(ash::vk::Result::ERROR_MEMORY_MAP_FAILED)
    }
    pub fn get_memory(
        &self,
        offset: ash::vk::DeviceSize,
        mut size: ash::vk::DeviceSize,
    ) -> Result<&mut [u8], ash::vk::Result> {
        assert!(self
            .memory_type
            .property_flags
            .contains(ash::vk::MemoryPropertyFlags::HOST_VISIBLE));
        if size == ash::vk::WHOLE_SIZE {
            size = self.size - offset;
        }
        self.get_memory_inner(offset, size)
    }
    pub fn map_memory(
        &mut self,
        offset: ash::vk::DeviceSize,
        mut size: ash::vk::DeviceSize,
    ) -> Result<(), ash::vk::Result> {
        if !self
            .memory_type
            .property_flags
            .contains(ash::vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            return Err(ash::vk::Result::ERROR_MEMORY_MAP_FAILED);
        }
        if size == ash::vk::WHOLE_SIZE {
            size = self.size - offset;
        }
        self.unmap_memory();
        self.map_exact_memory(offset, size)
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
            map_range: None,
        })
    }
}
