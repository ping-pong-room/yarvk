use crate::buffer::Buffer;
use crate::command::command_buffer::RenderPassScope::OUTSIDE;
use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level};
use crate::device::Device;
use crate::device_memory::State::{Bound, Unbound};
use crate::device_memory::{DeviceMemory, MemoryRequirement, State};
use crate::physical_device::SharingMode;
use crate::Handler;
use ash::vk::Handle;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub mod image_subresource_range;
pub mod image_view;

#[derive(Default)]
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

pub struct ImageBuilder {
    device: Arc<Device>,
    inner: ImageCreateInfo,
}

impl ImageBuilder {
    pub fn flags(mut self, flags: ash::vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image_type(mut self, image_type: ash::vk::ImageType) -> Self {
        self.inner.image_type = image_type;
        self
    }
    pub fn format(mut self, format: ash::vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn extent(mut self, extent: ash::vk::Extent3D) -> Self {
        self.inner.extent = extent;
        self
    }
    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.inner.mip_levels = mip_levels;
        self
    }
    pub fn array_layers(mut self, array_layers: u32) -> Self {
        self.inner.array_layers = array_layers;
        self
    }
    pub fn samples(mut self, samples: ash::vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn tiling(mut self, tiling: ash::vk::ImageTiling) -> Self {
        self.inner.tiling = tiling;
        self
    }
    pub fn usage(mut self, usage: ash::vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn initial_layout(mut self, initial_layout: ash::vk::ImageLayout) -> Self {
        self.inner.initial_layout = initial_layout;
        self
    }
    pub fn build(self) -> Result<Image<{ Unbound }>, ash::vk::Result> {
        let image_create_info = self.inner;
        let mut vk_iamge_create_info = ash::vk::ImageCreateInfo::builder()
            .flags(image_create_info.flags)
            .image_type(image_create_info.image_type)
            .format(image_create_info.format)
            .extent(image_create_info.extent)
            .mip_levels(image_create_info.mip_levels)
            .array_layers(image_create_info.array_layers)
            .samples(image_create_info.samples)
            .tiling(image_create_info.tiling)
            .usage(image_create_info.usage)
            .initial_layout(image_create_info.initial_layout);
        let mut _indices: Vec<u32> = Vec::default();
        match image_create_info.sharing_mode.clone() {
            SharingMode::EXCLUSIVE => {
                vk_iamge_create_info.sharing_mode = ash::vk::SharingMode::EXCLUSIVE;
            }
            SharingMode::CONCURRENT(queues) => {
                vk_iamge_create_info.sharing_mode = ash::vk::SharingMode::CONCURRENT;
                _indices = queues.iter().map(|queue| queue.index).collect();
                vk_iamge_create_info =
                    vk_iamge_create_info.queue_family_indices(_indices.as_slice());
            }
        }

        let vk_image_create_info = vk_iamge_create_info.build();
        let vk_image = unsafe {
            // Host Synchronization: none
            self.device
                .ash_device
                .create_image(&vk_image_create_info, None)?
        };
        let memory_requirements = unsafe {
            // Host Synchronization: none
            self.device
                .ash_device
                .get_image_memory_requirements(vk_image)
        };
        Ok(Image {
            device: self.device,
            vk_image,
            presentable: false,
            image_create_info: Arc::new(image_create_info),
            free_notification: None,
            memory_requirements,
        })
    }
}

pub struct Image<const STATE: State = Bound> {
    pub device: Arc<Device>,
    pub(crate) vk_image: ash::vk::Image,
    pub image_create_info: Arc<ImageCreateInfo>,
    pub(crate) presentable: bool,
    pub(crate) free_notification: Option<Box<dyn FnOnce(&Self) + Sync + Send>>,
    pub(crate) memory_requirements: ash::vk::MemoryRequirements,
}

impl<const STATE: State> Handler for Image<STATE> {
    fn handler(&self) -> u64 {
        self.vk_image.as_raw()
    }
}

impl<const STATE: State> PartialEq for Image<STATE> {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device && self.vk_image == other.vk_image
    }
}

impl<const STATE: State> Eq for Image<STATE> {}

impl<const STATE: State> Hash for Image<STATE> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // only hash image handler, not device, since it's rare to use multiple devices
        self.vk_image.hash(state)
    }
}

impl<const STATE: State> MemoryRequirement for Image<STATE> {
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

impl Image<{ Unbound }> {
    pub fn builder(device: Arc<Device>) -> ImageBuilder {
        ImageBuilder {
            device,
            inner: Default::default(),
        }
    }
    pub fn bind_memory(
        self,
        memory: &DeviceMemory,
        memory_offset: ash::vk::DeviceSize,
    ) -> Result<Arc<Image<{ Bound }>>, ash::vk::Result> {
        // DONE VUID-vkBindImageMemory-image-01044
        unsafe {
            // Host Synchronization: image
            self.device.ash_device.bind_image_memory(
                self.vk_image,
                memory.vk_device_memory,
                memory_offset,
            )?;
        }
        Ok(Arc::new(unsafe { std::mem::transmute(self) }))
    }

    pub fn bind_memory_with_free_notification(
        mut self,
        memory: &DeviceMemory,
        memory_offset: ash::vk::DeviceSize,
        free_notification: Box<dyn FnOnce(&Self) + Sync + Send>,
    ) -> Result<Arc<Image<{ Bound }>>, ash::vk::Result> {
        self.free_notification = Some(free_notification);
        self.bind_memory(memory, memory_offset)
    }
}

impl<const STATE: State> Drop for Image<STATE> {
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

pub type ImageFormatListCreateInfo = Vec<ash::vk::Format>;

impl<const LEVEL: Level, const ONE_TIME_SUBMIT: bool>
    CommandBuffer<LEVEL, { RECORDING }, { OUTSIDE }, ONE_TIME_SUBMIT>
{
    // DONE VUID-vkCmdCopyBufferToImage-commandBuffer-recording
    pub fn cmd_copy_buffer_to_image(
        &mut self,
        src_buffer: Arc<Buffer>,
        dst_image: Arc<Image>,
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
