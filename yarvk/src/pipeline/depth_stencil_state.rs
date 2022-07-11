use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceFeatures::DepthBounds;
use crate::device_features::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM::{
    RasterizationOrderDepthAttachmentAccess, RasterizationOrderStencilAttachmentAccess,
};

pub enum PipelineDepthStencilStateCreateFlags {
    RasterizationOrderAttachmentDepthAccessArm(
        Feature<{ RasterizationOrderDepthAttachmentAccess.into() }>,
    ),
    RasterizationOrderAttachmentStencilAccessArm(
        Feature<{ RasterizationOrderStencilAttachmentAccess.into() }>,
    ),
}

impl PipelineDepthStencilStateCreateFlags {
    pub(crate) fn to_ash(&self) -> ash::vk::PipelineDepthStencilStateCreateFlags {
        match self {
            // DONE VUID-VkPipelineDepthStencilStateCreateInfo-rasterizationOrderDepthAttachmentAccess-06463
            PipelineDepthStencilStateCreateFlags::RasterizationOrderAttachmentDepthAccessArm(_) => { ash::vk::PipelineDepthStencilStateCreateFlags::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM }
            // DONE VUID-VkPipelineDepthStencilStateCreateInfo-rasterizationOrderStencilAttachmentAccess-06464
            PipelineDepthStencilStateCreateFlags::RasterizationOrderAttachmentStencilAccessArm(_) => { ash::vk::PipelineDepthStencilStateCreateFlags::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM }
        }
    }
}

#[derive(Default)]
pub struct PipelineDepthStencilStateCreateInfo {
    flags: ash::vk::PipelineDepthStencilStateCreateFlags,
    depth_test_enable: bool,
    depth_write_enable: bool,
    depth_compare_op: ash::vk::CompareOp,
    depth_bounds_test_enable: bool,
    stencil_test_enable: bool,
    front: ash::vk::StencilOpState,
    back: ash::vk::StencilOpState,
    depth_bounds: Option<(/*min*/ f32, /*manx*/ f32)>,
}

impl PipelineDepthStencilStateCreateInfo {
    pub(crate) fn ash_builder(&self) -> ash::vk::PipelineDepthStencilStateCreateInfoBuilder {
        let mut builder = ash::vk::PipelineDepthStencilStateCreateInfo::builder()
            .flags(self.flags)
            .depth_test_enable(self.depth_test_enable)
            .depth_write_enable(self.depth_write_enable)
            .depth_compare_op(self.depth_compare_op)
            .depth_bounds_test_enable(self.depth_bounds_test_enable)
            .stencil_test_enable(self.stencil_test_enable)
            .front(self.front)
            .back(self.back);
        if let Some(depth_bounds) = self.depth_bounds {
            builder = builder
                .min_depth_bounds(depth_bounds.0)
                .max_depth_bounds(depth_bounds.1);
        }
        builder
    }
    pub fn builder() -> PipelineDepthStencilStateCreateInfoBuilder {
        PipelineDepthStencilStateCreateInfoBuilder {
            inner: Default::default(),
        }
    }
}

pub struct PipelineDepthStencilStateCreateInfoBuilder {
    inner: PipelineDepthStencilStateCreateInfo,
}

impl PipelineDepthStencilStateCreateInfoBuilder {
    pub fn add_flag(mut self, flag: PipelineDepthStencilStateCreateFlags) -> Self {
        self.inner.flags |= flag.to_ash();
        self
    }
    pub fn depth_test_enable(mut self) -> Self {
        self.inner.depth_test_enable = true;
        self
    }
    pub fn depth_write_enable(mut self) -> Self {
        self.inner.depth_write_enable = true;
        self
    }
    pub fn depth_compare_op(mut self, depth_compare_op: ash::vk::CompareOp) -> Self {
        self.inner.depth_compare_op = depth_compare_op;
        self
    }
    // DONE VUID-VkPipelineDepthStencilStateCreateInfo-depthBoundsTestEnable-00598
    pub fn depth_bounds_test_enable(mut self, _feature: Feature<{ DepthBounds.into() }>) -> Self {
        self.inner.depth_bounds_test_enable = true;
        self
    }
    pub fn stencil_test_enable(mut self) -> Self {
        self.inner.stencil_test_enable = true;
        self
    }
    pub fn front(mut self, front: ash::vk::StencilOpState) -> Self {
        self.inner.front = front;
        self
    }
    pub fn back(mut self, back: ash::vk::StencilOpState) -> Self {
        self.inner.back = back;
        self
    }
    pub fn depth_bounds(mut self, min_depth_bounds: f32, max_depth_bounds: f32) -> Self {
        self.inner.depth_bounds = Some((min_depth_bounds, max_depth_bounds));
        self
    }
    pub fn build(self) -> PipelineDepthStencilStateCreateInfo {
        self.inner
    }
}
