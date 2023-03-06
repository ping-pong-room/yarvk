use crate::binding_resource::{BindMemoryInfo, BindingResource};
use crate::buffer::{Buffer, BufferCreateFlags};
use crate::device::Device;
use crate::device_memory::dedicated_memory::{DedicatedResource, MemoryDedicatedAllocateInfo};
use crate::device_memory::{DeviceMemory, IMemoryRequirements, UnboundResource};
use crate::physical_device::SharingMode;
use ash::vk::DeviceSize;
use derive_more::{Deref, DerefMut};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct ContinuousBuffer {
    _phantom: PhantomData<usize>,
}

impl ContinuousBuffer {
    pub fn builder(device: &Arc<Device>) -> ContinuousBufferBuilder {
        ContinuousBufferBuilder {
            device: device.clone(),
            flags: Default::default(),
            size: 0,
            usage: Default::default(),
            sharing_mode: Default::default(),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct UnboundContinuousBuffer(pub(crate) Buffer);
#[derive(Deref, DerefMut)]
pub struct BoundContinuousBuffer {
    #[deref]
    #[deref_mut]
    buffer: Buffer,
    offset: ash::vk::DeviceSize,
}

impl IMemoryRequirements for UnboundContinuousBuffer {
    fn get_memory_requirements(&self) -> &ash::vk::MemoryRequirements {
        self.0.get_memory_requirements()
    }

    fn get_memory_requirements2<T: ash::vk::ExtendsMemoryRequirements2 + Default>(&self) -> T {
        self.0.get_memory_requirements2()
    }
}

impl IMemoryRequirements for BoundContinuousBuffer {
    fn get_memory_requirements(&self) -> &ash::vk::MemoryRequirements {
        self.buffer.get_memory_requirements()
    }

    fn get_memory_requirements2<T: ash::vk::ExtendsMemoryRequirements2 + Default>(&self) -> T {
        self.buffer.get_memory_requirements2()
    }
}

impl BindingResource for BoundContinuousBuffer {
    type RawTy = Buffer;

    fn raw(&self) -> &Self::RawTy {
        self.buffer.raw()
    }

    fn raw_mut(&mut self) -> &mut Self::RawTy {
        self.buffer.raw_mut()
    }

    fn offset(&self) -> DeviceSize {
        self.offset
    }

    fn size(&self) -> DeviceSize {
        self.memory_requirements.size
    }

    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl UnboundResource for UnboundContinuousBuffer {
    type RawTy = Buffer;
    type BoundType = BoundContinuousBuffer;

    fn device(&self) -> &Arc<Device> {
        &self.device
    }

    fn dedicated_info(&self) -> MemoryDedicatedAllocateInfo {
        MemoryDedicatedAllocateInfo {
            resource: DedicatedResource::Buffer(self),
        }
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
        Ok(BoundContinuousBuffer {
            buffer: self.0,
            offset: memory_offset,
        })
    }

    // TODO return original unbound resources if failed
    fn bind_memories<'a, It: IntoIterator<Item = BindMemoryInfo<'a, Self>>>(
        device: &Arc<Device>,
        it: It,
    ) -> Result<Vec<Self::BoundType>, ash::vk::Result>
    where
        Self: Sized,
    {
        let it = it.into_iter();
        let mut bounds = Vec::with_capacity(it.size_hint().0);
        let infos = it
            .map(|info| {
                let vk_infos = ash::vk::BindBufferMemoryInfo::builder()
                    .buffer(info.resource.ash_vk_buffer)
                    .memory_offset(info.memory_offset)
                    .memory(info.memory.vk_device_memory)
                    .build();
                bounds.push(BoundContinuousBuffer {
                    buffer: info.resource.0,
                    offset: info.memory_offset,
                });
                vk_infos
            })
            .collect::<Vec<_>>();
        unsafe { device.ash_device.bind_buffer_memory2(infos.as_slice())? };
        Ok(bounds)
    }
}

pub struct ContinuousBufferBuilder {
    device: Arc<Device>,
    flags: ash::vk::BufferCreateFlags,
    size: ash::vk::DeviceSize,
    usage: ash::vk::BufferUsageFlags,
    sharing_mode: SharingMode,
}

impl ContinuousBufferBuilder {
    pub fn add_flag(&mut self, flag: BufferCreateFlags) {
        self.flags |= flag.to_ash();
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
    }

    pub fn size(&mut self, size: ash::vk::DeviceSize) {
        self.size = size;
    }
    // TODO feature safety BufferUsageFlags
    pub fn usage(&mut self, usage: ash::vk::BufferUsageFlags) {
        self.usage = usage;
    }

    pub fn sharing_mode(&mut self, sharing_mode: SharingMode) {
        self.sharing_mode = sharing_mode;
    }

    pub fn build(&self) -> Result<UnboundContinuousBuffer, ash::vk::Result> {
        // DONE VUID-VkBufferCreateInfo-sharingMode-00913
        let mut create_info = ash::vk::BufferCreateInfo::builder()
            .flags(self.flags)
            .usage(self.usage)
            .size(self.size);
        let family_properties;
        match &self.sharing_mode {
            SharingMode::EXCLUSIVE => {
                create_info = create_info.sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            }
            SharingMode::CONCURRENT(queue_families) => {
                family_properties = queue_families
                    .iter()
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
        Ok(UnboundContinuousBuffer(Buffer {
            device: self.device.clone(),
            ash_vk_buffer,
            free_notification: None,
            memory_requirements,
        }))
    }
}
