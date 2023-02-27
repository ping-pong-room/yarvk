use crate::binding_resource::BindingResource;
use crate::Image;
use ash::vk::{ImageSubresource, SubresourceLayout};
use derive_more::{Deref, DerefMut};

#[derive(Deref, DerefMut, Default, Clone)]
pub struct ImageSubresourceRange(pub(crate) ash::vk::ImageSubresourceRange);

impl ImageSubresourceRange {
    pub fn builder() -> ImageSubresourceRangeBuilder {
        ImageSubresourceRangeBuilder::default()
    }
}

#[derive(Default)]
pub struct ImageSubresourceRangeBuilder {
    aspect_mask: ash::vk::ImageAspectFlags,
    base_mip_level: u32,
    level_count: u32,
    base_array_layer: u32,
    layer_count: u32,
}

impl ImageSubresourceRangeBuilder {
    pub fn aspect_mask(mut self, aspect_mask: ash::vk::ImageAspectFlags) -> Self {
        self.aspect_mask = aspect_mask;
        self
    }

    pub fn base_mip_level(mut self, base_mip_level: u32) -> Self {
        self.base_mip_level = base_mip_level;
        self
    }

    pub fn level_count(mut self, level_count: u32) -> Self {
        self.level_count = level_count;
        self
    }

    pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
        self.base_array_layer = base_array_layer;
        self
    }

    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.layer_count = layer_count;
        self
    }

    pub fn build(self) -> ImageSubresourceRange {
        // DONE VUID-VkImageSubresourceRange-levelCount-01720
        // DONE VUID-VkImageSubresourceRange-layerCount-01721
        ImageSubresourceRange(
            ash::vk::ImageSubresourceRange::builder()
                .aspect_mask(self.aspect_mask)
                .base_mip_level(self.base_mip_level)
                .level_count(self.level_count)
                .base_array_layer(self.base_array_layer)
                .layer_count(self.layer_count)
                .build(),
        )
    }
}

pub trait GetImageSubresourceLayout: BindingResource<RawTy = Image> {
    fn get_image_subresource_layout(&self, subresource: ImageSubresource) -> SubresourceLayout {
        unsafe {
            self.device()
                .ash_device
                .get_image_subresource_layout(self.raw().vk_image, subresource)
        }
    }
}

impl<T: BindingResource<RawTy = Image> + ?Sized> GetImageSubresourceLayout for T {}
