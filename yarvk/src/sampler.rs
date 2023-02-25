use crate::device::Device;
use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceFeatures::SamplerAnisotropy;
use crate::device_features::PhysicalDevicePortabilitySubsetFeaturesKHR::SamplerMipLodBias;
use std::sync::Arc;

pub struct Sampler {
    pub device: Arc<Device>,
    pub(crate) ash_vk_sampler: ash::vk::Sampler,
}

impl PartialEq for Sampler {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
            && self.ash_vk_sampler == other.ash_vk_sampler
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            // Host Synchronization: sampler
            self.device
                .ash_device
                .destroy_sampler(self.ash_vk_sampler, None);
        }
    }
}

impl Sampler {
    pub fn builder(device: &Arc<Device>) -> SamplerBuilder {
        SamplerBuilder {
            device: device.clone(),
            flags: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_mode: Default::default(),
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mip_lod_bias: 0.0,
            max_anisotropy: None,
            compare_op: None,
            min_lod: 0.0,
            max_lod: 0.0,
            border_color: Default::default(),
            unnormalized_coordinates: false,
        }
    }
}

#[derive(Clone)]
pub struct SamplerBuilder {
    device: Arc<Device>,
    flags: ash::vk::SamplerCreateFlags,
    mag_filter: ash::vk::Filter,
    min_filter: ash::vk::Filter,
    mipmap_mode: ash::vk::SamplerMipmapMode,
    address_mode_u: ash::vk::SamplerAddressMode,
    address_mode_v: ash::vk::SamplerAddressMode,
    address_mode_w: ash::vk::SamplerAddressMode,
    mip_lod_bias: f32,
    max_anisotropy: Option<f32>,
    compare_op: Option<ash::vk::CompareOp>,
    min_lod: f32,
    max_lod: f32,
    border_color: ash::vk::BorderColor,
    unnormalized_coordinates: bool,
}

impl SamplerBuilder {
    pub fn flags(mut self, flags: ash::vk::SamplerCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn mag_filter(mut self, mag_filter: ash::vk::Filter) -> Self {
        self.mag_filter = mag_filter;
        self
    }
    pub fn min_filter(mut self, min_filter: ash::vk::Filter) -> Self {
        self.min_filter = min_filter;
        self
    }
    pub fn mipmap_mode(mut self, mipmap_mode: ash::vk::SamplerMipmapMode) -> Self {
        self.mipmap_mode = mipmap_mode;
        self
    }
    pub fn address_mode_u(mut self, address_mode_u: ash::vk::SamplerAddressMode) -> Self {
        self.address_mode_u = address_mode_u;
        self
    }
    pub fn address_mode_v(mut self, address_mode_v: ash::vk::SamplerAddressMode) -> Self {
        self.address_mode_v = address_mode_v;
        self
    }
    pub fn address_mode_w(mut self, address_mode_w: ash::vk::SamplerAddressMode) -> Self {
        self.address_mode_w = address_mode_w;
        self
    }
    // DONE VUID-VkSamplerCreateInfo-samplerMipLodBias-04467
    pub fn mip_lod_bias(
        mut self,
        mip_lod_bias: f32,
        _feature: Feature<{ SamplerMipLodBias.into() }>,
    ) -> Self {
        self.mip_lod_bias = mip_lod_bias;
        self
    }
    // DONE VUID-VkSamplerCreateInfo-anisotropyEnable-01070
    pub fn max_anisotropy(
        mut self,
        max_anisotropy: f32,
        _feature: Feature<{ SamplerAnisotropy.into() }>,
    ) -> Self {
        self.max_anisotropy = Some(max_anisotropy);
        self
    }
    pub fn compare_op(mut self, compare_op: ash::vk::CompareOp) -> Self {
        self.compare_op = Some(compare_op);
        self
    }
    pub fn min_lod(mut self, min_lod: f32) -> Self {
        self.min_lod = min_lod;
        self
    }
    pub fn max_lod(mut self, max_lod: f32) -> Self {
        self.max_lod = max_lod;
        self
    }
    pub fn border_color(mut self, border_color: ash::vk::BorderColor) -> Self {
        self.border_color = border_color;
        self
    }
    pub fn unnormalized_coordinates(mut self) -> Self {
        self.unnormalized_coordinates = true;
        self
    }
    pub fn build(self) -> Result<Arc<Sampler>, ash::vk::Result> {
        let mut create_info_builder = ash::vk::SamplerCreateInfo::builder()
            .flags(self.flags)
            .mag_filter(self.mag_filter)
            .min_filter(self.min_filter)
            .mipmap_mode(self.mipmap_mode)
            .address_mode_u(self.address_mode_u)
            .address_mode_v(self.address_mode_v)
            .address_mode_w(self.address_mode_w)
            .mip_lod_bias(self.mip_lod_bias)
            .min_lod(self.min_lod)
            .max_lod(self.max_lod)
            .border_color(self.border_color)
            .unnormalized_coordinates(self.unnormalized_coordinates);

        if let Some(max_anisotropy) = self.max_anisotropy {
            create_info_builder = create_info_builder
                .max_anisotropy(max_anisotropy)
                .anisotropy_enable(true);
        }

        if let Some(compare_op) = self.compare_op {
            if compare_op != ash::vk::CompareOp::NEVER {
                create_info_builder = create_info_builder
                    .compare_op(compare_op)
                    .compare_enable(true);
            }
        }
        let create_info = create_info_builder.build();
        unsafe {
            // Host Synchronization: none
            let ash_vk_sampler = self.device.ash_device.create_sampler(&create_info, None)?;
            Ok(Arc::new(Sampler {
                device: self.device,
                ash_vk_sampler,
            }))
        }
    }
}
