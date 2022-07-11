
use crate::device_features::PhysicalDeviceFeatures::{GeometryShader, TessellationShader};
use crate::device_features::PhysicalDeviceFragmentDensityMapFeaturesEXT::FragmentDensityMap;
use crate::device_features::PhysicalDeviceMeshShaderFeaturesNV::{MeshShader, TaskShader};
use crate::device_features::PhysicalDeviceShadingRateImageFeaturesNV::ShadingRateImage;
use crate::device_features::PhysicalDeviceTransformFeedbackFeaturesEXT::TransformFeedback;
use crate::device_features::PhysicalDeviceVulkan13Features::Synchronization2;
use crate::device_features::{Feature};

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
pub enum PipelineStageFlags {
    TopOfPipe,
    DrawIndirect,
    VertexInput,
    VertexShader,
    TessellationControlShader(Feature<{ TessellationShader.into() }>),
    TessellationEvaluationShader(Feature<{ TessellationShader.into() }>),
    GeometryShader(Feature<{ GeometryShader.into() }>),
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
    NONE(Feature<{ Synchronization2.into() }>),
    TransformFeedbackExt(Feature<{ TransformFeedback.into() }>),
    ConditionalRenderingExt(),
    AccelerationStructureBuildKhr,
    RayTracingShaderKhr,
    TaskShaderNv(Feature<{ TaskShader.into() }>),
    MeshShaderNv(Feature<{ MeshShader.into() }>),
    FragmentDensityProcessExt(Feature<{ FragmentDensityMap.into() }>),
    FragmentShadingRateAttachmentKhr,
    CommandPreprocessNv,
    ShadingRateImageNv(Feature<{ ShadingRateImage.into() }>),
    RayTracingShaderNv,
    AccelerationStructureBuildNv,
    NoneKhr(Feature<{ Synchronization2.into() }>),
}

impl PipelineStageFlags {
    pub(crate) fn to_ash(&self) -> ash::vk::PipelineStageFlags {
        match self {
            PipelineStageFlags::TopOfPipe => ash::vk::PipelineStageFlags::TOP_OF_PIPE,
            PipelineStageFlags::DrawIndirect => ash::vk::PipelineStageFlags::DRAW_INDIRECT,
            PipelineStageFlags::VertexInput => ash::vk::PipelineStageFlags::VERTEX_INPUT,
            PipelineStageFlags::VertexShader => ash::vk::PipelineStageFlags::VERTEX_SHADER,
            PipelineStageFlags::TessellationControlShader(..) => {
                ash::vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
            }
            PipelineStageFlags::TessellationEvaluationShader(..) => {
                ash::vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER
            }
            PipelineStageFlags::GeometryShader(..) => ash::vk::PipelineStageFlags::GEOMETRY_SHADER,
            PipelineStageFlags::FragmentShader => ash::vk::PipelineStageFlags::FRAGMENT_SHADER,
            PipelineStageFlags::EarlyFragmentTests => {
                ash::vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
            }
            PipelineStageFlags::LateFragmentTests => {
                ash::vk::PipelineStageFlags::LATE_FRAGMENT_TESTS
            }
            PipelineStageFlags::ColorAttachmentOutput => {
                ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
            }
            PipelineStageFlags::ComputeShader => ash::vk::PipelineStageFlags::COMPUTE_SHADER,
            PipelineStageFlags::Transfer => ash::vk::PipelineStageFlags::TRANSFER,
            PipelineStageFlags::BottomOfPipe => ash::vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            PipelineStageFlags::Host => ash::vk::PipelineStageFlags::HOST,
            PipelineStageFlags::AllGraphics => ash::vk::PipelineStageFlags::ALL_GRAPHICS,
            PipelineStageFlags::AllCommands => ash::vk::PipelineStageFlags::ALL_COMMANDS,
            PipelineStageFlags::NONE(..) => ash::vk::PipelineStageFlags::NONE,
            PipelineStageFlags::TransformFeedbackExt(..) => {
                ash::vk::PipelineStageFlags::TRANSFORM_FEEDBACK_EXT
            }
            PipelineStageFlags::ConditionalRenderingExt(..) => {
                ash::vk::PipelineStageFlags::CONDITIONAL_RENDERING_EXT
            }
            PipelineStageFlags::AccelerationStructureBuildKhr => {
                ash::vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR
            }
            PipelineStageFlags::RayTracingShaderKhr => {
                ash::vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR
            }
            PipelineStageFlags::TaskShaderNv(..) => ash::vk::PipelineStageFlags::TASK_SHADER_NV,
            PipelineStageFlags::MeshShaderNv(..) => ash::vk::PipelineStageFlags::MESH_SHADER_NV,
            PipelineStageFlags::FragmentDensityProcessExt(..) => {
                ash::vk::PipelineStageFlags::FRAGMENT_DENSITY_PROCESS_EXT
            }
            PipelineStageFlags::FragmentShadingRateAttachmentKhr => {
                ash::vk::PipelineStageFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR
            }
            PipelineStageFlags::CommandPreprocessNv => {
                ash::vk::PipelineStageFlags::COMMAND_PREPROCESS_NV
            }
            PipelineStageFlags::ShadingRateImageNv(..) => {
                ash::vk::PipelineStageFlags::SHADING_RATE_IMAGE_NV
            }
            PipelineStageFlags::RayTracingShaderNv => {
                ash::vk::PipelineStageFlags::RAY_TRACING_SHADER_NV
            }
            PipelineStageFlags::AccelerationStructureBuildNv => {
                ash::vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_NV
            }
            PipelineStageFlags::NoneKhr(..) => ash::vk::PipelineStageFlags::NONE_KHR,
        }
    }
}
