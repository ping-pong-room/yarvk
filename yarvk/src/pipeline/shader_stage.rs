use crate::device_features::physical_device_features::{
    FeatureGeometryShader, FeatureTessellationShader,
};
use crate::device_features::physical_device_mesh_shader_features_nv::{
    FeatureMeshShader, FeatureTaskShader,
};
use crate::shader_module::ShaderModule;
use ash::vk::SpecializationMapEntry;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct SpecializationInfo<'a> {
    vk_specialization_info: ash::vk::SpecializationInfo,
    _phantom_data: PhantomData<&'a usize>,
}

impl<'a> SpecializationInfo<'a> {
    pub fn new(data: &'a [u8], map_entries: &'a [SpecializationMapEntry]) -> Self {
        Self {
            vk_specialization_info: ash::vk::SpecializationInfo::builder()
                .map_entries(map_entries)
                .data(data)
                .build(),
            _phantom_data: Default::default(),
        }
    }
}

/* DONE VUID-VkPipelineShaderStageCreateInfo-stage-00704
    VUID-VkPipelineShaderStageCreateInfo-stage-00705
    VUID-VkPipelineShaderStageCreateInfo-stage-02091
    VUID-VkPipelineShaderStageCreateInfo-stage-02092
    VUID-VkPipelineShaderStageCreateInfo-stage-00706
*/
pub enum ShaderStage {
    Vertex,
    TessellationControl(FeatureTessellationShader),
    TessellationEvaluation(FeatureTessellationShader),
    Geometry(FeatureGeometryShader),
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
    TaskNv(FeatureTaskShader),
    MeshNv(FeatureMeshShader),
    SubpassShadingHuawei,
}

impl ShaderStage {
    pub(crate) fn to_ash(&self) -> ash::vk::ShaderStageFlags {
        match self {
            ShaderStage::Vertex => ash::vk::ShaderStageFlags::VERTEX,
            ShaderStage::TessellationControl(_) => ash::vk::ShaderStageFlags::TESSELLATION_CONTROL,
            ShaderStage::TessellationEvaluation(_) => {
                ash::vk::ShaderStageFlags::TESSELLATION_EVALUATION
            }
            ShaderStage::Geometry(_) => ash::vk::ShaderStageFlags::GEOMETRY,
            ShaderStage::Fragment => ash::vk::ShaderStageFlags::FRAGMENT,
            ShaderStage::Compute => ash::vk::ShaderStageFlags::COMPUTE,
            ShaderStage::RaygenKhr => ash::vk::ShaderStageFlags::RAYGEN_KHR,
            ShaderStage::AnyHitKhr => ash::vk::ShaderStageFlags::ANY_HIT_KHR,
            ShaderStage::ClosestHitKhr => ash::vk::ShaderStageFlags::CLOSEST_HIT_KHR,
            ShaderStage::MissKhr => ash::vk::ShaderStageFlags::MISS_KHR,
            ShaderStage::IntersectionKhr => ash::vk::ShaderStageFlags::INTERSECTION_KHR,
            ShaderStage::CallableKhr => ash::vk::ShaderStageFlags::CALLABLE_KHR,
            ShaderStage::TaskNv(_) => ash::vk::ShaderStageFlags::TASK_NV,
            ShaderStage::MeshNv(_) => ash::vk::ShaderStageFlags::MESH_NV,
            ShaderStage::SubpassShadingHuawei => ash::vk::ShaderStageFlags::SUBPASS_SHADING_HUAWEI,
        }
    }
}

pub struct PipelineShaderStageCreateInfo<'a> {
    flags: ash::vk::PipelineShaderStageCreateFlags,
    pub(crate) stage: ash::vk::ShaderStageFlags,
    pub(crate) module: Arc<ShaderModule>,
    name: &'a ::std::ffi::CStr,
    specialization_info: Option<SpecializationInfo<'a>>,
}

impl<'a> PipelineShaderStageCreateInfo<'a> {
    pub(crate) fn ash_builder(&self) -> ash::vk::PipelineShaderStageCreateInfo {
        let mut builder = ash::vk::PipelineShaderStageCreateInfo::builder()
            .flags(self.flags)
            .module(self.module.ash_vk_shader_module)
            .stage(self.stage)
            .name(self.name);
        if let Some(specialization_info) = &self.specialization_info {
            builder = builder.specialization_info(&specialization_info.vk_specialization_info)
        }
        builder.build()
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
                specialization_info: None,
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
    pub fn stage(mut self, stage: ShaderStage) -> Self {
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
    pub fn specialization_info(mut self, specialization_info: SpecializationInfo<'a>) -> Self {
        self.inner.specialization_info = Some(specialization_info);
        self
    }
    pub fn build(self) -> PipelineShaderStageCreateInfo<'a> {
        self.inner
    }
}
