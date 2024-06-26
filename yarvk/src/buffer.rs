use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::device::Device;
use crate::device_memory::IMemoryRequirements;
use ash::vk::{DeviceSize, Handle};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub mod buffer_view;
pub mod continuous_buffer;
use crate::binding_resource::BindingResource;
use crate::command::command_buffer::RenderPassScope::{INSIDE, OUTSIDE};
use crate::device_features::physical_device_buffer_device_address_features::FeatureBufferDeviceAddressCaptureReplay;
use crate::device_features::physical_device_features::{
    FeatureSparseBinding, FeatureSparseResidencyAliased, FeatureSparseResidencyBuffer,
};
use crate::device_features::physical_device_protected_memory_features::FeatureProtectedMemory;
pub use buffer_view::*;
pub use continuous_buffer::*;

pub type IBuffer = dyn BindingResource<RawTy = Buffer>;

impl PartialEq for IBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device && self.ash_vk_buffer == other.ash_vk_buffer
    }
}

pub struct Buffer {
    pub device: Arc<Device>,
    pub(crate) ash_vk_buffer: ash::vk::Buffer,
    pub(crate) free_notification: Option<Box<dyn FnOnce(&Self) + Sync + Send>>,
    memory_requirements: ash::vk::MemoryRequirements,
}

impl BindingResource for Buffer {
    type RawTy = Self;

    fn raw(&self) -> &Buffer {
        self
    }
    fn raw_mut(&mut self) -> &mut Buffer {
        self
    }

    fn offset(&self) -> ash::vk::DeviceSize {
        ash::vk::DeviceSize::MAX
    }

    fn size(&self) -> DeviceSize {
        self.memory_requirements.size
    }

    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl Deref for IBuffer {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        self.raw()
    }
}

impl DerefMut for IBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.raw_mut()
    }
}

impl crate::Handle for Buffer {
    fn handle(&self) -> u64 {
        self.ash_vk_buffer.as_raw()
    }
}

impl IMemoryRequirements for Buffer {
    fn get_memory_requirements(&self) -> &ash::vk::MemoryRequirements {
        &self.memory_requirements
    }

    fn get_memory_requirements2<T: ash::vk::ExtendsMemoryRequirements2 + Default>(&self) -> T {
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
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            // TODO execution VUID-vkDestroyBuffer-buffer-00922
            self.device
                .ash_device
                .destroy_buffer(self.ash_vk_buffer, None);
        }
        if let Some(free_notification) = self.free_notification.take() {
            free_notification(self);
        }
    }
}

pub enum BufferCreateFlags {
    // DONE VUID-VkBufferCreateInfo-flags-00915
    SparseBinding(FeatureSparseBinding),
    // DONE VUID-VkBufferCreateInfo-flags-00916
    SparseResidency(FeatureSparseResidencyBuffer),
    // DONE VUID-VkBufferCreateInfo-flags-00917
    SparseAliased(FeatureSparseResidencyAliased),
    // DONE VUID-VkBufferCreateInfo-flags-01887
    PROTECTED(FeatureProtectedMemory),
    // DONE VUID-VkBufferCreateInfo-flags-03338
    DeviceAddressCaptureReplay(FeatureBufferDeviceAddressCaptureReplay),
}

impl BufferCreateFlags {
    fn to_ash(&self) -> ash::vk::BufferCreateFlags {
        match self {
            BufferCreateFlags::SparseBinding(..) => ash::vk::BufferCreateFlags::SPARSE_BINDING,
            BufferCreateFlags::SparseResidency(..) => ash::vk::BufferCreateFlags::SPARSE_RESIDENCY,
            BufferCreateFlags::SparseAliased(..) => ash::vk::BufferCreateFlags::SPARSE_ALIASED,
            BufferCreateFlags::PROTECTED(..) => ash::vk::BufferCreateFlags::PROTECTED,
            BufferCreateFlags::DeviceAddressCaptureReplay(..) => {
                ash::vk::BufferCreateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY
            }
        }
    }
}

impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { RECORDING }, SCOPE> {
    // DONE VUID-vkCmdBindVertexBuffers-commandBuffer-recording
    pub fn cmd_bind_vertex_buffers<It: IntoIterator<Item = Arc<IBuffer>>>(
        &mut self,
        first_binding: u32,
        buffers: It,
        offsets: &[ash::vk::DeviceSize],
    ) {
        let iter = buffers.into_iter();
        let mut ash_vk_buffers = Vec::with_capacity(iter.size_hint().0);
        for buffer in iter {
            ash_vk_buffers.push(buffer.raw().ash_vk_buffer);
            // TODO insert by raw automatically
            self.holding_resources
                .read_buffers
                .insert(buffer.raw().ash_vk_buffer.as_raw(), buffer.clone());
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
        buffer: Arc<IBuffer>,
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
}

impl<const LEVEL: Level> CommandBuffer<LEVEL, { RECORDING }, { INSIDE }> {
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

    pub fn cmd_draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_draw(
                self.vk_command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            );
        }
    }
}

impl<const LEVEL: Level> CommandBuffer<LEVEL, { RECORDING }, { OUTSIDE }> {
    pub fn cmd_update_buffer(
        &mut self,
        dst_buffer: &Arc<IBuffer>,
        dst_offset: ash::vk::DeviceSize,
        data: &[u8],
    ) {
        self.holding_resources
            .write_buffers
            .insert(dst_buffer.ash_vk_buffer.as_raw(), dst_buffer.clone());
        unsafe {
            self.device.ash_device.cmd_update_buffer(
                self.vk_command_buffer,
                dst_buffer.ash_vk_buffer,
                dst_offset,
                data,
            )
        }
    }

    pub fn cmd_copy_buffer(
        &mut self,
        src_buffer: Arc<IBuffer>,
        dst_buffer: Arc<IBuffer>,
        regions: &[ash::vk::BufferCopy],
    ) {
        self.holding_resources
            .read_buffers
            .insert(src_buffer.ash_vk_buffer.as_raw(), src_buffer.clone());
        self.holding_resources
            .write_buffers
            .insert(dst_buffer.ash_vk_buffer.as_raw(), dst_buffer.clone());
        unsafe {
            self.device.ash_device.cmd_copy_buffer(
                self.vk_command_buffer,
                src_buffer.ash_vk_buffer,
                dst_buffer.ash_vk_buffer,
                regions,
            );
        }
    }
}
