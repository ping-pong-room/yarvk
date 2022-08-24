use crate::device::Device;
use crate::extensions::{DeviceExtension, PhysicalDeviceExtensionType};
use crate::fence::{SignalingFence, UnsignaledFence};
use crate::image::{Image, ImageCreateInfo, ImageFormatListCreateInfo};
use crate::physical_device::SharingMode;
use crate::queue::Queue;
use crate::semaphore::Semaphore;
use crate::surface::Surface;
use ash::vk::Handle;

use std::mem::ManuallyDrop;
use std::sync::Arc;
use crate::device_memory::State::Bound;

pub struct SwapchainBuilder {
    surface: Arc<Surface>,
    device: Arc<Device>,
    min_image_count: u32,
    image_format: ash::vk::Format,
    image_color_space: ash::vk::ColorSpaceKHR,
    image_extent: ash::vk::Extent2D,
    image_array_layers: u32,
    image_usage: ash::vk::ImageUsageFlags,
    present_mode: ash::vk::PresentModeKHR,
    image_sharing_mode: SharingMode,
    pre_transform: ash::vk::SurfaceTransformFlagsKHR,
    composite_alpha: ash::vk::CompositeAlphaFlagsKHR,
    old_swapchain: Option<Swapchain>,
    flags: ash::vk::SwapchainCreateFlagsKHR,
    // DONE VUID-VkSwapchainCreateInfoKHR-pNext-04099
    image_format_list_create_info: ImageFormatListCreateInfo,
    full_screen_exclusive: Option<ash::vk::FullScreenExclusiveEXT>,
    clipped: bool,
}

impl SwapchainBuilder {
    // While acquired by the application, presentable images can be used in any way that
    // equivalent non-presentable images can be used. A presentable image is equivalent to a
    // non-presentable image created with the following VkImageCreateInfo parameters:
    fn get_image_create_info(&self) -> ImageCreateInfo {
        let mut image_create_info = ImageCreateInfo::default();
        if self
            .flags
            .contains(ash::vk::SwapchainCreateFlagsKHR::SPLIT_INSTANCE_BIND_REGIONS)
        {
            image_create_info.flags |= ash::vk::ImageCreateFlags::SPLIT_INSTANCE_BIND_REGIONS;
        }
        if self
            .flags
            .contains(ash::vk::SwapchainCreateFlagsKHR::PROTECTED)
        {
            image_create_info.flags |= ash::vk::ImageCreateFlags::PROTECTED;
        }
        if self
            .flags
            .contains(ash::vk::SwapchainCreateFlagsKHR::MUTABLE_FORMAT)
        {
            image_create_info.flags |= ash::vk::ImageCreateFlags::MUTABLE_FORMAT;
            image_create_info.flags |= ash::vk::ImageCreateFlags::EXTENDED_USAGE;
        }
        image_create_info.image_type = ash::vk::ImageType::TYPE_2D;
        image_create_info.format = self.image_format;
        image_create_info.extent = ash::vk::Extent3D {
            width: self.image_extent.width,
            height: self.image_extent.height,
            depth: 1,
        };
        image_create_info.mip_levels = 1;
        image_create_info.array_layers = self.image_array_layers;
        image_create_info.samples = ash::vk::SampleCountFlags::TYPE_1;
        image_create_info.tiling = ash::vk::ImageTiling::OPTIMAL;
        image_create_info.usage = self.image_usage;
        image_create_info.sharing_mode = self.image_sharing_mode.clone();
        image_create_info.initial_layout = ash::vk::ImageLayout::UNDEFINED;
        image_create_info
    }

    pub fn clipped(mut self) -> Self {
        self.clipped = true;
        self
    }

    pub fn image_color_space(mut self, image_color_space: ash::vk::ColorSpaceKHR) -> Self {
        self.image_color_space = image_color_space;
        self
    }

    pub fn flags(mut self, flags: ash::vk::SwapchainCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }

    pub fn old_swapchain(mut self, old_swapchain: Swapchain) -> Self {
        // Host Synchronization: pCreateInfo->oldSwapchain
        // Done VUID-VkSwapchainCreateInfoKHR-oldSwapchain-01933
        // DONE VUID-vkAcquireNextImageKHR-swapchain-01285
        self.old_swapchain = Some(old_swapchain);
        self
    }

    pub fn present_mode(mut self, present_mode: ash::vk::PresentModeKHR) -> Self {
        self.present_mode = present_mode;
        self
    }

    pub fn composite_alpha(mut self, composite_alpha: ash::vk::CompositeAlphaFlagsKHR) -> Self {
        self.composite_alpha = composite_alpha;
        self
    }

    pub fn pre_transform(mut self, pre_transform: ash::vk::SurfaceTransformFlagsKHR) -> Self {
        self.pre_transform = pre_transform;
        self
    }

    pub fn image_sharing_mode(mut self, image_sharing_mode: SharingMode) -> Self {
        self.image_sharing_mode = image_sharing_mode;
        self
    }

    pub fn image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.image_array_layers = image_array_layers;
        self
    }

    pub fn image_usage(mut self, image_usage: ash::vk::ImageUsageFlags) -> Self {
        self.image_usage = image_usage;
        self
    }

    pub fn image_extent(mut self, image_extent: ash::vk::Extent2D) -> Self {
        self.image_extent = image_extent;
        self
    }

    pub fn image_format(mut self, surface_format: ash::vk::Format) -> Self {
        self.image_format = surface_format;
        self
    }

    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.min_image_count = min_image_count;
        self
    }

    pub fn build(self) -> Result<Swapchain, ash::vk::Result> {
        let image_create_info = Arc::new(self.get_image_create_info());
        // Done VUID-VkSwapchainCreateInfoKHR-surface-01270
        // Done VUID-VkSwapchainCreateInfoKHR-imageUsage-parameter
        // Done VUID-VkSwapchainCreateInfoKHR-imageUsage-requiredbitmask
        let mut create_info = ash::vk::SwapchainCreateInfoKHR::builder()
            .min_image_count(self.min_image_count)
            .image_format(self.image_format)
            .image_color_space(self.image_color_space)
            .image_usage(self.image_usage)
            .image_extent(self.image_extent)
            .image_array_layers(self.image_array_layers)
            .flags(self.flags)
            .clipped(self.clipped)
            .present_mode(self.present_mode)
            .pre_transform(self.pre_transform)
            .composite_alpha(self.composite_alpha);
        let mut indices = Vec::new();
        match self.image_sharing_mode {
            SharingMode::CONCURRENT(families) => {
                indices.reserve(families.len());
                for queue_family_properties in families {
                    indices.push(queue_family_properties.index);
                }
                create_info = create_info.image_sharing_mode(ash::vk::SharingMode::CONCURRENT);
                create_info = create_info.queue_family_indices(&indices);
            }
            SharingMode::EXCLUSIVE => {
                create_info = create_info.image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE);
            }
        }

        let mut vk_image_format_list_create_info;
        if !self.image_format_list_create_info.is_empty() {
            vk_image_format_list_create_info = ash::vk::ImageFormatListCreateInfo::builder()
                .view_formats(self.image_format_list_create_info.as_slice())
                .build();
            create_info = create_info.push_next(&mut vk_image_format_list_create_info);
        }

        let mut ext = ash::vk::SurfaceFullScreenExclusiveInfoEXT::builder();
        if let Some(full_screen_exclusive) = self.full_screen_exclusive {
            ext.full_screen_exclusive = full_screen_exclusive;
            create_info = create_info.push_next(&mut ext);
        }

        if let Some(old_swapchain) = self.old_swapchain {
            let old_swapchain = ManuallyDrop::new(old_swapchain);
            create_info = create_info.old_swapchain(old_swapchain.vk_swapchain);
        }

        let swapchain_loader = {
            ash::extensions::khr::Swapchain::new(
                &self.surface.physical_device.instance.ash_instance,
                &self.device.ash_device,
            )
        };

        let vk_swapchain = unsafe {
            create_info = create_info.surface(self.surface.vk_surface_khr);
            // TODO Host Synchronization: pCreateInfo->surface, pCreateInfo->oldSwapchain
            swapchain_loader.create_swapchain(&create_info, None)?
        };
        // Host Synchronization: none
        let vk_images = unsafe { swapchain_loader.get_swapchain_images(vk_swapchain)? };
        let images = vk_images
            .into_iter()
            .map(|vk_image| {
                Arc::new(Image {
                    device: self.device.clone(),
                    vk_image,
                    presentable: true,
                    image_create_info: image_create_info.clone(),
                })
            })
            .collect();
        Ok(Swapchain {
            surface: self.surface,
            device: self.device,
            vk_swapchain,
            swapchain_loader,
            // image_create_info,
            images,
        })
    }
}

pub struct PresentInfo<'a> {
    wait_semaphores: Vec<&'a mut Semaphore>,
    swapchains_and_image_indices: Vec<(&'a mut Swapchain, u32)>,
    results: Vec<ash::vk::Result>,
}

impl<'a> PresentInfo<'a> {
    pub fn builder() -> PresentInfoBuilder<'a> {
        PresentInfoBuilder {
            wait_semaphores: vec![],
            swapchains_and_image_indices: vec![],
        }
    }
}

pub struct PresentInfoBuilder<'a> {
    wait_semaphores: Vec<&'a mut Semaphore>,
    swapchains_and_image_indices: Vec<(&'a mut Swapchain, u32)>,
}

impl<'a> PresentInfoBuilder<'a> {
    pub fn add_wait_semaphore(mut self, semaphore: &'a mut Semaphore) -> Self {
        self.wait_semaphores.push(semaphore);
        self
    }

    pub fn add_swapchain_and_image(mut self, swapchian: &'a mut Swapchain, image: &Image) -> Self {
        let image_index = swapchian.get_image_index(image)
            .expect("Each element of pImageIndices must be the index of a presentable image acquired from the swapchain specified by the corresponding element of the pSwapchains array");

        self.swapchains_and_image_indices
            .push((swapchian, image_index));
        self
    }

    pub fn build(self) -> PresentInfo<'a> {
        let image_counts = self.swapchains_and_image_indices.len();
        let _semaphore_counts = self.wait_semaphores.len();
        PresentInfo {
            wait_semaphores: self.wait_semaphores,
            swapchains_and_image_indices: self.swapchains_and_image_indices,
            results: vec![ash::vk::Result::SUCCESS; image_counts],
        }
    }
}

impl Queue {
    pub fn queue_present<'a>(
        &mut self,
        present_info: &'a mut PresentInfo,
    ) -> Result<&'a [ash::vk::Result], ash::vk::Result> {
        let queue = self;
        let mut ash_vk_semaphores = Vec::with_capacity(present_info.wait_semaphores.len());
        let mut ash_vk_swapchains =
            Vec::with_capacity(present_info.swapchains_and_image_indices.len());
        let mut image_indices = Vec::with_capacity(present_info.swapchains_and_image_indices.len());
        for result in &mut present_info.results {
            *result = ash::vk::Result::SUCCESS;
        }
        for semaphore in &present_info.wait_semaphores {
            ash_vk_semaphores.push( semaphore.ash_vk_semaphore);
        }

        for (swapchain, index) in &present_info.swapchains_and_image_indices {
            ash_vk_swapchains.push(swapchain.vk_swapchain);
            image_indices.push(*index);
        }

        let ash_vk_present_info = ash::vk::PresentInfoKHR::builder()
            .wait_semaphores(ash_vk_semaphores.as_slice())
            .image_indices(image_indices.as_slice())
            .swapchains(ash_vk_swapchains.as_slice())
            .results(present_info.results.as_mut_slice())
            .build();

        // MUST VUID-VkPresentInfoKHR-swapchainCount-arraylength
        let loader = &present_info
            .swapchains_and_image_indices
            .get(0)
            .expect("swapchainCount must be greater than 0")
            .0
            .swapchain_loader;
        unsafe {
            // Host Synchronization: queue, semaphores, swapchains
            // TODO suboptimal
            let _suboptimal = loader.queue_present(queue.vk_queue, &ash_vk_present_info)?;
        }

        Ok(present_info.results.as_slice())
    }
}

pub struct Swapchain {
    // Done VUID-vkDestroySurfaceKHR-surface-01266
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
    vk_swapchain: ash::vk::SwapchainKHR,
    swapchain_loader: ash::extensions::khr::Swapchain,
    // image_create_info: Arc<ImageCreateInfo>,
    images: Vec<Arc<Image<{ Bound }>>>,
}

impl Swapchain {
    pub fn builder(
        surface: Arc<Surface>,
        extension: DeviceExtension<{ PhysicalDeviceExtensionType::KhrSwapchain }>,
    ) -> SwapchainBuilder {
        SwapchainBuilder {
            surface,
            device: extension.device,
            min_image_count: 0,
            image_format: Default::default(),
            image_color_space: Default::default(),
            image_extent: Default::default(),
            image_array_layers: 0,
            image_usage: ash::vk::ImageUsageFlags::COLOR_ATTACHMENT,
            present_mode: Default::default(),
            image_sharing_mode: Default::default(),
            pre_transform: Default::default(),
            composite_alpha: Default::default(),
            old_swapchain: None,
            flags: Default::default(),
            image_format_list_create_info: vec![],
            full_screen_exclusive: None,
            clipped: false,
        }
    }
    pub fn get_swapchain_images(&self) -> &[Arc<Image<{ Bound }>>] {
        self.images.as_slice()
    }
    pub(crate) fn get_image_index(&self, image: &Image) -> Option<u32> {
        let handle = image.vk_image.as_raw();
        for i in 0..self.images.len() {
            if self.images[i].vk_image.as_raw() == handle {
                return Some(i as _);
            }
        }
        None
    }

    // DONE VUID-vkAcquireNextImageKHR-fence-01287
    // DONE VUID-vkAcquireNextImageKHR-semaphore-01780
    // DONE VUID-vkAcquireNextImageKHR-semaphore-03265
    pub fn acquire_next_image_both(
        &mut self,
        timeout: u64,
        semaphore: &Semaphore,
        fence: UnsignaledFence,
    ) -> Result<(Arc<Image<{ Bound }>>, SignalingFence<()>), ash::vk::Result> {
        unsafe {
            // Host Synchronization: swapchain semaphore fence
            let (index, _) = self.swapchain_loader.acquire_next_image(
                self.vk_swapchain,
                timeout,
                semaphore.ash_vk_semaphore,
                fence.vk_fence,
            )?;
            let image = self.images[index as usize].clone();
            let fence = fence.to_executing_fence(());
            Ok((image, fence))
        }
    }

    pub fn acquire_next_image_semaphore_only(
        &mut self,
        timeout: u64,
        semaphore: &Semaphore,
    ) -> Result<Arc<Image<{ Bound }>>, ash::vk::Result> {
        unsafe {
            // Host Synchronization: swapchain, semaphore, fence
            let (index, _) = self.swapchain_loader.acquire_next_image(
                self.vk_swapchain,
                timeout,
                semaphore.ash_vk_semaphore,
                ash::vk::Fence::null(),
            )?;
            let image = self.images[index as usize].clone();
            Ok(image)
        }
    }

    pub fn acquire_next_image_fence_only(
        &mut self,
        timeout: u64,
        fence: UnsignaledFence,
    ) -> Result<(Arc<Image<{ Bound }>>, SignalingFence<()>), ash::vk::Result> {
        let (index, _) = unsafe {
            // Host Synchronization: swapchain, semaphore, fence
            self.swapchain_loader.acquire_next_image(
                self.vk_swapchain,
                timeout,
                ash::vk::Semaphore::null(),
                fence.vk_fence,
            )?
        };
        let image = self.images[index as usize].clone();
        let fence = fence.to_executing_fence(());
        Ok((image, fence))
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            // TODO later VUID-vkDestroySwapchainKHR-swapchain-01282
            // TODO VUID-vkDestroySwapchainKHR-swapchain-01283
            // TODO VUID-vkDestroySwapchainKHR-swapchain-01284
            // Host Synchronization: swapchain
            self.swapchain_loader
                .destroy_swapchain(self.vk_swapchain, None);
        }
    }
}
