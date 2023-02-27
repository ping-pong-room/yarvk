use crate::binding_resource::BindingResource;
use crate::buffer::IBuffer;
use crate::command::command_buffer::RenderPassScope::OUTSIDE;
use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level};
use crate::device::Device;
use crate::device_memory::IMemoryRequirements;
use crate::physical_device::SharingMode;
use ash::vk::{DeviceSize, Handle};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub mod continuous_image;
pub mod image_subresource_range;
pub mod image_view;
pub use continuous_image::*;

pub type IImage = dyn BindingResource<RawTy = Image>;

impl Deref for IImage {
    type Target = Image;

    fn deref(&self) -> &Self::Target {
        self.raw()
    }
}

impl DerefMut for IImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.raw_mut()
    }
}

pub struct Image {
    pub device: Arc<Device>,
    pub(crate) vk_image: ash::vk::Image,
    pub(crate) presentable: bool,
    pub(crate) free_notification: Option<Box<dyn FnOnce(&Self) + Sync + Send>>,
    pub(crate) memory_requirements: ash::vk::MemoryRequirements,
}

impl Drop for Image {
    fn drop(&mut self) {
        // Host Synchronization: image
        unsafe {
            if !self.presentable {
                self.device.ash_device.destroy_image(self.vk_image, None);
                if let Some(free_notification) = self.free_notification.take() {
                    free_notification(self);
                }
            }
        }
    }
}

impl BindingResource for Image {
    type RawTy = Self;

    fn raw(&self) -> &Image {
        self
    }

    fn raw_mut(&mut self) -> &mut Image {
        self
    }

    fn offset(&self) -> DeviceSize {
        ash::vk::DeviceSize::MAX
    }

    fn size(&self) -> DeviceSize {
        self.memory_requirements.size
    }

    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl crate::Handle for Image {
    fn handle(&self) -> u64 {
        self.vk_image.as_raw()
    }
}

impl IMemoryRequirements for Image {
    fn get_memory_requirements(&self) -> &ash::vk::MemoryRequirements {
        &self.memory_requirements
    }

    fn get_memory_requirements2<T: ash::vk::ExtendsMemoryRequirements2 + Default>(&self) -> T {
        let mut t = T::default();
        let info = ash::vk::ImageMemoryRequirementsInfo2::builder()
            .image(self.vk_image)
            .build();
        let mut out = ash::vk::MemoryRequirements2::builder()
            .push_next(&mut t)
            .build();
        unsafe {
            // Host Synchronization: none
            self.device
                .ash_device
                .get_image_memory_requirements2(&info, &mut out);
        }
        t
    }
}

#[derive(Default, Clone)]
pub struct ImageCreateInfo {
    pub flags: ash::vk::ImageCreateFlags,
    pub image_type: ash::vk::ImageType,
    pub format: ash::vk::Format,
    pub extent: ash::vk::Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: ash::vk::SampleCountFlags,
    pub tiling: ash::vk::ImageTiling,
    pub usage: ash::vk::ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub initial_layout: ash::vk::ImageLayout,
}

pub type ImageFormatListCreateInfo = Vec<ash::vk::Format>;

impl<const LEVEL: Level> CommandBuffer<LEVEL, { RECORDING }, { OUTSIDE }> {
    // DONE VUID-vkCmdCopyBufferToImage-commandBuffer-recording
    pub fn cmd_copy_buffer_to_image(
        &mut self,
        src_buffer: Arc<IBuffer>,
        dst_image: Arc<IImage>,
        dst_image_layout: ash::vk::ImageLayout,
        regions: &[ash::vk::BufferImageCopy],
    ) {
        self.holding_resources
            .read_buffers
            .insert(src_buffer.ash_vk_buffer.as_raw(), src_buffer.clone());
        self.holding_resources
            .write_images
            .insert(dst_image.vk_image.as_raw(), dst_image.clone());
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_copy_buffer_to_image(
                self.vk_command_buffer,
                src_buffer.ash_vk_buffer,
                dst_image.vk_image,
                dst_image_layout,
                regions,
            );
        }
    }
}
