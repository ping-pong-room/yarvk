use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceFeatures::{AlphaToOne, SampleRateShading};

pub struct PipelineMultisampleStateCreateInfo {
    rasterization_samples: ash::vk::SampleCountFlags,
    sample_shading_enable: bool,
    min_sample_shading: f32,
    sample_mask: [u32; 2],
    alpha_to_coverage_enable: bool,
    alpha_to_one_enable: bool,
}

impl Default for PipelineMultisampleStateCreateInfo {
    fn default() -> Self {
        PipelineMultisampleStateCreateInfo {
            rasterization_samples: ash::vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: false,
            min_sample_shading: 0.0,
            sample_mask: [0xFFFF, 0xFFFF],
            alpha_to_coverage_enable: false,
            alpha_to_one_enable: false,
        }
    }
}

impl PipelineMultisampleStateCreateInfo {
    pub(crate) fn ash_builder(&self) -> ash::vk::PipelineMultisampleStateCreateInfoBuilder {
        ash::vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(self.rasterization_samples)
            .sample_shading_enable(self.sample_shading_enable)
            .min_sample_shading(self.min_sample_shading)
            .sample_mask(self.sample_mask.as_slice())
            .alpha_to_coverage_enable(self.alpha_to_coverage_enable)
            .alpha_to_one_enable(self.alpha_to_one_enable)
    }

    pub fn builder() -> PipelineMultisampleStateCreateInfoBuilder {
        PipelineMultisampleStateCreateInfoBuilder {
            inner: Default::default(),
        }
    }
}

pub struct PipelineMultisampleStateCreateInfoBuilder {
    inner: PipelineMultisampleStateCreateInfo,
}

impl PipelineMultisampleStateCreateInfoBuilder {
    pub fn rasterization_samples(
        mut self,
        rasterization_samples: ash::vk::SampleCountFlags,
    ) -> Self {
        self.inner.rasterization_samples = rasterization_samples;
        self
    }
    // DONE VUID-VkPipelineMultisampleStateCreateInfo-sampleShadingEnable-00784
    pub fn sample_shading_enable(
        mut self,
        _feature: Feature<{ SampleRateShading.into() }>,
    ) -> Self {
        self.inner.sample_shading_enable = true;
        self
    }
    pub fn min_sample_shading(mut self, min_sample_shading: f32) -> Self {
        self.inner.min_sample_shading = min_sample_shading;
        self
    }

    pub fn sample_mask(mut self, sample_mask: u64) -> Self {
        let sample_mask = sample_mask;
        self.inner.sample_mask = unsafe { std::mem::transmute(sample_mask.to_le_bytes()) };
        self
    }
    pub fn alpha_to_coverage_enable(mut self) -> Self {
        self.inner.alpha_to_coverage_enable = true;
        self
    }
    // DONE VUID-VkPipelineMultisampleStateCreateInfo-alphaToOneEnable-00785
    pub fn alpha_to_one_enable(mut self, _feature: Feature<{ AlphaToOne.into() }>) -> Self {
        self.inner.alpha_to_one_enable = true;
        self
    }
    pub fn build(self) -> PipelineMultisampleStateCreateInfo {
        self.inner
    }
}
