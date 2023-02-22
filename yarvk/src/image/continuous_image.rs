use crate::binding_resource::{BindMemoryInfo, BindingResource};
use crate::device::Device;
use crate::device_memory::{DeviceMemory, IMemoryRequirements, UnboundResource};
use crate::image::{Image, ImageCreateInfo};
use crate::physical_device::SharingMode;
use ash::vk::{DeviceSize, ExtendsMemoryRequirements2, MemoryRequirements};
use derive_more::{Deref, DerefMut};
use std::marker::PhantomData;
use std::sync::Arc;
use crate::device_memory::dedicated_memory::DedicatedResource;
use crate::device_memory::dedicated_memory::MemoryDedicatedAllocateInfo;

pub struct ContinuousImage {
    _phantom: PhantomData<usize>,
}

impl ContinuousImage {
    pub fn builder(device: Arc<Device>) -> ContinuousImageBuilder {
        ContinuousImageBuilder {
            device,
            inner: Default::default(),
        }
    }
}

pub struct ContinuousImageBuilder {
    device: Arc<Device>,
    inner: ImageCreateInfo,
}

impl ContinuousImageBuilder {
    pub fn flags(&mut self, flags: ash::vk::ImageCreateFlags) {
        self.inner.flags = flags;
    }
    pub fn image_type(&mut self, image_type: ash::vk::ImageType) {
        self.inner.image_type = image_type;
    }
    pub fn format(&mut self, format: ash::vk::Format) {
        self.inner.format = format;
    }
    pub fn extent(&mut self, extent: ash::vk::Extent3D) {
        self.inner.extent = extent;
    }
    pub fn mip_levels(&mut self, mip_levels: u32) {
        self.inner.mip_levels = mip_levels;
    }
    pub fn array_layers(&mut self, array_layers: u32) {
        self.inner.array_layers = array_layers;
    }
    pub fn samples(&mut self, samples: ash::vk::SampleCountFlags) {
        self.inner.samples = samples;
    }
    pub fn tiling(&mut self, tiling: ash::vk::ImageTiling) {
        self.inner.tiling = tiling;
    }
    pub fn usage(&mut self, usage: ash::vk::ImageUsageFlags) {
        self.inner.usage = usage;
    }
    pub fn sharing_mode(&mut self, sharing_mode: SharingMode) {
        self.inner.sharing_mode = sharing_mode;
    }
    pub fn initial_layout(&mut self, initial_layout: ash::vk::ImageLayout) {
        self.inner.initial_layout = initial_layout;
    }
    pub fn build(&self) -> Result<UnboundContinuousImage, ash::vk::Result> {
        let image_create_info = &self.inner;
        let mut vk_image_create_info = ash::vk::ImageCreateInfo::builder()
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
                vk_image_create_info.sharing_mode = ash::vk::SharingMode::EXCLUSIVE;
            }
            SharingMode::CONCURRENT(queues) => {
                vk_image_create_info.sharing_mode = ash::vk::SharingMode::CONCURRENT;
                _indices = queues.iter().map(|queue| queue.index).collect();
                vk_image_create_info =
                    vk_image_create_info.queue_family_indices(_indices.as_slice());
            }
        }

        let vk_image_create_info = vk_image_create_info.build();
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
        Ok(UnboundContinuousImage {
            0: Image {
                device: self.device.clone(),
                vk_image,
                presentable: false,
                free_notification: None,
                memory_requirements,
            },
        })
    }
}

#[derive(Deref, DerefMut)]
pub struct UnboundContinuousImage(pub(crate) Image);

#[derive(Deref, DerefMut)]
pub struct BoundContinuousImage {
    #[deref]
    #[deref_mut]
    pub(crate) image: Image,
    pub(crate) offset: DeviceSize,
}

impl IMemoryRequirements for UnboundContinuousImage {
    fn get_memory_requirements(&self) -> &MemoryRequirements {
        self.0.get_memory_requirements()
    }

    fn get_memory_requirements2<T: ExtendsMemoryRequirements2 + Default>(&self) -> T {
        self.0.get_memory_requirements2()
    }
}

impl IMemoryRequirements for BoundContinuousImage {
    fn get_memory_requirements(&self) -> &MemoryRequirements {
        self.image.get_memory_requirements()
    }

    fn get_memory_requirements2<T: ExtendsMemoryRequirements2 + Default>(&self) -> T {
        self.image.get_memory_requirements2()
    }
}

impl BindingResource for BoundContinuousImage {
    type RawTy = Image;

    fn raw(&self) -> &Self::RawTy {
        self.image.raw()
    }

    fn raw_mut(&mut self) -> &mut Self::RawTy {
        self.image.raw_mut()
    }

    fn offset(&self) -> DeviceSize {
        self.offset
    }
}

impl UnboundResource for UnboundContinuousImage {
    type RawTy = Image;
    type BoundType = BoundContinuousImage;

    fn device(&self) -> &Arc<Device> {
        &self.device
    }

    fn dedicated_info(&self) -> MemoryDedicatedAllocateInfo {
        MemoryDedicatedAllocateInfo {
            resource: DedicatedResource::Image(self),
        }
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
        Ok(BoundContinuousImage {
            image: self.0,
            offset: memory_offset,
        })
    }

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
                let vk_infos = ash::vk::BindImageMemoryInfo::builder()
                    .image(info.resource.vk_image)
                    .memory_offset(info.memory_offset)
                    .memory(info.memory.vk_device_memory)
                    .build();
                bounds.push(BoundContinuousImage {
                    image: info.resource.0,
                    offset: info.memory_offset,
                });
                vk_infos
            })
            .collect::<Vec<_>>();
        unsafe { device.ash_device.bind_image_memory2(infos.as_slice())? };
        Ok(bounds)
    }
}
