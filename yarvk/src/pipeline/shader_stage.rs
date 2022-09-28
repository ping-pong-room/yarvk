use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceFeatures::{GeometryShader, TessellationShader};
use crate::device_features::PhysicalDeviceMeshShaderFeaturesNV::{MeshShader, TaskShader};
use crate::shader_module::ShaderModule;
use std::sync::Arc;
/* DONE VUID-VkPipelineShaderStageCreateInfo-stage-00704
    VUID-VkPipelineShaderStageCreateInfo-stage-00705
    VUID-VkPipelineShaderStageCreateInfo-stage-02091
    VUID-VkPipelineShaderStageCreateInfo-stage-02092
    VUID-VkPipelineShaderStageCreateInfo-stage-00706
*/
pub enum ShaderStageFlags {
    Vertex,
    TessellationControl(Feature<{ TessellationShader.into() }>),
    TessellationEvaluation(Feature<{ TessellationShader.into() }>),
    Geometry(Feature<{ GeometryShader.into() }>),
    Fragment,
    Compute,
    // according to the spec, all_graphics and all do not contain all stages, exclude them in yarvk
    // AllGraphics,
    // All,
    RaygenKhr,
    AnyHitKhr,
    ClosestHitKhr,
    MissKhr,
    IntersectionKhr,
    CallableKhr,
    TaskNv(Feature<{ TaskShader.into() }>),
    MeshNv(Feature<{ MeshShader.into() }>),
    SubpassShadingHuawei,
}

impl ShaderStageFlags {
    pub(crate) fn to_ash(&self) -> ash::vk::ShaderStageFlags {
        match self {
            ShaderStageFlags::Vertex => ash::vk::ShaderStageFlags::VERTEX,
            ShaderStageFlags::TessellationControl(_) => {
                ash::vk::ShaderStageFlags::TESSELLATION_CONTROL
            }
            ShaderStageFlags::TessellationEvaluation(_) => {
                ash::vk::ShaderStageFlags::TESSELLATION_EVALUATION
            }
            ShaderStageFlags::Geometry(_) => ash::vk::ShaderStageFlags::GEOMETRY,
            ShaderStageFlags::Fragment => ash::vk::ShaderStageFlags::FRAGMENT,
            ShaderStageFlags::Compute => ash::vk::ShaderStageFlags::COMPUTE,
            ShaderStageFlags::RaygenKhr => ash::vk::ShaderStageFlags::RAYGEN_KHR,
            ShaderStageFlags::AnyHitKhr => ash::vk::ShaderStageFlags::ANY_HIT_KHR,
            ShaderStageFlags::ClosestHitKhr => ash::vk::ShaderStageFlags::CLOSEST_HIT_KHR,
            ShaderStageFlags::MissKhr => ash::vk::ShaderStageFlags::MISS_KHR,
            ShaderStageFlags::IntersectionKhr => ash::vk::ShaderStageFlags::INTERSECTION_KHR,
            ShaderStageFlags::CallableKhr => ash::vk::ShaderStageFlags::CALLABLE_KHR,
            ShaderStageFlags::TaskNv(_) => ash::vk::ShaderStageFlags::TASK_NV,
            ShaderStageFlags::MeshNv(_) => ash::vk::ShaderStageFlags::MESH_NV,
            ShaderStageFlags::SubpassShadingHuawei => {
                ash::vk::ShaderStageFlags::SUBPASS_SHADING_HUAWEI
            }
        }
    }
}

pub struct PipelineShaderStageCreateInfo<'a> {
    flags: ash::vk::PipelineShaderStageCreateFlags,
    pub(crate) stage: ash::vk::ShaderStageFlags,
    pub(crate) module: Arc<ShaderModule>,
    name: &'a ::std::ffi::CStr,
}

impl<'a> PipelineShaderStageCreateInfo<'a> {
    pub(crate) fn ash_builder(&self) -> ash::vk::PipelineShaderStageCreateInfo {
        ash::vk::PipelineShaderStageCreateInfo::builder()
            .flags(self.flags)
            .module(self.module.ash_vk_shader_module)
            .stage(self.stage)
            .name(self.name)
            .build()
    }
    pub fn builder(
        module: Arc<ShaderModule>,
        name: &'a ::std::ffi::CStr,
    ) -> PipelineShaderStageCreateInfoBuilder<'a> {
        PipelineShaderStageCreateInfoBuilder {
            inner: PipelineShaderStageCreateInfo {
                flags: Default::default(),
                stage: Default::default(),
                module,
                name,
            },
        }
    }
}

pub struct PipelineShaderStageCreateInfoBuilder<'a> {
    inner: PipelineShaderStageCreateInfo<'a>,
}

impl<'a> PipelineShaderStageCreateInfoBuilder<'a> {
    pub fn flags(mut self, flags: ash::vk::PipelineShaderStageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn stage(mut self, stage: ShaderStageFlags) -> Self {
        self.inner.stage = stage.to_ash();
        self
    }
    pub fn module(mut self, module: Arc<ShaderModule>) -> Self {
        self.inner.module = module;
        self
    }
    pub fn name(mut self, name: &'a ::std::ffi::CStr) -> Self {
        self.inner.name = name;
        self
    }
    pub fn build(self) -> PipelineShaderStageCreateInfo<'a> {
        self.inner
    }
}
