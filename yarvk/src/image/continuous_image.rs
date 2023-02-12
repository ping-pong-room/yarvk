use crate::device::Device;
use crate::device_memory::State::{Bound, Unbound};
use crate::device_memory::{UnBoundMemory, DeviceMemory, MemoryRequirement, State};
use crate::image::{Image, ImageCreateInfo, RawImage};
use crate::physical_device::SharingMode;
use ash::vk::{DeviceSize, ExtendsMemoryRequirements2, MemoryRequirements};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct ContinuousImageBuilder {
    device: Arc<Device>,
    inner: ImageCreateInfo,
}

impl ContinuousImageBuilder {
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
    pub fn build(self) -> Result<ContinuousImage<{ Unbound }>, ash::vk::Result> {
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
        Ok(ContinuousImage {
            0: RawImage {
                device: self.device,
                vk_image,
                presentable: false,
                image_create_info: Arc::new(image_create_info),
                free_notification: None,
                memory_requirements,
            },
        })
    }
}

pub struct ContinuousImage<const STATE: State = Bound>(pub(crate) RawImage);

impl<const STATE: State> MemoryRequirement for ContinuousImage<STATE> {
    fn get_memory_requirements(&self) -> &MemoryRequirements {
        self.0.get_memory_requirements()
    }

    fn get_memory_requirements2<T: ExtendsMemoryRequirements2 + Default>(&self) -> T {
        self.0.get_memory_requirements2()
    }
}

impl<const STATE: State> Deref for ContinuousImage<STATE> {
    type Target = RawImage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const STATE: State> DerefMut for ContinuousImage<STATE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Do not impl Buffer for unbound buffer
impl Image for ContinuousImage<{ Bound }> {
    fn raw(&self) -> &RawImage {
        self.0.raw()
    }

    fn raw_mut(&mut self) -> &mut RawImage {
        self.0.raw_mut()
    }
}

impl UnBoundMemory for ContinuousImage<{ Unbound }> {
    type BoundType = ContinuousImage<{ Bound }>;

    fn device(&self) -> &Arc<Device> {
        &self.device
    }

    fn bind_memory(
        self,
        memory: &DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result<Self::BoundType, ash::vk::Result> {
        // DONE VUID-vkBindImageMemory-image-01044
        unsafe {
            // Host Synchronization: image
            self.device.ash_device.bind_image_memory(
                self.vk_image,
                memory.vk_device_memory,
                memory_offset,
            )?;
        }
        Ok(unsafe { std::mem::transmute(self) })
    }
}

impl ContinuousImage<{ Unbound }> {
    pub fn builder(device: Arc<Device>) -> ContinuousImageBuilder {
        ContinuousImageBuilder {
            device,
            inner: Default::default(),
        }
    }
}
