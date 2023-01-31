use crate::device_features::Feature;

#[derive(PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
    LineListWithAdjacency,
    LineStripWithAdjacency,
    TriangleListWithAdjacency,
    TriangleStripWithAdjacency,
    PatchList,
}

impl PrimitiveTopology {
    pub(crate) fn to_ash(&self) -> ash::vk::PrimitiveTopology {
        match self {
            PrimitiveTopology::PointList => ash::vk::PrimitiveTopology::POINT_LIST,
            PrimitiveTopology::LineList => ash::vk::PrimitiveTopology::LINE_LIST,
            PrimitiveTopology::LineStrip => ash::vk::PrimitiveTopology::LINE_STRIP,
            PrimitiveTopology::TriangleList => ash::vk::PrimitiveTopology::TRIANGLE_LIST,
            PrimitiveTopology::TriangleStrip => ash::vk::PrimitiveTopology::TRIANGLE_STRIP,
            PrimitiveTopology::TriangleFan => ash::vk::PrimitiveTopology::TRIANGLE_FAN,
            PrimitiveTopology::LineListWithAdjacency => {
                ash::vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY
            }
            PrimitiveTopology::LineStripWithAdjacency => {
                ash::vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY
            }
            PrimitiveTopology::TriangleListWithAdjacency => {
                ash::vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY
            }
            PrimitiveTopology::TriangleStripWithAdjacency => {
                ash::vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY
            }
            PrimitiveTopology::PatchList => ash::vk::PrimitiveTopology::PATCH_LIST,
        }
    }
}

pub struct RestartBuilder<const TOPOLOGY: PrimitiveTopology> {
    primitive_restart_enable: bool,
}

macro_rules! fn_topology_build {
    ($feature: expr, $($topology: expr),*) => {$(
        impl RestartBuilder<{ $topology }> {
            pub fn build(self, _feature: Feature<{ $feature.into() }>) -> PipelineInputAssemblyStateCreateInfo {
                PipelineInputAssemblyStateCreateInfo {
                    topology: $topology.to_ash(),
                    primitive_restart_enable: self.primitive_restart_enable,
                }
            }
        })*
    };
}
// DONE VUID-VkPipelineInputAssemblyStateCreateInfo-topology-00429
fn_topology_build!(
    crate::device_features::PhysicalDeviceFeatures::GeometryShader,
    PrimitiveTopology::LineListWithAdjacency,
    PrimitiveTopology::LineStripWithAdjacency,
    PrimitiveTopology::TriangleListWithAdjacency,
    PrimitiveTopology::TriangleStripWithAdjacency
);
// DONE VUID-VkPipelineInputAssemblyStateCreateInfo-topology-00430
fn_topology_build!(
    crate::device_features::PhysicalDeviceFeatures::TessellationShader,
    PrimitiveTopology::PatchList
);
// DONE VUID-VkPipelineInputAssemblyStateCreateInfo-triangleFans-04452
fn_topology_build!(
    crate::device_features::PhysicalDevicePortabilitySubsetFeaturesKHR::TriangleFans,
    PrimitiveTopology::TriangleFan
);
macro_rules! fn_topology_build_no_feature {
    ($($topology: expr),*) => {$(
        impl RestartBuilder<{ $topology }> {
            pub fn build(self) -> PipelineInputAssemblyStateCreateInfo {
                PipelineInputAssemblyStateCreateInfo {
                    topology: $topology.to_ash(),
                    primitive_restart_enable: self.primitive_restart_enable,
                }
            }
        })*
    };
}
fn_topology_build_no_feature!(
    PrimitiveTopology::PointList,
    PrimitiveTopology::LineList,
    PrimitiveTopology::LineStrip,
    PrimitiveTopology::TriangleList,
    PrimitiveTopology::TriangleStrip
);

macro_rules! fn_restart_enable {
    ($feature: expr, $($topology: expr),*) => {$(
        impl RestartBuilder<{$topology}> {
            pub fn restart_enable(mut self, _feature: Feature<{ $feature.into() }>) -> Self {
                self.primitive_restart_enable = true;
                self
            }
        }
    )*};
}

// DONE VUID-VkPipelineInputAssemblyStateCreateInfo-topology-06252
fn_restart_enable!(crate::device_features::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT::PrimitiveTopologyListRestart,
    PrimitiveTopology::PointList,
    PrimitiveTopology::LineList,
    PrimitiveTopology::TriangleList,
    PrimitiveTopology::LineListWithAdjacency,
    PrimitiveTopology::TriangleListWithAdjacency
);

// DONE VUID-VkPipelineInputAssemblyStateCreateInfo-topology-06253
fn_restart_enable!(crate::device_features::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT::PrimitiveTopologyPatchListRestart,
    PrimitiveTopology::PatchList
);
macro_rules! fn_restart_enable_no_feature {
    ($($topology: expr),*) => {$(
        impl RestartBuilder<{$topology}> {
            pub fn restart_enable(mut self) -> Self {
                self.primitive_restart_enable = true;
                self
            }
        }
    )*};
}
fn_restart_enable_no_feature!(
    PrimitiveTopology::LineStrip,
    PrimitiveTopology::TriangleStrip,
    PrimitiveTopology::TriangleFan,
    PrimitiveTopology::LineStripWithAdjacency,
    PrimitiveTopology::TriangleStripWithAdjacency
);

#[derive(Default)]
pub struct PipelineInputAssemblyStateCreateInfo {
    topology: ash::vk::PrimitiveTopology,
    primitive_restart_enable: bool,
}

impl PipelineInputAssemblyStateCreateInfo {
    pub fn builder() -> PipelineInputAssemblyStateCreateInfoBuilder {
        PipelineInputAssemblyStateCreateInfoBuilder::default()
    }
    pub(crate) fn ash_builder(&self) -> ash::vk::PipelineInputAssemblyStateCreateInfoBuilder {
        ash::vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(self.topology)
            .primitive_restart_enable(self.primitive_restart_enable)
    }
}

#[derive(Default)]
pub struct PipelineInputAssemblyStateCreateInfoBuilder {}

impl PipelineInputAssemblyStateCreateInfoBuilder {
    pub fn topology<const TOPOLOGY: PrimitiveTopology>(&self) -> RestartBuilder<TOPOLOGY> {
        RestartBuilder {
            primitive_restart_enable: false,
        }
    }
}
