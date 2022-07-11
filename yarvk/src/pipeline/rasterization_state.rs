use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceFeatures::{DepthClamp, FillModeNonSolid};
use crate::extensions::DeviceExtension;
use crate::extensions::PhysicalDeviceExtensionType::NvFillRectangle;


pub enum PolygonMode {
    Fill,
    // DONE VUID-VkPipelineRasterizationStateCreateInfo-polygonMode-01507
    Line(Feature<{ FillModeNonSolid.into() }>),
    Point(Feature<{ FillModeNonSolid.into() }>),
    // DONE VUID-VkPipelineRasterizationStateCreateInfo-polygonMode-01414
    FillRectangleNv(DeviceExtension<{ NvFillRectangle.into() }>),
}

impl PolygonMode {
    pub(crate) fn to_ash(&self) -> ash::vk::PolygonMode {
        match self {
            PolygonMode::Fill => ash::vk::PolygonMode::FILL,
            PolygonMode::Line(_) => ash::vk::PolygonMode::LINE,
            PolygonMode::Point(_) => ash::vk::PolygonMode::POINT,
            PolygonMode::FillRectangleNv(_) => ash::vk::PolygonMode::FILL_RECTANGLE_NV,
        }
    }
}

#[derive(Default)]
pub struct PipelineRasterizationStateCreateInfo {
    depth_clamp_enable: bool,
    rasterizer_discard_enable: bool,
    polygon_mode: ash::vk::PolygonMode,
    cull_mode: ash::vk::CullModeFlags,
    front_face: ash::vk::FrontFace,
    depth_bias_enable: bool,
    depth_bias_constant_factor: Option<f32>,
    depth_bias_clamp: Option<f32>,
    depth_bias_slope_factor: Option<f32>,
    line_width: Option<f32>,
}

impl PipelineRasterizationStateCreateInfo {
    pub fn builder() -> PipelineRasterizationStateCreateInfoBuilder {
        PipelineRasterizationStateCreateInfoBuilder {
            inner: Default::default(),
        }
    }
    pub(crate) fn ash_builder(&self) -> ash::vk::PipelineRasterizationStateCreateInfoBuilder {
        let mut builder = ash::vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(self.depth_clamp_enable)
            .rasterizer_discard_enable(self.rasterizer_discard_enable)
            .polygon_mode(self.polygon_mode)
            .cull_mode(self.cull_mode)
            .front_face(self.front_face)
            .depth_bias_enable(self.depth_bias_enable);
        if let Some(line_width) = self.line_width {
            builder = builder.line_width(line_width);
        }
        if let Some(constant_factor) = self.depth_bias_constant_factor {
            builder = builder.depth_bias_constant_factor(constant_factor);
        }
        if let Some(clamp) = self.depth_bias_clamp {
            builder = builder.depth_bias_clamp(clamp);
        }
        if let Some(slope_factor) = self.depth_bias_slope_factor {
            builder = builder.depth_bias_slope_factor(slope_factor);
        }
        builder
    }
}

pub struct PipelineRasterizationStateCreateInfoBuilder {
    inner: PipelineRasterizationStateCreateInfo,
}

impl PipelineRasterizationStateCreateInfoBuilder {
    // DONE VUID-VkPipelineRasterizationStateCreateInfo-depthClampEnable-00782
    pub fn depth_clamp_enable(
        mut self,
        depth_clamp_enable: bool,
        _feature: Feature<{ DepthClamp.into() }>,
    ) -> Self {
        self.inner.depth_clamp_enable = depth_clamp_enable.into();
        self
    }
    pub fn rasterizer_discard_enable(mut self) -> Self {
        self.inner.rasterizer_discard_enable = true;
        self
    }
    pub fn polygon_mode(mut self, polygon_mode: PolygonMode) -> Self {
        self.inner.polygon_mode = polygon_mode.to_ash();
        self
    }
    pub fn cull_mode(mut self, cull_mode: ash::vk::CullModeFlags) -> Self {
        self.inner.cull_mode = cull_mode;
        self
    }
    pub fn front_face(mut self, front_face: ash::vk::FrontFace) -> Self {
        self.inner.front_face = front_face;
        self
    }
    pub fn depth_bias(
        mut self,
        constant_factor: Option<f32>,
        clamp: Option<f32>,
        slope_factor: Option<f32>,
    ) -> Self {
        self.inner.depth_bias_enable = true;
        self.inner.depth_bias_constant_factor = constant_factor;
        self.inner.depth_bias_clamp = clamp;
        self.inner.depth_bias_slope_factor = slope_factor;
        self
    }
    pub fn line_width(mut self, line_width: f32) -> Self {
        self.inner.line_width = Some(line_width);
        self
    }
    pub fn build(self) -> PipelineRasterizationStateCreateInfo {
        self.inner
    }
}
