/* DONE
    VUID-VkSubpassDependency-srcStageMask-04091
    VUID-VkSubmitInfo-pWaitDstStageMask-04091
    VUID-VkSubpassDependency-dstStageMask-04091
    VUID-VkSubmitInfo-pWaitDstStageMask-04090
    VUID-VkSubpassDependency-dstStageMask-04090
    VUID-VkSubpassDependency-srcStageMask-04090
    VUID-VkSubpassDependency-srcStageMask-04092
    VUID-VkSubpassDependency-dstStageMask-04092
    VUID-VkSubmitInfo-pWaitDstStageMask-04092
    VUID-VkSubpassDependency-srcStageMask-04093
    VUID-VkSubpassDependency-dstStageMask-04093
    VUID-VkSubpassDependency-dstStageMask-04094
    VUID-VkSubpassDependency-srcStageMask-04094
    VUID-VkSubpassDependency-dstStageMask-04095
    VUID-VkSubpassDependency-srcStageMask-04095
    VUID-VkSubpassDependency-dstStageMask-04096
    VUID-VkSubpassDependency-srcStageMask-04096
    VUID-VkSubpassDependency-dstStageMask-04097
    VUID-VkSubpassDependency-srcStageMask-04097
    VUID-VkSubpassDependency-srcStageMask-03937
    VUID-VkSubpassDependency-dstStageMask-03937
*/
use crate::device_features::physical_device_features::{
    FeatureGeometryShader, FeatureTessellationShader,
};
use crate::device_features::physical_device_fragment_density_map_features_ext::FeatureFragmentDensityMap;
use crate::device_features::physical_device_mesh_shader_features_nv::{
    FeatureMeshShader, FeatureTaskShader,
};
use crate::device_features::physical_device_shading_rate_image_features_nv::FeatureShadingRateImage;
use crate::device_features::physical_device_synchronization2_features::FeatureSynchronization2;
use crate::device_features::physical_device_transform_feedback_features_ext::FeatureTransformFeedback;

pub enum PipelineStageFlag {
    TopOfPipe,
    DrawIndirect,
    VertexInput,
    VertexShader,
    TessellationControlShader(FeatureTessellationShader),
    TessellationEvaluationShader(FeatureTessellationShader),
    GeometryShader(FeatureGeometryShader),
    FragmentShader,
    EarlyFragmentTests,
    LateFragmentTests,
    ColorAttachmentOutput,
    ComputeShader,
    Transfer,
    BottomOfPipe,
    Host,
    AllGraphics,
    AllCommands,
    NONE(FeatureSynchronization2),
    TransformFeedbackExt(FeatureTransformFeedback),
    ConditionalRenderingExt(),
    AccelerationStructureBuildKhr,
    RayTracingShaderKhr,
    TaskShaderNv(FeatureTaskShader),
    MeshShaderNv(FeatureMeshShader),
    FragmentDensityProcessExt(FeatureFragmentDensityMap),
    FragmentShadingRateAttachmentKhr,
    CommandPreprocessNv,
    ShadingRateImageNv(FeatureShadingRateImage),
    RayTracingShaderNv,
    AccelerationStructureBuildNv,
    NoneKhr(FeatureSynchronization2),
}

impl PipelineStageFlag {
    pub(crate) fn to_ash(&self) -> ash::vk::PipelineStageFlags {
        match self {
            PipelineStageFlag::TopOfPipe => ash::vk::PipelineStageFlags::TOP_OF_PIPE,
            PipelineStageFlag::DrawIndirect => ash::vk::PipelineStageFlags::DRAW_INDIRECT,
            PipelineStageFlag::VertexInput => ash::vk::PipelineStageFlags::VERTEX_INPUT,
            PipelineStageFlag::VertexShader => ash::vk::PipelineStageFlags::VERTEX_SHADER,
            PipelineStageFlag::TessellationControlShader(..) => {
                ash::vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
            }
            PipelineStageFlag::TessellationEvaluationShader(..) => {
                ash::vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER
            }
            PipelineStageFlag::GeometryShader(..) => ash::vk::PipelineStageFlags::GEOMETRY_SHADER,
            PipelineStageFlag::FragmentShader => ash::vk::PipelineStageFlags::FRAGMENT_SHADER,
            PipelineStageFlag::EarlyFragmentTests => {
                ash::vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
            }
            PipelineStageFlag::LateFragmentTests => {
                ash::vk::PipelineStageFlags::LATE_FRAGMENT_TESTS
            }
            PipelineStageFlag::ColorAttachmentOutput => {
                ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
            }
            PipelineStageFlag::ComputeShader => ash::vk::PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlag::Transfer => ash::vk::PipelineStageFlags::TRANSFER,
            PipelineStageFlag::BottomOfPipe => ash::vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            PipelineStageFlag::Host => ash::vk::PipelineStageFlags::HOST,
            PipelineStageFlag::AllGraphics => ash::vk::PipelineStageFlags::ALL_GRAPHICS,
            PipelineStageFlag::AllCommands => ash::vk::PipelineStageFlags::ALL_COMMANDS,
            PipelineStageFlag::NONE(..) => ash::vk::PipelineStageFlags::NONE,
            PipelineStageFlag::TransformFeedbackExt(..) => {
                ash::vk::PipelineStageFlags::TRANSFORM_FEEDBACK_EXT
            }
            PipelineStageFlag::ConditionalRenderingExt(..) => {
                ash::vk::PipelineStageFlags::CONDITIONAL_RENDERING_EXT
            }
            PipelineStageFlag::AccelerationStructureBuildKhr => {
                ash::vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR
            }
            PipelineStageFlag::RayTracingShaderKhr => {
                ash::vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR
            }
            PipelineStageFlag::TaskShaderNv(..) => ash::vk::PipelineStageFlags::TASK_SHADER_NV,
            PipelineStageFlag::MeshShaderNv(..) => ash::vk::PipelineStageFlags::MESH_SHADER_NV,
            PipelineStageFlag::FragmentDensityProcessExt(..) => {
                ash::vk::PipelineStageFlags::FRAGMENT_DENSITY_PROCESS_EXT
            }
            PipelineStageFlag::FragmentShadingRateAttachmentKhr => {
                ash::vk::PipelineStageFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR
            }
            PipelineStageFlag::CommandPreprocessNv => {
                ash::vk::PipelineStageFlags::COMMAND_PREPROCESS_NV
            }
            PipelineStageFlag::ShadingRateImageNv(..) => {
                ash::vk::PipelineStageFlags::SHADING_RATE_IMAGE_NV
            }
            PipelineStageFlag::RayTracingShaderNv => {
                ash::vk::PipelineStageFlags::RAY_TRACING_SHADER_NV
            }
            PipelineStageFlag::AccelerationStructureBuildNv => {
                ash::vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_NV
            }
            PipelineStageFlag::NoneKhr(..) => ash::vk::PipelineStageFlags::NONE_KHR,
        }
    }
}

impl From<PipelineStageFlag> for PipelineStageFlags {
    fn from(value: PipelineStageFlag) -> Self {
        Self {
            vk_flags: value.to_ash(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct PipelineStageFlags {
    pub(crate) vk_flags: ash::vk::PipelineStageFlags,
}

impl PipelineStageFlags {
    pub fn new(flag: PipelineStageFlag) -> Self {
        Self {
            vk_flags: flag.to_ash(),
        }
    }
    pub fn add(&mut self, flag: PipelineStageFlag) {
        self.vk_flags |= flag.to_ash()
    }
}
