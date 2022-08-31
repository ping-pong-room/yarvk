use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::device::Device;
use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceFeatures::{
    SparseBinding, SparseResidencyAliased, SparseResidencyBuffer,
};
use crate::device_features::PhysicalDeviceVulkan11Features::ProtectedMemory;
use crate::device_features::PhysicalDeviceVulkan12Features::BufferDeviceAddressCaptureReplay;
use crate::device_memory::{DeviceMemory, State};
use crate::physical_device::SharingMode;

use std::sync::Arc;
use ash::vk::Handle;
use crate::device_memory::State::{Bound, Unbound};

pub enum BufferCreateFlags {
    // DONE VUID-VkBufferCreateInfo-flags-00915
    SparseBinding(Feature<{ SparseBinding.into() }>),
    // DONE VUID-VkBufferCreateInfo-flags-00916
    SparseResidency(Feature<{ SparseResidencyBuffer.into() }>),
    // DONE VUID-VkBufferCreateInfo-flags-00917
    SparseAliased(Feature<{ SparseResidencyAliased.into() }>),
    // DONE VUID-VkBufferCreateInfo-flags-01887
    PROTECTED(Feature<{ ProtectedMemory.into() }>),
    // DONE VUID-VkBufferCreateInfo-flags-03338
    DeviceAddressCaptureReplay(Feature<{ BufferDeviceAddressCaptureReplay.into() }>),
}

impl BufferCreateFlags {
    fn to_ash(&self) -> ash::vk::BufferCreateFlags {
        return match self {
            BufferCreateFlags::SparseBinding(..) => ash::vk::BufferCreateFlags::SPARSE_BINDING,
            BufferCreateFlags::SparseResidency(..) => ash::vk::BufferCreateFlags::SPARSE_RESIDENCY,
            BufferCreateFlags::SparseAliased(..) => ash::vk::BufferCreateFlags::SPARSE_ALIASED,
            BufferCreateFlags::PROTECTED(..) => ash::vk::BufferCreateFlags::PROTECTED,
            BufferCreateFlags::DeviceAddressCaptureReplay(..) => {
                ash::vk::BufferCreateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY
            }
        };
    }
}

pub struct Buffer<const STATE: State = Bound> {
    pub device: Arc<Device>,
    pub(crate) ash_vk_buffer: ash::vk::Buffer,
}

impl<const STATE: State> Drop for Buffer<STATE> {
    fn drop(&mut self) {
        unsafe {
            // TODO execution VUID-vkDestroyBuffer-buffer-00922
            self.device
                .ash_device
                .destroy_buffer(self.ash_vk_buffer, None);
        }
    }
}

impl Buffer<{ Unbound }> {
    pub fn get_memory_requirements(&self) -> ash::vk::MemoryRequirements {
        unsafe {
            // Host Synchronization: none
            self.device
                .ash_device
                .get_buffer_memory_requirements(self.ash_vk_buffer)
        }
    }

    pub fn get_memory_requirements2<T: ash::vk::ExtendsMemoryRequirements2 + Default>(
        &self,
    ) -> T {
        let mut t = T::default();
        let info = ash::vk::BufferMemoryRequirementsInfo2::builder()
            .buffer(self.ash_vk_buffer)
            .build();
        let mut out = ash::vk::MemoryRequirements2::builder()
            .push_next(&mut t)
            .build();
        unsafe {
            // Host Synchronization: none
            self.device
                .ash_device
                .get_buffer_memory_requirements2(&info, &mut out);
        }
        t
    }

    pub fn bind_memory(
        self,
        memory: &DeviceMemory,
        memory_offset: ash::vk::DeviceSize,
    ) -> Result<Arc<Buffer<{ Bound }>>, ash::vk::Result> {
        // TODO why device_memory do not need to be synchronized?
        // Host Synchronization buffer
        unsafe {
            self.device.ash_device.bind_buffer_memory(
                self.ash_vk_buffer,
                memory.vk_device_memory,
                memory_offset,
            )?;
        }
        Ok(Arc::new(unsafe { std::mem::transmute(self) }))
    }
}

impl Buffer<{ Unbound }> {
    pub fn builder(device: Arc<Device>) -> BufferBuilder {
        BufferBuilder {
            device,
            flags: Default::default(),
            size: 0,
            usage: Default::default(),
            sharing_mode: Default::default(),
        }
    }
}

pub struct BufferBuilder {
    device: Arc<Device>,
    flags: ash::vk::BufferCreateFlags,
    size: ash::vk::DeviceSize,
    usage: ash::vk::BufferUsageFlags,
    sharing_mode: SharingMode,
}

impl BufferBuilder {
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

    pub fn build(mut self) -> Result<Buffer<{ Unbound }>, ash::vk::Result> {
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
        Ok(Buffer {
            device: self.device,
            ash_vk_buffer,
        })
    }
}

pub struct BufferView {
    pub buffer: Arc<Buffer>,
    pub(crate) ash_vk_buffer_view: ash::vk::BufferView,
}

impl BufferView {
    pub fn builder(buffer: Arc<Buffer>) -> BufferViewBuilder {
        BufferViewBuilder {
            buffer,
            flags: Default::default(),
            format: Default::default(),
            offset: 0,
            range: 0,
        }
    }
}

impl Drop for BufferView {
    fn drop(&mut self) {
        unsafe {
            // Host Synchronization: bufferView
            self.buffer
                .device
                .ash_device
                .destroy_buffer_view(self.ash_vk_buffer_view, None);
        }
    }
}

pub struct BufferViewBuilder {
    buffer: Arc<Buffer>,
    pub flags: ash::vk::BufferViewCreateFlags,
    pub format: ash::vk::Format,
    pub offset: ash::vk::DeviceSize,
    pub range: ash::vk::DeviceSize,
}

impl BufferViewBuilder {
    pub fn flags(mut self, flags: ash::vk::BufferViewCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn format(mut self, format: ash::vk::Format) -> Self {
        self.format = format;
        self
    }
    pub fn offset(mut self, offset: ash::vk::DeviceSize) -> Self {
        self.offset = offset;
        self
    }
    pub fn range(mut self, range: ash::vk::DeviceSize) -> Self {
        self.range = range;
        self
    }
    pub fn build(self) -> Result<Arc<BufferView>, ash::vk::Result> {
        let create_info = ash::vk::BufferViewCreateInfo::builder()
            .flags(self.flags)
            .format(self.format)
            .offset(self.offset)
            .range(self.range)
            .buffer(self.buffer.ash_vk_buffer);
        unsafe {
            // Host Synchronization: none
            let ash_vk_buffer_view = self
                .buffer
                .device
                .ash_device
                .create_buffer_view(&create_info, None)?;
            Ok(Arc::new(BufferView {
                buffer: self.buffer,
                ash_vk_buffer_view,
            }))
        }
    }
}

impl<const LEVEL: Level, const SCOPE: RenderPassScope, const ONE_TIME_SUBMIT: bool> CommandBuffer<LEVEL, { RECORDING }, SCOPE, ONE_TIME_SUBMIT> {
    // DONE VUID-vkCmdBindVertexBuffers-commandBuffer-recording
    pub fn cmd_bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        buffers: &[Arc<Buffer>],
        offsets: &[ash::vk::DeviceSize],
    ) {
        let mut ash_vk_buffers = Vec::with_capacity(buffers.len());
        for buffer in buffers {
            ash_vk_buffers.push(buffer.ash_vk_buffer);
            // TODO insert by raw automatically
            self.holding_resources
                .read_buffers
                .insert(buffer.ash_vk_buffer.as_raw(), buffer.clone());
        }
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_bind_vertex_buffers(
                self.vk_command_buffer,
                first_binding,
                ash_vk_buffers.as_slice(),
                offsets,
            );
        }
    }

    // DONE VUID-vkCmdBindIndexBuffer-commandBuffer-recording
    pub fn cmd_bind_index_buffer(
        &mut self,
        buffer: Arc<Buffer>,
        offset: ash::vk::DeviceSize,
        index_type: ash::vk::IndexType,
    ) {
        // TODO insert by raw automatically
        self.holding_resources
            .read_buffers
            .insert(buffer.ash_vk_buffer.as_raw(), buffer.clone());
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            // TODO index_type feature safety
            self.device.ash_device.cmd_bind_index_buffer(
                self.vk_command_buffer,
                buffer.ash_vk_buffer,
                offset,
                index_type,
            );
        }
    }

    pub fn cmd_draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            // TODO index_type feature safety
            self.device.ash_device.cmd_draw_indexed(
                self.vk_command_buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            );
        }
    }
}
