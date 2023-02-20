use crate::device::Device;
use crate::device_features::{
    Feature, FeatureType, PhysicalDeviceFeatures, PhysicalDeviceFragmentDensityMap2FeaturesEXT,
    PhysicalDeviceFragmentDensityMapFeaturesEXT,
};
use crate::image::image_subresource_range::ImageSubresourceRange;
use crate::image::Image;
use std::sync::Arc;

pub enum ImageViewType {
    Type1d,
    Type2d,
    Type3d,
    TypeCube,
    Type1dArray,
    Type2dArray,
    // DONE VUID-VkImageViewCreateInfo-viewType-01004
    TypeCubeArray(Feature<{ FeatureType::DeviceFeatures(PhysicalDeviceFeatures::ImageCubeArray) }>),
}

impl ImageViewType {
    fn to_ash_type(&self) -> ash::vk::ImageViewType {
        match self {
            ImageViewType::Type1d => ash::vk::ImageViewType::TYPE_1D,
            ImageViewType::Type2d => ash::vk::ImageViewType::TYPE_2D,
            ImageViewType::Type3d => ash::vk::ImageViewType::TYPE_3D,
            ImageViewType::TypeCube => ash::vk::ImageViewType::CUBE,
            ImageViewType::Type1dArray => ash::vk::ImageViewType::TYPE_1D_ARRAY,
            ImageViewType::Type2dArray => ash::vk::ImageViewType::TYPE_2D_ARRAY,
            ImageViewType::TypeCubeArray(_) => ash::vk::ImageViewType::CUBE_ARRAY,
        }
    }
}

pub enum ImageViewCreateFlags {
    // DONE VUID-VkImageViewCreateInfo-flags-02572
    FragmentDensityMapDynamicBitExt(
        Feature<
            {
                FeatureType::DeviceFragmentDensityMapFeaturesEXT(
                    PhysicalDeviceFragmentDensityMapFeaturesEXT::FragmentDensityMapDynamic,
                )
            },
        >,
    ),
    // DONE VUID-VkImageViewCreateInfo-flags-03567
    FragmentDensityMapDeferredBitExt(
        Feature<
            {
                FeatureType::DeviceFragmentDensityMap2FeaturesEXT(
                    PhysicalDeviceFragmentDensityMap2FeaturesEXT::FragmentDensityMapDeferred,
                )
            },
        >,
    ),
}

impl ImageViewCreateFlags {
    fn to_ash_flags(&self) -> ash::vk::ImageViewCreateFlags {
        match self {
            ImageViewCreateFlags::FragmentDensityMapDynamicBitExt(_) => {
                ash::vk::ImageViewCreateFlags::FRAGMENT_DENSITY_MAP_DYNAMIC_EXT
            }
            ImageViewCreateFlags::FragmentDensityMapDeferredBitExt(_) => {
                ash::vk::ImageViewCreateFlags::FRAGMENT_DENSITY_MAP_DEFERRED_EXT
            }
        }
    }
}

pub struct ImageView {
    pub image: Arc<Image>,
    pub(crate) ash_vk_image_view: ash::vk::ImageView,
}

impl PartialEq for ImageView {
    fn eq(&self, other: &Self) -> bool {
        self.image.device == other.image.device
            && self.ash_vk_image_view == other.ash_vk_image_view
    }
}

impl ImageView {
    pub fn builder(image: Arc<Image>) -> ImageViewBuilder {
        ImageViewBuilder::new(image)
    }
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe {
            // TODO Host Synchronization: image
            self.image
                .device
                .ash_device
                .destroy_image_view(self.ash_vk_image_view, None);
        }
    }
}

pub struct ImageViewBuilder {
    device: Arc<Device>,
    image: Arc<Image>,
    view_type: ash::vk::ImageViewType,
    format: ash::vk::Format,
    flags: ash::vk::ImageViewCreateFlags,
    components: ash::vk::ComponentMapping,
    subresource_range: ImageSubresourceRange,
    vk_sampler_ycbcr_conversion_info: Option<ash::vk::SamplerYcbcrConversionInfo>,
}

impl ImageViewBuilder {
    fn new(image: Arc<Image>) -> Self {
        let device = image.device.clone();
        Self {
            device,
            image,
            view_type: Default::default(),
            flags: ash::vk::ImageViewCreateFlags::default(),
            format: Default::default(),
            components: ash::vk::ComponentMapping {
                r: ash::vk::ComponentSwizzle::IDENTITY,
                g: ash::vk::ComponentSwizzle::IDENTITY,
                b: ash::vk::ComponentSwizzle::IDENTITY,
                a: ash::vk::ComponentSwizzle::IDENTITY,
            },
            subresource_range: ImageSubresourceRange::default(),
            vk_sampler_ycbcr_conversion_info: None,
        }
    }
    pub fn view_type(mut self, view_type: ImageViewType) -> Self {
        self.view_type = view_type.to_ash_type();
        self
    }
    pub fn format(mut self, format: ash::vk::Format) -> Self {
        self.format = format;
        self
    }
    pub fn add_flag(mut self, flag: ImageViewCreateFlags) -> Self {
        self.flags |= flag.to_ash_flags();
        self
    }
    pub fn components(mut self, components: ash::vk::ComponentMapping) -> Self {
        self.components = components;
        self
    }
    pub fn subresource_range(mut self, subresource_range: ImageSubresourceRange) -> Self {
        self.subresource_range = subresource_range;
        self
    }
    pub fn sampler_ycbcr_conversion_info(
        mut self,
        sampler_ycbcr_conversion_info: ash::vk::SamplerYcbcrConversionInfo,
    ) -> Self {
        self.vk_sampler_ycbcr_conversion_info = Some(sampler_ycbcr_conversion_info);
        self
    }
    pub fn build(mut self) -> Result<Arc<ImageView>, ash::vk::Result> {
        let mut create_view_info = ash::vk::ImageViewCreateInfo::builder();
        create_view_info = create_view_info.image(self.image.vk_image);
        create_view_info = create_view_info.view_type(self.view_type);
        create_view_info = create_view_info.flags(self.flags);
        create_view_info = create_view_info.components(self.components);
        create_view_info = create_view_info.subresource_range(self.subresource_range.0);
        create_view_info = create_view_info.format(self.format);
        if let Some(vk_sampler_ycbcr_conversion_info) = &mut self.vk_sampler_ycbcr_conversion_info {
            create_view_info = create_view_info.push_next(vk_sampler_ycbcr_conversion_info);
        }
        let vk_image_view = unsafe {
            // Host Synchronization: none
            self.device
                .ash_device
                .create_image_view(&create_view_info, None)?
        };
        Ok(Arc::new(ImageView {
            image: self.image,
            ash_vk_image_view: vk_image_view,
        }))
    }
}
