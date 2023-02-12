use crate::buffer::{Buffer, BufferCreateFlags, RawBuffer};
use crate::device::Device;
use crate::device_memory::State::{Bound, Unbound};
use crate::device_memory::{UnBoundMemory, DeviceMemory, MemoryRequirement, State};
use crate::physical_device::SharingMode;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct ContinuousBuffer<const STATE: State = Bound>(RawBuffer);

impl<const STATE: State> MemoryRequirement for ContinuousBuffer<STATE> {
    fn get_memory_requirements(&self) -> &ash::vk::MemoryRequirements {
        self.0.get_memory_requirements()
    }

    fn get_memory_requirements2<T: ash::vk::ExtendsMemoryRequirements2 + Default>(&self) -> T {
        self.0.get_memory_requirements2()
    }
}

impl<const STATE: State> Deref for ContinuousBuffer<STATE> {
    type Target = RawBuffer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const STATE: State> DerefMut for ContinuousBuffer<STATE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Do not impl Buffer for unbound buffer
impl Buffer for ContinuousBuffer<{ Bound }> {
    fn raw(&self) -> &RawBuffer {
        &self.0
    }

    fn raw_mut(&mut self) -> &mut RawBuffer {
        &mut self.0
    }
}

impl UnBoundMemory for ContinuousBuffer<{ Unbound }> {
    type BoundType = ContinuousBuffer<{ Bound }>;

    fn device(&self) -> &Arc<Device> {
        &self.device
    }

    fn bind_memory(
        self,
        memory: &DeviceMemory,
        memory_offset: ash::vk::DeviceSize,
    ) -> Result<Self::BoundType, ash::vk::Result> {
        // Host Synchronization buffer
        unsafe {
            self.device.ash_device.bind_buffer_memory(
                self.ash_vk_buffer,
                memory.vk_device_memory,
                memory_offset,
            )?;
        }
        Ok(unsafe { std::mem::transmute(self) })
    }
}

impl ContinuousBuffer<{ Unbound }> {
    pub fn builder(device: Arc<Device>) -> ContinuousBufferBuilder {
        ContinuousBufferBuilder {
            device,
            flags: Default::default(),
            size: 0,
            usage: Default::default(),
            sharing_mode: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct ContinuousBufferBuilder {
    device: Arc<Device>,
    flags: ash::vk::BufferCreateFlags,
    size: ash::vk::DeviceSize,
    usage: ash::vk::BufferUsageFlags,
    sharing_mode: SharingMode,
}

impl ContinuousBufferBuilder {
    pub fn add_flag(mut self, flag: BufferCreateFlags) -> Self {
        self.flags |= flag.to_ash();
        self
    }

    pub fn size(mut self, size: ash::vk::DeviceSize) -> Self {
        self.size = size;
        self
    }

    pub fn usage(mut self, usage: ash::vk::BufferUsageFlags) -> Self {
        self.usage = usage;
        self
    }

    pub fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
        self.sharing_mode = sharing_mode;
        self
    }

    pub fn build(mut self) -> Result<ContinuousBuffer<{ Unbound }>, ash::vk::Result> {
        // SILENCE VUID-VkBufferCreateInfo-flags-00918
        if self
            .flags
            .contains(ash::vk::BufferCreateFlags::SPARSE_RESIDENCY)
            || self
                .flags
                .contains(ash::vk::BufferCreateFlags::SPARSE_ALIASED)
        {
            if !self
                .flags
                .contains(ash::vk::BufferCreateFlags::SPARSE_BINDING)
            {
                self.flags |= ash::vk::BufferCreateFlags::SPARSE_BINDING;
            }
        }
        // DONE VUID-VkBufferCreateInfo-sharingMode-00913
        let mut create_info = ash::vk::BufferCreateInfo::builder()
            .flags(self.flags)
            .usage(self.usage)
            .size(self.size);
        let family_properties;
        match self.sharing_mode {
            SharingMode::EXCLUSIVE => {
                create_info = create_info.sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            }
            SharingMode::CONCURRENT(queue_families) => {
                family_properties = queue_families
                    .into_iter()
                    .map(|property| property.index)
                    .collect::<Vec<_>>();
                create_info = create_info
                    .sharing_mode(ash::vk::SharingMode::CONCURRENT)
                    .queue_family_indices(family_properties.as_slice())
            }
        }
        // Host Synchronization: none
        let ash_vk_buffer = unsafe { self.device.ash_device.create_buffer(&create_info, None)? };
        let memory_requirements = unsafe {
            // Host Synchronization: none
            self.device
                .ash_device
                .get_buffer_memory_requirements(ash_vk_buffer)
        };
        Ok(ContinuousBuffer {
            0: RawBuffer {
                device: self.device,
                ash_vk_buffer,
                free_notification: None,
                memory_requirements,
            },
        })
    }
}
