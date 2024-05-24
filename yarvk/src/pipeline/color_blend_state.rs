use crate::device_features::physical_device_features::{FeatureDualSrcBlend, FeatureLogicOp};
use crate::device_features::physical_device_rasterization_order_attachment_access_features_arm::FeatureRasterizationOrderColorAttachmentAccess;

// DONE VUID-VkPipelineColorBlendAttachmentState-srcColorBlendFactor-00608
// DONE VUID-VkPipelineColorBlendAttachmentState-dstColorBlendFactor-00609
// DONE VUID-VkPipelineColorBlendAttachmentState-srcAlphaBlendFactor-00610
// DONE VUID-VkPipelineColorBlendAttachmentState-dstAlphaBlendFactor-00611
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SrcAlphaSaturate,
    Src1Color(FeatureDualSrcBlend),
    OneMinusSrc1Color(FeatureDualSrcBlend),
    Src1Alpha(FeatureDualSrcBlend),
    OneMinusSrc1Alpha(FeatureDualSrcBlend),
}

impl BlendFactor {
    pub(crate) fn to_ash(&self) -> ash::vk::BlendFactor {
        match self {
            BlendFactor::Zero => ash::vk::BlendFactor::ZERO,
            BlendFactor::One => ash::vk::BlendFactor::ONE,
            BlendFactor::SrcColor => ash::vk::BlendFactor::SRC_COLOR,
            BlendFactor::OneMinusSrcColor => ash::vk::BlendFactor::ONE_MINUS_SRC_COLOR,
            BlendFactor::DstColor => ash::vk::BlendFactor::DST_COLOR,
            BlendFactor::OneMinusDstColor => ash::vk::BlendFactor::ONE_MINUS_DST_COLOR,
            BlendFactor::SrcAlpha => ash::vk::BlendFactor::SRC_ALPHA,
            BlendFactor::OneMinusSrcAlpha => ash::vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            BlendFactor::DstAlpha => ash::vk::BlendFactor::DST_ALPHA,
            BlendFactor::OneMinusDstAlpha => ash::vk::BlendFactor::ONE_MINUS_DST_ALPHA,
            BlendFactor::ConstantColor => ash::vk::BlendFactor::CONSTANT_COLOR,
            BlendFactor::OneMinusConstantColor => ash::vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
            BlendFactor::ConstantAlpha => ash::vk::BlendFactor::CONSTANT_ALPHA,
            BlendFactor::OneMinusConstantAlpha => ash::vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA,
            BlendFactor::SrcAlphaSaturate => ash::vk::BlendFactor::SRC_ALPHA_SATURATE,
            BlendFactor::Src1Color(_) => ash::vk::BlendFactor::SRC1_COLOR,
            BlendFactor::OneMinusSrc1Color(_) => ash::vk::BlendFactor::ONE_MINUS_SRC1_COLOR,
            BlendFactor::Src1Alpha(_) => ash::vk::BlendFactor::SRC1_ALPHA,
            BlendFactor::OneMinusSrc1Alpha(_) => ash::vk::BlendFactor::ONE_MINUS_SRC1_ALPHA,
        }
    }
}

#[derive(Default)]
pub struct PipelineColorBlendAttachmentState {
    blend_enable: bool,
    src_color_blend_factor: ash::vk::BlendFactor,
    dst_color_blend_factor: ash::vk::BlendFactor,
    color_blend_op: ash::vk::BlendOp,
    src_alpha_blend_factor: ash::vk::BlendFactor,
    dst_alpha_blend_factor: ash::vk::BlendFactor,
    alpha_blend_op: ash::vk::BlendOp,
    color_write_mask: ash::vk::ColorComponentFlags,
}

impl PipelineColorBlendAttachmentState {
    pub fn builder() -> PipelineColorBlendAttachmentStateBuilder {
        PipelineColorBlendAttachmentStateBuilder {
            inner: Default::default(),
        }
    }
    pub(crate) fn ash_builder(&self) -> ash::vk::PipelineColorBlendAttachmentStateBuilder {
        ash::vk::PipelineColorBlendAttachmentState::builder()
            .blend_enable(self.blend_enable)
            .src_color_blend_factor(self.src_color_blend_factor)
            .dst_color_blend_factor(self.dst_color_blend_factor)
            .color_blend_op(self.color_blend_op)
            .src_alpha_blend_factor(self.src_alpha_blend_factor)
            .dst_alpha_blend_factor(self.dst_alpha_blend_factor)
            .alpha_blend_op(self.alpha_blend_op)
            .color_write_mask(self.color_write_mask)
    }
}

pub struct PipelineColorBlendAttachmentStateBuilder {
    inner: PipelineColorBlendAttachmentState,
}

impl PipelineColorBlendAttachmentStateBuilder {
    pub fn blend_enable(mut self) -> Self {
        self.inner.blend_enable = true;
        self
    }
    pub fn src_color_blend_factor(mut self, src_color_blend_factor: BlendFactor) -> Self {
        self.inner.src_color_blend_factor = src_color_blend_factor.to_ash();
        self
    }
    pub fn dst_color_blend_factor(mut self, dst_color_blend_factor: BlendFactor) -> Self {
        self.inner.dst_color_blend_factor = dst_color_blend_factor.to_ash();
        self
    }
    pub fn color_blend_op(mut self, color_blend_op: ash::vk::BlendOp) -> Self {
        self.inner.color_blend_op = color_blend_op;
        self
    }
    pub fn src_alpha_blend_factor(mut self, src_alpha_blend_factor: BlendFactor) -> Self {
        self.inner.src_alpha_blend_factor = src_alpha_blend_factor.to_ash();
        self
    }
    pub fn dst_alpha_blend_factor(mut self, dst_alpha_blend_factor: BlendFactor) -> Self {
        self.inner.dst_alpha_blend_factor = dst_alpha_blend_factor.to_ash();
        self
    }
    pub fn alpha_blend_op(mut self, alpha_blend_op: ash::vk::BlendOp) -> Self {
        self.inner.alpha_blend_op = alpha_blend_op;
        self
    }
    pub fn color_write_mask(mut self, color_write_mask: ash::vk::ColorComponentFlags) -> Self {
        self.inner.color_write_mask = color_write_mask;
        self
    }
    pub fn build(self) -> PipelineColorBlendAttachmentState {
        self.inner
    }
}

pub enum PipelineColorBlendStateCreateFlags {
    // DONE VUID-VkPipelineColorBlendStateCreateInfo-rasterizationOrderColorAttachmentAccess-06465
    RasterizationOrderAttachmentAccessArm(FeatureRasterizationOrderColorAttachmentAccess),
}

impl PipelineColorBlendStateCreateFlags {
    pub(crate) fn to_ash(&self) -> ash::vk::PipelineColorBlendStateCreateFlags {
        match self {
            PipelineColorBlendStateCreateFlags::RasterizationOrderAttachmentAccessArm(_) => {
                ash::vk::PipelineColorBlendStateCreateFlags::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM
            }
        }
    }
}

#[derive(Default)]
pub struct PipelineColorBlendStateCreateInfo {
    flags: ash::vk::PipelineColorBlendStateCreateFlags,
    logic_op: Option<ash::vk::LogicOp>,
    attachments: Vec<ash::vk::PipelineColorBlendAttachmentState>,
    blend_constants: Option<[f32; 4]>,
}

impl PipelineColorBlendStateCreateInfo {
    pub(crate) fn ash_builder(&self) -> ash::vk::PipelineColorBlendStateCreateInfoBuilder {
        let mut builder = ash::vk::PipelineColorBlendStateCreateInfo::builder()
            .flags(self.flags)
            .attachments(self.attachments.as_slice());
        if let Some(blend_constants) = self.blend_constants {
            builder = builder.blend_constants(blend_constants);
        }
        if let Some(logic_op) = self.logic_op {
            builder = builder.logic_op_enable(true).logic_op(logic_op);
        }
        builder
    }

    pub fn builder() -> PipelineColorBlendStateCreateInfoBuilder {
        PipelineColorBlendStateCreateInfoBuilder {
            inner: Default::default(),
        }
    }
}

pub struct PipelineColorBlendStateCreateInfoBuilder {
    inner: PipelineColorBlendStateCreateInfo,
}

impl PipelineColorBlendStateCreateInfoBuilder {
    pub fn add_flag(mut self, flag: PipelineColorBlendStateCreateFlags) -> Self {
        self.inner.flags |= flag.to_ash();
        self
    }
    // DONE VUID-VkPipelineColorBlendStateCreateInfo-logicOpEnable-00606
    pub fn logic_op(mut self, logic_op: ash::vk::LogicOp, _feature: FeatureLogicOp) -> Self {
        // SILENCE VUID-VkPipelineColorBlendStateCreateInfo-logicOpEnable-00607
        self.inner.logic_op = Some(logic_op);
        self
    }
    pub fn add_attachment(mut self, attachments: PipelineColorBlendAttachmentState) -> Self {
        self.inner
            .attachments
            .push(attachments.ash_builder().build());
        self
    }
    pub fn blend_constants(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.inner.blend_constants = Some([r, g, b, a]);
        self
    }
    pub fn build(self) -> PipelineColorBlendStateCreateInfo {
        self.inner
    }
}
