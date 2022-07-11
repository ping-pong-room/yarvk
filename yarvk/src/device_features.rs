pub trait VkDeviceFeature {
    type SubFeatureEnumTy;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy>;
}
pub trait SubPhysicalFeature {
    type VkStruct: Default + ash::vk::ExtendsPhysicalDeviceFeatures2 + VkDeviceFeature;
    fn register(&self, vk_struct: &mut Self::VkStruct);
}
pub trait ToPhysicalFeature {
    type PhysicalDeviceFeatureTy;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy;
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    ShaderBufferFloat16Atomics,
    ShaderBufferFloat16AtomicAdd,
    ShaderBufferFloat16AtomicMinMax,
    ShaderBufferFloat32AtomicMinMax,
    ShaderBufferFloat64AtomicMinMax,
    ShaderSharedFloat16Atomics,
    ShaderSharedFloat16AtomicAdd,
    ShaderSharedFloat16AtomicMinMax,
    ShaderSharedFloat32AtomicMinMax,
    ShaderSharedFloat64AtomicMinMax,
    ShaderImageFloat32AtomicMinMax,
    SparseImageFloat32AtomicMinMax,
}
impl const From<PhysicalDeviceShaderAtomicFloat2FeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceShaderAtomicFloat2FeaturesEXT) -> Self {
        FeatureType::DeviceShaderAtomicFloat2FeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderAtomicFloat2FeaturesEXT {
    ShaderBufferFloat16Atomics,
    ShaderBufferFloat16AtomicAdd,
    ShaderBufferFloat16AtomicMinMax,
    ShaderBufferFloat32AtomicMinMax,
    ShaderBufferFloat64AtomicMinMax,
    ShaderSharedFloat16Atomics,
    ShaderSharedFloat16AtomicAdd,
    ShaderSharedFloat16AtomicMinMax,
    ShaderSharedFloat32AtomicMinMax,
    ShaderSharedFloat64AtomicMinMax,
    ShaderImageFloat32AtomicMinMax,
    SparseImageFloat32AtomicMinMax,
}
impl ToPhysicalFeature for DeviceShaderAtomicFloat2FeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat16Atomics => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat16Atomics
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat16AtomicAdd => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat16AtomicAdd
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat16AtomicMinMax => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat16AtomicMinMax
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat32AtomicMinMax => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat32AtomicMinMax
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat64AtomicMinMax => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat64AtomicMinMax
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat16Atomics => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat16Atomics
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat16AtomicAdd => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat16AtomicAdd
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat16AtomicMinMax => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat16AtomicMinMax
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat32AtomicMinMax => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat32AtomicMinMax
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat64AtomicMinMax => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat64AtomicMinMax
            }
            DeviceShaderAtomicFloat2FeaturesEXT::ShaderImageFloat32AtomicMinMax => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderImageFloat32AtomicMinMax
            }
            DeviceShaderAtomicFloat2FeaturesEXT::SparseImageFloat32AtomicMinMax => {
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::SparseImageFloat32AtomicMinMax
            }
        }
    }
}
impl From<DeviceShaderAtomicFloat2FeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceShaderAtomicFloat2FeaturesEXT) -> Self {
        DeviceFeature::DeviceShaderAtomicFloat2FeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_buffer_float16_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat16Atomics);
        }
        if self.shader_buffer_float16_atomic_add != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat16AtomicAdd);
        }
        if self.shader_buffer_float16_atomic_min_max != 0 {
            set.insert(
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat16AtomicMinMax,
            );
        }
        if self.shader_buffer_float32_atomic_min_max != 0 {
            set.insert(
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat32AtomicMinMax,
            );
        }
        if self.shader_buffer_float64_atomic_min_max != 0 {
            set.insert(
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderBufferFloat64AtomicMinMax,
            );
        }
        if self.shader_shared_float16_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat16Atomics);
        }
        if self.shader_shared_float16_atomic_add != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat16AtomicAdd);
        }
        if self.shader_shared_float16_atomic_min_max != 0 {
            set.insert(
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat16AtomicMinMax,
            );
        }
        if self.shader_shared_float32_atomic_min_max != 0 {
            set.insert(
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat32AtomicMinMax,
            );
        }
        if self.shader_shared_float64_atomic_min_max != 0 {
            set.insert(
                PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderSharedFloat64AtomicMinMax,
            );
        }
        if self.shader_image_float32_atomic_min_max != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloat2FeaturesEXT::ShaderImageFloat32AtomicMinMax);
        }
        if self.sparse_image_float32_atomic_min_max != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloat2FeaturesEXT::SparseImageFloat32AtomicMinMax);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderBufferFloat16Atomics => vk_struct.shader_buffer_float16_atomics = 1,
            Self::ShaderBufferFloat16AtomicAdd => vk_struct.shader_buffer_float16_atomic_add = 1,
            Self::ShaderBufferFloat16AtomicMinMax => {
                vk_struct.shader_buffer_float16_atomic_min_max = 1
            }
            Self::ShaderBufferFloat32AtomicMinMax => {
                vk_struct.shader_buffer_float32_atomic_min_max = 1
            }
            Self::ShaderBufferFloat64AtomicMinMax => {
                vk_struct.shader_buffer_float64_atomic_min_max = 1
            }
            Self::ShaderSharedFloat16Atomics => vk_struct.shader_shared_float16_atomics = 1,
            Self::ShaderSharedFloat16AtomicAdd => vk_struct.shader_shared_float16_atomic_add = 1,
            Self::ShaderSharedFloat16AtomicMinMax => {
                vk_struct.shader_shared_float16_atomic_min_max = 1
            }
            Self::ShaderSharedFloat32AtomicMinMax => {
                vk_struct.shader_shared_float32_atomic_min_max = 1
            }
            Self::ShaderSharedFloat64AtomicMinMax => {
                vk_struct.shader_shared_float64_atomic_min_max = 1
            }
            Self::ShaderImageFloat32AtomicMinMax => {
                vk_struct.shader_image_float32_atomic_min_max = 1
            }
            Self::SparseImageFloat32AtomicMinMax => {
                vk_struct.sparse_image_float32_atomic_min_max = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceCoverageReductionModeFeaturesNV {
    CoverageReductionMode,
}
impl const From<PhysicalDeviceCoverageReductionModeFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceCoverageReductionModeFeaturesNV) -> Self {
        FeatureType::DeviceCoverageReductionModeFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceCoverageReductionModeFeaturesNV {
    CoverageReductionMode,
}
impl ToPhysicalFeature for DeviceCoverageReductionModeFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceCoverageReductionModeFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceCoverageReductionModeFeaturesNV::CoverageReductionMode => {
                PhysicalDeviceCoverageReductionModeFeaturesNV::CoverageReductionMode
            }
        }
    }
}
impl From<DeviceCoverageReductionModeFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceCoverageReductionModeFeaturesNV) -> Self {
        DeviceFeature::DeviceCoverageReductionModeFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceCoverageReductionModeFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceCoverageReductionModeFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.coverage_reduction_mode != 0 {
            set.insert(PhysicalDeviceCoverageReductionModeFeaturesNV::CoverageReductionMode);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceCoverageReductionModeFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceCoverageReductionModeFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::CoverageReductionMode => vk_struct.coverage_reduction_mode = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceSynchronization2Features {
    Synchronization2,
}
impl const From<PhysicalDeviceSynchronization2Features> for FeatureType {
    fn from(feature: PhysicalDeviceSynchronization2Features) -> Self {
        FeatureType::DeviceSynchronization2Features(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceSynchronization2Features {
    Synchronization2,
}
impl ToPhysicalFeature for DeviceSynchronization2Features {
    type PhysicalDeviceFeatureTy = PhysicalDeviceSynchronization2Features;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceSynchronization2Features::Synchronization2 => {
                PhysicalDeviceSynchronization2Features::Synchronization2
            }
        }
    }
}
impl From<DeviceSynchronization2Features> for DeviceFeature {
    fn from(feature: DeviceSynchronization2Features) -> Self {
        DeviceFeature::DeviceSynchronization2Features(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceSynchronization2Features {
    type SubFeatureEnumTy = PhysicalDeviceSynchronization2Features;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.synchronization2 != 0 {
            set.insert(PhysicalDeviceSynchronization2Features::Synchronization2);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceSynchronization2Features {
    type VkStruct = ash::vk::PhysicalDeviceSynchronization2Features;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::Synchronization2 => vk_struct.synchronization2 = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    VertexInputDynamicState,
}
impl const From<PhysicalDeviceVertexInputDynamicStateFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceVertexInputDynamicStateFeaturesEXT) -> Self {
        FeatureType::DeviceVertexInputDynamicStateFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceVertexInputDynamicStateFeaturesEXT {
    VertexInputDynamicState,
}
impl ToPhysicalFeature for DeviceVertexInputDynamicStateFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceVertexInputDynamicStateFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceVertexInputDynamicStateFeaturesEXT::VertexInputDynamicState => {
                PhysicalDeviceVertexInputDynamicStateFeaturesEXT::VertexInputDynamicState
            }
        }
    }
}
impl From<DeviceVertexInputDynamicStateFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceVertexInputDynamicStateFeaturesEXT) -> Self {
        DeviceFeature::DeviceVertexInputDynamicStateFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceVertexInputDynamicStateFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.vertex_input_dynamic_state != 0 {
            set.insert(PhysicalDeviceVertexInputDynamicStateFeaturesEXT::VertexInputDynamicState);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::VertexInputDynamicState => vk_struct.vertex_input_dynamic_state = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceMultiviewFeatures {
    Multiview,
    MultiviewGeometryShader,
    MultiviewTessellationShader,
}
impl const From<PhysicalDeviceMultiviewFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceMultiviewFeatures) -> Self {
        FeatureType::DeviceMultiviewFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceMultiviewFeatures {
    Multiview,
    MultiviewGeometryShader,
    MultiviewTessellationShader,
}
impl ToPhysicalFeature for DeviceMultiviewFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceMultiviewFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceMultiviewFeatures::Multiview => PhysicalDeviceMultiviewFeatures::Multiview,
            DeviceMultiviewFeatures::MultiviewGeometryShader => {
                PhysicalDeviceMultiviewFeatures::MultiviewGeometryShader
            }
            DeviceMultiviewFeatures::MultiviewTessellationShader => {
                PhysicalDeviceMultiviewFeatures::MultiviewTessellationShader
            }
        }
    }
}
impl From<DeviceMultiviewFeatures> for DeviceFeature {
    fn from(feature: DeviceMultiviewFeatures) -> Self {
        DeviceFeature::DeviceMultiviewFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceMultiviewFeatures {
    type SubFeatureEnumTy = PhysicalDeviceMultiviewFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.multiview != 0 {
            set.insert(PhysicalDeviceMultiviewFeatures::Multiview);
        }
        if self.multiview_geometry_shader != 0 {
            set.insert(PhysicalDeviceMultiviewFeatures::MultiviewGeometryShader);
        }
        if self.multiview_tessellation_shader != 0 {
            set.insert(PhysicalDeviceMultiviewFeatures::MultiviewTessellationShader);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceMultiviewFeatures {
    type VkStruct = ash::vk::PhysicalDeviceMultiviewFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::Multiview => vk_struct.multiview = 1,
            Self::MultiviewGeometryShader => vk_struct.multiview_geometry_shader = 1,
            Self::MultiviewTessellationShader => vk_struct.multiview_tessellation_shader = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    ShaderIntegerFunctions2,
}
impl const From<PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL> for FeatureType {
    fn from(feature: PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL) -> Self {
        FeatureType::DeviceShaderIntegerFunctions2FeaturesINTEL(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderIntegerFunctions2FeaturesINTEL {
    ShaderIntegerFunctions2,
}
impl ToPhysicalFeature for DeviceShaderIntegerFunctions2FeaturesINTEL {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderIntegerFunctions2FeaturesINTEL::ShaderIntegerFunctions2 => {
                PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL::ShaderIntegerFunctions2
            }
        }
    }
}
impl From<DeviceShaderIntegerFunctions2FeaturesINTEL> for DeviceFeature {
    fn from(feature: DeviceShaderIntegerFunctions2FeaturesINTEL) -> Self {
        DeviceFeature::DeviceShaderIntegerFunctions2FeaturesINTEL(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    type SubFeatureEnumTy = PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_integer_functions2 != 0 {
            set.insert(PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL::ShaderIntegerFunctions2);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    type VkStruct = ash::vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderIntegerFunctions2 => vk_struct.shader_integer_functions2 = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevicePrivateDataFeatures {
    PrivateData,
}
impl const From<PhysicalDevicePrivateDataFeatures> for FeatureType {
    fn from(feature: PhysicalDevicePrivateDataFeatures) -> Self {
        FeatureType::DevicePrivateDataFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DevicePrivateDataFeatures {
    PrivateData,
}
impl ToPhysicalFeature for DevicePrivateDataFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDevicePrivateDataFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DevicePrivateDataFeatures::PrivateData => {
                PhysicalDevicePrivateDataFeatures::PrivateData
            }
        }
    }
}
impl From<DevicePrivateDataFeatures> for DeviceFeature {
    fn from(feature: DevicePrivateDataFeatures) -> Self {
        DeviceFeature::DevicePrivateDataFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevicePrivateDataFeatures {
    type SubFeatureEnumTy = PhysicalDevicePrivateDataFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.private_data != 0 {
            set.insert(PhysicalDevicePrivateDataFeatures::PrivateData);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevicePrivateDataFeatures {
    type VkStruct = ash::vk::PhysicalDevicePrivateDataFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::PrivateData => vk_struct.private_data = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevicePipelineCreationCacheControlFeatures {
    PipelineCreationCacheControl,
}
impl const From<PhysicalDevicePipelineCreationCacheControlFeatures> for FeatureType {
    fn from(feature: PhysicalDevicePipelineCreationCacheControlFeatures) -> Self {
        FeatureType::DevicePipelineCreationCacheControlFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DevicePipelineCreationCacheControlFeatures {
    PipelineCreationCacheControl,
}
impl ToPhysicalFeature for DevicePipelineCreationCacheControlFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDevicePipelineCreationCacheControlFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DevicePipelineCreationCacheControlFeatures::PipelineCreationCacheControl => {
                PhysicalDevicePipelineCreationCacheControlFeatures::PipelineCreationCacheControl
            }
        }
    }
}
impl From<DevicePipelineCreationCacheControlFeatures> for DeviceFeature {
    fn from(feature: DevicePipelineCreationCacheControlFeatures) -> Self {
        DeviceFeature::DevicePipelineCreationCacheControlFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevicePipelineCreationCacheControlFeatures {
    type SubFeatureEnumTy = PhysicalDevicePipelineCreationCacheControlFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.pipeline_creation_cache_control != 0 {
            set.insert(
                PhysicalDevicePipelineCreationCacheControlFeatures::PipelineCreationCacheControl,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevicePipelineCreationCacheControlFeatures {
    type VkStruct = ash::vk::PhysicalDevicePipelineCreationCacheControlFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::PipelineCreationCacheControl => vk_struct.pipeline_creation_cache_control = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    MutableDescriptorType,
}
impl const From<PhysicalDeviceMutableDescriptorTypeFeaturesVALVE> for FeatureType {
    fn from(feature: PhysicalDeviceMutableDescriptorTypeFeaturesVALVE) -> Self {
        FeatureType::DeviceMutableDescriptorTypeFeaturesVALVE(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceMutableDescriptorTypeFeaturesVALVE {
    MutableDescriptorType,
}
impl ToPhysicalFeature for DeviceMutableDescriptorTypeFeaturesVALVE {
    type PhysicalDeviceFeatureTy = PhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceMutableDescriptorTypeFeaturesVALVE::MutableDescriptorType => {
                PhysicalDeviceMutableDescriptorTypeFeaturesVALVE::MutableDescriptorType
            }
        }
    }
}
impl From<DeviceMutableDescriptorTypeFeaturesVALVE> for DeviceFeature {
    fn from(feature: DeviceMutableDescriptorTypeFeaturesVALVE) -> Self {
        DeviceFeature::DeviceMutableDescriptorTypeFeaturesVALVE(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    type SubFeatureEnumTy = PhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.mutable_descriptor_type != 0 {
            set.insert(PhysicalDeviceMutableDescriptorTypeFeaturesVALVE::MutableDescriptorType);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    type VkStruct = ash::vk::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::MutableDescriptorType => vk_struct.mutable_descriptor_type = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceTimelineSemaphoreFeatures {
    TimelineSemaphore,
}
impl const From<PhysicalDeviceTimelineSemaphoreFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceTimelineSemaphoreFeatures) -> Self {
        FeatureType::DeviceTimelineSemaphoreFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceTimelineSemaphoreFeatures {
    TimelineSemaphore,
}
impl ToPhysicalFeature for DeviceTimelineSemaphoreFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceTimelineSemaphoreFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceTimelineSemaphoreFeatures::TimelineSemaphore => {
                PhysicalDeviceTimelineSemaphoreFeatures::TimelineSemaphore
            }
        }
    }
}
impl From<DeviceTimelineSemaphoreFeatures> for DeviceFeature {
    fn from(feature: DeviceTimelineSemaphoreFeatures) -> Self {
        DeviceFeature::DeviceTimelineSemaphoreFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceTimelineSemaphoreFeatures {
    type SubFeatureEnumTy = PhysicalDeviceTimelineSemaphoreFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.timeline_semaphore != 0 {
            set.insert(PhysicalDeviceTimelineSemaphoreFeatures::TimelineSemaphore);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceTimelineSemaphoreFeatures {
    type VkStruct = ash::vk::PhysicalDeviceTimelineSemaphoreFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::TimelineSemaphore => vk_struct.timeline_semaphore = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceRayTracingPipelineFeaturesKHR {
    RayTracingPipeline,
    RayTracingPipelineShaderGroupHandleCaptureReplay,
    RayTracingPipelineShaderGroupHandleCaptureReplayMixed,
    RayTracingPipelineTraceRaysIndirect,
    RayTraversalPrimitiveCulling,
}
impl const From<PhysicalDeviceRayTracingPipelineFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDeviceRayTracingPipelineFeaturesKHR) -> Self {
        FeatureType::DeviceRayTracingPipelineFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceRayTracingPipelineFeaturesKHR {
    RayTracingPipeline,
    RayTracingPipelineShaderGroupHandleCaptureReplay,
    RayTracingPipelineShaderGroupHandleCaptureReplayMixed,
    RayTracingPipelineTraceRaysIndirect,
    RayTraversalPrimitiveCulling,
}
impl ToPhysicalFeature for DeviceRayTracingPipelineFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDeviceRayTracingPipelineFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceRayTracingPipelineFeaturesKHR :: RayTracingPipeline => PhysicalDeviceRayTracingPipelineFeaturesKHR :: RayTracingPipeline , DeviceRayTracingPipelineFeaturesKHR :: RayTracingPipelineShaderGroupHandleCaptureReplay => PhysicalDeviceRayTracingPipelineFeaturesKHR :: RayTracingPipelineShaderGroupHandleCaptureReplay , DeviceRayTracingPipelineFeaturesKHR :: RayTracingPipelineShaderGroupHandleCaptureReplayMixed => PhysicalDeviceRayTracingPipelineFeaturesKHR :: RayTracingPipelineShaderGroupHandleCaptureReplayMixed , DeviceRayTracingPipelineFeaturesKHR :: RayTracingPipelineTraceRaysIndirect => PhysicalDeviceRayTracingPipelineFeaturesKHR :: RayTracingPipelineTraceRaysIndirect , DeviceRayTracingPipelineFeaturesKHR :: RayTraversalPrimitiveCulling => PhysicalDeviceRayTracingPipelineFeaturesKHR :: RayTraversalPrimitiveCulling , }
    }
}
impl From<DeviceRayTracingPipelineFeaturesKHR> for DeviceFeature {
    fn from(feature: DeviceRayTracingPipelineFeaturesKHR) -> Self {
        DeviceFeature::DeviceRayTracingPipelineFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDeviceRayTracingPipelineFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.ray_tracing_pipeline != 0 {
            set.insert(PhysicalDeviceRayTracingPipelineFeaturesKHR::RayTracingPipeline);
        }
        if self.ray_tracing_pipeline_shader_group_handle_capture_replay != 0 {
            set . insert (PhysicalDeviceRayTracingPipelineFeaturesKHR :: RayTracingPipelineShaderGroupHandleCaptureReplay) ;
        }
        if self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed != 0 {
            set . insert (PhysicalDeviceRayTracingPipelineFeaturesKHR :: RayTracingPipelineShaderGroupHandleCaptureReplayMixed) ;
        }
        if self.ray_tracing_pipeline_trace_rays_indirect != 0 {
            set.insert(
                PhysicalDeviceRayTracingPipelineFeaturesKHR::RayTracingPipelineTraceRaysIndirect,
            );
        }
        if self.ray_traversal_primitive_culling != 0 {
            set.insert(PhysicalDeviceRayTracingPipelineFeaturesKHR::RayTraversalPrimitiveCulling);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceRayTracingPipelineFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::RayTracingPipeline => vk_struct.ray_tracing_pipeline = 1,
            Self::RayTracingPipelineShaderGroupHandleCaptureReplay => {
                vk_struct.ray_tracing_pipeline_shader_group_handle_capture_replay = 1
            }
            Self::RayTracingPipelineShaderGroupHandleCaptureReplayMixed => {
                vk_struct.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed = 1
            }
            Self::RayTracingPipelineTraceRaysIndirect => {
                vk_struct.ray_tracing_pipeline_trace_rays_indirect = 1
            }
            Self::RayTraversalPrimitiveCulling => vk_struct.ray_traversal_primitive_culling = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceVulkanMemoryModelFeatures {
    VulkanMemoryModel,
    VulkanMemoryModelDeviceScope,
    VulkanMemoryModelAvailabilityVisibilityChains,
}
impl const From<PhysicalDeviceVulkanMemoryModelFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceVulkanMemoryModelFeatures) -> Self {
        FeatureType::DeviceVulkanMemoryModelFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceVulkanMemoryModelFeatures {
    VulkanMemoryModel,
    VulkanMemoryModelDeviceScope,
    VulkanMemoryModelAvailabilityVisibilityChains,
}
impl ToPhysicalFeature for DeviceVulkanMemoryModelFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceVulkanMemoryModelFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceVulkanMemoryModelFeatures :: VulkanMemoryModel => PhysicalDeviceVulkanMemoryModelFeatures :: VulkanMemoryModel , DeviceVulkanMemoryModelFeatures :: VulkanMemoryModelDeviceScope => PhysicalDeviceVulkanMemoryModelFeatures :: VulkanMemoryModelDeviceScope , DeviceVulkanMemoryModelFeatures :: VulkanMemoryModelAvailabilityVisibilityChains => PhysicalDeviceVulkanMemoryModelFeatures :: VulkanMemoryModelAvailabilityVisibilityChains , }
    }
}
impl From<DeviceVulkanMemoryModelFeatures> for DeviceFeature {
    fn from(feature: DeviceVulkanMemoryModelFeatures) -> Self {
        DeviceFeature::DeviceVulkanMemoryModelFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceVulkanMemoryModelFeatures {
    type SubFeatureEnumTy = PhysicalDeviceVulkanMemoryModelFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.vulkan_memory_model != 0 {
            set.insert(PhysicalDeviceVulkanMemoryModelFeatures::VulkanMemoryModel);
        }
        if self.vulkan_memory_model_device_scope != 0 {
            set.insert(PhysicalDeviceVulkanMemoryModelFeatures::VulkanMemoryModelDeviceScope);
        }
        if self.vulkan_memory_model_availability_visibility_chains != 0 {
            set . insert (PhysicalDeviceVulkanMemoryModelFeatures :: VulkanMemoryModelAvailabilityVisibilityChains) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceVulkanMemoryModelFeatures {
    type VkStruct = ash::vk::PhysicalDeviceVulkanMemoryModelFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::VulkanMemoryModel => vk_struct.vulkan_memory_model = 1,
            Self::VulkanMemoryModelDeviceScope => vk_struct.vulkan_memory_model_device_scope = 1,
            Self::VulkanMemoryModelAvailabilityVisibilityChains => {
                vk_struct.vulkan_memory_model_availability_visibility_chains = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    ExtendedDynamicState2,
    ExtendedDynamicState2LogicOp,
    ExtendedDynamicState2PatchControlPoints,
}
impl const From<PhysicalDeviceExtendedDynamicState2FeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceExtendedDynamicState2FeaturesEXT) -> Self {
        FeatureType::DeviceExtendedDynamicState2FeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceExtendedDynamicState2FeaturesEXT {
    ExtendedDynamicState2,
    ExtendedDynamicState2LogicOp,
    ExtendedDynamicState2PatchControlPoints,
}
impl ToPhysicalFeature for DeviceExtendedDynamicState2FeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceExtendedDynamicState2FeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceExtendedDynamicState2FeaturesEXT :: ExtendedDynamicState2 => PhysicalDeviceExtendedDynamicState2FeaturesEXT :: ExtendedDynamicState2 , DeviceExtendedDynamicState2FeaturesEXT :: ExtendedDynamicState2LogicOp => PhysicalDeviceExtendedDynamicState2FeaturesEXT :: ExtendedDynamicState2LogicOp , DeviceExtendedDynamicState2FeaturesEXT :: ExtendedDynamicState2PatchControlPoints => PhysicalDeviceExtendedDynamicState2FeaturesEXT :: ExtendedDynamicState2PatchControlPoints , }
    }
}
impl From<DeviceExtendedDynamicState2FeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceExtendedDynamicState2FeaturesEXT) -> Self {
        DeviceFeature::DeviceExtendedDynamicState2FeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceExtendedDynamicState2FeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.extended_dynamic_state2 != 0 {
            set.insert(PhysicalDeviceExtendedDynamicState2FeaturesEXT::ExtendedDynamicState2);
        }
        if self.extended_dynamic_state2_logic_op != 0 {
            set.insert(
                PhysicalDeviceExtendedDynamicState2FeaturesEXT::ExtendedDynamicState2LogicOp,
            );
        }
        if self.extended_dynamic_state2_patch_control_points != 0 {
            set . insert (PhysicalDeviceExtendedDynamicState2FeaturesEXT :: ExtendedDynamicState2PatchControlPoints) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ExtendedDynamicState2 => vk_struct.extended_dynamic_state2 = 1,
            Self::ExtendedDynamicState2LogicOp => vk_struct.extended_dynamic_state2_logic_op = 1,
            Self::ExtendedDynamicState2PatchControlPoints => {
                vk_struct.extended_dynamic_state2_patch_control_points = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    ShaderImageInt64Atomics,
    SparseImageInt64Atomics,
}
impl const From<PhysicalDeviceShaderImageAtomicInt64FeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceShaderImageAtomicInt64FeaturesEXT) -> Self {
        FeatureType::DeviceShaderImageAtomicInt64FeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderImageAtomicInt64FeaturesEXT {
    ShaderImageInt64Atomics,
    SparseImageInt64Atomics,
}
impl ToPhysicalFeature for DeviceShaderImageAtomicInt64FeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderImageAtomicInt64FeaturesEXT::ShaderImageInt64Atomics => {
                PhysicalDeviceShaderImageAtomicInt64FeaturesEXT::ShaderImageInt64Atomics
            }
            DeviceShaderImageAtomicInt64FeaturesEXT::SparseImageInt64Atomics => {
                PhysicalDeviceShaderImageAtomicInt64FeaturesEXT::SparseImageInt64Atomics
            }
        }
    }
}
impl From<DeviceShaderImageAtomicInt64FeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceShaderImageAtomicInt64FeaturesEXT) -> Self {
        DeviceFeature::DeviceShaderImageAtomicInt64FeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_image_int64_atomics != 0 {
            set.insert(PhysicalDeviceShaderImageAtomicInt64FeaturesEXT::ShaderImageInt64Atomics);
        }
        if self.sparse_image_int64_atomics != 0 {
            set.insert(PhysicalDeviceShaderImageAtomicInt64FeaturesEXT::SparseImageInt64Atomics);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderImageInt64Atomics => vk_struct.shader_image_int64_atomics = 1,
            Self::SparseImageInt64Atomics => vk_struct.sparse_image_int64_atomics = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceInheritedViewportScissorFeaturesNV {
    InheritedViewportScissor2D,
}
impl const From<PhysicalDeviceInheritedViewportScissorFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceInheritedViewportScissorFeaturesNV) -> Self {
        FeatureType::DeviceInheritedViewportScissorFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceInheritedViewportScissorFeaturesNV {
    InheritedViewportScissor2D,
}
impl ToPhysicalFeature for DeviceInheritedViewportScissorFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceInheritedViewportScissorFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceInheritedViewportScissorFeaturesNV::InheritedViewportScissor2D => {
                PhysicalDeviceInheritedViewportScissorFeaturesNV::InheritedViewportScissor2D
            }
        }
    }
}
impl From<DeviceInheritedViewportScissorFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceInheritedViewportScissorFeaturesNV) -> Self {
        DeviceFeature::DeviceInheritedViewportScissorFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceInheritedViewportScissorFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceInheritedViewportScissorFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.inherited_viewport_scissor2_d != 0 {
            set.insert(
                PhysicalDeviceInheritedViewportScissorFeaturesNV::InheritedViewportScissor2D,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceInheritedViewportScissorFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceInheritedViewportScissorFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::InheritedViewportScissor2D => vk_struct.inherited_viewport_scissor2_d = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    Ycbcr2plane444Formats,
}
impl const From<PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT) -> Self {
        FeatureType::DeviceYcbcr2Plane444FormatsFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceYcbcr2Plane444FormatsFeaturesEXT {
    Ycbcr2plane444Formats,
}
impl ToPhysicalFeature for DeviceYcbcr2Plane444FormatsFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceYcbcr2Plane444FormatsFeaturesEXT::Ycbcr2plane444Formats => {
                PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT::Ycbcr2plane444Formats
            }
        }
    }
}
impl From<DeviceYcbcr2Plane444FormatsFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceYcbcr2Plane444FormatsFeaturesEXT) -> Self {
        DeviceFeature::DeviceYcbcr2Plane444FormatsFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.ycbcr2plane444_formats != 0 {
            set.insert(PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT::Ycbcr2plane444Formats);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::Ycbcr2plane444Formats => vk_struct.ycbcr2plane444_formats = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevicePresentWaitFeaturesKHR {
    PresentWait,
}
impl const From<PhysicalDevicePresentWaitFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDevicePresentWaitFeaturesKHR) -> Self {
        FeatureType::DevicePresentWaitFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DevicePresentWaitFeaturesKHR {
    PresentWait,
}
impl ToPhysicalFeature for DevicePresentWaitFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDevicePresentWaitFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DevicePresentWaitFeaturesKHR::PresentWait => {
                PhysicalDevicePresentWaitFeaturesKHR::PresentWait
            }
        }
    }
}
impl From<DevicePresentWaitFeaturesKHR> for DeviceFeature {
    fn from(feature: DevicePresentWaitFeaturesKHR) -> Self {
        DeviceFeature::DevicePresentWaitFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevicePresentWaitFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDevicePresentWaitFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.present_wait != 0 {
            set.insert(PhysicalDevicePresentWaitFeaturesKHR::PresentWait);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevicePresentWaitFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDevicePresentWaitFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::PresentWait => vk_struct.present_wait = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    ComputeDerivativeGroupQuads,
    ComputeDerivativeGroupLinear,
}
impl const From<PhysicalDeviceComputeShaderDerivativesFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceComputeShaderDerivativesFeaturesNV) -> Self {
        FeatureType::DeviceComputeShaderDerivativesFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceComputeShaderDerivativesFeaturesNV {
    ComputeDerivativeGroupQuads,
    ComputeDerivativeGroupLinear,
}
impl ToPhysicalFeature for DeviceComputeShaderDerivativesFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceComputeShaderDerivativesFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceComputeShaderDerivativesFeaturesNV::ComputeDerivativeGroupQuads => {
                PhysicalDeviceComputeShaderDerivativesFeaturesNV::ComputeDerivativeGroupQuads
            }
            DeviceComputeShaderDerivativesFeaturesNV::ComputeDerivativeGroupLinear => {
                PhysicalDeviceComputeShaderDerivativesFeaturesNV::ComputeDerivativeGroupLinear
            }
        }
    }
}
impl From<DeviceComputeShaderDerivativesFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceComputeShaderDerivativesFeaturesNV) -> Self {
        DeviceFeature::DeviceComputeShaderDerivativesFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceComputeShaderDerivativesFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.compute_derivative_group_quads != 0 {
            set.insert(
                PhysicalDeviceComputeShaderDerivativesFeaturesNV::ComputeDerivativeGroupQuads,
            );
        }
        if self.compute_derivative_group_linear != 0 {
            set.insert(
                PhysicalDeviceComputeShaderDerivativesFeaturesNV::ComputeDerivativeGroupLinear,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ComputeDerivativeGroupQuads => vk_struct.compute_derivative_group_quads = 1,
            Self::ComputeDerivativeGroupLinear => vk_struct.compute_derivative_group_linear = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceVulkan13Features {
    RobustImageAccess,
    InlineUniformBlock,
    DescriptorBindingInlineUniformBlockUpdateAfterBind,
    PipelineCreationCacheControl,
    PrivateData,
    ShaderDemoteToHelperInvocation,
    ShaderTerminateInvocation,
    SubgroupSizeControl,
    ComputeFullSubgroups,
    Synchronization2,
    TextureCompressionAstcHdr,
    ShaderZeroInitializeWorkgroupMemory,
    DynamicRendering,
    ShaderIntegerDotProduct,
    Maintenance4,
}
impl const From<PhysicalDeviceVulkan13Features> for FeatureType {
    fn from(feature: PhysicalDeviceVulkan13Features) -> Self {
        FeatureType::DeviceVulkan13Features(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceVulkan13Features {
    RobustImageAccess,
    InlineUniformBlock,
    DescriptorBindingInlineUniformBlockUpdateAfterBind,
    PipelineCreationCacheControl,
    PrivateData,
    ShaderDemoteToHelperInvocation,
    ShaderTerminateInvocation,
    SubgroupSizeControl,
    ComputeFullSubgroups,
    Synchronization2,
    TextureCompressionAstcHdr,
    ShaderZeroInitializeWorkgroupMemory,
    DynamicRendering,
    ShaderIntegerDotProduct,
    Maintenance4,
}
impl ToPhysicalFeature for DeviceVulkan13Features {
    type PhysicalDeviceFeatureTy = PhysicalDeviceVulkan13Features;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceVulkan13Features::RobustImageAccess => {
                PhysicalDeviceVulkan13Features::RobustImageAccess
            }
            DeviceVulkan13Features::InlineUniformBlock => {
                PhysicalDeviceVulkan13Features::InlineUniformBlock
            }
            DeviceVulkan13Features::DescriptorBindingInlineUniformBlockUpdateAfterBind => {
                PhysicalDeviceVulkan13Features::DescriptorBindingInlineUniformBlockUpdateAfterBind
            }
            DeviceVulkan13Features::PipelineCreationCacheControl => {
                PhysicalDeviceVulkan13Features::PipelineCreationCacheControl
            }
            DeviceVulkan13Features::PrivateData => PhysicalDeviceVulkan13Features::PrivateData,
            DeviceVulkan13Features::ShaderDemoteToHelperInvocation => {
                PhysicalDeviceVulkan13Features::ShaderDemoteToHelperInvocation
            }
            DeviceVulkan13Features::ShaderTerminateInvocation => {
                PhysicalDeviceVulkan13Features::ShaderTerminateInvocation
            }
            DeviceVulkan13Features::SubgroupSizeControl => {
                PhysicalDeviceVulkan13Features::SubgroupSizeControl
            }
            DeviceVulkan13Features::ComputeFullSubgroups => {
                PhysicalDeviceVulkan13Features::ComputeFullSubgroups
            }
            DeviceVulkan13Features::Synchronization2 => {
                PhysicalDeviceVulkan13Features::Synchronization2
            }
            DeviceVulkan13Features::TextureCompressionAstcHdr => {
                PhysicalDeviceVulkan13Features::TextureCompressionAstcHdr
            }
            DeviceVulkan13Features::ShaderZeroInitializeWorkgroupMemory => {
                PhysicalDeviceVulkan13Features::ShaderZeroInitializeWorkgroupMemory
            }
            DeviceVulkan13Features::DynamicRendering => {
                PhysicalDeviceVulkan13Features::DynamicRendering
            }
            DeviceVulkan13Features::ShaderIntegerDotProduct => {
                PhysicalDeviceVulkan13Features::ShaderIntegerDotProduct
            }
            DeviceVulkan13Features::Maintenance4 => PhysicalDeviceVulkan13Features::Maintenance4,
        }
    }
}
impl From<DeviceVulkan13Features> for DeviceFeature {
    fn from(feature: DeviceVulkan13Features) -> Self {
        DeviceFeature::DeviceVulkan13Features(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceVulkan13Features {
    type SubFeatureEnumTy = PhysicalDeviceVulkan13Features;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.robust_image_access != 0 {
            set.insert(PhysicalDeviceVulkan13Features::RobustImageAccess);
        }
        if self.inline_uniform_block != 0 {
            set.insert(PhysicalDeviceVulkan13Features::InlineUniformBlock);
        }
        if self.descriptor_binding_inline_uniform_block_update_after_bind != 0 {
            set.insert(
                PhysicalDeviceVulkan13Features::DescriptorBindingInlineUniformBlockUpdateAfterBind,
            );
        }
        if self.pipeline_creation_cache_control != 0 {
            set.insert(PhysicalDeviceVulkan13Features::PipelineCreationCacheControl);
        }
        if self.private_data != 0 {
            set.insert(PhysicalDeviceVulkan13Features::PrivateData);
        }
        if self.shader_demote_to_helper_invocation != 0 {
            set.insert(PhysicalDeviceVulkan13Features::ShaderDemoteToHelperInvocation);
        }
        if self.shader_terminate_invocation != 0 {
            set.insert(PhysicalDeviceVulkan13Features::ShaderTerminateInvocation);
        }
        if self.subgroup_size_control != 0 {
            set.insert(PhysicalDeviceVulkan13Features::SubgroupSizeControl);
        }
        if self.compute_full_subgroups != 0 {
            set.insert(PhysicalDeviceVulkan13Features::ComputeFullSubgroups);
        }
        if self.synchronization2 != 0 {
            set.insert(PhysicalDeviceVulkan13Features::Synchronization2);
        }
        if self.texture_compression_astc_hdr != 0 {
            set.insert(PhysicalDeviceVulkan13Features::TextureCompressionAstcHdr);
        }
        if self.shader_zero_initialize_workgroup_memory != 0 {
            set.insert(PhysicalDeviceVulkan13Features::ShaderZeroInitializeWorkgroupMemory);
        }
        if self.dynamic_rendering != 0 {
            set.insert(PhysicalDeviceVulkan13Features::DynamicRendering);
        }
        if self.shader_integer_dot_product != 0 {
            set.insert(PhysicalDeviceVulkan13Features::ShaderIntegerDotProduct);
        }
        if self.maintenance4 != 0 {
            set.insert(PhysicalDeviceVulkan13Features::Maintenance4);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceVulkan13Features {
    type VkStruct = ash::vk::PhysicalDeviceVulkan13Features;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::RobustImageAccess => vk_struct.robust_image_access = 1,
            Self::InlineUniformBlock => vk_struct.inline_uniform_block = 1,
            Self::DescriptorBindingInlineUniformBlockUpdateAfterBind => {
                vk_struct.descriptor_binding_inline_uniform_block_update_after_bind = 1
            }
            Self::PipelineCreationCacheControl => vk_struct.pipeline_creation_cache_control = 1,
            Self::PrivateData => vk_struct.private_data = 1,
            Self::ShaderDemoteToHelperInvocation => {
                vk_struct.shader_demote_to_helper_invocation = 1
            }
            Self::ShaderTerminateInvocation => vk_struct.shader_terminate_invocation = 1,
            Self::SubgroupSizeControl => vk_struct.subgroup_size_control = 1,
            Self::ComputeFullSubgroups => vk_struct.compute_full_subgroups = 1,
            Self::Synchronization2 => vk_struct.synchronization2 = 1,
            Self::TextureCompressionAstcHdr => vk_struct.texture_compression_astc_hdr = 1,
            Self::ShaderZeroInitializeWorkgroupMemory => {
                vk_struct.shader_zero_initialize_workgroup_memory = 1
            }
            Self::DynamicRendering => vk_struct.dynamic_rendering = 1,
            Self::ShaderIntegerDotProduct => vk_struct.shader_integer_dot_product = 1,
            Self::Maintenance4 => vk_struct.maintenance4 = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    ExternalMemoryRdma,
}
impl const From<PhysicalDeviceExternalMemoryRDMAFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceExternalMemoryRDMAFeaturesNV) -> Self {
        FeatureType::DeviceExternalMemoryRDMAFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceExternalMemoryRDMAFeaturesNV {
    ExternalMemoryRdma,
}
impl ToPhysicalFeature for DeviceExternalMemoryRDMAFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceExternalMemoryRDMAFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceExternalMemoryRDMAFeaturesNV::ExternalMemoryRdma => {
                PhysicalDeviceExternalMemoryRDMAFeaturesNV::ExternalMemoryRdma
            }
        }
    }
}
impl From<DeviceExternalMemoryRDMAFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceExternalMemoryRDMAFeaturesNV) -> Self {
        DeviceFeature::DeviceExternalMemoryRDMAFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceExternalMemoryRDMAFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.external_memory_rdma != 0 {
            set.insert(PhysicalDeviceExternalMemoryRDMAFeaturesNV::ExternalMemoryRdma);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceExternalMemoryRDMAFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ExternalMemoryRdma => vk_struct.external_memory_rdma = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceProtectedMemoryFeatures {
    ProtectedMemory,
}
impl const From<PhysicalDeviceProtectedMemoryFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceProtectedMemoryFeatures) -> Self {
        FeatureType::DeviceProtectedMemoryFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceProtectedMemoryFeatures {
    ProtectedMemory,
}
impl ToPhysicalFeature for DeviceProtectedMemoryFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceProtectedMemoryFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceProtectedMemoryFeatures::ProtectedMemory => {
                PhysicalDeviceProtectedMemoryFeatures::ProtectedMemory
            }
        }
    }
}
impl From<DeviceProtectedMemoryFeatures> for DeviceFeature {
    fn from(feature: DeviceProtectedMemoryFeatures) -> Self {
        DeviceFeature::DeviceProtectedMemoryFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceProtectedMemoryFeatures {
    type SubFeatureEnumTy = PhysicalDeviceProtectedMemoryFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.protected_memory != 0 {
            set.insert(PhysicalDeviceProtectedMemoryFeatures::ProtectedMemory);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceProtectedMemoryFeatures {
    type VkStruct = ash::vk::PhysicalDeviceProtectedMemoryFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ProtectedMemory => vk_struct.protected_memory = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceDynamicRenderingFeatures {
    DynamicRendering,
}
impl const From<PhysicalDeviceDynamicRenderingFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceDynamicRenderingFeatures) -> Self {
        FeatureType::DeviceDynamicRenderingFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceDynamicRenderingFeatures {
    DynamicRendering,
}
impl ToPhysicalFeature for DeviceDynamicRenderingFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceDynamicRenderingFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceDynamicRenderingFeatures::DynamicRendering => {
                PhysicalDeviceDynamicRenderingFeatures::DynamicRendering
            }
        }
    }
}
impl From<DeviceDynamicRenderingFeatures> for DeviceFeature {
    fn from(feature: DeviceDynamicRenderingFeatures) -> Self {
        DeviceFeature::DeviceDynamicRenderingFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceDynamicRenderingFeatures {
    type SubFeatureEnumTy = PhysicalDeviceDynamicRenderingFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.dynamic_rendering != 0 {
            set.insert(PhysicalDeviceDynamicRenderingFeatures::DynamicRendering);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceDynamicRenderingFeatures {
    type VkStruct = ash::vk::PhysicalDeviceDynamicRenderingFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DynamicRendering => vk_struct.dynamic_rendering = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceFragmentDensityMapFeaturesEXT {
    FragmentDensityMap,
    FragmentDensityMapDynamic,
    FragmentDensityMapNonSubsampledImages,
}
impl const From<PhysicalDeviceFragmentDensityMapFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceFragmentDensityMapFeaturesEXT) -> Self {
        FeatureType::DeviceFragmentDensityMapFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceFragmentDensityMapFeaturesEXT {
    FragmentDensityMap,
    FragmentDensityMapDynamic,
    FragmentDensityMapNonSubsampledImages,
}
impl ToPhysicalFeature for DeviceFragmentDensityMapFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceFragmentDensityMapFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceFragmentDensityMapFeaturesEXT::FragmentDensityMap => {
                PhysicalDeviceFragmentDensityMapFeaturesEXT::FragmentDensityMap
            }
            DeviceFragmentDensityMapFeaturesEXT::FragmentDensityMapDynamic => {
                PhysicalDeviceFragmentDensityMapFeaturesEXT::FragmentDensityMapDynamic
            }
            DeviceFragmentDensityMapFeaturesEXT::FragmentDensityMapNonSubsampledImages => {
                PhysicalDeviceFragmentDensityMapFeaturesEXT::FragmentDensityMapNonSubsampledImages
            }
        }
    }
}
impl From<DeviceFragmentDensityMapFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceFragmentDensityMapFeaturesEXT) -> Self {
        DeviceFeature::DeviceFragmentDensityMapFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceFragmentDensityMapFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceFragmentDensityMapFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.fragment_density_map != 0 {
            set.insert(PhysicalDeviceFragmentDensityMapFeaturesEXT::FragmentDensityMap);
        }
        if self.fragment_density_map_dynamic != 0 {
            set.insert(PhysicalDeviceFragmentDensityMapFeaturesEXT::FragmentDensityMapDynamic);
        }
        if self.fragment_density_map_non_subsampled_images != 0 {
            set.insert(
                PhysicalDeviceFragmentDensityMapFeaturesEXT::FragmentDensityMapNonSubsampledImages,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceFragmentDensityMapFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceFragmentDensityMapFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::FragmentDensityMap => vk_struct.fragment_density_map = 1,
            Self::FragmentDensityMapDynamic => vk_struct.fragment_density_map_dynamic = 1,
            Self::FragmentDensityMapNonSubsampledImages => {
                vk_struct.fragment_density_map_non_subsampled_images = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderFloat16Int8Features {
    ShaderFloat16,
    ShaderInt8,
}
impl const From<PhysicalDeviceShaderFloat16Int8Features> for FeatureType {
    fn from(feature: PhysicalDeviceShaderFloat16Int8Features) -> Self {
        FeatureType::DeviceShaderFloat16Int8Features(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderFloat16Int8Features {
    ShaderFloat16,
    ShaderInt8,
}
impl ToPhysicalFeature for DeviceShaderFloat16Int8Features {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderFloat16Int8Features;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderFloat16Int8Features::ShaderFloat16 => {
                PhysicalDeviceShaderFloat16Int8Features::ShaderFloat16
            }
            DeviceShaderFloat16Int8Features::ShaderInt8 => {
                PhysicalDeviceShaderFloat16Int8Features::ShaderInt8
            }
        }
    }
}
impl From<DeviceShaderFloat16Int8Features> for DeviceFeature {
    fn from(feature: DeviceShaderFloat16Int8Features) -> Self {
        DeviceFeature::DeviceShaderFloat16Int8Features(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderFloat16Int8Features {
    type SubFeatureEnumTy = PhysicalDeviceShaderFloat16Int8Features;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_float16 != 0 {
            set.insert(PhysicalDeviceShaderFloat16Int8Features::ShaderFloat16);
        }
        if self.shader_int8 != 0 {
            set.insert(PhysicalDeviceShaderFloat16Int8Features::ShaderInt8);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderFloat16Int8Features {
    type VkStruct = ash::vk::PhysicalDeviceShaderFloat16Int8Features;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderFloat16 => vk_struct.shader_float16 = 1,
            Self::ShaderInt8 => vk_struct.shader_int8 = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    InvocationMask,
}
impl const From<PhysicalDeviceInvocationMaskFeaturesHUAWEI> for FeatureType {
    fn from(feature: PhysicalDeviceInvocationMaskFeaturesHUAWEI) -> Self {
        FeatureType::DeviceInvocationMaskFeaturesHUAWEI(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceInvocationMaskFeaturesHUAWEI {
    InvocationMask,
}
impl ToPhysicalFeature for DeviceInvocationMaskFeaturesHUAWEI {
    type PhysicalDeviceFeatureTy = PhysicalDeviceInvocationMaskFeaturesHUAWEI;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceInvocationMaskFeaturesHUAWEI::InvocationMask => {
                PhysicalDeviceInvocationMaskFeaturesHUAWEI::InvocationMask
            }
        }
    }
}
impl From<DeviceInvocationMaskFeaturesHUAWEI> for DeviceFeature {
    fn from(feature: DeviceInvocationMaskFeaturesHUAWEI) -> Self {
        DeviceFeature::DeviceInvocationMaskFeaturesHUAWEI(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    type SubFeatureEnumTy = PhysicalDeviceInvocationMaskFeaturesHUAWEI;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.invocation_mask != 0 {
            set.insert(PhysicalDeviceInvocationMaskFeaturesHUAWEI::InvocationMask);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    type VkStruct = ash::vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::InvocationMask => vk_struct.invocation_mask = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceRayQueryFeaturesKHR {
    RayQuery,
}
impl const From<PhysicalDeviceRayQueryFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDeviceRayQueryFeaturesKHR) -> Self {
        FeatureType::DeviceRayQueryFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceRayQueryFeaturesKHR {
    RayQuery,
}
impl ToPhysicalFeature for DeviceRayQueryFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDeviceRayQueryFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceRayQueryFeaturesKHR::RayQuery => PhysicalDeviceRayQueryFeaturesKHR::RayQuery,
        }
    }
}
impl From<DeviceRayQueryFeaturesKHR> for DeviceFeature {
    fn from(feature: DeviceRayQueryFeaturesKHR) -> Self {
        DeviceFeature::DeviceRayQueryFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceRayQueryFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDeviceRayQueryFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.ray_query != 0 {
            set.insert(PhysicalDeviceRayQueryFeaturesKHR::RayQuery);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceRayQueryFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDeviceRayQueryFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::RayQuery => vk_struct.ray_query = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    SubpassShading,
}
impl const From<PhysicalDeviceSubpassShadingFeaturesHUAWEI> for FeatureType {
    fn from(feature: PhysicalDeviceSubpassShadingFeaturesHUAWEI) -> Self {
        FeatureType::DeviceSubpassShadingFeaturesHUAWEI(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceSubpassShadingFeaturesHUAWEI {
    SubpassShading,
}
impl ToPhysicalFeature for DeviceSubpassShadingFeaturesHUAWEI {
    type PhysicalDeviceFeatureTy = PhysicalDeviceSubpassShadingFeaturesHUAWEI;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceSubpassShadingFeaturesHUAWEI::SubpassShading => {
                PhysicalDeviceSubpassShadingFeaturesHUAWEI::SubpassShading
            }
        }
    }
}
impl From<DeviceSubpassShadingFeaturesHUAWEI> for DeviceFeature {
    fn from(feature: DeviceSubpassShadingFeaturesHUAWEI) -> Self {
        DeviceFeature::DeviceSubpassShadingFeaturesHUAWEI(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    type SubFeatureEnumTy = PhysicalDeviceSubpassShadingFeaturesHUAWEI;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.subpass_shading != 0 {
            set.insert(PhysicalDeviceSubpassShadingFeaturesHUAWEI::SubpassShading);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    type VkStruct = ash::vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::SubpassShading => vk_struct.subpass_shading = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    ShaderSubgroupExtendedTypes,
}
impl const From<PhysicalDeviceShaderSubgroupExtendedTypesFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceShaderSubgroupExtendedTypesFeatures) -> Self {
        FeatureType::DeviceShaderSubgroupExtendedTypesFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderSubgroupExtendedTypesFeatures {
    ShaderSubgroupExtendedTypes,
}
impl ToPhysicalFeature for DeviceShaderSubgroupExtendedTypesFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderSubgroupExtendedTypesFeatures::ShaderSubgroupExtendedTypes => {
                PhysicalDeviceShaderSubgroupExtendedTypesFeatures::ShaderSubgroupExtendedTypes
            }
        }
    }
}
impl From<DeviceShaderSubgroupExtendedTypesFeatures> for DeviceFeature {
    fn from(feature: DeviceShaderSubgroupExtendedTypesFeatures) -> Self {
        DeviceFeature::DeviceShaderSubgroupExtendedTypesFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    type SubFeatureEnumTy = PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_subgroup_extended_types != 0 {
            set.insert(
                PhysicalDeviceShaderSubgroupExtendedTypesFeatures::ShaderSubgroupExtendedTypes,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    type VkStruct = ash::vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderSubgroupExtendedTypes => vk_struct.shader_subgroup_extended_types = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    WorkgroupMemoryExplicitLayout,
    WorkgroupMemoryExplicitLayoutScalarBlockLayout,
    WorkgroupMemoryExplicitLayout8BitAccess,
    WorkgroupMemoryExplicitLayout16BitAccess,
}
impl const From<PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR) -> Self {
        FeatureType::DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    WorkgroupMemoryExplicitLayout,
    WorkgroupMemoryExplicitLayoutScalarBlockLayout,
    WorkgroupMemoryExplicitLayout8BitAccess,
    WorkgroupMemoryExplicitLayout16BitAccess,
}
impl ToPhysicalFeature for DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayout => PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayout , DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayoutScalarBlockLayout => PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayoutScalarBlockLayout , DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayout8BitAccess => PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayout8BitAccess , DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayout16BitAccess => PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayout16BitAccess , }
    }
}
impl From<DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR> for DeviceFeature {
    fn from(feature: DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR) -> Self {
        DeviceFeature::DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.workgroup_memory_explicit_layout != 0 {
            set . insert (PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayout) ;
        }
        if self.workgroup_memory_explicit_layout_scalar_block_layout != 0 {
            set . insert (PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayoutScalarBlockLayout) ;
        }
        if self.workgroup_memory_explicit_layout8_bit_access != 0 {
            set . insert (PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayout8BitAccess) ;
        }
        if self.workgroup_memory_explicit_layout16_bit_access != 0 {
            set . insert (PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: WorkgroupMemoryExplicitLayout16BitAccess) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::WorkgroupMemoryExplicitLayout => vk_struct.workgroup_memory_explicit_layout = 1,
            Self::WorkgroupMemoryExplicitLayoutScalarBlockLayout => {
                vk_struct.workgroup_memory_explicit_layout_scalar_block_layout = 1
            }
            Self::WorkgroupMemoryExplicitLayout8BitAccess => {
                vk_struct.workgroup_memory_explicit_layout8_bit_access = 1
            }
            Self::WorkgroupMemoryExplicitLayout16BitAccess => {
                vk_struct.workgroup_memory_explicit_layout16_bit_access = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    RayTracingMotionBlur,
    RayTracingMotionBlurPipelineTraceRaysIndirect,
}
impl const From<PhysicalDeviceRayTracingMotionBlurFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceRayTracingMotionBlurFeaturesNV) -> Self {
        FeatureType::DeviceRayTracingMotionBlurFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceRayTracingMotionBlurFeaturesNV {
    RayTracingMotionBlur,
    RayTracingMotionBlurPipelineTraceRaysIndirect,
}
impl ToPhysicalFeature for DeviceRayTracingMotionBlurFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceRayTracingMotionBlurFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceRayTracingMotionBlurFeaturesNV :: RayTracingMotionBlur => PhysicalDeviceRayTracingMotionBlurFeaturesNV :: RayTracingMotionBlur , DeviceRayTracingMotionBlurFeaturesNV :: RayTracingMotionBlurPipelineTraceRaysIndirect => PhysicalDeviceRayTracingMotionBlurFeaturesNV :: RayTracingMotionBlurPipelineTraceRaysIndirect , }
    }
}
impl From<DeviceRayTracingMotionBlurFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceRayTracingMotionBlurFeaturesNV) -> Self {
        DeviceFeature::DeviceRayTracingMotionBlurFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceRayTracingMotionBlurFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.ray_tracing_motion_blur != 0 {
            set.insert(PhysicalDeviceRayTracingMotionBlurFeaturesNV::RayTracingMotionBlur);
        }
        if self.ray_tracing_motion_blur_pipeline_trace_rays_indirect != 0 {
            set . insert (PhysicalDeviceRayTracingMotionBlurFeaturesNV :: RayTracingMotionBlurPipelineTraceRaysIndirect) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::RayTracingMotionBlur => vk_struct.ray_tracing_motion_blur = 1,
            Self::RayTracingMotionBlurPipelineTraceRaysIndirect => {
                vk_struct.ray_tracing_motion_blur_pipeline_trace_rays_indirect = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceMaintenance4Features {
    Maintenance4,
}
impl const From<PhysicalDeviceMaintenance4Features> for FeatureType {
    fn from(feature: PhysicalDeviceMaintenance4Features) -> Self {
        FeatureType::DeviceMaintenance4Features(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceMaintenance4Features {
    Maintenance4,
}
impl ToPhysicalFeature for DeviceMaintenance4Features {
    type PhysicalDeviceFeatureTy = PhysicalDeviceMaintenance4Features;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceMaintenance4Features::Maintenance4 => {
                PhysicalDeviceMaintenance4Features::Maintenance4
            }
        }
    }
}
impl From<DeviceMaintenance4Features> for DeviceFeature {
    fn from(feature: DeviceMaintenance4Features) -> Self {
        DeviceFeature::DeviceMaintenance4Features(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceMaintenance4Features {
    type SubFeatureEnumTy = PhysicalDeviceMaintenance4Features;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.maintenance4 != 0 {
            set.insert(PhysicalDeviceMaintenance4Features::Maintenance4);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceMaintenance4Features {
    type VkStruct = ash::vk::PhysicalDeviceMaintenance4Features;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::Maintenance4 => vk_struct.maintenance4 = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevice4444FormatsFeaturesEXT {
    FormatA4r4g4b4,
    FormatA4b4g4r4,
}
impl const From<PhysicalDevice4444FormatsFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDevice4444FormatsFeaturesEXT) -> Self {
        FeatureType::Device4444FormatsFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum Device4444FormatsFeaturesEXT {
    FormatA4r4g4b4,
    FormatA4b4g4r4,
}
impl ToPhysicalFeature for Device4444FormatsFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDevice4444FormatsFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            Device4444FormatsFeaturesEXT::FormatA4r4g4b4 => {
                PhysicalDevice4444FormatsFeaturesEXT::FormatA4r4g4b4
            }
            Device4444FormatsFeaturesEXT::FormatA4b4g4r4 => {
                PhysicalDevice4444FormatsFeaturesEXT::FormatA4b4g4r4
            }
        }
    }
}
impl From<Device4444FormatsFeaturesEXT> for DeviceFeature {
    fn from(feature: Device4444FormatsFeaturesEXT) -> Self {
        DeviceFeature::Device4444FormatsFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevice4444FormatsFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDevice4444FormatsFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.format_a4r4g4b4 != 0 {
            set.insert(PhysicalDevice4444FormatsFeaturesEXT::FormatA4r4g4b4);
        }
        if self.format_a4b4g4r4 != 0 {
            set.insert(PhysicalDevice4444FormatsFeaturesEXT::FormatA4b4g4r4);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevice4444FormatsFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDevice4444FormatsFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::FormatA4r4g4b4 => vk_struct.format_a4r4g4b4 = 1,
            Self::FormatA4b4g4r4 => vk_struct.format_a4b4g4r4 = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    PrimitiveTopologyListRestart,
    PrimitiveTopologyPatchListRestart,
}
impl const From<PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT) -> Self {
        FeatureType::DevicePrimitiveTopologyListRestartFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DevicePrimitiveTopologyListRestartFeaturesEXT {
    PrimitiveTopologyListRestart,
    PrimitiveTopologyPatchListRestart,
}
impl ToPhysicalFeature for DevicePrimitiveTopologyListRestartFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DevicePrimitiveTopologyListRestartFeaturesEXT :: PrimitiveTopologyListRestart => PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT :: PrimitiveTopologyListRestart , DevicePrimitiveTopologyListRestartFeaturesEXT :: PrimitiveTopologyPatchListRestart => PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT :: PrimitiveTopologyPatchListRestart , }
    }
}
impl From<DevicePrimitiveTopologyListRestartFeaturesEXT> for DeviceFeature {
    fn from(feature: DevicePrimitiveTopologyListRestartFeaturesEXT) -> Self {
        DeviceFeature::DevicePrimitiveTopologyListRestartFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.primitive_topology_list_restart != 0 {
            set.insert(
                PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT::PrimitiveTopologyListRestart,
            );
        }
        if self.primitive_topology_patch_list_restart != 0 {
            set . insert (PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT :: PrimitiveTopologyPatchListRestart) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::PrimitiveTopologyListRestart => vk_struct.primitive_topology_list_restart = 1,
            Self::PrimitiveTopologyPatchListRestart => {
                vk_struct.primitive_topology_patch_list_restart = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderIntegerDotProductFeatures {
    ShaderIntegerDotProduct,
}
impl const From<PhysicalDeviceShaderIntegerDotProductFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceShaderIntegerDotProductFeatures) -> Self {
        FeatureType::DeviceShaderIntegerDotProductFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderIntegerDotProductFeatures {
    ShaderIntegerDotProduct,
}
impl ToPhysicalFeature for DeviceShaderIntegerDotProductFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderIntegerDotProductFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderIntegerDotProductFeatures::ShaderIntegerDotProduct => {
                PhysicalDeviceShaderIntegerDotProductFeatures::ShaderIntegerDotProduct
            }
        }
    }
}
impl From<DeviceShaderIntegerDotProductFeatures> for DeviceFeature {
    fn from(feature: DeviceShaderIntegerDotProductFeatures) -> Self {
        DeviceFeature::DeviceShaderIntegerDotProductFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderIntegerDotProductFeatures {
    type SubFeatureEnumTy = PhysicalDeviceShaderIntegerDotProductFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_integer_dot_product != 0 {
            set.insert(PhysicalDeviceShaderIntegerDotProductFeatures::ShaderIntegerDotProduct);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderIntegerDotProductFeatures {
    type VkStruct = ash::vk::PhysicalDeviceShaderIntegerDotProductFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderIntegerDotProduct => vk_struct.shader_integer_dot_product = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceInlineUniformBlockFeatures {
    InlineUniformBlock,
    DescriptorBindingInlineUniformBlockUpdateAfterBind,
}
impl const From<PhysicalDeviceInlineUniformBlockFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceInlineUniformBlockFeatures) -> Self {
        FeatureType::DeviceInlineUniformBlockFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceInlineUniformBlockFeatures {
    InlineUniformBlock,
    DescriptorBindingInlineUniformBlockUpdateAfterBind,
}
impl ToPhysicalFeature for DeviceInlineUniformBlockFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceInlineUniformBlockFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceInlineUniformBlockFeatures :: InlineUniformBlock => PhysicalDeviceInlineUniformBlockFeatures :: InlineUniformBlock , DeviceInlineUniformBlockFeatures :: DescriptorBindingInlineUniformBlockUpdateAfterBind => PhysicalDeviceInlineUniformBlockFeatures :: DescriptorBindingInlineUniformBlockUpdateAfterBind , }
    }
}
impl From<DeviceInlineUniformBlockFeatures> for DeviceFeature {
    fn from(feature: DeviceInlineUniformBlockFeatures) -> Self {
        DeviceFeature::DeviceInlineUniformBlockFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceInlineUniformBlockFeatures {
    type SubFeatureEnumTy = PhysicalDeviceInlineUniformBlockFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.inline_uniform_block != 0 {
            set.insert(PhysicalDeviceInlineUniformBlockFeatures::InlineUniformBlock);
        }
        if self.descriptor_binding_inline_uniform_block_update_after_bind != 0 {
            set . insert (PhysicalDeviceInlineUniformBlockFeatures :: DescriptorBindingInlineUniformBlockUpdateAfterBind) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceInlineUniformBlockFeatures {
    type VkStruct = ash::vk::PhysicalDeviceInlineUniformBlockFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::InlineUniformBlock => vk_struct.inline_uniform_block = 1,
            Self::DescriptorBindingInlineUniformBlockUpdateAfterBind => {
                vk_struct.descriptor_binding_inline_uniform_block_update_after_bind = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    SeparateDepthStencilLayouts,
}
impl const From<PhysicalDeviceSeparateDepthStencilLayoutsFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceSeparateDepthStencilLayoutsFeatures) -> Self {
        FeatureType::DeviceSeparateDepthStencilLayoutsFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceSeparateDepthStencilLayoutsFeatures {
    SeparateDepthStencilLayouts,
}
impl ToPhysicalFeature for DeviceSeparateDepthStencilLayoutsFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceSeparateDepthStencilLayoutsFeatures::SeparateDepthStencilLayouts => {
                PhysicalDeviceSeparateDepthStencilLayoutsFeatures::SeparateDepthStencilLayouts
            }
        }
    }
}
impl From<DeviceSeparateDepthStencilLayoutsFeatures> for DeviceFeature {
    fn from(feature: DeviceSeparateDepthStencilLayoutsFeatures) -> Self {
        DeviceFeature::DeviceSeparateDepthStencilLayoutsFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    type SubFeatureEnumTy = PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.separate_depth_stencil_layouts != 0 {
            set.insert(
                PhysicalDeviceSeparateDepthStencilLayoutsFeatures::SeparateDepthStencilLayouts,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    type VkStruct = ash::vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::SeparateDepthStencilLayouts => vk_struct.separate_depth_stencil_layouts = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    FormatRgba10x6WithoutYCbCrSampler,
}
impl const From<PhysicalDeviceRGBA10X6FormatsFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceRGBA10X6FormatsFeaturesEXT) -> Self {
        FeatureType::DeviceRGBA10X6FormatsFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceRGBA10X6FormatsFeaturesEXT {
    FormatRgba10x6WithoutYCbCrSampler,
}
impl ToPhysicalFeature for DeviceRGBA10X6FormatsFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceRGBA10X6FormatsFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceRGBA10X6FormatsFeaturesEXT::FormatRgba10x6WithoutYCbCrSampler => {
                PhysicalDeviceRGBA10X6FormatsFeaturesEXT::FormatRgba10x6WithoutYCbCrSampler
            }
        }
    }
}
impl From<DeviceRGBA10X6FormatsFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceRGBA10X6FormatsFeaturesEXT) -> Self {
        DeviceFeature::DeviceRGBA10X6FormatsFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceRGBA10X6FormatsFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.format_rgba10x6_without_y_cb_cr_sampler != 0 {
            set.insert(PhysicalDeviceRGBA10X6FormatsFeaturesEXT::FormatRgba10x6WithoutYCbCrSampler);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::FormatRgba10x6WithoutYCbCrSampler => {
                vk_struct.format_rgba10x6_without_y_cb_cr_sampler = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShadingRateImageFeaturesNV {
    ShadingRateImage,
    ShadingRateCoarseSampleOrder,
}
impl const From<PhysicalDeviceShadingRateImageFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceShadingRateImageFeaturesNV) -> Self {
        FeatureType::DeviceShadingRateImageFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShadingRateImageFeaturesNV {
    ShadingRateImage,
    ShadingRateCoarseSampleOrder,
}
impl ToPhysicalFeature for DeviceShadingRateImageFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShadingRateImageFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShadingRateImageFeaturesNV::ShadingRateImage => {
                PhysicalDeviceShadingRateImageFeaturesNV::ShadingRateImage
            }
            DeviceShadingRateImageFeaturesNV::ShadingRateCoarseSampleOrder => {
                PhysicalDeviceShadingRateImageFeaturesNV::ShadingRateCoarseSampleOrder
            }
        }
    }
}
impl From<DeviceShadingRateImageFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceShadingRateImageFeaturesNV) -> Self {
        DeviceFeature::DeviceShadingRateImageFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShadingRateImageFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceShadingRateImageFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shading_rate_image != 0 {
            set.insert(PhysicalDeviceShadingRateImageFeaturesNV::ShadingRateImage);
        }
        if self.shading_rate_coarse_sample_order != 0 {
            set.insert(PhysicalDeviceShadingRateImageFeaturesNV::ShadingRateCoarseSampleOrder);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShadingRateImageFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceShadingRateImageFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShadingRateImage => vk_struct.shading_rate_image = 1,
            Self::ShadingRateCoarseSampleOrder => vk_struct.shading_rate_coarse_sample_order = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    DescriptorSetHostMapping,
}
impl const From<PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE> for FeatureType {
    fn from(feature: PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE) -> Self {
        FeatureType::DeviceDescriptorSetHostMappingFeaturesVALVE(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceDescriptorSetHostMappingFeaturesVALVE {
    DescriptorSetHostMapping,
}
impl ToPhysicalFeature for DeviceDescriptorSetHostMappingFeaturesVALVE {
    type PhysicalDeviceFeatureTy = PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceDescriptorSetHostMappingFeaturesVALVE::DescriptorSetHostMapping => {
                PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE::DescriptorSetHostMapping
            }
        }
    }
}
impl From<DeviceDescriptorSetHostMappingFeaturesVALVE> for DeviceFeature {
    fn from(feature: DeviceDescriptorSetHostMappingFeaturesVALVE) -> Self {
        DeviceFeature::DeviceDescriptorSetHostMappingFeaturesVALVE(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    type SubFeatureEnumTy = PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.descriptor_set_host_mapping != 0 {
            set.insert(
                PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE::DescriptorSetHostMapping,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    type VkStruct = ash::vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DescriptorSetHostMapping => vk_struct.descriptor_set_host_mapping = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    BufferDeviceAddress,
    BufferDeviceAddressCaptureReplay,
    BufferDeviceAddressMultiDevice,
}
impl const From<PhysicalDeviceBufferDeviceAddressFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceBufferDeviceAddressFeaturesEXT) -> Self {
        FeatureType::DeviceBufferDeviceAddressFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceBufferDeviceAddressFeaturesEXT {
    BufferDeviceAddress,
    BufferDeviceAddressCaptureReplay,
    BufferDeviceAddressMultiDevice,
}
impl ToPhysicalFeature for DeviceBufferDeviceAddressFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceBufferDeviceAddressFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceBufferDeviceAddressFeaturesEXT::BufferDeviceAddress => {
                PhysicalDeviceBufferDeviceAddressFeaturesEXT::BufferDeviceAddress
            }
            DeviceBufferDeviceAddressFeaturesEXT::BufferDeviceAddressCaptureReplay => {
                PhysicalDeviceBufferDeviceAddressFeaturesEXT::BufferDeviceAddressCaptureReplay
            }
            DeviceBufferDeviceAddressFeaturesEXT::BufferDeviceAddressMultiDevice => {
                PhysicalDeviceBufferDeviceAddressFeaturesEXT::BufferDeviceAddressMultiDevice
            }
        }
    }
}
impl From<DeviceBufferDeviceAddressFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceBufferDeviceAddressFeaturesEXT) -> Self {
        DeviceFeature::DeviceBufferDeviceAddressFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceBufferDeviceAddressFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.buffer_device_address != 0 {
            set.insert(PhysicalDeviceBufferDeviceAddressFeaturesEXT::BufferDeviceAddress);
        }
        if self.buffer_device_address_capture_replay != 0 {
            set.insert(
                PhysicalDeviceBufferDeviceAddressFeaturesEXT::BufferDeviceAddressCaptureReplay,
            );
        }
        if self.buffer_device_address_multi_device != 0 {
            set.insert(
                PhysicalDeviceBufferDeviceAddressFeaturesEXT::BufferDeviceAddressMultiDevice,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::BufferDeviceAddress => vk_struct.buffer_device_address = 1,
            Self::BufferDeviceAddressCaptureReplay => {
                vk_struct.buffer_device_address_capture_replay = 1
            }
            Self::BufferDeviceAddressMultiDevice => {
                vk_struct.buffer_device_address_multi_device = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    PipelineExecutableInfo,
}
impl const From<PhysicalDevicePipelineExecutablePropertiesFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDevicePipelineExecutablePropertiesFeaturesKHR) -> Self {
        FeatureType::DevicePipelineExecutablePropertiesFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DevicePipelineExecutablePropertiesFeaturesKHR {
    PipelineExecutableInfo,
}
impl ToPhysicalFeature for DevicePipelineExecutablePropertiesFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DevicePipelineExecutablePropertiesFeaturesKHR::PipelineExecutableInfo => {
                PhysicalDevicePipelineExecutablePropertiesFeaturesKHR::PipelineExecutableInfo
            }
        }
    }
}
impl From<DevicePipelineExecutablePropertiesFeaturesKHR> for DeviceFeature {
    fn from(feature: DevicePipelineExecutablePropertiesFeaturesKHR) -> Self {
        DeviceFeature::DevicePipelineExecutablePropertiesFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.pipeline_executable_info != 0 {
            set.insert(
                PhysicalDevicePipelineExecutablePropertiesFeaturesKHR::PipelineExecutableInfo,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::PipelineExecutableInfo => vk_struct.pipeline_executable_info = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceHostQueryResetFeatures {
    HostQueryReset,
}
impl const From<PhysicalDeviceHostQueryResetFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceHostQueryResetFeatures) -> Self {
        FeatureType::DeviceHostQueryResetFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceHostQueryResetFeatures {
    HostQueryReset,
}
impl ToPhysicalFeature for DeviceHostQueryResetFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceHostQueryResetFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceHostQueryResetFeatures::HostQueryReset => {
                PhysicalDeviceHostQueryResetFeatures::HostQueryReset
            }
        }
    }
}
impl From<DeviceHostQueryResetFeatures> for DeviceFeature {
    fn from(feature: DeviceHostQueryResetFeatures) -> Self {
        DeviceFeature::DeviceHostQueryResetFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceHostQueryResetFeatures {
    type SubFeatureEnumTy = PhysicalDeviceHostQueryResetFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.host_query_reset != 0 {
            set.insert(PhysicalDeviceHostQueryResetFeatures::HostQueryReset);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceHostQueryResetFeatures {
    type VkStruct = ash::vk::PhysicalDeviceHostQueryResetFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::HostQueryReset => vk_struct.host_query_reset = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    RasterizationOrderColorAttachmentAccess,
    RasterizationOrderDepthAttachmentAccess,
    RasterizationOrderStencilAttachmentAccess,
}
impl const From<PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM> for FeatureType {
    fn from(feature: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM) -> Self {
        FeatureType::DeviceRasterizationOrderAttachmentAccessFeaturesARM(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceRasterizationOrderAttachmentAccessFeaturesARM {
    RasterizationOrderColorAttachmentAccess,
    RasterizationOrderDepthAttachmentAccess,
    RasterizationOrderStencilAttachmentAccess,
}
impl ToPhysicalFeature for DeviceRasterizationOrderAttachmentAccessFeaturesARM {
    type PhysicalDeviceFeatureTy = PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceRasterizationOrderAttachmentAccessFeaturesARM :: RasterizationOrderColorAttachmentAccess => PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM :: RasterizationOrderColorAttachmentAccess , DeviceRasterizationOrderAttachmentAccessFeaturesARM :: RasterizationOrderDepthAttachmentAccess => PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM :: RasterizationOrderDepthAttachmentAccess , DeviceRasterizationOrderAttachmentAccessFeaturesARM :: RasterizationOrderStencilAttachmentAccess => PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM :: RasterizationOrderStencilAttachmentAccess , }
    }
}
impl From<DeviceRasterizationOrderAttachmentAccessFeaturesARM> for DeviceFeature {
    fn from(feature: DeviceRasterizationOrderAttachmentAccessFeaturesARM) -> Self {
        DeviceFeature::DeviceRasterizationOrderAttachmentAccessFeaturesARM(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    type SubFeatureEnumTy = PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.rasterization_order_color_attachment_access != 0 {
            set . insert (PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM :: RasterizationOrderColorAttachmentAccess) ;
        }
        if self.rasterization_order_depth_attachment_access != 0 {
            set . insert (PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM :: RasterizationOrderDepthAttachmentAccess) ;
        }
        if self.rasterization_order_stencil_attachment_access != 0 {
            set . insert (PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM :: RasterizationOrderStencilAttachmentAccess) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    type VkStruct = ash::vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::RasterizationOrderColorAttachmentAccess => {
                vk_struct.rasterization_order_color_attachment_access = 1
            }
            Self::RasterizationOrderDepthAttachmentAccess => {
                vk_struct.rasterization_order_depth_attachment_access = 1
            }
            Self::RasterizationOrderStencilAttachmentAccess => {
                vk_struct.rasterization_order_stencil_attachment_access = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    FragmentDensityMapOffset,
}
impl const From<PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM> for FeatureType {
    fn from(feature: PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM) -> Self {
        FeatureType::DeviceFragmentDensityMapOffsetFeaturesQCOM(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceFragmentDensityMapOffsetFeaturesQCOM {
    FragmentDensityMapOffset,
}
impl ToPhysicalFeature for DeviceFragmentDensityMapOffsetFeaturesQCOM {
    type PhysicalDeviceFeatureTy = PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceFragmentDensityMapOffsetFeaturesQCOM::FragmentDensityMapOffset => {
                PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM::FragmentDensityMapOffset
            }
        }
    }
}
impl From<DeviceFragmentDensityMapOffsetFeaturesQCOM> for DeviceFeature {
    fn from(feature: DeviceFragmentDensityMapOffsetFeaturesQCOM) -> Self {
        DeviceFeature::DeviceFragmentDensityMapOffsetFeaturesQCOM(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    type SubFeatureEnumTy = PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.fragment_density_map_offset != 0 {
            set.insert(
                PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM::FragmentDensityMapOffset,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
    type VkStruct = ash::vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::FragmentDensityMapOffset => vk_struct.fragment_density_map_offset = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceTransformFeedbackFeaturesEXT {
    TransformFeedback,
    GeometryStreams,
}
impl const From<PhysicalDeviceTransformFeedbackFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceTransformFeedbackFeaturesEXT) -> Self {
        FeatureType::DeviceTransformFeedbackFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceTransformFeedbackFeaturesEXT {
    TransformFeedback,
    GeometryStreams,
}
impl ToPhysicalFeature for DeviceTransformFeedbackFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceTransformFeedbackFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceTransformFeedbackFeaturesEXT::TransformFeedback => {
                PhysicalDeviceTransformFeedbackFeaturesEXT::TransformFeedback
            }
            DeviceTransformFeedbackFeaturesEXT::GeometryStreams => {
                PhysicalDeviceTransformFeedbackFeaturesEXT::GeometryStreams
            }
        }
    }
}
impl From<DeviceTransformFeedbackFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceTransformFeedbackFeaturesEXT) -> Self {
        DeviceFeature::DeviceTransformFeedbackFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceTransformFeedbackFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceTransformFeedbackFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.transform_feedback != 0 {
            set.insert(PhysicalDeviceTransformFeedbackFeaturesEXT::TransformFeedback);
        }
        if self.geometry_streams != 0 {
            set.insert(PhysicalDeviceTransformFeedbackFeaturesEXT::GeometryStreams);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceTransformFeedbackFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceTransformFeedbackFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::TransformFeedback => vk_struct.transform_feedback = 1,
            Self::GeometryStreams => vk_struct.geometry_streams = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevicePerformanceQueryFeaturesKHR {
    PerformanceCounterQueryPools,
    PerformanceCounterMultipleQueryPools,
}
impl const From<PhysicalDevicePerformanceQueryFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDevicePerformanceQueryFeaturesKHR) -> Self {
        FeatureType::DevicePerformanceQueryFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DevicePerformanceQueryFeaturesKHR {
    PerformanceCounterQueryPools,
    PerformanceCounterMultipleQueryPools,
}
impl ToPhysicalFeature for DevicePerformanceQueryFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDevicePerformanceQueryFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DevicePerformanceQueryFeaturesKHR::PerformanceCounterQueryPools => {
                PhysicalDevicePerformanceQueryFeaturesKHR::PerformanceCounterQueryPools
            }
            DevicePerformanceQueryFeaturesKHR::PerformanceCounterMultipleQueryPools => {
                PhysicalDevicePerformanceQueryFeaturesKHR::PerformanceCounterMultipleQueryPools
            }
        }
    }
}
impl From<DevicePerformanceQueryFeaturesKHR> for DeviceFeature {
    fn from(feature: DevicePerformanceQueryFeaturesKHR) -> Self {
        DeviceFeature::DevicePerformanceQueryFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevicePerformanceQueryFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDevicePerformanceQueryFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.performance_counter_query_pools != 0 {
            set.insert(PhysicalDevicePerformanceQueryFeaturesKHR::PerformanceCounterQueryPools);
        }
        if self.performance_counter_multiple_query_pools != 0 {
            set.insert(
                PhysicalDevicePerformanceQueryFeaturesKHR::PerformanceCounterMultipleQueryPools,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevicePerformanceQueryFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDevicePerformanceQueryFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::PerformanceCounterQueryPools => vk_struct.performance_counter_query_pools = 1,
            Self::PerformanceCounterMultipleQueryPools => {
                vk_struct.performance_counter_multiple_query_pools = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    DedicatedAllocationImageAliasing,
}
impl const From<PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV) -> Self {
        FeatureType::DeviceDedicatedAllocationImageAliasingFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceDedicatedAllocationImageAliasingFeaturesNV {
    DedicatedAllocationImageAliasing,
}
impl ToPhysicalFeature for DeviceDedicatedAllocationImageAliasingFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceDedicatedAllocationImageAliasingFeaturesNV :: DedicatedAllocationImageAliasing => PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV :: DedicatedAllocationImageAliasing , }
    }
}
impl From<DeviceDedicatedAllocationImageAliasingFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceDedicatedAllocationImageAliasingFeaturesNV) -> Self {
        DeviceFeature::DeviceDedicatedAllocationImageAliasingFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.dedicated_allocation_image_aliasing != 0 {
            set . insert (PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV :: DedicatedAllocationImageAliasing) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DedicatedAllocationImageAliasing => {
                vk_struct.dedicated_allocation_image_aliasing = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceImageViewMinLodFeaturesEXT {
    MinLod,
}
impl const From<PhysicalDeviceImageViewMinLodFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceImageViewMinLodFeaturesEXT) -> Self {
        FeatureType::DeviceImageViewMinLodFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceImageViewMinLodFeaturesEXT {
    MinLod,
}
impl ToPhysicalFeature for DeviceImageViewMinLodFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceImageViewMinLodFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceImageViewMinLodFeaturesEXT::MinLod => {
                PhysicalDeviceImageViewMinLodFeaturesEXT::MinLod
            }
        }
    }
}
impl From<DeviceImageViewMinLodFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceImageViewMinLodFeaturesEXT) -> Self {
        DeviceFeature::DeviceImageViewMinLodFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceImageViewMinLodFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceImageViewMinLodFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.min_lod != 0 {
            set.insert(PhysicalDeviceImageViewMinLodFeaturesEXT::MinLod);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceImageViewMinLodFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceImageViewMinLodFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::MinLod => vk_struct.min_lod = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    ShaderSubgroupUniformControlFlow,
}
impl const From<PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR) -> Self {
        FeatureType::DeviceShaderSubgroupUniformControlFlowFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    ShaderSubgroupUniformControlFlow,
}
impl ToPhysicalFeature for DeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceShaderSubgroupUniformControlFlowFeaturesKHR :: ShaderSubgroupUniformControlFlow => PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR :: ShaderSubgroupUniformControlFlow , }
    }
}
impl From<DeviceShaderSubgroupUniformControlFlowFeaturesKHR> for DeviceFeature {
    fn from(feature: DeviceShaderSubgroupUniformControlFlowFeaturesKHR) -> Self {
        DeviceFeature::DeviceShaderSubgroupUniformControlFlowFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_subgroup_uniform_control_flow != 0 {
            set . insert (PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR :: ShaderSubgroupUniformControlFlow) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderSubgroupUniformControlFlow => {
                vk_struct.shader_subgroup_uniform_control_flow = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceFeatures {
    RobustBufferAccess,
    FullDrawIndexUint32,
    ImageCubeArray,
    IndependentBlend,
    GeometryShader,
    TessellationShader,
    SampleRateShading,
    DualSrcBlend,
    LogicOp,
    MultiDrawIndirect,
    DrawIndirectFirstInstance,
    DepthClamp,
    DepthBiasClamp,
    FillModeNonSolid,
    DepthBounds,
    WideLines,
    LargePoints,
    AlphaToOne,
    MultiViewport,
    SamplerAnisotropy,
    TextureCompressionEtc2,
    TextureCompressionAstcLdr,
    TextureCompressionBc,
    OcclusionQueryPrecise,
    PipelineStatisticsQuery,
    VertexPipelineStoresAndAtomics,
    FragmentStoresAndAtomics,
    ShaderTessellationAndGeometryPointSize,
    ShaderImageGatherExtended,
    ShaderStorageImageExtendedFormats,
    ShaderStorageImageMultisample,
    ShaderStorageImageReadWithoutFormat,
    ShaderStorageImageWriteWithoutFormat,
    ShaderUniformBufferArrayDynamicIndexing,
    ShaderSampledImageArrayDynamicIndexing,
    ShaderStorageBufferArrayDynamicIndexing,
    ShaderStorageImageArrayDynamicIndexing,
    ShaderClipDistance,
    ShaderCullDistance,
    ShaderFloat64,
    ShaderInt64,
    ShaderInt16,
    ShaderResourceResidency,
    ShaderResourceMinLod,
    SparseBinding,
    SparseResidencyBuffer,
    SparseResidencyImage2D,
    SparseResidencyImage3D,
    SparseResidency2Samples,
    SparseResidency4Samples,
    SparseResidency8Samples,
    SparseResidency16Samples,
    SparseResidencyAliased,
    VariableMultisampleRate,
    InheritedQueries,
}
impl const From<PhysicalDeviceFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceFeatures) -> Self {
        FeatureType::DeviceFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceFeatures {
    RobustBufferAccess,
    FullDrawIndexUint32,
    ImageCubeArray,
    IndependentBlend,
    GeometryShader,
    TessellationShader,
    SampleRateShading,
    DualSrcBlend,
    LogicOp,
    MultiDrawIndirect,
    DrawIndirectFirstInstance,
    DepthClamp,
    DepthBiasClamp,
    FillModeNonSolid,
    DepthBounds,
    WideLines,
    LargePoints,
    AlphaToOne,
    MultiViewport,
    SamplerAnisotropy,
    TextureCompressionEtc2,
    TextureCompressionAstcLdr,
    TextureCompressionBc,
    OcclusionQueryPrecise,
    PipelineStatisticsQuery,
    VertexPipelineStoresAndAtomics,
    FragmentStoresAndAtomics,
    ShaderTessellationAndGeometryPointSize,
    ShaderImageGatherExtended,
    ShaderStorageImageExtendedFormats,
    ShaderStorageImageMultisample,
    ShaderStorageImageReadWithoutFormat,
    ShaderStorageImageWriteWithoutFormat,
    ShaderUniformBufferArrayDynamicIndexing,
    ShaderSampledImageArrayDynamicIndexing,
    ShaderStorageBufferArrayDynamicIndexing,
    ShaderStorageImageArrayDynamicIndexing,
    ShaderClipDistance,
    ShaderCullDistance,
    ShaderFloat64,
    ShaderInt64,
    ShaderInt16,
    ShaderResourceResidency,
    ShaderResourceMinLod,
    SparseBinding,
    SparseResidencyBuffer,
    SparseResidencyImage2D,
    SparseResidencyImage3D,
    SparseResidency2Samples,
    SparseResidency4Samples,
    SparseResidency8Samples,
    SparseResidency16Samples,
    SparseResidencyAliased,
    VariableMultisampleRate,
    InheritedQueries,
}
impl ToPhysicalFeature for DeviceFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceFeatures::RobustBufferAccess => PhysicalDeviceFeatures::RobustBufferAccess,
            DeviceFeatures::FullDrawIndexUint32 => PhysicalDeviceFeatures::FullDrawIndexUint32,
            DeviceFeatures::ImageCubeArray => PhysicalDeviceFeatures::ImageCubeArray,
            DeviceFeatures::IndependentBlend => PhysicalDeviceFeatures::IndependentBlend,
            DeviceFeatures::GeometryShader => PhysicalDeviceFeatures::GeometryShader,
            DeviceFeatures::TessellationShader => PhysicalDeviceFeatures::TessellationShader,
            DeviceFeatures::SampleRateShading => PhysicalDeviceFeatures::SampleRateShading,
            DeviceFeatures::DualSrcBlend => PhysicalDeviceFeatures::DualSrcBlend,
            DeviceFeatures::LogicOp => PhysicalDeviceFeatures::LogicOp,
            DeviceFeatures::MultiDrawIndirect => PhysicalDeviceFeatures::MultiDrawIndirect,
            DeviceFeatures::DrawIndirectFirstInstance => {
                PhysicalDeviceFeatures::DrawIndirectFirstInstance
            }
            DeviceFeatures::DepthClamp => PhysicalDeviceFeatures::DepthClamp,
            DeviceFeatures::DepthBiasClamp => PhysicalDeviceFeatures::DepthBiasClamp,
            DeviceFeatures::FillModeNonSolid => PhysicalDeviceFeatures::FillModeNonSolid,
            DeviceFeatures::DepthBounds => PhysicalDeviceFeatures::DepthBounds,
            DeviceFeatures::WideLines => PhysicalDeviceFeatures::WideLines,
            DeviceFeatures::LargePoints => PhysicalDeviceFeatures::LargePoints,
            DeviceFeatures::AlphaToOne => PhysicalDeviceFeatures::AlphaToOne,
            DeviceFeatures::MultiViewport => PhysicalDeviceFeatures::MultiViewport,
            DeviceFeatures::SamplerAnisotropy => PhysicalDeviceFeatures::SamplerAnisotropy,
            DeviceFeatures::TextureCompressionEtc2 => {
                PhysicalDeviceFeatures::TextureCompressionEtc2
            }
            DeviceFeatures::TextureCompressionAstcLdr => {
                PhysicalDeviceFeatures::TextureCompressionAstcLdr
            }
            DeviceFeatures::TextureCompressionBc => PhysicalDeviceFeatures::TextureCompressionBc,
            DeviceFeatures::OcclusionQueryPrecise => PhysicalDeviceFeatures::OcclusionQueryPrecise,
            DeviceFeatures::PipelineStatisticsQuery => {
                PhysicalDeviceFeatures::PipelineStatisticsQuery
            }
            DeviceFeatures::VertexPipelineStoresAndAtomics => {
                PhysicalDeviceFeatures::VertexPipelineStoresAndAtomics
            }
            DeviceFeatures::FragmentStoresAndAtomics => {
                PhysicalDeviceFeatures::FragmentStoresAndAtomics
            }
            DeviceFeatures::ShaderTessellationAndGeometryPointSize => {
                PhysicalDeviceFeatures::ShaderTessellationAndGeometryPointSize
            }
            DeviceFeatures::ShaderImageGatherExtended => {
                PhysicalDeviceFeatures::ShaderImageGatherExtended
            }
            DeviceFeatures::ShaderStorageImageExtendedFormats => {
                PhysicalDeviceFeatures::ShaderStorageImageExtendedFormats
            }
            DeviceFeatures::ShaderStorageImageMultisample => {
                PhysicalDeviceFeatures::ShaderStorageImageMultisample
            }
            DeviceFeatures::ShaderStorageImageReadWithoutFormat => {
                PhysicalDeviceFeatures::ShaderStorageImageReadWithoutFormat
            }
            DeviceFeatures::ShaderStorageImageWriteWithoutFormat => {
                PhysicalDeviceFeatures::ShaderStorageImageWriteWithoutFormat
            }
            DeviceFeatures::ShaderUniformBufferArrayDynamicIndexing => {
                PhysicalDeviceFeatures::ShaderUniformBufferArrayDynamicIndexing
            }
            DeviceFeatures::ShaderSampledImageArrayDynamicIndexing => {
                PhysicalDeviceFeatures::ShaderSampledImageArrayDynamicIndexing
            }
            DeviceFeatures::ShaderStorageBufferArrayDynamicIndexing => {
                PhysicalDeviceFeatures::ShaderStorageBufferArrayDynamicIndexing
            }
            DeviceFeatures::ShaderStorageImageArrayDynamicIndexing => {
                PhysicalDeviceFeatures::ShaderStorageImageArrayDynamicIndexing
            }
            DeviceFeatures::ShaderClipDistance => PhysicalDeviceFeatures::ShaderClipDistance,
            DeviceFeatures::ShaderCullDistance => PhysicalDeviceFeatures::ShaderCullDistance,
            DeviceFeatures::ShaderFloat64 => PhysicalDeviceFeatures::ShaderFloat64,
            DeviceFeatures::ShaderInt64 => PhysicalDeviceFeatures::ShaderInt64,
            DeviceFeatures::ShaderInt16 => PhysicalDeviceFeatures::ShaderInt16,
            DeviceFeatures::ShaderResourceResidency => {
                PhysicalDeviceFeatures::ShaderResourceResidency
            }
            DeviceFeatures::ShaderResourceMinLod => PhysicalDeviceFeatures::ShaderResourceMinLod,
            DeviceFeatures::SparseBinding => PhysicalDeviceFeatures::SparseBinding,
            DeviceFeatures::SparseResidencyBuffer => PhysicalDeviceFeatures::SparseResidencyBuffer,
            DeviceFeatures::SparseResidencyImage2D => {
                PhysicalDeviceFeatures::SparseResidencyImage2D
            }
            DeviceFeatures::SparseResidencyImage3D => {
                PhysicalDeviceFeatures::SparseResidencyImage3D
            }
            DeviceFeatures::SparseResidency2Samples => {
                PhysicalDeviceFeatures::SparseResidency2Samples
            }
            DeviceFeatures::SparseResidency4Samples => {
                PhysicalDeviceFeatures::SparseResidency4Samples
            }
            DeviceFeatures::SparseResidency8Samples => {
                PhysicalDeviceFeatures::SparseResidency8Samples
            }
            DeviceFeatures::SparseResidency16Samples => {
                PhysicalDeviceFeatures::SparseResidency16Samples
            }
            DeviceFeatures::SparseResidencyAliased => {
                PhysicalDeviceFeatures::SparseResidencyAliased
            }
            DeviceFeatures::VariableMultisampleRate => {
                PhysicalDeviceFeatures::VariableMultisampleRate
            }
            DeviceFeatures::InheritedQueries => PhysicalDeviceFeatures::InheritedQueries,
        }
    }
}
impl From<DeviceFeatures> for DeviceFeature {
    fn from(feature: DeviceFeatures) -> Self {
        DeviceFeature::DeviceFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceFeatures {
    type SubFeatureEnumTy = PhysicalDeviceFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.robust_buffer_access != 0 {
            set.insert(PhysicalDeviceFeatures::RobustBufferAccess);
        }
        if self.full_draw_index_uint32 != 0 {
            set.insert(PhysicalDeviceFeatures::FullDrawIndexUint32);
        }
        if self.image_cube_array != 0 {
            set.insert(PhysicalDeviceFeatures::ImageCubeArray);
        }
        if self.independent_blend != 0 {
            set.insert(PhysicalDeviceFeatures::IndependentBlend);
        }
        if self.geometry_shader != 0 {
            set.insert(PhysicalDeviceFeatures::GeometryShader);
        }
        if self.tessellation_shader != 0 {
            set.insert(PhysicalDeviceFeatures::TessellationShader);
        }
        if self.sample_rate_shading != 0 {
            set.insert(PhysicalDeviceFeatures::SampleRateShading);
        }
        if self.dual_src_blend != 0 {
            set.insert(PhysicalDeviceFeatures::DualSrcBlend);
        }
        if self.logic_op != 0 {
            set.insert(PhysicalDeviceFeatures::LogicOp);
        }
        if self.multi_draw_indirect != 0 {
            set.insert(PhysicalDeviceFeatures::MultiDrawIndirect);
        }
        if self.draw_indirect_first_instance != 0 {
            set.insert(PhysicalDeviceFeatures::DrawIndirectFirstInstance);
        }
        if self.depth_clamp != 0 {
            set.insert(PhysicalDeviceFeatures::DepthClamp);
        }
        if self.depth_bias_clamp != 0 {
            set.insert(PhysicalDeviceFeatures::DepthBiasClamp);
        }
        if self.fill_mode_non_solid != 0 {
            set.insert(PhysicalDeviceFeatures::FillModeNonSolid);
        }
        if self.depth_bounds != 0 {
            set.insert(PhysicalDeviceFeatures::DepthBounds);
        }
        if self.wide_lines != 0 {
            set.insert(PhysicalDeviceFeatures::WideLines);
        }
        if self.large_points != 0 {
            set.insert(PhysicalDeviceFeatures::LargePoints);
        }
        if self.alpha_to_one != 0 {
            set.insert(PhysicalDeviceFeatures::AlphaToOne);
        }
        if self.multi_viewport != 0 {
            set.insert(PhysicalDeviceFeatures::MultiViewport);
        }
        if self.sampler_anisotropy != 0 {
            set.insert(PhysicalDeviceFeatures::SamplerAnisotropy);
        }
        if self.texture_compression_etc2 != 0 {
            set.insert(PhysicalDeviceFeatures::TextureCompressionEtc2);
        }
        if self.texture_compression_astc_ldr != 0 {
            set.insert(PhysicalDeviceFeatures::TextureCompressionAstcLdr);
        }
        if self.texture_compression_bc != 0 {
            set.insert(PhysicalDeviceFeatures::TextureCompressionBc);
        }
        if self.occlusion_query_precise != 0 {
            set.insert(PhysicalDeviceFeatures::OcclusionQueryPrecise);
        }
        if self.pipeline_statistics_query != 0 {
            set.insert(PhysicalDeviceFeatures::PipelineStatisticsQuery);
        }
        if self.vertex_pipeline_stores_and_atomics != 0 {
            set.insert(PhysicalDeviceFeatures::VertexPipelineStoresAndAtomics);
        }
        if self.fragment_stores_and_atomics != 0 {
            set.insert(PhysicalDeviceFeatures::FragmentStoresAndAtomics);
        }
        if self.shader_tessellation_and_geometry_point_size != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderTessellationAndGeometryPointSize);
        }
        if self.shader_image_gather_extended != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderImageGatherExtended);
        }
        if self.shader_storage_image_extended_formats != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderStorageImageExtendedFormats);
        }
        if self.shader_storage_image_multisample != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderStorageImageMultisample);
        }
        if self.shader_storage_image_read_without_format != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderStorageImageReadWithoutFormat);
        }
        if self.shader_storage_image_write_without_format != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderStorageImageWriteWithoutFormat);
        }
        if self.shader_uniform_buffer_array_dynamic_indexing != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderUniformBufferArrayDynamicIndexing);
        }
        if self.shader_sampled_image_array_dynamic_indexing != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderSampledImageArrayDynamicIndexing);
        }
        if self.shader_storage_buffer_array_dynamic_indexing != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderStorageBufferArrayDynamicIndexing);
        }
        if self.shader_storage_image_array_dynamic_indexing != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderStorageImageArrayDynamicIndexing);
        }
        if self.shader_clip_distance != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderClipDistance);
        }
        if self.shader_cull_distance != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderCullDistance);
        }
        if self.shader_float64 != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderFloat64);
        }
        if self.shader_int64 != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderInt64);
        }
        if self.shader_int16 != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderInt16);
        }
        if self.shader_resource_residency != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderResourceResidency);
        }
        if self.shader_resource_min_lod != 0 {
            set.insert(PhysicalDeviceFeatures::ShaderResourceMinLod);
        }
        if self.sparse_binding != 0 {
            set.insert(PhysicalDeviceFeatures::SparseBinding);
        }
        if self.sparse_residency_buffer != 0 {
            set.insert(PhysicalDeviceFeatures::SparseResidencyBuffer);
        }
        if self.sparse_residency_image2_d != 0 {
            set.insert(PhysicalDeviceFeatures::SparseResidencyImage2D);
        }
        if self.sparse_residency_image3_d != 0 {
            set.insert(PhysicalDeviceFeatures::SparseResidencyImage3D);
        }
        if self.sparse_residency2_samples != 0 {
            set.insert(PhysicalDeviceFeatures::SparseResidency2Samples);
        }
        if self.sparse_residency4_samples != 0 {
            set.insert(PhysicalDeviceFeatures::SparseResidency4Samples);
        }
        if self.sparse_residency8_samples != 0 {
            set.insert(PhysicalDeviceFeatures::SparseResidency8Samples);
        }
        if self.sparse_residency16_samples != 0 {
            set.insert(PhysicalDeviceFeatures::SparseResidency16Samples);
        }
        if self.sparse_residency_aliased != 0 {
            set.insert(PhysicalDeviceFeatures::SparseResidencyAliased);
        }
        if self.variable_multisample_rate != 0 {
            set.insert(PhysicalDeviceFeatures::VariableMultisampleRate);
        }
        if self.inherited_queries != 0 {
            set.insert(PhysicalDeviceFeatures::InheritedQueries);
        }
        set
    }
}
impl PhysicalDeviceFeatures {
    fn register(&self, vk_struct: &mut ash::vk::PhysicalDeviceFeatures) {
        match self {
            Self::RobustBufferAccess => vk_struct.robust_buffer_access = 1,
            Self::FullDrawIndexUint32 => vk_struct.full_draw_index_uint32 = 1,
            Self::ImageCubeArray => vk_struct.image_cube_array = 1,
            Self::IndependentBlend => vk_struct.independent_blend = 1,
            Self::GeometryShader => vk_struct.geometry_shader = 1,
            Self::TessellationShader => vk_struct.tessellation_shader = 1,
            Self::SampleRateShading => vk_struct.sample_rate_shading = 1,
            Self::DualSrcBlend => vk_struct.dual_src_blend = 1,
            Self::LogicOp => vk_struct.logic_op = 1,
            Self::MultiDrawIndirect => vk_struct.multi_draw_indirect = 1,
            Self::DrawIndirectFirstInstance => vk_struct.draw_indirect_first_instance = 1,
            Self::DepthClamp => vk_struct.depth_clamp = 1,
            Self::DepthBiasClamp => vk_struct.depth_bias_clamp = 1,
            Self::FillModeNonSolid => vk_struct.fill_mode_non_solid = 1,
            Self::DepthBounds => vk_struct.depth_bounds = 1,
            Self::WideLines => vk_struct.wide_lines = 1,
            Self::LargePoints => vk_struct.large_points = 1,
            Self::AlphaToOne => vk_struct.alpha_to_one = 1,
            Self::MultiViewport => vk_struct.multi_viewport = 1,
            Self::SamplerAnisotropy => vk_struct.sampler_anisotropy = 1,
            Self::TextureCompressionEtc2 => vk_struct.texture_compression_etc2 = 1,
            Self::TextureCompressionAstcLdr => vk_struct.texture_compression_astc_ldr = 1,
            Self::TextureCompressionBc => vk_struct.texture_compression_bc = 1,
            Self::OcclusionQueryPrecise => vk_struct.occlusion_query_precise = 1,
            Self::PipelineStatisticsQuery => vk_struct.pipeline_statistics_query = 1,
            Self::VertexPipelineStoresAndAtomics => {
                vk_struct.vertex_pipeline_stores_and_atomics = 1
            }
            Self::FragmentStoresAndAtomics => vk_struct.fragment_stores_and_atomics = 1,
            Self::ShaderTessellationAndGeometryPointSize => {
                vk_struct.shader_tessellation_and_geometry_point_size = 1
            }
            Self::ShaderImageGatherExtended => vk_struct.shader_image_gather_extended = 1,
            Self::ShaderStorageImageExtendedFormats => {
                vk_struct.shader_storage_image_extended_formats = 1
            }
            Self::ShaderStorageImageMultisample => vk_struct.shader_storage_image_multisample = 1,
            Self::ShaderStorageImageReadWithoutFormat => {
                vk_struct.shader_storage_image_read_without_format = 1
            }
            Self::ShaderStorageImageWriteWithoutFormat => {
                vk_struct.shader_storage_image_write_without_format = 1
            }
            Self::ShaderUniformBufferArrayDynamicIndexing => {
                vk_struct.shader_uniform_buffer_array_dynamic_indexing = 1
            }
            Self::ShaderSampledImageArrayDynamicIndexing => {
                vk_struct.shader_sampled_image_array_dynamic_indexing = 1
            }
            Self::ShaderStorageBufferArrayDynamicIndexing => {
                vk_struct.shader_storage_buffer_array_dynamic_indexing = 1
            }
            Self::ShaderStorageImageArrayDynamicIndexing => {
                vk_struct.shader_storage_image_array_dynamic_indexing = 1
            }
            Self::ShaderClipDistance => vk_struct.shader_clip_distance = 1,
            Self::ShaderCullDistance => vk_struct.shader_cull_distance = 1,
            Self::ShaderFloat64 => vk_struct.shader_float64 = 1,
            Self::ShaderInt64 => vk_struct.shader_int64 = 1,
            Self::ShaderInt16 => vk_struct.shader_int16 = 1,
            Self::ShaderResourceResidency => vk_struct.shader_resource_residency = 1,
            Self::ShaderResourceMinLod => vk_struct.shader_resource_min_lod = 1,
            Self::SparseBinding => vk_struct.sparse_binding = 1,
            Self::SparseResidencyBuffer => vk_struct.sparse_residency_buffer = 1,
            Self::SparseResidencyImage2D => vk_struct.sparse_residency_image2_d = 1,
            Self::SparseResidencyImage3D => vk_struct.sparse_residency_image3_d = 1,
            Self::SparseResidency2Samples => vk_struct.sparse_residency2_samples = 1,
            Self::SparseResidency4Samples => vk_struct.sparse_residency4_samples = 1,
            Self::SparseResidency8Samples => vk_struct.sparse_residency8_samples = 1,
            Self::SparseResidency16Samples => vk_struct.sparse_residency16_samples = 1,
            Self::SparseResidencyAliased => vk_struct.sparse_residency_aliased = 1,
            Self::VariableMultisampleRate => vk_struct.variable_multisample_rate = 1,
            Self::InheritedQueries => vk_struct.inherited_queries = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceIndexTypeUint8FeaturesEXT {
    IndexTypeUint8,
}
impl const From<PhysicalDeviceIndexTypeUint8FeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceIndexTypeUint8FeaturesEXT) -> Self {
        FeatureType::DeviceIndexTypeUint8FeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceIndexTypeUint8FeaturesEXT {
    IndexTypeUint8,
}
impl ToPhysicalFeature for DeviceIndexTypeUint8FeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceIndexTypeUint8FeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceIndexTypeUint8FeaturesEXT::IndexTypeUint8 => {
                PhysicalDeviceIndexTypeUint8FeaturesEXT::IndexTypeUint8
            }
        }
    }
}
impl From<DeviceIndexTypeUint8FeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceIndexTypeUint8FeaturesEXT) -> Self {
        DeviceFeature::DeviceIndexTypeUint8FeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceIndexTypeUint8FeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceIndexTypeUint8FeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.index_type_uint8 != 0 {
            set.insert(PhysicalDeviceIndexTypeUint8FeaturesEXT::IndexTypeUint8);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceIndexTypeUint8FeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceIndexTypeUint8FeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::IndexTypeUint8 => vk_struct.index_type_uint8 = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceVulkan12Features {
    SamplerMirrorClampToEdge,
    DrawIndirectCount,
    StorageBuffer8BitAccess,
    UniformAndStorageBuffer8BitAccess,
    StoragePushConstant8,
    ShaderBufferInt64Atomics,
    ShaderSharedInt64Atomics,
    ShaderFloat16,
    ShaderInt8,
    DescriptorIndexing,
    ShaderInputAttachmentArrayDynamicIndexing,
    ShaderUniformTexelBufferArrayDynamicIndexing,
    ShaderStorageTexelBufferArrayDynamicIndexing,
    ShaderUniformBufferArrayNonUniformIndexing,
    ShaderSampledImageArrayNonUniformIndexing,
    ShaderStorageBufferArrayNonUniformIndexing,
    ShaderStorageImageArrayNonUniformIndexing,
    ShaderInputAttachmentArrayNonUniformIndexing,
    ShaderUniformTexelBufferArrayNonUniformIndexing,
    ShaderStorageTexelBufferArrayNonUniformIndexing,
    DescriptorBindingUniformBufferUpdateAfterBind,
    DescriptorBindingSampledImageUpdateAfterBind,
    DescriptorBindingStorageImageUpdateAfterBind,
    DescriptorBindingStorageBufferUpdateAfterBind,
    DescriptorBindingUniformTexelBufferUpdateAfterBind,
    DescriptorBindingStorageTexelBufferUpdateAfterBind,
    DescriptorBindingUpdateUnusedWhilePending,
    DescriptorBindingPartiallyBound,
    DescriptorBindingVariableDescriptorCount,
    RuntimeDescriptorArray,
    SamplerFilterMinmax,
    ScalarBlockLayout,
    ImagelessFramebuffer,
    UniformBufferStandardLayout,
    ShaderSubgroupExtendedTypes,
    SeparateDepthStencilLayouts,
    HostQueryReset,
    TimelineSemaphore,
    BufferDeviceAddress,
    BufferDeviceAddressCaptureReplay,
    BufferDeviceAddressMultiDevice,
    VulkanMemoryModel,
    VulkanMemoryModelDeviceScope,
    VulkanMemoryModelAvailabilityVisibilityChains,
    ShaderOutputViewportIndex,
    ShaderOutputLayer,
    SubgroupBroadcastDynamicId,
}
impl const From<PhysicalDeviceVulkan12Features> for FeatureType {
    fn from(feature: PhysicalDeviceVulkan12Features) -> Self {
        FeatureType::DeviceVulkan12Features(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceVulkan12Features {
    SamplerMirrorClampToEdge,
    DrawIndirectCount,
    StorageBuffer8BitAccess,
    UniformAndStorageBuffer8BitAccess,
    StoragePushConstant8,
    ShaderBufferInt64Atomics,
    ShaderSharedInt64Atomics,
    ShaderFloat16,
    ShaderInt8,
    DescriptorIndexing,
    ShaderInputAttachmentArrayDynamicIndexing,
    ShaderUniformTexelBufferArrayDynamicIndexing,
    ShaderStorageTexelBufferArrayDynamicIndexing,
    ShaderUniformBufferArrayNonUniformIndexing,
    ShaderSampledImageArrayNonUniformIndexing,
    ShaderStorageBufferArrayNonUniformIndexing,
    ShaderStorageImageArrayNonUniformIndexing,
    ShaderInputAttachmentArrayNonUniformIndexing,
    ShaderUniformTexelBufferArrayNonUniformIndexing,
    ShaderStorageTexelBufferArrayNonUniformIndexing,
    DescriptorBindingUniformBufferUpdateAfterBind,
    DescriptorBindingSampledImageUpdateAfterBind,
    DescriptorBindingStorageImageUpdateAfterBind,
    DescriptorBindingStorageBufferUpdateAfterBind,
    DescriptorBindingUniformTexelBufferUpdateAfterBind,
    DescriptorBindingStorageTexelBufferUpdateAfterBind,
    DescriptorBindingUpdateUnusedWhilePending,
    DescriptorBindingPartiallyBound,
    DescriptorBindingVariableDescriptorCount,
    RuntimeDescriptorArray,
    SamplerFilterMinmax,
    ScalarBlockLayout,
    ImagelessFramebuffer,
    UniformBufferStandardLayout,
    ShaderSubgroupExtendedTypes,
    SeparateDepthStencilLayouts,
    HostQueryReset,
    TimelineSemaphore,
    BufferDeviceAddress,
    BufferDeviceAddressCaptureReplay,
    BufferDeviceAddressMultiDevice,
    VulkanMemoryModel,
    VulkanMemoryModelDeviceScope,
    VulkanMemoryModelAvailabilityVisibilityChains,
    ShaderOutputViewportIndex,
    ShaderOutputLayer,
    SubgroupBroadcastDynamicId,
}
impl ToPhysicalFeature for DeviceVulkan12Features {
    type PhysicalDeviceFeatureTy = PhysicalDeviceVulkan12Features;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceVulkan12Features::SamplerMirrorClampToEdge => {
                PhysicalDeviceVulkan12Features::SamplerMirrorClampToEdge
            }
            DeviceVulkan12Features::DrawIndirectCount => {
                PhysicalDeviceVulkan12Features::DrawIndirectCount
            }
            DeviceVulkan12Features::StorageBuffer8BitAccess => {
                PhysicalDeviceVulkan12Features::StorageBuffer8BitAccess
            }
            DeviceVulkan12Features::UniformAndStorageBuffer8BitAccess => {
                PhysicalDeviceVulkan12Features::UniformAndStorageBuffer8BitAccess
            }
            DeviceVulkan12Features::StoragePushConstant8 => {
                PhysicalDeviceVulkan12Features::StoragePushConstant8
            }
            DeviceVulkan12Features::ShaderBufferInt64Atomics => {
                PhysicalDeviceVulkan12Features::ShaderBufferInt64Atomics
            }
            DeviceVulkan12Features::ShaderSharedInt64Atomics => {
                PhysicalDeviceVulkan12Features::ShaderSharedInt64Atomics
            }
            DeviceVulkan12Features::ShaderFloat16 => PhysicalDeviceVulkan12Features::ShaderFloat16,
            DeviceVulkan12Features::ShaderInt8 => PhysicalDeviceVulkan12Features::ShaderInt8,
            DeviceVulkan12Features::DescriptorIndexing => {
                PhysicalDeviceVulkan12Features::DescriptorIndexing
            }
            DeviceVulkan12Features::ShaderInputAttachmentArrayDynamicIndexing => {
                PhysicalDeviceVulkan12Features::ShaderInputAttachmentArrayDynamicIndexing
            }
            DeviceVulkan12Features::ShaderUniformTexelBufferArrayDynamicIndexing => {
                PhysicalDeviceVulkan12Features::ShaderUniformTexelBufferArrayDynamicIndexing
            }
            DeviceVulkan12Features::ShaderStorageTexelBufferArrayDynamicIndexing => {
                PhysicalDeviceVulkan12Features::ShaderStorageTexelBufferArrayDynamicIndexing
            }
            DeviceVulkan12Features::ShaderUniformBufferArrayNonUniformIndexing => {
                PhysicalDeviceVulkan12Features::ShaderUniformBufferArrayNonUniformIndexing
            }
            DeviceVulkan12Features::ShaderSampledImageArrayNonUniformIndexing => {
                PhysicalDeviceVulkan12Features::ShaderSampledImageArrayNonUniformIndexing
            }
            DeviceVulkan12Features::ShaderStorageBufferArrayNonUniformIndexing => {
                PhysicalDeviceVulkan12Features::ShaderStorageBufferArrayNonUniformIndexing
            }
            DeviceVulkan12Features::ShaderStorageImageArrayNonUniformIndexing => {
                PhysicalDeviceVulkan12Features::ShaderStorageImageArrayNonUniformIndexing
            }
            DeviceVulkan12Features::ShaderInputAttachmentArrayNonUniformIndexing => {
                PhysicalDeviceVulkan12Features::ShaderInputAttachmentArrayNonUniformIndexing
            }
            DeviceVulkan12Features::ShaderUniformTexelBufferArrayNonUniformIndexing => {
                PhysicalDeviceVulkan12Features::ShaderUniformTexelBufferArrayNonUniformIndexing
            }
            DeviceVulkan12Features::ShaderStorageTexelBufferArrayNonUniformIndexing => {
                PhysicalDeviceVulkan12Features::ShaderStorageTexelBufferArrayNonUniformIndexing
            }
            DeviceVulkan12Features::DescriptorBindingUniformBufferUpdateAfterBind => {
                PhysicalDeviceVulkan12Features::DescriptorBindingUniformBufferUpdateAfterBind
            }
            DeviceVulkan12Features::DescriptorBindingSampledImageUpdateAfterBind => {
                PhysicalDeviceVulkan12Features::DescriptorBindingSampledImageUpdateAfterBind
            }
            DeviceVulkan12Features::DescriptorBindingStorageImageUpdateAfterBind => {
                PhysicalDeviceVulkan12Features::DescriptorBindingStorageImageUpdateAfterBind
            }
            DeviceVulkan12Features::DescriptorBindingStorageBufferUpdateAfterBind => {
                PhysicalDeviceVulkan12Features::DescriptorBindingStorageBufferUpdateAfterBind
            }
            DeviceVulkan12Features::DescriptorBindingUniformTexelBufferUpdateAfterBind => {
                PhysicalDeviceVulkan12Features::DescriptorBindingUniformTexelBufferUpdateAfterBind
            }
            DeviceVulkan12Features::DescriptorBindingStorageTexelBufferUpdateAfterBind => {
                PhysicalDeviceVulkan12Features::DescriptorBindingStorageTexelBufferUpdateAfterBind
            }
            DeviceVulkan12Features::DescriptorBindingUpdateUnusedWhilePending => {
                PhysicalDeviceVulkan12Features::DescriptorBindingUpdateUnusedWhilePending
            }
            DeviceVulkan12Features::DescriptorBindingPartiallyBound => {
                PhysicalDeviceVulkan12Features::DescriptorBindingPartiallyBound
            }
            DeviceVulkan12Features::DescriptorBindingVariableDescriptorCount => {
                PhysicalDeviceVulkan12Features::DescriptorBindingVariableDescriptorCount
            }
            DeviceVulkan12Features::RuntimeDescriptorArray => {
                PhysicalDeviceVulkan12Features::RuntimeDescriptorArray
            }
            DeviceVulkan12Features::SamplerFilterMinmax => {
                PhysicalDeviceVulkan12Features::SamplerFilterMinmax
            }
            DeviceVulkan12Features::ScalarBlockLayout => {
                PhysicalDeviceVulkan12Features::ScalarBlockLayout
            }
            DeviceVulkan12Features::ImagelessFramebuffer => {
                PhysicalDeviceVulkan12Features::ImagelessFramebuffer
            }
            DeviceVulkan12Features::UniformBufferStandardLayout => {
                PhysicalDeviceVulkan12Features::UniformBufferStandardLayout
            }
            DeviceVulkan12Features::ShaderSubgroupExtendedTypes => {
                PhysicalDeviceVulkan12Features::ShaderSubgroupExtendedTypes
            }
            DeviceVulkan12Features::SeparateDepthStencilLayouts => {
                PhysicalDeviceVulkan12Features::SeparateDepthStencilLayouts
            }
            DeviceVulkan12Features::HostQueryReset => {
                PhysicalDeviceVulkan12Features::HostQueryReset
            }
            DeviceVulkan12Features::TimelineSemaphore => {
                PhysicalDeviceVulkan12Features::TimelineSemaphore
            }
            DeviceVulkan12Features::BufferDeviceAddress => {
                PhysicalDeviceVulkan12Features::BufferDeviceAddress
            }
            DeviceVulkan12Features::BufferDeviceAddressCaptureReplay => {
                PhysicalDeviceVulkan12Features::BufferDeviceAddressCaptureReplay
            }
            DeviceVulkan12Features::BufferDeviceAddressMultiDevice => {
                PhysicalDeviceVulkan12Features::BufferDeviceAddressMultiDevice
            }
            DeviceVulkan12Features::VulkanMemoryModel => {
                PhysicalDeviceVulkan12Features::VulkanMemoryModel
            }
            DeviceVulkan12Features::VulkanMemoryModelDeviceScope => {
                PhysicalDeviceVulkan12Features::VulkanMemoryModelDeviceScope
            }
            DeviceVulkan12Features::VulkanMemoryModelAvailabilityVisibilityChains => {
                PhysicalDeviceVulkan12Features::VulkanMemoryModelAvailabilityVisibilityChains
            }
            DeviceVulkan12Features::ShaderOutputViewportIndex => {
                PhysicalDeviceVulkan12Features::ShaderOutputViewportIndex
            }
            DeviceVulkan12Features::ShaderOutputLayer => {
                PhysicalDeviceVulkan12Features::ShaderOutputLayer
            }
            DeviceVulkan12Features::SubgroupBroadcastDynamicId => {
                PhysicalDeviceVulkan12Features::SubgroupBroadcastDynamicId
            }
        }
    }
}
impl From<DeviceVulkan12Features> for DeviceFeature {
    fn from(feature: DeviceVulkan12Features) -> Self {
        DeviceFeature::DeviceVulkan12Features(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceVulkan12Features {
    type SubFeatureEnumTy = PhysicalDeviceVulkan12Features;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.sampler_mirror_clamp_to_edge != 0 {
            set.insert(PhysicalDeviceVulkan12Features::SamplerMirrorClampToEdge);
        }
        if self.draw_indirect_count != 0 {
            set.insert(PhysicalDeviceVulkan12Features::DrawIndirectCount);
        }
        if self.storage_buffer8_bit_access != 0 {
            set.insert(PhysicalDeviceVulkan12Features::StorageBuffer8BitAccess);
        }
        if self.uniform_and_storage_buffer8_bit_access != 0 {
            set.insert(PhysicalDeviceVulkan12Features::UniformAndStorageBuffer8BitAccess);
        }
        if self.storage_push_constant8 != 0 {
            set.insert(PhysicalDeviceVulkan12Features::StoragePushConstant8);
        }
        if self.shader_buffer_int64_atomics != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderBufferInt64Atomics);
        }
        if self.shader_shared_int64_atomics != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderSharedInt64Atomics);
        }
        if self.shader_float16 != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderFloat16);
        }
        if self.shader_int8 != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderInt8);
        }
        if self.descriptor_indexing != 0 {
            set.insert(PhysicalDeviceVulkan12Features::DescriptorIndexing);
        }
        if self.shader_input_attachment_array_dynamic_indexing != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderInputAttachmentArrayDynamicIndexing);
        }
        if self.shader_uniform_texel_buffer_array_dynamic_indexing != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::ShaderUniformTexelBufferArrayDynamicIndexing,
            );
        }
        if self.shader_storage_texel_buffer_array_dynamic_indexing != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::ShaderStorageTexelBufferArrayDynamicIndexing,
            );
        }
        if self.shader_uniform_buffer_array_non_uniform_indexing != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderUniformBufferArrayNonUniformIndexing);
        }
        if self.shader_sampled_image_array_non_uniform_indexing != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderSampledImageArrayNonUniformIndexing);
        }
        if self.shader_storage_buffer_array_non_uniform_indexing != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderStorageBufferArrayNonUniformIndexing);
        }
        if self.shader_storage_image_array_non_uniform_indexing != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderStorageImageArrayNonUniformIndexing);
        }
        if self.shader_input_attachment_array_non_uniform_indexing != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::ShaderInputAttachmentArrayNonUniformIndexing,
            );
        }
        if self.shader_uniform_texel_buffer_array_non_uniform_indexing != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::ShaderUniformTexelBufferArrayNonUniformIndexing,
            );
        }
        if self.shader_storage_texel_buffer_array_non_uniform_indexing != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::ShaderStorageTexelBufferArrayNonUniformIndexing,
            );
        }
        if self.descriptor_binding_uniform_buffer_update_after_bind != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::DescriptorBindingUniformBufferUpdateAfterBind,
            );
        }
        if self.descriptor_binding_sampled_image_update_after_bind != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::DescriptorBindingSampledImageUpdateAfterBind,
            );
        }
        if self.descriptor_binding_storage_image_update_after_bind != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::DescriptorBindingStorageImageUpdateAfterBind,
            );
        }
        if self.descriptor_binding_storage_buffer_update_after_bind != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::DescriptorBindingStorageBufferUpdateAfterBind,
            );
        }
        if self.descriptor_binding_uniform_texel_buffer_update_after_bind != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::DescriptorBindingUniformTexelBufferUpdateAfterBind,
            );
        }
        if self.descriptor_binding_storage_texel_buffer_update_after_bind != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::DescriptorBindingStorageTexelBufferUpdateAfterBind,
            );
        }
        if self.descriptor_binding_update_unused_while_pending != 0 {
            set.insert(PhysicalDeviceVulkan12Features::DescriptorBindingUpdateUnusedWhilePending);
        }
        if self.descriptor_binding_partially_bound != 0 {
            set.insert(PhysicalDeviceVulkan12Features::DescriptorBindingPartiallyBound);
        }
        if self.descriptor_binding_variable_descriptor_count != 0 {
            set.insert(PhysicalDeviceVulkan12Features::DescriptorBindingVariableDescriptorCount);
        }
        if self.runtime_descriptor_array != 0 {
            set.insert(PhysicalDeviceVulkan12Features::RuntimeDescriptorArray);
        }
        if self.sampler_filter_minmax != 0 {
            set.insert(PhysicalDeviceVulkan12Features::SamplerFilterMinmax);
        }
        if self.scalar_block_layout != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ScalarBlockLayout);
        }
        if self.imageless_framebuffer != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ImagelessFramebuffer);
        }
        if self.uniform_buffer_standard_layout != 0 {
            set.insert(PhysicalDeviceVulkan12Features::UniformBufferStandardLayout);
        }
        if self.shader_subgroup_extended_types != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderSubgroupExtendedTypes);
        }
        if self.separate_depth_stencil_layouts != 0 {
            set.insert(PhysicalDeviceVulkan12Features::SeparateDepthStencilLayouts);
        }
        if self.host_query_reset != 0 {
            set.insert(PhysicalDeviceVulkan12Features::HostQueryReset);
        }
        if self.timeline_semaphore != 0 {
            set.insert(PhysicalDeviceVulkan12Features::TimelineSemaphore);
        }
        if self.buffer_device_address != 0 {
            set.insert(PhysicalDeviceVulkan12Features::BufferDeviceAddress);
        }
        if self.buffer_device_address_capture_replay != 0 {
            set.insert(PhysicalDeviceVulkan12Features::BufferDeviceAddressCaptureReplay);
        }
        if self.buffer_device_address_multi_device != 0 {
            set.insert(PhysicalDeviceVulkan12Features::BufferDeviceAddressMultiDevice);
        }
        if self.vulkan_memory_model != 0 {
            set.insert(PhysicalDeviceVulkan12Features::VulkanMemoryModel);
        }
        if self.vulkan_memory_model_device_scope != 0 {
            set.insert(PhysicalDeviceVulkan12Features::VulkanMemoryModelDeviceScope);
        }
        if self.vulkan_memory_model_availability_visibility_chains != 0 {
            set.insert(
                PhysicalDeviceVulkan12Features::VulkanMemoryModelAvailabilityVisibilityChains,
            );
        }
        if self.shader_output_viewport_index != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderOutputViewportIndex);
        }
        if self.shader_output_layer != 0 {
            set.insert(PhysicalDeviceVulkan12Features::ShaderOutputLayer);
        }
        if self.subgroup_broadcast_dynamic_id != 0 {
            set.insert(PhysicalDeviceVulkan12Features::SubgroupBroadcastDynamicId);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceVulkan12Features {
    type VkStruct = ash::vk::PhysicalDeviceVulkan12Features;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::SamplerMirrorClampToEdge => vk_struct.sampler_mirror_clamp_to_edge = 1,
            Self::DrawIndirectCount => vk_struct.draw_indirect_count = 1,
            Self::StorageBuffer8BitAccess => vk_struct.storage_buffer8_bit_access = 1,
            Self::UniformAndStorageBuffer8BitAccess => {
                vk_struct.uniform_and_storage_buffer8_bit_access = 1
            }
            Self::StoragePushConstant8 => vk_struct.storage_push_constant8 = 1,
            Self::ShaderBufferInt64Atomics => vk_struct.shader_buffer_int64_atomics = 1,
            Self::ShaderSharedInt64Atomics => vk_struct.shader_shared_int64_atomics = 1,
            Self::ShaderFloat16 => vk_struct.shader_float16 = 1,
            Self::ShaderInt8 => vk_struct.shader_int8 = 1,
            Self::DescriptorIndexing => vk_struct.descriptor_indexing = 1,
            Self::ShaderInputAttachmentArrayDynamicIndexing => {
                vk_struct.shader_input_attachment_array_dynamic_indexing = 1
            }
            Self::ShaderUniformTexelBufferArrayDynamicIndexing => {
                vk_struct.shader_uniform_texel_buffer_array_dynamic_indexing = 1
            }
            Self::ShaderStorageTexelBufferArrayDynamicIndexing => {
                vk_struct.shader_storage_texel_buffer_array_dynamic_indexing = 1
            }
            Self::ShaderUniformBufferArrayNonUniformIndexing => {
                vk_struct.shader_uniform_buffer_array_non_uniform_indexing = 1
            }
            Self::ShaderSampledImageArrayNonUniformIndexing => {
                vk_struct.shader_sampled_image_array_non_uniform_indexing = 1
            }
            Self::ShaderStorageBufferArrayNonUniformIndexing => {
                vk_struct.shader_storage_buffer_array_non_uniform_indexing = 1
            }
            Self::ShaderStorageImageArrayNonUniformIndexing => {
                vk_struct.shader_storage_image_array_non_uniform_indexing = 1
            }
            Self::ShaderInputAttachmentArrayNonUniformIndexing => {
                vk_struct.shader_input_attachment_array_non_uniform_indexing = 1
            }
            Self::ShaderUniformTexelBufferArrayNonUniformIndexing => {
                vk_struct.shader_uniform_texel_buffer_array_non_uniform_indexing = 1
            }
            Self::ShaderStorageTexelBufferArrayNonUniformIndexing => {
                vk_struct.shader_storage_texel_buffer_array_non_uniform_indexing = 1
            }
            Self::DescriptorBindingUniformBufferUpdateAfterBind => {
                vk_struct.descriptor_binding_uniform_buffer_update_after_bind = 1
            }
            Self::DescriptorBindingSampledImageUpdateAfterBind => {
                vk_struct.descriptor_binding_sampled_image_update_after_bind = 1
            }
            Self::DescriptorBindingStorageImageUpdateAfterBind => {
                vk_struct.descriptor_binding_storage_image_update_after_bind = 1
            }
            Self::DescriptorBindingStorageBufferUpdateAfterBind => {
                vk_struct.descriptor_binding_storage_buffer_update_after_bind = 1
            }
            Self::DescriptorBindingUniformTexelBufferUpdateAfterBind => {
                vk_struct.descriptor_binding_uniform_texel_buffer_update_after_bind = 1
            }
            Self::DescriptorBindingStorageTexelBufferUpdateAfterBind => {
                vk_struct.descriptor_binding_storage_texel_buffer_update_after_bind = 1
            }
            Self::DescriptorBindingUpdateUnusedWhilePending => {
                vk_struct.descriptor_binding_update_unused_while_pending = 1
            }
            Self::DescriptorBindingPartiallyBound => {
                vk_struct.descriptor_binding_partially_bound = 1
            }
            Self::DescriptorBindingVariableDescriptorCount => {
                vk_struct.descriptor_binding_variable_descriptor_count = 1
            }
            Self::RuntimeDescriptorArray => vk_struct.runtime_descriptor_array = 1,
            Self::SamplerFilterMinmax => vk_struct.sampler_filter_minmax = 1,
            Self::ScalarBlockLayout => vk_struct.scalar_block_layout = 1,
            Self::ImagelessFramebuffer => vk_struct.imageless_framebuffer = 1,
            Self::UniformBufferStandardLayout => vk_struct.uniform_buffer_standard_layout = 1,
            Self::ShaderSubgroupExtendedTypes => vk_struct.shader_subgroup_extended_types = 1,
            Self::SeparateDepthStencilLayouts => vk_struct.separate_depth_stencil_layouts = 1,
            Self::HostQueryReset => vk_struct.host_query_reset = 1,
            Self::TimelineSemaphore => vk_struct.timeline_semaphore = 1,
            Self::BufferDeviceAddress => vk_struct.buffer_device_address = 1,
            Self::BufferDeviceAddressCaptureReplay => {
                vk_struct.buffer_device_address_capture_replay = 1
            }
            Self::BufferDeviceAddressMultiDevice => {
                vk_struct.buffer_device_address_multi_device = 1
            }
            Self::VulkanMemoryModel => vk_struct.vulkan_memory_model = 1,
            Self::VulkanMemoryModelDeviceScope => vk_struct.vulkan_memory_model_device_scope = 1,
            Self::VulkanMemoryModelAvailabilityVisibilityChains => {
                vk_struct.vulkan_memory_model_availability_visibility_chains = 1
            }
            Self::ShaderOutputViewportIndex => vk_struct.shader_output_viewport_index = 1,
            Self::ShaderOutputLayer => vk_struct.shader_output_layer = 1,
            Self::SubgroupBroadcastDynamicId => vk_struct.subgroup_broadcast_dynamic_id = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceDiagnosticsConfigFeaturesNV {
    DiagnosticsConfig,
}
impl const From<PhysicalDeviceDiagnosticsConfigFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceDiagnosticsConfigFeaturesNV) -> Self {
        FeatureType::DeviceDiagnosticsConfigFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceDiagnosticsConfigFeaturesNV {
    DiagnosticsConfig,
}
impl ToPhysicalFeature for DeviceDiagnosticsConfigFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceDiagnosticsConfigFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceDiagnosticsConfigFeaturesNV::DiagnosticsConfig => {
                PhysicalDeviceDiagnosticsConfigFeaturesNV::DiagnosticsConfig
            }
        }
    }
}
impl From<DeviceDiagnosticsConfigFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceDiagnosticsConfigFeaturesNV) -> Self {
        DeviceFeature::DeviceDiagnosticsConfigFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceDiagnosticsConfigFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceDiagnosticsConfigFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.diagnostics_config != 0 {
            set.insert(PhysicalDeviceDiagnosticsConfigFeaturesNV::DiagnosticsConfig);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceDiagnosticsConfigFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceDiagnosticsConfigFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DiagnosticsConfig => vk_struct.diagnostics_config = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceFragmentShadingRateFeaturesKHR {
    PipelineFragmentShadingRate,
    PrimitiveFragmentShadingRate,
    AttachmentFragmentShadingRate,
}
impl const From<PhysicalDeviceFragmentShadingRateFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDeviceFragmentShadingRateFeaturesKHR) -> Self {
        FeatureType::DeviceFragmentShadingRateFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceFragmentShadingRateFeaturesKHR {
    PipelineFragmentShadingRate,
    PrimitiveFragmentShadingRate,
    AttachmentFragmentShadingRate,
}
impl ToPhysicalFeature for DeviceFragmentShadingRateFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDeviceFragmentShadingRateFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceFragmentShadingRateFeaturesKHR::PipelineFragmentShadingRate => {
                PhysicalDeviceFragmentShadingRateFeaturesKHR::PipelineFragmentShadingRate
            }
            DeviceFragmentShadingRateFeaturesKHR::PrimitiveFragmentShadingRate => {
                PhysicalDeviceFragmentShadingRateFeaturesKHR::PrimitiveFragmentShadingRate
            }
            DeviceFragmentShadingRateFeaturesKHR::AttachmentFragmentShadingRate => {
                PhysicalDeviceFragmentShadingRateFeaturesKHR::AttachmentFragmentShadingRate
            }
        }
    }
}
impl From<DeviceFragmentShadingRateFeaturesKHR> for DeviceFeature {
    fn from(feature: DeviceFragmentShadingRateFeaturesKHR) -> Self {
        DeviceFeature::DeviceFragmentShadingRateFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceFragmentShadingRateFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDeviceFragmentShadingRateFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.pipeline_fragment_shading_rate != 0 {
            set.insert(PhysicalDeviceFragmentShadingRateFeaturesKHR::PipelineFragmentShadingRate);
        }
        if self.primitive_fragment_shading_rate != 0 {
            set.insert(PhysicalDeviceFragmentShadingRateFeaturesKHR::PrimitiveFragmentShadingRate);
        }
        if self.attachment_fragment_shading_rate != 0 {
            set.insert(PhysicalDeviceFragmentShadingRateFeaturesKHR::AttachmentFragmentShadingRate);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceFragmentShadingRateFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDeviceFragmentShadingRateFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::PipelineFragmentShadingRate => vk_struct.pipeline_fragment_shading_rate = 1,
            Self::PrimitiveFragmentShadingRate => vk_struct.primitive_fragment_shading_rate = 1,
            Self::AttachmentFragmentShadingRate => vk_struct.attachment_fragment_shading_rate = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceLineRasterizationFeaturesEXT {
    RectangularLines,
    BresenhamLines,
    SmoothLines,
    StippledRectangularLines,
    StippledBresenhamLines,
    StippledSmoothLines,
}
impl const From<PhysicalDeviceLineRasterizationFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceLineRasterizationFeaturesEXT) -> Self {
        FeatureType::DeviceLineRasterizationFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceLineRasterizationFeaturesEXT {
    RectangularLines,
    BresenhamLines,
    SmoothLines,
    StippledRectangularLines,
    StippledBresenhamLines,
    StippledSmoothLines,
}
impl ToPhysicalFeature for DeviceLineRasterizationFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceLineRasterizationFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceLineRasterizationFeaturesEXT::RectangularLines => {
                PhysicalDeviceLineRasterizationFeaturesEXT::RectangularLines
            }
            DeviceLineRasterizationFeaturesEXT::BresenhamLines => {
                PhysicalDeviceLineRasterizationFeaturesEXT::BresenhamLines
            }
            DeviceLineRasterizationFeaturesEXT::SmoothLines => {
                PhysicalDeviceLineRasterizationFeaturesEXT::SmoothLines
            }
            DeviceLineRasterizationFeaturesEXT::StippledRectangularLines => {
                PhysicalDeviceLineRasterizationFeaturesEXT::StippledRectangularLines
            }
            DeviceLineRasterizationFeaturesEXT::StippledBresenhamLines => {
                PhysicalDeviceLineRasterizationFeaturesEXT::StippledBresenhamLines
            }
            DeviceLineRasterizationFeaturesEXT::StippledSmoothLines => {
                PhysicalDeviceLineRasterizationFeaturesEXT::StippledSmoothLines
            }
        }
    }
}
impl From<DeviceLineRasterizationFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceLineRasterizationFeaturesEXT) -> Self {
        DeviceFeature::DeviceLineRasterizationFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceLineRasterizationFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.rectangular_lines != 0 {
            set.insert(PhysicalDeviceLineRasterizationFeaturesEXT::RectangularLines);
        }
        if self.bresenham_lines != 0 {
            set.insert(PhysicalDeviceLineRasterizationFeaturesEXT::BresenhamLines);
        }
        if self.smooth_lines != 0 {
            set.insert(PhysicalDeviceLineRasterizationFeaturesEXT::SmoothLines);
        }
        if self.stippled_rectangular_lines != 0 {
            set.insert(PhysicalDeviceLineRasterizationFeaturesEXT::StippledRectangularLines);
        }
        if self.stippled_bresenham_lines != 0 {
            set.insert(PhysicalDeviceLineRasterizationFeaturesEXT::StippledBresenhamLines);
        }
        if self.stippled_smooth_lines != 0 {
            set.insert(PhysicalDeviceLineRasterizationFeaturesEXT::StippledSmoothLines);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceLineRasterizationFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::RectangularLines => vk_struct.rectangular_lines = 1,
            Self::BresenhamLines => vk_struct.bresenham_lines = 1,
            Self::SmoothLines => vk_struct.smooth_lines = 1,
            Self::StippledRectangularLines => vk_struct.stippled_rectangular_lines = 1,
            Self::StippledBresenhamLines => vk_struct.stippled_bresenham_lines = 1,
            Self::StippledSmoothLines => vk_struct.stippled_smooth_lines = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceProvokingVertexFeaturesEXT {
    ProvokingVertexLast,
    TransformFeedbackPreservesProvokingVertex,
}
impl const From<PhysicalDeviceProvokingVertexFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceProvokingVertexFeaturesEXT) -> Self {
        FeatureType::DeviceProvokingVertexFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceProvokingVertexFeaturesEXT {
    ProvokingVertexLast,
    TransformFeedbackPreservesProvokingVertex,
}
impl ToPhysicalFeature for DeviceProvokingVertexFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceProvokingVertexFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceProvokingVertexFeaturesEXT::ProvokingVertexLast => {
                PhysicalDeviceProvokingVertexFeaturesEXT::ProvokingVertexLast
            }
            DeviceProvokingVertexFeaturesEXT::TransformFeedbackPreservesProvokingVertex => {
                PhysicalDeviceProvokingVertexFeaturesEXT::TransformFeedbackPreservesProvokingVertex
            }
        }
    }
}
impl From<DeviceProvokingVertexFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceProvokingVertexFeaturesEXT) -> Self {
        DeviceFeature::DeviceProvokingVertexFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceProvokingVertexFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceProvokingVertexFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.provoking_vertex_last != 0 {
            set.insert(PhysicalDeviceProvokingVertexFeaturesEXT::ProvokingVertexLast);
        }
        if self.transform_feedback_preserves_provoking_vertex != 0 {
            set.insert(
                PhysicalDeviceProvokingVertexFeaturesEXT::TransformFeedbackPreservesProvokingVertex,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceProvokingVertexFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceProvokingVertexFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ProvokingVertexLast => vk_struct.provoking_vertex_last = 1,
            Self::TransformFeedbackPreservesProvokingVertex => {
                vk_struct.transform_feedback_preserves_provoking_vertex = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    ExtendedDynamicState,
}
impl const From<PhysicalDeviceExtendedDynamicStateFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceExtendedDynamicStateFeaturesEXT) -> Self {
        FeatureType::DeviceExtendedDynamicStateFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceExtendedDynamicStateFeaturesEXT {
    ExtendedDynamicState,
}
impl ToPhysicalFeature for DeviceExtendedDynamicStateFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceExtendedDynamicStateFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceExtendedDynamicStateFeaturesEXT::ExtendedDynamicState => {
                PhysicalDeviceExtendedDynamicStateFeaturesEXT::ExtendedDynamicState
            }
        }
    }
}
impl From<DeviceExtendedDynamicStateFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceExtendedDynamicStateFeaturesEXT) -> Self {
        DeviceFeature::DeviceExtendedDynamicStateFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceExtendedDynamicStateFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.extended_dynamic_state != 0 {
            set.insert(PhysicalDeviceExtendedDynamicStateFeaturesEXT::ExtendedDynamicState);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ExtendedDynamicState => vk_struct.extended_dynamic_state = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevicePresentIdFeaturesKHR {
    PresentId,
}
impl const From<PhysicalDevicePresentIdFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDevicePresentIdFeaturesKHR) -> Self {
        FeatureType::DevicePresentIdFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DevicePresentIdFeaturesKHR {
    PresentId,
}
impl ToPhysicalFeature for DevicePresentIdFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDevicePresentIdFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DevicePresentIdFeaturesKHR::PresentId => PhysicalDevicePresentIdFeaturesKHR::PresentId,
        }
    }
}
impl From<DevicePresentIdFeaturesKHR> for DeviceFeature {
    fn from(feature: DevicePresentIdFeaturesKHR) -> Self {
        DeviceFeature::DevicePresentIdFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevicePresentIdFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDevicePresentIdFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.present_id != 0 {
            set.insert(PhysicalDevicePresentIdFeaturesKHR::PresentId);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevicePresentIdFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDevicePresentIdFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::PresentId => vk_struct.present_id = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    DeviceMemoryReport,
}
impl const From<PhysicalDeviceDeviceMemoryReportFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceDeviceMemoryReportFeaturesEXT) -> Self {
        FeatureType::DeviceDeviceMemoryReportFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceDeviceMemoryReportFeaturesEXT {
    DeviceMemoryReport,
}
impl ToPhysicalFeature for DeviceDeviceMemoryReportFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceDeviceMemoryReportFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceDeviceMemoryReportFeaturesEXT::DeviceMemoryReport => {
                PhysicalDeviceDeviceMemoryReportFeaturesEXT::DeviceMemoryReport
            }
        }
    }
}
impl From<DeviceDeviceMemoryReportFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceDeviceMemoryReportFeaturesEXT) -> Self {
        DeviceFeature::DeviceDeviceMemoryReportFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceDeviceMemoryReportFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.device_memory_report != 0 {
            set.insert(PhysicalDeviceDeviceMemoryReportFeaturesEXT::DeviceMemoryReport);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DeviceMemoryReport => vk_struct.device_memory_report = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevice16BitStorageFeatures {
    StorageBuffer16BitAccess,
    UniformAndStorageBuffer16BitAccess,
    StoragePushConstant16,
    StorageInputOutput16,
}
impl const From<PhysicalDevice16BitStorageFeatures> for FeatureType {
    fn from(feature: PhysicalDevice16BitStorageFeatures) -> Self {
        FeatureType::Device16BitStorageFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum Device16BitStorageFeatures {
    StorageBuffer16BitAccess,
    UniformAndStorageBuffer16BitAccess,
    StoragePushConstant16,
    StorageInputOutput16,
}
impl ToPhysicalFeature for Device16BitStorageFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDevice16BitStorageFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            Device16BitStorageFeatures::StorageBuffer16BitAccess => {
                PhysicalDevice16BitStorageFeatures::StorageBuffer16BitAccess
            }
            Device16BitStorageFeatures::UniformAndStorageBuffer16BitAccess => {
                PhysicalDevice16BitStorageFeatures::UniformAndStorageBuffer16BitAccess
            }
            Device16BitStorageFeatures::StoragePushConstant16 => {
                PhysicalDevice16BitStorageFeatures::StoragePushConstant16
            }
            Device16BitStorageFeatures::StorageInputOutput16 => {
                PhysicalDevice16BitStorageFeatures::StorageInputOutput16
            }
        }
    }
}
impl From<Device16BitStorageFeatures> for DeviceFeature {
    fn from(feature: Device16BitStorageFeatures) -> Self {
        DeviceFeature::Device16BitStorageFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevice16BitStorageFeatures {
    type SubFeatureEnumTy = PhysicalDevice16BitStorageFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.storage_buffer16_bit_access != 0 {
            set.insert(PhysicalDevice16BitStorageFeatures::StorageBuffer16BitAccess);
        }
        if self.uniform_and_storage_buffer16_bit_access != 0 {
            set.insert(PhysicalDevice16BitStorageFeatures::UniformAndStorageBuffer16BitAccess);
        }
        if self.storage_push_constant16 != 0 {
            set.insert(PhysicalDevice16BitStorageFeatures::StoragePushConstant16);
        }
        if self.storage_input_output16 != 0 {
            set.insert(PhysicalDevice16BitStorageFeatures::StorageInputOutput16);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevice16BitStorageFeatures {
    type VkStruct = ash::vk::PhysicalDevice16BitStorageFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::StorageBuffer16BitAccess => vk_struct.storage_buffer16_bit_access = 1,
            Self::UniformAndStorageBuffer16BitAccess => {
                vk_struct.uniform_and_storage_buffer16_bit_access = 1
            }
            Self::StoragePushConstant16 => vk_struct.storage_push_constant16 = 1,
            Self::StorageInputOutput16 => vk_struct.storage_input_output16 = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceBufferDeviceAddressFeatures {
    BufferDeviceAddress,
    BufferDeviceAddressCaptureReplay,
    BufferDeviceAddressMultiDevice,
}
impl const From<PhysicalDeviceBufferDeviceAddressFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceBufferDeviceAddressFeatures) -> Self {
        FeatureType::DeviceBufferDeviceAddressFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceBufferDeviceAddressFeatures {
    BufferDeviceAddress,
    BufferDeviceAddressCaptureReplay,
    BufferDeviceAddressMultiDevice,
}
impl ToPhysicalFeature for DeviceBufferDeviceAddressFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceBufferDeviceAddressFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceBufferDeviceAddressFeatures::BufferDeviceAddress => {
                PhysicalDeviceBufferDeviceAddressFeatures::BufferDeviceAddress
            }
            DeviceBufferDeviceAddressFeatures::BufferDeviceAddressCaptureReplay => {
                PhysicalDeviceBufferDeviceAddressFeatures::BufferDeviceAddressCaptureReplay
            }
            DeviceBufferDeviceAddressFeatures::BufferDeviceAddressMultiDevice => {
                PhysicalDeviceBufferDeviceAddressFeatures::BufferDeviceAddressMultiDevice
            }
        }
    }
}
impl From<DeviceBufferDeviceAddressFeatures> for DeviceFeature {
    fn from(feature: DeviceBufferDeviceAddressFeatures) -> Self {
        DeviceFeature::DeviceBufferDeviceAddressFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceBufferDeviceAddressFeatures {
    type SubFeatureEnumTy = PhysicalDeviceBufferDeviceAddressFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.buffer_device_address != 0 {
            set.insert(PhysicalDeviceBufferDeviceAddressFeatures::BufferDeviceAddress);
        }
        if self.buffer_device_address_capture_replay != 0 {
            set.insert(PhysicalDeviceBufferDeviceAddressFeatures::BufferDeviceAddressCaptureReplay);
        }
        if self.buffer_device_address_multi_device != 0 {
            set.insert(PhysicalDeviceBufferDeviceAddressFeatures::BufferDeviceAddressMultiDevice);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceBufferDeviceAddressFeatures {
    type VkStruct = ash::vk::PhysicalDeviceBufferDeviceAddressFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::BufferDeviceAddress => vk_struct.buffer_device_address = 1,
            Self::BufferDeviceAddressCaptureReplay => {
                vk_struct.buffer_device_address_capture_replay = 1
            }
            Self::BufferDeviceAddressMultiDevice => {
                vk_struct.buffer_device_address_multi_device = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceCoherentMemoryFeaturesAMD {
    DeviceCoherentMemory,
}
impl const From<PhysicalDeviceCoherentMemoryFeaturesAMD> for FeatureType {
    fn from(feature: PhysicalDeviceCoherentMemoryFeaturesAMD) -> Self {
        FeatureType::DeviceCoherentMemoryFeaturesAMD(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceCoherentMemoryFeaturesAMD {
    DeviceCoherentMemory,
}
impl ToPhysicalFeature for DeviceCoherentMemoryFeaturesAMD {
    type PhysicalDeviceFeatureTy = PhysicalDeviceCoherentMemoryFeaturesAMD;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceCoherentMemoryFeaturesAMD::DeviceCoherentMemory => {
                PhysicalDeviceCoherentMemoryFeaturesAMD::DeviceCoherentMemory
            }
        }
    }
}
impl From<DeviceCoherentMemoryFeaturesAMD> for DeviceFeature {
    fn from(feature: DeviceCoherentMemoryFeaturesAMD) -> Self {
        DeviceFeature::DeviceCoherentMemoryFeaturesAMD(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceCoherentMemoryFeaturesAMD {
    type SubFeatureEnumTy = PhysicalDeviceCoherentMemoryFeaturesAMD;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.device_coherent_memory != 0 {
            set.insert(PhysicalDeviceCoherentMemoryFeaturesAMD::DeviceCoherentMemory);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceCoherentMemoryFeaturesAMD {
    type VkStruct = ash::vk::PhysicalDeviceCoherentMemoryFeaturesAMD;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DeviceCoherentMemory => vk_struct.device_coherent_memory = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceColorWriteEnableFeaturesEXT {
    ColorWriteEnable,
}
impl const From<PhysicalDeviceColorWriteEnableFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceColorWriteEnableFeaturesEXT) -> Self {
        FeatureType::DeviceColorWriteEnableFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceColorWriteEnableFeaturesEXT {
    ColorWriteEnable,
}
impl ToPhysicalFeature for DeviceColorWriteEnableFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceColorWriteEnableFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceColorWriteEnableFeaturesEXT::ColorWriteEnable => {
                PhysicalDeviceColorWriteEnableFeaturesEXT::ColorWriteEnable
            }
        }
    }
}
impl From<DeviceColorWriteEnableFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceColorWriteEnableFeaturesEXT) -> Self {
        DeviceFeature::DeviceColorWriteEnableFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceColorWriteEnableFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceColorWriteEnableFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.color_write_enable != 0 {
            set.insert(PhysicalDeviceColorWriteEnableFeaturesEXT::ColorWriteEnable);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceColorWriteEnableFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceColorWriteEnableFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ColorWriteEnable => vk_struct.color_write_enable = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceMultiDrawFeaturesEXT {
    MultiDraw,
}
impl const From<PhysicalDeviceMultiDrawFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceMultiDrawFeaturesEXT) -> Self {
        FeatureType::DeviceMultiDrawFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceMultiDrawFeaturesEXT {
    MultiDraw,
}
impl ToPhysicalFeature for DeviceMultiDrawFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceMultiDrawFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceMultiDrawFeaturesEXT::MultiDraw => PhysicalDeviceMultiDrawFeaturesEXT::MultiDraw,
        }
    }
}
impl From<DeviceMultiDrawFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceMultiDrawFeaturesEXT) -> Self {
        DeviceFeature::DeviceMultiDrawFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceMultiDrawFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceMultiDrawFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.multi_draw != 0 {
            set.insert(PhysicalDeviceMultiDrawFeaturesEXT::MultiDraw);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceMultiDrawFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceMultiDrawFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::MultiDraw => vk_struct.multi_draw = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceSamplerYcbcrConversionFeatures {
    SamplerYcbcrConversion,
}
impl const From<PhysicalDeviceSamplerYcbcrConversionFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceSamplerYcbcrConversionFeatures) -> Self {
        FeatureType::DeviceSamplerYcbcrConversionFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceSamplerYcbcrConversionFeatures {
    SamplerYcbcrConversion,
}
impl ToPhysicalFeature for DeviceSamplerYcbcrConversionFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceSamplerYcbcrConversionFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceSamplerYcbcrConversionFeatures::SamplerYcbcrConversion => {
                PhysicalDeviceSamplerYcbcrConversionFeatures::SamplerYcbcrConversion
            }
        }
    }
}
impl From<DeviceSamplerYcbcrConversionFeatures> for DeviceFeature {
    fn from(feature: DeviceSamplerYcbcrConversionFeatures) -> Self {
        DeviceFeature::DeviceSamplerYcbcrConversionFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceSamplerYcbcrConversionFeatures {
    type SubFeatureEnumTy = PhysicalDeviceSamplerYcbcrConversionFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.sampler_ycbcr_conversion != 0 {
            set.insert(PhysicalDeviceSamplerYcbcrConversionFeatures::SamplerYcbcrConversion);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceSamplerYcbcrConversionFeatures {
    type VkStruct = ash::vk::PhysicalDeviceSamplerYcbcrConversionFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::SamplerYcbcrConversion => vk_struct.sampler_ycbcr_conversion = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceCustomBorderColorFeaturesEXT {
    CustomBorderColors,
    CustomBorderColorWithoutFormat,
}
impl const From<PhysicalDeviceCustomBorderColorFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceCustomBorderColorFeaturesEXT) -> Self {
        FeatureType::DeviceCustomBorderColorFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceCustomBorderColorFeaturesEXT {
    CustomBorderColors,
    CustomBorderColorWithoutFormat,
}
impl ToPhysicalFeature for DeviceCustomBorderColorFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceCustomBorderColorFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceCustomBorderColorFeaturesEXT::CustomBorderColors => {
                PhysicalDeviceCustomBorderColorFeaturesEXT::CustomBorderColors
            }
            DeviceCustomBorderColorFeaturesEXT::CustomBorderColorWithoutFormat => {
                PhysicalDeviceCustomBorderColorFeaturesEXT::CustomBorderColorWithoutFormat
            }
        }
    }
}
impl From<DeviceCustomBorderColorFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceCustomBorderColorFeaturesEXT) -> Self {
        DeviceFeature::DeviceCustomBorderColorFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceCustomBorderColorFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceCustomBorderColorFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.custom_border_colors != 0 {
            set.insert(PhysicalDeviceCustomBorderColorFeaturesEXT::CustomBorderColors);
        }
        if self.custom_border_color_without_format != 0 {
            set.insert(PhysicalDeviceCustomBorderColorFeaturesEXT::CustomBorderColorWithoutFormat);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceCustomBorderColorFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceCustomBorderColorFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::CustomBorderColors => vk_struct.custom_border_colors = 1,
            Self::CustomBorderColorWithoutFormat => {
                vk_struct.custom_border_color_without_format = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    BorderColorSwizzle,
    BorderColorSwizzleFromImage,
}
impl const From<PhysicalDeviceBorderColorSwizzleFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceBorderColorSwizzleFeaturesEXT) -> Self {
        FeatureType::DeviceBorderColorSwizzleFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceBorderColorSwizzleFeaturesEXT {
    BorderColorSwizzle,
    BorderColorSwizzleFromImage,
}
impl ToPhysicalFeature for DeviceBorderColorSwizzleFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceBorderColorSwizzleFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceBorderColorSwizzleFeaturesEXT::BorderColorSwizzle => {
                PhysicalDeviceBorderColorSwizzleFeaturesEXT::BorderColorSwizzle
            }
            DeviceBorderColorSwizzleFeaturesEXT::BorderColorSwizzleFromImage => {
                PhysicalDeviceBorderColorSwizzleFeaturesEXT::BorderColorSwizzleFromImage
            }
        }
    }
}
impl From<DeviceBorderColorSwizzleFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceBorderColorSwizzleFeaturesEXT) -> Self {
        DeviceFeature::DeviceBorderColorSwizzleFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceBorderColorSwizzleFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.border_color_swizzle != 0 {
            set.insert(PhysicalDeviceBorderColorSwizzleFeaturesEXT::BorderColorSwizzle);
        }
        if self.border_color_swizzle_from_image != 0 {
            set.insert(PhysicalDeviceBorderColorSwizzleFeaturesEXT::BorderColorSwizzleFromImage);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::BorderColorSwizzle => vk_struct.border_color_swizzle = 1,
            Self::BorderColorSwizzleFromImage => vk_struct.border_color_swizzle_from_image = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceAccelerationStructureFeaturesKHR {
    AccelerationStructure,
    AccelerationStructureCaptureReplay,
    AccelerationStructureIndirectBuild,
    AccelerationStructureHostCommands,
    DescriptorBindingAccelerationStructureUpdateAfterBind,
}
impl const From<PhysicalDeviceAccelerationStructureFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDeviceAccelerationStructureFeaturesKHR) -> Self {
        FeatureType::DeviceAccelerationStructureFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceAccelerationStructureFeaturesKHR {
    AccelerationStructure,
    AccelerationStructureCaptureReplay,
    AccelerationStructureIndirectBuild,
    AccelerationStructureHostCommands,
    DescriptorBindingAccelerationStructureUpdateAfterBind,
}
impl ToPhysicalFeature for DeviceAccelerationStructureFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDeviceAccelerationStructureFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceAccelerationStructureFeaturesKHR :: AccelerationStructure => PhysicalDeviceAccelerationStructureFeaturesKHR :: AccelerationStructure , DeviceAccelerationStructureFeaturesKHR :: AccelerationStructureCaptureReplay => PhysicalDeviceAccelerationStructureFeaturesKHR :: AccelerationStructureCaptureReplay , DeviceAccelerationStructureFeaturesKHR :: AccelerationStructureIndirectBuild => PhysicalDeviceAccelerationStructureFeaturesKHR :: AccelerationStructureIndirectBuild , DeviceAccelerationStructureFeaturesKHR :: AccelerationStructureHostCommands => PhysicalDeviceAccelerationStructureFeaturesKHR :: AccelerationStructureHostCommands , DeviceAccelerationStructureFeaturesKHR :: DescriptorBindingAccelerationStructureUpdateAfterBind => PhysicalDeviceAccelerationStructureFeaturesKHR :: DescriptorBindingAccelerationStructureUpdateAfterBind , }
    }
}
impl From<DeviceAccelerationStructureFeaturesKHR> for DeviceFeature {
    fn from(feature: DeviceAccelerationStructureFeaturesKHR) -> Self {
        DeviceFeature::DeviceAccelerationStructureFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDeviceAccelerationStructureFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.acceleration_structure != 0 {
            set.insert(PhysicalDeviceAccelerationStructureFeaturesKHR::AccelerationStructure);
        }
        if self.acceleration_structure_capture_replay != 0 {
            set.insert(
                PhysicalDeviceAccelerationStructureFeaturesKHR::AccelerationStructureCaptureReplay,
            );
        }
        if self.acceleration_structure_indirect_build != 0 {
            set.insert(
                PhysicalDeviceAccelerationStructureFeaturesKHR::AccelerationStructureIndirectBuild,
            );
        }
        if self.acceleration_structure_host_commands != 0 {
            set.insert(
                PhysicalDeviceAccelerationStructureFeaturesKHR::AccelerationStructureHostCommands,
            );
        }
        if self.descriptor_binding_acceleration_structure_update_after_bind != 0 {
            set . insert (PhysicalDeviceAccelerationStructureFeaturesKHR :: DescriptorBindingAccelerationStructureUpdateAfterBind) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceAccelerationStructureFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::AccelerationStructure => vk_struct.acceleration_structure = 1,
            Self::AccelerationStructureCaptureReplay => {
                vk_struct.acceleration_structure_capture_replay = 1
            }
            Self::AccelerationStructureIndirectBuild => {
                vk_struct.acceleration_structure_indirect_build = 1
            }
            Self::AccelerationStructureHostCommands => {
                vk_struct.acceleration_structure_host_commands = 1
            }
            Self::DescriptorBindingAccelerationStructureUpdateAfterBind => {
                vk_struct.descriptor_binding_acceleration_structure_update_after_bind = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    AdvancedBlendCoherentOperations,
}
impl const From<PhysicalDeviceBlendOperationAdvancedFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceBlendOperationAdvancedFeaturesEXT) -> Self {
        FeatureType::DeviceBlendOperationAdvancedFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceBlendOperationAdvancedFeaturesEXT {
    AdvancedBlendCoherentOperations,
}
impl ToPhysicalFeature for DeviceBlendOperationAdvancedFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceBlendOperationAdvancedFeaturesEXT::AdvancedBlendCoherentOperations => {
                PhysicalDeviceBlendOperationAdvancedFeaturesEXT::AdvancedBlendCoherentOperations
            }
        }
    }
}
impl From<DeviceBlendOperationAdvancedFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceBlendOperationAdvancedFeaturesEXT) -> Self {
        DeviceFeature::DeviceBlendOperationAdvancedFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.advanced_blend_coherent_operations != 0 {
            set.insert(
                PhysicalDeviceBlendOperationAdvancedFeaturesEXT::AdvancedBlendCoherentOperations,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::AdvancedBlendCoherentOperations => {
                vk_struct.advanced_blend_coherent_operations = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceLinearColorAttachmentFeaturesNV {
    LinearColorAttachment,
}
impl const From<PhysicalDeviceLinearColorAttachmentFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceLinearColorAttachmentFeaturesNV) -> Self {
        FeatureType::DeviceLinearColorAttachmentFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceLinearColorAttachmentFeaturesNV {
    LinearColorAttachment,
}
impl ToPhysicalFeature for DeviceLinearColorAttachmentFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceLinearColorAttachmentFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceLinearColorAttachmentFeaturesNV::LinearColorAttachment => {
                PhysicalDeviceLinearColorAttachmentFeaturesNV::LinearColorAttachment
            }
        }
    }
}
impl From<DeviceLinearColorAttachmentFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceLinearColorAttachmentFeaturesNV) -> Self {
        DeviceFeature::DeviceLinearColorAttachmentFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceLinearColorAttachmentFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceLinearColorAttachmentFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.linear_color_attachment != 0 {
            set.insert(PhysicalDeviceLinearColorAttachmentFeaturesNV::LinearColorAttachment);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceLinearColorAttachmentFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceLinearColorAttachmentFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::LinearColorAttachment => vk_struct.linear_color_attachment = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceConditionalRenderingFeaturesEXT {
    ConditionalRendering,
    InheritedConditionalRendering,
}
impl const From<PhysicalDeviceConditionalRenderingFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceConditionalRenderingFeaturesEXT) -> Self {
        FeatureType::DeviceConditionalRenderingFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceConditionalRenderingFeaturesEXT {
    ConditionalRendering,
    InheritedConditionalRendering,
}
impl ToPhysicalFeature for DeviceConditionalRenderingFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceConditionalRenderingFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceConditionalRenderingFeaturesEXT::ConditionalRendering => {
                PhysicalDeviceConditionalRenderingFeaturesEXT::ConditionalRendering
            }
            DeviceConditionalRenderingFeaturesEXT::InheritedConditionalRendering => {
                PhysicalDeviceConditionalRenderingFeaturesEXT::InheritedConditionalRendering
            }
        }
    }
}
impl From<DeviceConditionalRenderingFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceConditionalRenderingFeaturesEXT) -> Self {
        DeviceFeature::DeviceConditionalRenderingFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceConditionalRenderingFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceConditionalRenderingFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.conditional_rendering != 0 {
            set.insert(PhysicalDeviceConditionalRenderingFeaturesEXT::ConditionalRendering);
        }
        if self.inherited_conditional_rendering != 0 {
            set.insert(
                PhysicalDeviceConditionalRenderingFeaturesEXT::InheritedConditionalRendering,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceConditionalRenderingFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceConditionalRenderingFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ConditionalRendering => vk_struct.conditional_rendering = 1,
            Self::InheritedConditionalRendering => vk_struct.inherited_conditional_rendering = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    YcbcrImageArrays,
}
impl const From<PhysicalDeviceYcbcrImageArraysFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceYcbcrImageArraysFeaturesEXT) -> Self {
        FeatureType::DeviceYcbcrImageArraysFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceYcbcrImageArraysFeaturesEXT {
    YcbcrImageArrays,
}
impl ToPhysicalFeature for DeviceYcbcrImageArraysFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceYcbcrImageArraysFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceYcbcrImageArraysFeaturesEXT::YcbcrImageArrays => {
                PhysicalDeviceYcbcrImageArraysFeaturesEXT::YcbcrImageArrays
            }
        }
    }
}
impl From<DeviceYcbcrImageArraysFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceYcbcrImageArraysFeaturesEXT) -> Self {
        DeviceFeature::DeviceYcbcrImageArraysFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceYcbcrImageArraysFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.ycbcr_image_arrays != 0 {
            set.insert(PhysicalDeviceYcbcrImageArraysFeaturesEXT::YcbcrImageArrays);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::YcbcrImageArrays => vk_struct.ycbcr_image_arrays = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceImageRobustnessFeatures {
    RobustImageAccess,
}
impl const From<PhysicalDeviceImageRobustnessFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceImageRobustnessFeatures) -> Self {
        FeatureType::DeviceImageRobustnessFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceImageRobustnessFeatures {
    RobustImageAccess,
}
impl ToPhysicalFeature for DeviceImageRobustnessFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceImageRobustnessFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceImageRobustnessFeatures::RobustImageAccess => {
                PhysicalDeviceImageRobustnessFeatures::RobustImageAccess
            }
        }
    }
}
impl From<DeviceImageRobustnessFeatures> for DeviceFeature {
    fn from(feature: DeviceImageRobustnessFeatures) -> Self {
        DeviceFeature::DeviceImageRobustnessFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceImageRobustnessFeatures {
    type SubFeatureEnumTy = PhysicalDeviceImageRobustnessFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.robust_image_access != 0 {
            set.insert(PhysicalDeviceImageRobustnessFeatures::RobustImageAccess);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceImageRobustnessFeatures {
    type VkStruct = ash::vk::PhysicalDeviceImageRobustnessFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::RobustImageAccess => vk_struct.robust_image_access = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevice8BitStorageFeatures {
    StorageBuffer8BitAccess,
    UniformAndStorageBuffer8BitAccess,
    StoragePushConstant8,
}
impl const From<PhysicalDevice8BitStorageFeatures> for FeatureType {
    fn from(feature: PhysicalDevice8BitStorageFeatures) -> Self {
        FeatureType::Device8BitStorageFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum Device8BitStorageFeatures {
    StorageBuffer8BitAccess,
    UniformAndStorageBuffer8BitAccess,
    StoragePushConstant8,
}
impl ToPhysicalFeature for Device8BitStorageFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDevice8BitStorageFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            Device8BitStorageFeatures::StorageBuffer8BitAccess => {
                PhysicalDevice8BitStorageFeatures::StorageBuffer8BitAccess
            }
            Device8BitStorageFeatures::UniformAndStorageBuffer8BitAccess => {
                PhysicalDevice8BitStorageFeatures::UniformAndStorageBuffer8BitAccess
            }
            Device8BitStorageFeatures::StoragePushConstant8 => {
                PhysicalDevice8BitStorageFeatures::StoragePushConstant8
            }
        }
    }
}
impl From<Device8BitStorageFeatures> for DeviceFeature {
    fn from(feature: Device8BitStorageFeatures) -> Self {
        DeviceFeature::Device8BitStorageFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevice8BitStorageFeatures {
    type SubFeatureEnumTy = PhysicalDevice8BitStorageFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.storage_buffer8_bit_access != 0 {
            set.insert(PhysicalDevice8BitStorageFeatures::StorageBuffer8BitAccess);
        }
        if self.uniform_and_storage_buffer8_bit_access != 0 {
            set.insert(PhysicalDevice8BitStorageFeatures::UniformAndStorageBuffer8BitAccess);
        }
        if self.storage_push_constant8 != 0 {
            set.insert(PhysicalDevice8BitStorageFeatures::StoragePushConstant8);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevice8BitStorageFeatures {
    type VkStruct = ash::vk::PhysicalDevice8BitStorageFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::StorageBuffer8BitAccess => vk_struct.storage_buffer8_bit_access = 1,
            Self::UniformAndStorageBuffer8BitAccess => {
                vk_struct.uniform_and_storage_buffer8_bit_access = 1
            }
            Self::StoragePushConstant8 => vk_struct.storage_push_constant8 = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceVulkan11Features {
    StorageBuffer16BitAccess,
    UniformAndStorageBuffer16BitAccess,
    StoragePushConstant16,
    StorageInputOutput16,
    Multiview,
    MultiviewGeometryShader,
    MultiviewTessellationShader,
    VariablePointersStorageBuffer,
    VariablePointers,
    ProtectedMemory,
    SamplerYcbcrConversion,
    ShaderDrawParameters,
}
impl const From<PhysicalDeviceVulkan11Features> for FeatureType {
    fn from(feature: PhysicalDeviceVulkan11Features) -> Self {
        FeatureType::DeviceVulkan11Features(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceVulkan11Features {
    StorageBuffer16BitAccess,
    UniformAndStorageBuffer16BitAccess,
    StoragePushConstant16,
    StorageInputOutput16,
    Multiview,
    MultiviewGeometryShader,
    MultiviewTessellationShader,
    VariablePointersStorageBuffer,
    VariablePointers,
    ProtectedMemory,
    SamplerYcbcrConversion,
    ShaderDrawParameters,
}
impl ToPhysicalFeature for DeviceVulkan11Features {
    type PhysicalDeviceFeatureTy = PhysicalDeviceVulkan11Features;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceVulkan11Features::StorageBuffer16BitAccess => {
                PhysicalDeviceVulkan11Features::StorageBuffer16BitAccess
            }
            DeviceVulkan11Features::UniformAndStorageBuffer16BitAccess => {
                PhysicalDeviceVulkan11Features::UniformAndStorageBuffer16BitAccess
            }
            DeviceVulkan11Features::StoragePushConstant16 => {
                PhysicalDeviceVulkan11Features::StoragePushConstant16
            }
            DeviceVulkan11Features::StorageInputOutput16 => {
                PhysicalDeviceVulkan11Features::StorageInputOutput16
            }
            DeviceVulkan11Features::Multiview => PhysicalDeviceVulkan11Features::Multiview,
            DeviceVulkan11Features::MultiviewGeometryShader => {
                PhysicalDeviceVulkan11Features::MultiviewGeometryShader
            }
            DeviceVulkan11Features::MultiviewTessellationShader => {
                PhysicalDeviceVulkan11Features::MultiviewTessellationShader
            }
            DeviceVulkan11Features::VariablePointersStorageBuffer => {
                PhysicalDeviceVulkan11Features::VariablePointersStorageBuffer
            }
            DeviceVulkan11Features::VariablePointers => {
                PhysicalDeviceVulkan11Features::VariablePointers
            }
            DeviceVulkan11Features::ProtectedMemory => {
                PhysicalDeviceVulkan11Features::ProtectedMemory
            }
            DeviceVulkan11Features::SamplerYcbcrConversion => {
                PhysicalDeviceVulkan11Features::SamplerYcbcrConversion
            }
            DeviceVulkan11Features::ShaderDrawParameters => {
                PhysicalDeviceVulkan11Features::ShaderDrawParameters
            }
        }
    }
}
impl From<DeviceVulkan11Features> for DeviceFeature {
    fn from(feature: DeviceVulkan11Features) -> Self {
        DeviceFeature::DeviceVulkan11Features(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceVulkan11Features {
    type SubFeatureEnumTy = PhysicalDeviceVulkan11Features;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.storage_buffer16_bit_access != 0 {
            set.insert(PhysicalDeviceVulkan11Features::StorageBuffer16BitAccess);
        }
        if self.uniform_and_storage_buffer16_bit_access != 0 {
            set.insert(PhysicalDeviceVulkan11Features::UniformAndStorageBuffer16BitAccess);
        }
        if self.storage_push_constant16 != 0 {
            set.insert(PhysicalDeviceVulkan11Features::StoragePushConstant16);
        }
        if self.storage_input_output16 != 0 {
            set.insert(PhysicalDeviceVulkan11Features::StorageInputOutput16);
        }
        if self.multiview != 0 {
            set.insert(PhysicalDeviceVulkan11Features::Multiview);
        }
        if self.multiview_geometry_shader != 0 {
            set.insert(PhysicalDeviceVulkan11Features::MultiviewGeometryShader);
        }
        if self.multiview_tessellation_shader != 0 {
            set.insert(PhysicalDeviceVulkan11Features::MultiviewTessellationShader);
        }
        if self.variable_pointers_storage_buffer != 0 {
            set.insert(PhysicalDeviceVulkan11Features::VariablePointersStorageBuffer);
        }
        if self.variable_pointers != 0 {
            set.insert(PhysicalDeviceVulkan11Features::VariablePointers);
        }
        if self.protected_memory != 0 {
            set.insert(PhysicalDeviceVulkan11Features::ProtectedMemory);
        }
        if self.sampler_ycbcr_conversion != 0 {
            set.insert(PhysicalDeviceVulkan11Features::SamplerYcbcrConversion);
        }
        if self.shader_draw_parameters != 0 {
            set.insert(PhysicalDeviceVulkan11Features::ShaderDrawParameters);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceVulkan11Features {
    type VkStruct = ash::vk::PhysicalDeviceVulkan11Features;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::StorageBuffer16BitAccess => vk_struct.storage_buffer16_bit_access = 1,
            Self::UniformAndStorageBuffer16BitAccess => {
                vk_struct.uniform_and_storage_buffer16_bit_access = 1
            }
            Self::StoragePushConstant16 => vk_struct.storage_push_constant16 = 1,
            Self::StorageInputOutput16 => vk_struct.storage_input_output16 = 1,
            Self::Multiview => vk_struct.multiview = 1,
            Self::MultiviewGeometryShader => vk_struct.multiview_geometry_shader = 1,
            Self::MultiviewTessellationShader => vk_struct.multiview_tessellation_shader = 1,
            Self::VariablePointersStorageBuffer => vk_struct.variable_pointers_storage_buffer = 1,
            Self::VariablePointers => vk_struct.variable_pointers = 1,
            Self::ProtectedMemory => vk_struct.protected_memory = 1,
            Self::SamplerYcbcrConversion => vk_struct.sampler_ycbcr_conversion = 1,
            Self::ShaderDrawParameters => vk_struct.shader_draw_parameters = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceDepthClipControlFeaturesEXT {
    DepthClipControl,
}
impl const From<PhysicalDeviceDepthClipControlFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceDepthClipControlFeaturesEXT) -> Self {
        FeatureType::DeviceDepthClipControlFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceDepthClipControlFeaturesEXT {
    DepthClipControl,
}
impl ToPhysicalFeature for DeviceDepthClipControlFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceDepthClipControlFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceDepthClipControlFeaturesEXT::DepthClipControl => {
                PhysicalDeviceDepthClipControlFeaturesEXT::DepthClipControl
            }
        }
    }
}
impl From<DeviceDepthClipControlFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceDepthClipControlFeaturesEXT) -> Self {
        DeviceFeature::DeviceDepthClipControlFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceDepthClipControlFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceDepthClipControlFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.depth_clip_control != 0 {
            set.insert(PhysicalDeviceDepthClipControlFeaturesEXT::DepthClipControl);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceDepthClipControlFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceDepthClipControlFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DepthClipControl => vk_struct.depth_clip_control = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceASTCDecodeFeaturesEXT {
    DecodeModeSharedExponent,
}
impl const From<PhysicalDeviceASTCDecodeFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceASTCDecodeFeaturesEXT) -> Self {
        FeatureType::DeviceASTCDecodeFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceASTCDecodeFeaturesEXT {
    DecodeModeSharedExponent,
}
impl ToPhysicalFeature for DeviceASTCDecodeFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceASTCDecodeFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceASTCDecodeFeaturesEXT::DecodeModeSharedExponent => {
                PhysicalDeviceASTCDecodeFeaturesEXT::DecodeModeSharedExponent
            }
        }
    }
}
impl From<DeviceASTCDecodeFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceASTCDecodeFeaturesEXT) -> Self {
        DeviceFeature::DeviceASTCDecodeFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceASTCDecodeFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceASTCDecodeFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.decode_mode_shared_exponent != 0 {
            set.insert(PhysicalDeviceASTCDecodeFeaturesEXT::DecodeModeSharedExponent);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceASTCDecodeFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceASTCDecodeFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DecodeModeSharedExponent => vk_struct.decode_mode_shared_exponent = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceCooperativeMatrixFeaturesNV {
    CooperativeMatrix,
    CooperativeMatrixRobustBufferAccess,
}
impl const From<PhysicalDeviceCooperativeMatrixFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceCooperativeMatrixFeaturesNV) -> Self {
        FeatureType::DeviceCooperativeMatrixFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceCooperativeMatrixFeaturesNV {
    CooperativeMatrix,
    CooperativeMatrixRobustBufferAccess,
}
impl ToPhysicalFeature for DeviceCooperativeMatrixFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceCooperativeMatrixFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceCooperativeMatrixFeaturesNV::CooperativeMatrix => {
                PhysicalDeviceCooperativeMatrixFeaturesNV::CooperativeMatrix
            }
            DeviceCooperativeMatrixFeaturesNV::CooperativeMatrixRobustBufferAccess => {
                PhysicalDeviceCooperativeMatrixFeaturesNV::CooperativeMatrixRobustBufferAccess
            }
        }
    }
}
impl From<DeviceCooperativeMatrixFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceCooperativeMatrixFeaturesNV) -> Self {
        DeviceFeature::DeviceCooperativeMatrixFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceCooperativeMatrixFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceCooperativeMatrixFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.cooperative_matrix != 0 {
            set.insert(PhysicalDeviceCooperativeMatrixFeaturesNV::CooperativeMatrix);
        }
        if self.cooperative_matrix_robust_buffer_access != 0 {
            set.insert(
                PhysicalDeviceCooperativeMatrixFeaturesNV::CooperativeMatrixRobustBufferAccess,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceCooperativeMatrixFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceCooperativeMatrixFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::CooperativeMatrix => vk_struct.cooperative_matrix = 1,
            Self::CooperativeMatrixRobustBufferAccess => {
                vk_struct.cooperative_matrix_robust_buffer_access = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceVariablePointersFeatures {
    VariablePointersStorageBuffer,
    VariablePointers,
}
impl const From<PhysicalDeviceVariablePointersFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceVariablePointersFeatures) -> Self {
        FeatureType::DeviceVariablePointersFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceVariablePointersFeatures {
    VariablePointersStorageBuffer,
    VariablePointers,
}
impl ToPhysicalFeature for DeviceVariablePointersFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceVariablePointersFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceVariablePointersFeatures::VariablePointersStorageBuffer => {
                PhysicalDeviceVariablePointersFeatures::VariablePointersStorageBuffer
            }
            DeviceVariablePointersFeatures::VariablePointers => {
                PhysicalDeviceVariablePointersFeatures::VariablePointers
            }
        }
    }
}
impl From<DeviceVariablePointersFeatures> for DeviceFeature {
    fn from(feature: DeviceVariablePointersFeatures) -> Self {
        DeviceFeature::DeviceVariablePointersFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceVariablePointersFeatures {
    type SubFeatureEnumTy = PhysicalDeviceVariablePointersFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.variable_pointers_storage_buffer != 0 {
            set.insert(PhysicalDeviceVariablePointersFeatures::VariablePointersStorageBuffer);
        }
        if self.variable_pointers != 0 {
            set.insert(PhysicalDeviceVariablePointersFeatures::VariablePointers);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceVariablePointersFeatures {
    type VkStruct = ash::vk::PhysicalDeviceVariablePointersFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::VariablePointersStorageBuffer => vk_struct.variable_pointers_storage_buffer = 1,
            Self::VariablePointers => vk_struct.variable_pointers = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevicePortabilitySubsetFeaturesKHR {
    ConstantAlphaColorBlendFactors,
    Events,
    ImageViewFormatReinterpretation,
    ImageViewFormatSwizzle,
    ImageView2DOn3DImage,
    MultisampleArrayImage,
    MutableComparisonSamplers,
    PointPolygons,
    SamplerMipLodBias,
    SeparateStencilMaskRef,
    ShaderSampleRateInterpolationFunctions,
    TessellationIsolines,
    TessellationPointMode,
    TriangleFans,
    VertexAttributeAccessBeyondStride,
}
impl const From<PhysicalDevicePortabilitySubsetFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDevicePortabilitySubsetFeaturesKHR) -> Self {
        FeatureType::DevicePortabilitySubsetFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DevicePortabilitySubsetFeaturesKHR {
    ConstantAlphaColorBlendFactors,
    Events,
    ImageViewFormatReinterpretation,
    ImageViewFormatSwizzle,
    ImageView2DOn3DImage,
    MultisampleArrayImage,
    MutableComparisonSamplers,
    PointPolygons,
    SamplerMipLodBias,
    SeparateStencilMaskRef,
    ShaderSampleRateInterpolationFunctions,
    TessellationIsolines,
    TessellationPointMode,
    TriangleFans,
    VertexAttributeAccessBeyondStride,
}
impl ToPhysicalFeature for DevicePortabilitySubsetFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDevicePortabilitySubsetFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DevicePortabilitySubsetFeaturesKHR::ConstantAlphaColorBlendFactors => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::ConstantAlphaColorBlendFactors
            }
            DevicePortabilitySubsetFeaturesKHR::Events => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::Events
            }
            DevicePortabilitySubsetFeaturesKHR::ImageViewFormatReinterpretation => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::ImageViewFormatReinterpretation
            }
            DevicePortabilitySubsetFeaturesKHR::ImageViewFormatSwizzle => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::ImageViewFormatSwizzle
            }
            DevicePortabilitySubsetFeaturesKHR::ImageView2DOn3DImage => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::ImageView2DOn3DImage
            }
            DevicePortabilitySubsetFeaturesKHR::MultisampleArrayImage => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::MultisampleArrayImage
            }
            DevicePortabilitySubsetFeaturesKHR::MutableComparisonSamplers => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::MutableComparisonSamplers
            }
            DevicePortabilitySubsetFeaturesKHR::PointPolygons => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::PointPolygons
            }
            DevicePortabilitySubsetFeaturesKHR::SamplerMipLodBias => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::SamplerMipLodBias
            }
            DevicePortabilitySubsetFeaturesKHR::SeparateStencilMaskRef => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::SeparateStencilMaskRef
            }
            DevicePortabilitySubsetFeaturesKHR::ShaderSampleRateInterpolationFunctions => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::ShaderSampleRateInterpolationFunctions
            }
            DevicePortabilitySubsetFeaturesKHR::TessellationIsolines => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::TessellationIsolines
            }
            DevicePortabilitySubsetFeaturesKHR::TessellationPointMode => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::TessellationPointMode
            }
            DevicePortabilitySubsetFeaturesKHR::TriangleFans => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::TriangleFans
            }
            DevicePortabilitySubsetFeaturesKHR::VertexAttributeAccessBeyondStride => {
                PhysicalDevicePortabilitySubsetFeaturesKHR::VertexAttributeAccessBeyondStride
            }
        }
    }
}
impl From<DevicePortabilitySubsetFeaturesKHR> for DeviceFeature {
    fn from(feature: DevicePortabilitySubsetFeaturesKHR) -> Self {
        DeviceFeature::DevicePortabilitySubsetFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDevicePortabilitySubsetFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.constant_alpha_color_blend_factors != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::ConstantAlphaColorBlendFactors);
        }
        if self.events != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::Events);
        }
        if self.image_view_format_reinterpretation != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::ImageViewFormatReinterpretation);
        }
        if self.image_view_format_swizzle != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::ImageViewFormatSwizzle);
        }
        if self.image_view2_d_on3_d_image != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::ImageView2DOn3DImage);
        }
        if self.multisample_array_image != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::MultisampleArrayImage);
        }
        if self.mutable_comparison_samplers != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::MutableComparisonSamplers);
        }
        if self.point_polygons != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::PointPolygons);
        }
        if self.sampler_mip_lod_bias != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::SamplerMipLodBias);
        }
        if self.separate_stencil_mask_ref != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::SeparateStencilMaskRef);
        }
        if self.shader_sample_rate_interpolation_functions != 0 {
            set.insert(
                PhysicalDevicePortabilitySubsetFeaturesKHR::ShaderSampleRateInterpolationFunctions,
            );
        }
        if self.tessellation_isolines != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::TessellationIsolines);
        }
        if self.tessellation_point_mode != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::TessellationPointMode);
        }
        if self.triangle_fans != 0 {
            set.insert(PhysicalDevicePortabilitySubsetFeaturesKHR::TriangleFans);
        }
        if self.vertex_attribute_access_beyond_stride != 0 {
            set.insert(
                PhysicalDevicePortabilitySubsetFeaturesKHR::VertexAttributeAccessBeyondStride,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevicePortabilitySubsetFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ConstantAlphaColorBlendFactors => {
                vk_struct.constant_alpha_color_blend_factors = 1
            }
            Self::Events => vk_struct.events = 1,
            Self::ImageViewFormatReinterpretation => {
                vk_struct.image_view_format_reinterpretation = 1
            }
            Self::ImageViewFormatSwizzle => vk_struct.image_view_format_swizzle = 1,
            Self::ImageView2DOn3DImage => vk_struct.image_view2_d_on3_d_image = 1,
            Self::MultisampleArrayImage => vk_struct.multisample_array_image = 1,
            Self::MutableComparisonSamplers => vk_struct.mutable_comparison_samplers = 1,
            Self::PointPolygons => vk_struct.point_polygons = 1,
            Self::SamplerMipLodBias => vk_struct.sampler_mip_lod_bias = 1,
            Self::SeparateStencilMaskRef => vk_struct.separate_stencil_mask_ref = 1,
            Self::ShaderSampleRateInterpolationFunctions => {
                vk_struct.shader_sample_rate_interpolation_functions = 1
            }
            Self::TessellationIsolines => vk_struct.tessellation_isolines = 1,
            Self::TessellationPointMode => vk_struct.tessellation_point_mode = 1,
            Self::TriangleFans => vk_struct.triangle_fans = 1,
            Self::VertexAttributeAccessBeyondStride => {
                vk_struct.vertex_attribute_access_beyond_stride = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderImageFootprintFeaturesNV {
    ImageFootprint,
}
impl const From<PhysicalDeviceShaderImageFootprintFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceShaderImageFootprintFeaturesNV) -> Self {
        FeatureType::DeviceShaderImageFootprintFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderImageFootprintFeaturesNV {
    ImageFootprint,
}
impl ToPhysicalFeature for DeviceShaderImageFootprintFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderImageFootprintFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderImageFootprintFeaturesNV::ImageFootprint => {
                PhysicalDeviceShaderImageFootprintFeaturesNV::ImageFootprint
            }
        }
    }
}
impl From<DeviceShaderImageFootprintFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceShaderImageFootprintFeaturesNV) -> Self {
        DeviceFeature::DeviceShaderImageFootprintFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderImageFootprintFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceShaderImageFootprintFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.image_footprint != 0 {
            set.insert(PhysicalDeviceShaderImageFootprintFeaturesNV::ImageFootprint);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderImageFootprintFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceShaderImageFootprintFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ImageFootprint => vk_struct.image_footprint = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    TexelBufferAlignment,
}
impl const From<PhysicalDeviceTexelBufferAlignmentFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceTexelBufferAlignmentFeaturesEXT) -> Self {
        FeatureType::DeviceTexelBufferAlignmentFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceTexelBufferAlignmentFeaturesEXT {
    TexelBufferAlignment,
}
impl ToPhysicalFeature for DeviceTexelBufferAlignmentFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceTexelBufferAlignmentFeaturesEXT::TexelBufferAlignment => {
                PhysicalDeviceTexelBufferAlignmentFeaturesEXT::TexelBufferAlignment
            }
        }
    }
}
impl From<DeviceTexelBufferAlignmentFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceTexelBufferAlignmentFeaturesEXT) -> Self {
        DeviceFeature::DeviceTexelBufferAlignmentFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.texel_buffer_alignment != 0 {
            set.insert(PhysicalDeviceTexelBufferAlignmentFeaturesEXT::TexelBufferAlignment);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::TexelBufferAlignment => vk_struct.texel_buffer_alignment = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceImagelessFramebufferFeatures {
    ImagelessFramebuffer,
}
impl const From<PhysicalDeviceImagelessFramebufferFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceImagelessFramebufferFeatures) -> Self {
        FeatureType::DeviceImagelessFramebufferFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceImagelessFramebufferFeatures {
    ImagelessFramebuffer,
}
impl ToPhysicalFeature for DeviceImagelessFramebufferFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceImagelessFramebufferFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceImagelessFramebufferFeatures::ImagelessFramebuffer => {
                PhysicalDeviceImagelessFramebufferFeatures::ImagelessFramebuffer
            }
        }
    }
}
impl From<DeviceImagelessFramebufferFeatures> for DeviceFeature {
    fn from(feature: DeviceImagelessFramebufferFeatures) -> Self {
        DeviceFeature::DeviceImagelessFramebufferFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceImagelessFramebufferFeatures {
    type SubFeatureEnumTy = PhysicalDeviceImagelessFramebufferFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.imageless_framebuffer != 0 {
            set.insert(PhysicalDeviceImagelessFramebufferFeatures::ImagelessFramebuffer);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceImagelessFramebufferFeatures {
    type VkStruct = ash::vk::PhysicalDeviceImagelessFramebufferFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ImagelessFramebuffer => vk_struct.imageless_framebuffer = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    ShaderSmBuiltins,
}
impl const From<PhysicalDeviceShaderSMBuiltinsFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceShaderSMBuiltinsFeaturesNV) -> Self {
        FeatureType::DeviceShaderSMBuiltinsFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderSMBuiltinsFeaturesNV {
    ShaderSmBuiltins,
}
impl ToPhysicalFeature for DeviceShaderSMBuiltinsFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderSMBuiltinsFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderSMBuiltinsFeaturesNV::ShaderSmBuiltins => {
                PhysicalDeviceShaderSMBuiltinsFeaturesNV::ShaderSmBuiltins
            }
        }
    }
}
impl From<DeviceShaderSMBuiltinsFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceShaderSMBuiltinsFeaturesNV) -> Self {
        DeviceFeature::DeviceShaderSMBuiltinsFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceShaderSMBuiltinsFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_sm_builtins != 0 {
            set.insert(PhysicalDeviceShaderSMBuiltinsFeaturesNV::ShaderSmBuiltins);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderSmBuiltins => vk_struct.shader_sm_builtins = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    FragmentShaderBarycentric,
}
impl const From<PhysicalDeviceFragmentShaderBarycentricFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceFragmentShaderBarycentricFeaturesNV) -> Self {
        FeatureType::DeviceFragmentShaderBarycentricFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceFragmentShaderBarycentricFeaturesNV {
    FragmentShaderBarycentric,
}
impl ToPhysicalFeature for DeviceFragmentShaderBarycentricFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceFragmentShaderBarycentricFeaturesNV::FragmentShaderBarycentric => {
                PhysicalDeviceFragmentShaderBarycentricFeaturesNV::FragmentShaderBarycentric
            }
        }
    }
}
impl From<DeviceFragmentShaderBarycentricFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceFragmentShaderBarycentricFeaturesNV) -> Self {
        DeviceFeature::DeviceFragmentShaderBarycentricFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.fragment_shader_barycentric != 0 {
            set.insert(
                PhysicalDeviceFragmentShaderBarycentricFeaturesNV::FragmentShaderBarycentric,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::FragmentShaderBarycentric => vk_struct.fragment_shader_barycentric = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceTextureCompressionASTCHDRFeatures {
    TextureCompressionAstcHdr,
}
impl const From<PhysicalDeviceTextureCompressionASTCHDRFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceTextureCompressionASTCHDRFeatures) -> Self {
        FeatureType::DeviceTextureCompressionASTCHDRFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceTextureCompressionASTCHDRFeatures {
    TextureCompressionAstcHdr,
}
impl ToPhysicalFeature for DeviceTextureCompressionASTCHDRFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceTextureCompressionASTCHDRFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceTextureCompressionASTCHDRFeatures::TextureCompressionAstcHdr => {
                PhysicalDeviceTextureCompressionASTCHDRFeatures::TextureCompressionAstcHdr
            }
        }
    }
}
impl From<DeviceTextureCompressionASTCHDRFeatures> for DeviceFeature {
    fn from(feature: DeviceTextureCompressionASTCHDRFeatures) -> Self {
        DeviceFeature::DeviceTextureCompressionASTCHDRFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceTextureCompressionASTCHDRFeatures {
    type SubFeatureEnumTy = PhysicalDeviceTextureCompressionASTCHDRFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.texture_compression_astc_hdr != 0 {
            set.insert(PhysicalDeviceTextureCompressionASTCHDRFeatures::TextureCompressionAstcHdr);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceTextureCompressionASTCHDRFeatures {
    type VkStruct = ash::vk::PhysicalDeviceTextureCompressionASTCHDRFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::TextureCompressionAstcHdr => vk_struct.texture_compression_astc_hdr = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    PageableDeviceLocalMemory,
}
impl const From<PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT) -> Self {
        FeatureType::DevicePageableDeviceLocalMemoryFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DevicePageableDeviceLocalMemoryFeaturesEXT {
    PageableDeviceLocalMemory,
}
impl ToPhysicalFeature for DevicePageableDeviceLocalMemoryFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DevicePageableDeviceLocalMemoryFeaturesEXT::PageableDeviceLocalMemory => {
                PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT::PageableDeviceLocalMemory
            }
        }
    }
}
impl From<DevicePageableDeviceLocalMemoryFeaturesEXT> for DeviceFeature {
    fn from(feature: DevicePageableDeviceLocalMemoryFeaturesEXT) -> Self {
        DeviceFeature::DevicePageableDeviceLocalMemoryFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.pageable_device_local_memory != 0 {
            set.insert(
                PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT::PageableDeviceLocalMemory,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::PageableDeviceLocalMemory => vk_struct.pageable_device_local_memory = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    ShaderZeroInitializeWorkgroupMemory,
}
impl const From<PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures) -> Self {
        FeatureType::DeviceZeroInitializeWorkgroupMemoryFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceZeroInitializeWorkgroupMemoryFeatures {
    ShaderZeroInitializeWorkgroupMemory,
}
impl ToPhysicalFeature for DeviceZeroInitializeWorkgroupMemoryFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceZeroInitializeWorkgroupMemoryFeatures :: ShaderZeroInitializeWorkgroupMemory => PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures :: ShaderZeroInitializeWorkgroupMemory , }
    }
}
impl From<DeviceZeroInitializeWorkgroupMemoryFeatures> for DeviceFeature {
    fn from(feature: DeviceZeroInitializeWorkgroupMemoryFeatures) -> Self {
        DeviceFeature::DeviceZeroInitializeWorkgroupMemoryFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    type SubFeatureEnumTy = PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_zero_initialize_workgroup_memory != 0 {
            set . insert (PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures :: ShaderZeroInitializeWorkgroupMemory) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    type VkStruct = ash::vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderZeroInitializeWorkgroupMemory => {
                vk_struct.shader_zero_initialize_workgroup_memory = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceRobustness2FeaturesEXT {
    RobustBufferAccess2,
    RobustImageAccess2,
    NullDescriptor,
}
impl const From<PhysicalDeviceRobustness2FeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceRobustness2FeaturesEXT) -> Self {
        FeatureType::DeviceRobustness2FeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceRobustness2FeaturesEXT {
    RobustBufferAccess2,
    RobustImageAccess2,
    NullDescriptor,
}
impl ToPhysicalFeature for DeviceRobustness2FeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceRobustness2FeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceRobustness2FeaturesEXT::RobustBufferAccess2 => {
                PhysicalDeviceRobustness2FeaturesEXT::RobustBufferAccess2
            }
            DeviceRobustness2FeaturesEXT::RobustImageAccess2 => {
                PhysicalDeviceRobustness2FeaturesEXT::RobustImageAccess2
            }
            DeviceRobustness2FeaturesEXT::NullDescriptor => {
                PhysicalDeviceRobustness2FeaturesEXT::NullDescriptor
            }
        }
    }
}
impl From<DeviceRobustness2FeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceRobustness2FeaturesEXT) -> Self {
        DeviceFeature::DeviceRobustness2FeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceRobustness2FeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceRobustness2FeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.robust_buffer_access2 != 0 {
            set.insert(PhysicalDeviceRobustness2FeaturesEXT::RobustBufferAccess2);
        }
        if self.robust_image_access2 != 0 {
            set.insert(PhysicalDeviceRobustness2FeaturesEXT::RobustImageAccess2);
        }
        if self.null_descriptor != 0 {
            set.insert(PhysicalDeviceRobustness2FeaturesEXT::NullDescriptor);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceRobustness2FeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceRobustness2FeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::RobustBufferAccess2 => vk_struct.robust_buffer_access2 = 1,
            Self::RobustImageAccess2 => vk_struct.robust_image_access2 = 1,
            Self::NullDescriptor => vk_struct.null_descriptor = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderTerminateInvocationFeatures {
    ShaderTerminateInvocation,
}
impl const From<PhysicalDeviceShaderTerminateInvocationFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceShaderTerminateInvocationFeatures) -> Self {
        FeatureType::DeviceShaderTerminateInvocationFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderTerminateInvocationFeatures {
    ShaderTerminateInvocation,
}
impl ToPhysicalFeature for DeviceShaderTerminateInvocationFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderTerminateInvocationFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderTerminateInvocationFeatures::ShaderTerminateInvocation => {
                PhysicalDeviceShaderTerminateInvocationFeatures::ShaderTerminateInvocation
            }
        }
    }
}
impl From<DeviceShaderTerminateInvocationFeatures> for DeviceFeature {
    fn from(feature: DeviceShaderTerminateInvocationFeatures) -> Self {
        DeviceFeature::DeviceShaderTerminateInvocationFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderTerminateInvocationFeatures {
    type SubFeatureEnumTy = PhysicalDeviceShaderTerminateInvocationFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_terminate_invocation != 0 {
            set.insert(PhysicalDeviceShaderTerminateInvocationFeatures::ShaderTerminateInvocation);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderTerminateInvocationFeatures {
    type VkStruct = ash::vk::PhysicalDeviceShaderTerminateInvocationFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderTerminateInvocation => vk_struct.shader_terminate_invocation = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    GlobalPriorityQuery,
}
impl const From<PhysicalDeviceGlobalPriorityQueryFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDeviceGlobalPriorityQueryFeaturesKHR) -> Self {
        FeatureType::DeviceGlobalPriorityQueryFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceGlobalPriorityQueryFeaturesKHR {
    GlobalPriorityQuery,
}
impl ToPhysicalFeature for DeviceGlobalPriorityQueryFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceGlobalPriorityQueryFeaturesKHR::GlobalPriorityQuery => {
                PhysicalDeviceGlobalPriorityQueryFeaturesKHR::GlobalPriorityQuery
            }
        }
    }
}
impl From<DeviceGlobalPriorityQueryFeaturesKHR> for DeviceFeature {
    fn from(feature: DeviceGlobalPriorityQueryFeaturesKHR) -> Self {
        DeviceFeature::DeviceGlobalPriorityQueryFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.global_priority_query != 0 {
            set.insert(PhysicalDeviceGlobalPriorityQueryFeaturesKHR::GlobalPriorityQuery);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::GlobalPriorityQuery => vk_struct.global_priority_query = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    ShaderBufferFloat32Atomics,
    ShaderBufferFloat32AtomicAdd,
    ShaderBufferFloat64Atomics,
    ShaderBufferFloat64AtomicAdd,
    ShaderSharedFloat32Atomics,
    ShaderSharedFloat32AtomicAdd,
    ShaderSharedFloat64Atomics,
    ShaderSharedFloat64AtomicAdd,
    ShaderImageFloat32Atomics,
    ShaderImageFloat32AtomicAdd,
    SparseImageFloat32Atomics,
    SparseImageFloat32AtomicAdd,
}
impl const From<PhysicalDeviceShaderAtomicFloatFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceShaderAtomicFloatFeaturesEXT) -> Self {
        FeatureType::DeviceShaderAtomicFloatFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderAtomicFloatFeaturesEXT {
    ShaderBufferFloat32Atomics,
    ShaderBufferFloat32AtomicAdd,
    ShaderBufferFloat64Atomics,
    ShaderBufferFloat64AtomicAdd,
    ShaderSharedFloat32Atomics,
    ShaderSharedFloat32AtomicAdd,
    ShaderSharedFloat64Atomics,
    ShaderSharedFloat64AtomicAdd,
    ShaderImageFloat32Atomics,
    ShaderImageFloat32AtomicAdd,
    SparseImageFloat32Atomics,
    SparseImageFloat32AtomicAdd,
}
impl ToPhysicalFeature for DeviceShaderAtomicFloatFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderAtomicFloatFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat32Atomics => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat32Atomics
            }
            DeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat32AtomicAdd => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat32AtomicAdd
            }
            DeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat64Atomics => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat64Atomics
            }
            DeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat64AtomicAdd => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat64AtomicAdd
            }
            DeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat32Atomics => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat32Atomics
            }
            DeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat32AtomicAdd => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat32AtomicAdd
            }
            DeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat64Atomics => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat64Atomics
            }
            DeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat64AtomicAdd => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat64AtomicAdd
            }
            DeviceShaderAtomicFloatFeaturesEXT::ShaderImageFloat32Atomics => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderImageFloat32Atomics
            }
            DeviceShaderAtomicFloatFeaturesEXT::ShaderImageFloat32AtomicAdd => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderImageFloat32AtomicAdd
            }
            DeviceShaderAtomicFloatFeaturesEXT::SparseImageFloat32Atomics => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::SparseImageFloat32Atomics
            }
            DeviceShaderAtomicFloatFeaturesEXT::SparseImageFloat32AtomicAdd => {
                PhysicalDeviceShaderAtomicFloatFeaturesEXT::SparseImageFloat32AtomicAdd
            }
        }
    }
}
impl From<DeviceShaderAtomicFloatFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceShaderAtomicFloatFeaturesEXT) -> Self {
        DeviceFeature::DeviceShaderAtomicFloatFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceShaderAtomicFloatFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_buffer_float32_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat32Atomics);
        }
        if self.shader_buffer_float32_atomic_add != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat32AtomicAdd);
        }
        if self.shader_buffer_float64_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat64Atomics);
        }
        if self.shader_buffer_float64_atomic_add != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderBufferFloat64AtomicAdd);
        }
        if self.shader_shared_float32_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat32Atomics);
        }
        if self.shader_shared_float32_atomic_add != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat32AtomicAdd);
        }
        if self.shader_shared_float64_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat64Atomics);
        }
        if self.shader_shared_float64_atomic_add != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderSharedFloat64AtomicAdd);
        }
        if self.shader_image_float32_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderImageFloat32Atomics);
        }
        if self.shader_image_float32_atomic_add != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::ShaderImageFloat32AtomicAdd);
        }
        if self.sparse_image_float32_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::SparseImageFloat32Atomics);
        }
        if self.sparse_image_float32_atomic_add != 0 {
            set.insert(PhysicalDeviceShaderAtomicFloatFeaturesEXT::SparseImageFloat32AtomicAdd);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderBufferFloat32Atomics => vk_struct.shader_buffer_float32_atomics = 1,
            Self::ShaderBufferFloat32AtomicAdd => vk_struct.shader_buffer_float32_atomic_add = 1,
            Self::ShaderBufferFloat64Atomics => vk_struct.shader_buffer_float64_atomics = 1,
            Self::ShaderBufferFloat64AtomicAdd => vk_struct.shader_buffer_float64_atomic_add = 1,
            Self::ShaderSharedFloat32Atomics => vk_struct.shader_shared_float32_atomics = 1,
            Self::ShaderSharedFloat32AtomicAdd => vk_struct.shader_shared_float32_atomic_add = 1,
            Self::ShaderSharedFloat64Atomics => vk_struct.shader_shared_float64_atomics = 1,
            Self::ShaderSharedFloat64AtomicAdd => vk_struct.shader_shared_float64_atomic_add = 1,
            Self::ShaderImageFloat32Atomics => vk_struct.shader_image_float32_atomics = 1,
            Self::ShaderImageFloat32AtomicAdd => vk_struct.shader_image_float32_atomic_add = 1,
            Self::SparseImageFloat32Atomics => vk_struct.sparse_image_float32_atomics = 1,
            Self::SparseImageFloat32AtomicAdd => vk_struct.sparse_image_float32_atomic_add = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    VertexAttributeInstanceRateDivisor,
    VertexAttributeInstanceRateZeroDivisor,
}
impl const From<PhysicalDeviceVertexAttributeDivisorFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceVertexAttributeDivisorFeaturesEXT) -> Self {
        FeatureType::DeviceVertexAttributeDivisorFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceVertexAttributeDivisorFeaturesEXT {
    VertexAttributeInstanceRateDivisor,
    VertexAttributeInstanceRateZeroDivisor,
}
impl ToPhysicalFeature for DeviceVertexAttributeDivisorFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceVertexAttributeDivisorFeaturesEXT :: VertexAttributeInstanceRateDivisor => PhysicalDeviceVertexAttributeDivisorFeaturesEXT :: VertexAttributeInstanceRateDivisor , DeviceVertexAttributeDivisorFeaturesEXT :: VertexAttributeInstanceRateZeroDivisor => PhysicalDeviceVertexAttributeDivisorFeaturesEXT :: VertexAttributeInstanceRateZeroDivisor , }
    }
}
impl From<DeviceVertexAttributeDivisorFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceVertexAttributeDivisorFeaturesEXT) -> Self {
        DeviceFeature::DeviceVertexAttributeDivisorFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.vertex_attribute_instance_rate_divisor != 0 {
            set.insert(
                PhysicalDeviceVertexAttributeDivisorFeaturesEXT::VertexAttributeInstanceRateDivisor,
            );
        }
        if self.vertex_attribute_instance_rate_zero_divisor != 0 {
            set . insert (PhysicalDeviceVertexAttributeDivisorFeaturesEXT :: VertexAttributeInstanceRateZeroDivisor) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::VertexAttributeInstanceRateDivisor => {
                vk_struct.vertex_attribute_instance_rate_divisor = 1
            }
            Self::VertexAttributeInstanceRateZeroDivisor => {
                vk_struct.vertex_attribute_instance_rate_zero_divisor = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceDescriptorIndexingFeatures {
    ShaderInputAttachmentArrayDynamicIndexing,
    ShaderUniformTexelBufferArrayDynamicIndexing,
    ShaderStorageTexelBufferArrayDynamicIndexing,
    ShaderUniformBufferArrayNonUniformIndexing,
    ShaderSampledImageArrayNonUniformIndexing,
    ShaderStorageBufferArrayNonUniformIndexing,
    ShaderStorageImageArrayNonUniformIndexing,
    ShaderInputAttachmentArrayNonUniformIndexing,
    ShaderUniformTexelBufferArrayNonUniformIndexing,
    ShaderStorageTexelBufferArrayNonUniformIndexing,
    DescriptorBindingUniformBufferUpdateAfterBind,
    DescriptorBindingSampledImageUpdateAfterBind,
    DescriptorBindingStorageImageUpdateAfterBind,
    DescriptorBindingStorageBufferUpdateAfterBind,
    DescriptorBindingUniformTexelBufferUpdateAfterBind,
    DescriptorBindingStorageTexelBufferUpdateAfterBind,
    DescriptorBindingUpdateUnusedWhilePending,
    DescriptorBindingPartiallyBound,
    DescriptorBindingVariableDescriptorCount,
    RuntimeDescriptorArray,
}
impl const From<PhysicalDeviceDescriptorIndexingFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceDescriptorIndexingFeatures) -> Self {
        FeatureType::DeviceDescriptorIndexingFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceDescriptorIndexingFeatures {
    ShaderInputAttachmentArrayDynamicIndexing,
    ShaderUniformTexelBufferArrayDynamicIndexing,
    ShaderStorageTexelBufferArrayDynamicIndexing,
    ShaderUniformBufferArrayNonUniformIndexing,
    ShaderSampledImageArrayNonUniformIndexing,
    ShaderStorageBufferArrayNonUniformIndexing,
    ShaderStorageImageArrayNonUniformIndexing,
    ShaderInputAttachmentArrayNonUniformIndexing,
    ShaderUniformTexelBufferArrayNonUniformIndexing,
    ShaderStorageTexelBufferArrayNonUniformIndexing,
    DescriptorBindingUniformBufferUpdateAfterBind,
    DescriptorBindingSampledImageUpdateAfterBind,
    DescriptorBindingStorageImageUpdateAfterBind,
    DescriptorBindingStorageBufferUpdateAfterBind,
    DescriptorBindingUniformTexelBufferUpdateAfterBind,
    DescriptorBindingStorageTexelBufferUpdateAfterBind,
    DescriptorBindingUpdateUnusedWhilePending,
    DescriptorBindingPartiallyBound,
    DescriptorBindingVariableDescriptorCount,
    RuntimeDescriptorArray,
}
impl ToPhysicalFeature for DeviceDescriptorIndexingFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceDescriptorIndexingFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self { DeviceDescriptorIndexingFeatures :: ShaderInputAttachmentArrayDynamicIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderInputAttachmentArrayDynamicIndexing , DeviceDescriptorIndexingFeatures :: ShaderUniformTexelBufferArrayDynamicIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderUniformTexelBufferArrayDynamicIndexing , DeviceDescriptorIndexingFeatures :: ShaderStorageTexelBufferArrayDynamicIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderStorageTexelBufferArrayDynamicIndexing , DeviceDescriptorIndexingFeatures :: ShaderUniformBufferArrayNonUniformIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderUniformBufferArrayNonUniformIndexing , DeviceDescriptorIndexingFeatures :: ShaderSampledImageArrayNonUniformIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderSampledImageArrayNonUniformIndexing , DeviceDescriptorIndexingFeatures :: ShaderStorageBufferArrayNonUniformIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderStorageBufferArrayNonUniformIndexing , DeviceDescriptorIndexingFeatures :: ShaderStorageImageArrayNonUniformIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderStorageImageArrayNonUniformIndexing , DeviceDescriptorIndexingFeatures :: ShaderInputAttachmentArrayNonUniformIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderInputAttachmentArrayNonUniformIndexing , DeviceDescriptorIndexingFeatures :: ShaderUniformTexelBufferArrayNonUniformIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderUniformTexelBufferArrayNonUniformIndexing , DeviceDescriptorIndexingFeatures :: ShaderStorageTexelBufferArrayNonUniformIndexing => PhysicalDeviceDescriptorIndexingFeatures :: ShaderStorageTexelBufferArrayNonUniformIndexing , DeviceDescriptorIndexingFeatures :: DescriptorBindingUniformBufferUpdateAfterBind => PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingUniformBufferUpdateAfterBind , DeviceDescriptorIndexingFeatures :: DescriptorBindingSampledImageUpdateAfterBind => PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingSampledImageUpdateAfterBind , DeviceDescriptorIndexingFeatures :: DescriptorBindingStorageImageUpdateAfterBind => PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingStorageImageUpdateAfterBind , DeviceDescriptorIndexingFeatures :: DescriptorBindingStorageBufferUpdateAfterBind => PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingStorageBufferUpdateAfterBind , DeviceDescriptorIndexingFeatures :: DescriptorBindingUniformTexelBufferUpdateAfterBind => PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingUniformTexelBufferUpdateAfterBind , DeviceDescriptorIndexingFeatures :: DescriptorBindingStorageTexelBufferUpdateAfterBind => PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingStorageTexelBufferUpdateAfterBind , DeviceDescriptorIndexingFeatures :: DescriptorBindingUpdateUnusedWhilePending => PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingUpdateUnusedWhilePending , DeviceDescriptorIndexingFeatures :: DescriptorBindingPartiallyBound => PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingPartiallyBound , DeviceDescriptorIndexingFeatures :: DescriptorBindingVariableDescriptorCount => PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingVariableDescriptorCount , DeviceDescriptorIndexingFeatures :: RuntimeDescriptorArray => PhysicalDeviceDescriptorIndexingFeatures :: RuntimeDescriptorArray , }
    }
}
impl From<DeviceDescriptorIndexingFeatures> for DeviceFeature {
    fn from(feature: DeviceDescriptorIndexingFeatures) -> Self {
        DeviceFeature::DeviceDescriptorIndexingFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceDescriptorIndexingFeatures {
    type SubFeatureEnumTy = PhysicalDeviceDescriptorIndexingFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_input_attachment_array_dynamic_indexing != 0 {
            set.insert(
                PhysicalDeviceDescriptorIndexingFeatures::ShaderInputAttachmentArrayDynamicIndexing,
            );
        }
        if self.shader_uniform_texel_buffer_array_dynamic_indexing != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: ShaderUniformTexelBufferArrayDynamicIndexing) ;
        }
        if self.shader_storage_texel_buffer_array_dynamic_indexing != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: ShaderStorageTexelBufferArrayDynamicIndexing) ;
        }
        if self.shader_uniform_buffer_array_non_uniform_indexing != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: ShaderUniformBufferArrayNonUniformIndexing) ;
        }
        if self.shader_sampled_image_array_non_uniform_indexing != 0 {
            set.insert(
                PhysicalDeviceDescriptorIndexingFeatures::ShaderSampledImageArrayNonUniformIndexing,
            );
        }
        if self.shader_storage_buffer_array_non_uniform_indexing != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: ShaderStorageBufferArrayNonUniformIndexing) ;
        }
        if self.shader_storage_image_array_non_uniform_indexing != 0 {
            set.insert(
                PhysicalDeviceDescriptorIndexingFeatures::ShaderStorageImageArrayNonUniformIndexing,
            );
        }
        if self.shader_input_attachment_array_non_uniform_indexing != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: ShaderInputAttachmentArrayNonUniformIndexing) ;
        }
        if self.shader_uniform_texel_buffer_array_non_uniform_indexing != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: ShaderUniformTexelBufferArrayNonUniformIndexing) ;
        }
        if self.shader_storage_texel_buffer_array_non_uniform_indexing != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: ShaderStorageTexelBufferArrayNonUniformIndexing) ;
        }
        if self.descriptor_binding_uniform_buffer_update_after_bind != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingUniformBufferUpdateAfterBind) ;
        }
        if self.descriptor_binding_sampled_image_update_after_bind != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingSampledImageUpdateAfterBind) ;
        }
        if self.descriptor_binding_storage_image_update_after_bind != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingStorageImageUpdateAfterBind) ;
        }
        if self.descriptor_binding_storage_buffer_update_after_bind != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingStorageBufferUpdateAfterBind) ;
        }
        if self.descriptor_binding_uniform_texel_buffer_update_after_bind != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingUniformTexelBufferUpdateAfterBind) ;
        }
        if self.descriptor_binding_storage_texel_buffer_update_after_bind != 0 {
            set . insert (PhysicalDeviceDescriptorIndexingFeatures :: DescriptorBindingStorageTexelBufferUpdateAfterBind) ;
        }
        if self.descriptor_binding_update_unused_while_pending != 0 {
            set.insert(
                PhysicalDeviceDescriptorIndexingFeatures::DescriptorBindingUpdateUnusedWhilePending,
            );
        }
        if self.descriptor_binding_partially_bound != 0 {
            set.insert(PhysicalDeviceDescriptorIndexingFeatures::DescriptorBindingPartiallyBound);
        }
        if self.descriptor_binding_variable_descriptor_count != 0 {
            set.insert(
                PhysicalDeviceDescriptorIndexingFeatures::DescriptorBindingVariableDescriptorCount,
            );
        }
        if self.runtime_descriptor_array != 0 {
            set.insert(PhysicalDeviceDescriptorIndexingFeatures::RuntimeDescriptorArray);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceDescriptorIndexingFeatures {
    type VkStruct = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderInputAttachmentArrayDynamicIndexing => {
                vk_struct.shader_input_attachment_array_dynamic_indexing = 1
            }
            Self::ShaderUniformTexelBufferArrayDynamicIndexing => {
                vk_struct.shader_uniform_texel_buffer_array_dynamic_indexing = 1
            }
            Self::ShaderStorageTexelBufferArrayDynamicIndexing => {
                vk_struct.shader_storage_texel_buffer_array_dynamic_indexing = 1
            }
            Self::ShaderUniformBufferArrayNonUniformIndexing => {
                vk_struct.shader_uniform_buffer_array_non_uniform_indexing = 1
            }
            Self::ShaderSampledImageArrayNonUniformIndexing => {
                vk_struct.shader_sampled_image_array_non_uniform_indexing = 1
            }
            Self::ShaderStorageBufferArrayNonUniformIndexing => {
                vk_struct.shader_storage_buffer_array_non_uniform_indexing = 1
            }
            Self::ShaderStorageImageArrayNonUniformIndexing => {
                vk_struct.shader_storage_image_array_non_uniform_indexing = 1
            }
            Self::ShaderInputAttachmentArrayNonUniformIndexing => {
                vk_struct.shader_input_attachment_array_non_uniform_indexing = 1
            }
            Self::ShaderUniformTexelBufferArrayNonUniformIndexing => {
                vk_struct.shader_uniform_texel_buffer_array_non_uniform_indexing = 1
            }
            Self::ShaderStorageTexelBufferArrayNonUniformIndexing => {
                vk_struct.shader_storage_texel_buffer_array_non_uniform_indexing = 1
            }
            Self::DescriptorBindingUniformBufferUpdateAfterBind => {
                vk_struct.descriptor_binding_uniform_buffer_update_after_bind = 1
            }
            Self::DescriptorBindingSampledImageUpdateAfterBind => {
                vk_struct.descriptor_binding_sampled_image_update_after_bind = 1
            }
            Self::DescriptorBindingStorageImageUpdateAfterBind => {
                vk_struct.descriptor_binding_storage_image_update_after_bind = 1
            }
            Self::DescriptorBindingStorageBufferUpdateAfterBind => {
                vk_struct.descriptor_binding_storage_buffer_update_after_bind = 1
            }
            Self::DescriptorBindingUniformTexelBufferUpdateAfterBind => {
                vk_struct.descriptor_binding_uniform_texel_buffer_update_after_bind = 1
            }
            Self::DescriptorBindingStorageTexelBufferUpdateAfterBind => {
                vk_struct.descriptor_binding_storage_texel_buffer_update_after_bind = 1
            }
            Self::DescriptorBindingUpdateUnusedWhilePending => {
                vk_struct.descriptor_binding_update_unused_while_pending = 1
            }
            Self::DescriptorBindingPartiallyBound => {
                vk_struct.descriptor_binding_partially_bound = 1
            }
            Self::DescriptorBindingVariableDescriptorCount => {
                vk_struct.descriptor_binding_variable_descriptor_count = 1
            }
            Self::RuntimeDescriptorArray => vk_struct.runtime_descriptor_array = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    FragmentDensityMapDeferred,
}
impl const From<PhysicalDeviceFragmentDensityMap2FeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceFragmentDensityMap2FeaturesEXT) -> Self {
        FeatureType::DeviceFragmentDensityMap2FeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceFragmentDensityMap2FeaturesEXT {
    FragmentDensityMapDeferred,
}
impl ToPhysicalFeature for DeviceFragmentDensityMap2FeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceFragmentDensityMap2FeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceFragmentDensityMap2FeaturesEXT::FragmentDensityMapDeferred => {
                PhysicalDeviceFragmentDensityMap2FeaturesEXT::FragmentDensityMapDeferred
            }
        }
    }
}
impl From<DeviceFragmentDensityMap2FeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceFragmentDensityMap2FeaturesEXT) -> Self {
        DeviceFeature::DeviceFragmentDensityMap2FeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceFragmentDensityMap2FeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.fragment_density_map_deferred != 0 {
            set.insert(PhysicalDeviceFragmentDensityMap2FeaturesEXT::FragmentDensityMapDeferred);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::FragmentDensityMapDeferred => vk_struct.fragment_density_map_deferred = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceUniformBufferStandardLayoutFeatures {
    UniformBufferStandardLayout,
}
impl const From<PhysicalDeviceUniformBufferStandardLayoutFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceUniformBufferStandardLayoutFeatures) -> Self {
        FeatureType::DeviceUniformBufferStandardLayoutFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceUniformBufferStandardLayoutFeatures {
    UniformBufferStandardLayout,
}
impl ToPhysicalFeature for DeviceUniformBufferStandardLayoutFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceUniformBufferStandardLayoutFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceUniformBufferStandardLayoutFeatures::UniformBufferStandardLayout => {
                PhysicalDeviceUniformBufferStandardLayoutFeatures::UniformBufferStandardLayout
            }
        }
    }
}
impl From<DeviceUniformBufferStandardLayoutFeatures> for DeviceFeature {
    fn from(feature: DeviceUniformBufferStandardLayoutFeatures) -> Self {
        DeviceFeature::DeviceUniformBufferStandardLayoutFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceUniformBufferStandardLayoutFeatures {
    type SubFeatureEnumTy = PhysicalDeviceUniformBufferStandardLayoutFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.uniform_buffer_standard_layout != 0 {
            set.insert(
                PhysicalDeviceUniformBufferStandardLayoutFeatures::UniformBufferStandardLayout,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceUniformBufferStandardLayoutFeatures {
    type VkStruct = ash::vk::PhysicalDeviceUniformBufferStandardLayoutFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::UniformBufferStandardLayout => vk_struct.uniform_buffer_standard_layout = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceMemoryPriorityFeaturesEXT {
    MemoryPriority,
}
impl const From<PhysicalDeviceMemoryPriorityFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceMemoryPriorityFeaturesEXT) -> Self {
        FeatureType::DeviceMemoryPriorityFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceMemoryPriorityFeaturesEXT {
    MemoryPriority,
}
impl ToPhysicalFeature for DeviceMemoryPriorityFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceMemoryPriorityFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceMemoryPriorityFeaturesEXT::MemoryPriority => {
                PhysicalDeviceMemoryPriorityFeaturesEXT::MemoryPriority
            }
        }
    }
}
impl From<DeviceMemoryPriorityFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceMemoryPriorityFeaturesEXT) -> Self {
        DeviceFeature::DeviceMemoryPriorityFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceMemoryPriorityFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceMemoryPriorityFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.memory_priority != 0 {
            set.insert(PhysicalDeviceMemoryPriorityFeaturesEXT::MemoryPriority);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceMemoryPriorityFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceMemoryPriorityFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::MemoryPriority => vk_struct.memory_priority = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceScalarBlockLayoutFeatures {
    ScalarBlockLayout,
}
impl const From<PhysicalDeviceScalarBlockLayoutFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceScalarBlockLayoutFeatures) -> Self {
        FeatureType::DeviceScalarBlockLayoutFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceScalarBlockLayoutFeatures {
    ScalarBlockLayout,
}
impl ToPhysicalFeature for DeviceScalarBlockLayoutFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceScalarBlockLayoutFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceScalarBlockLayoutFeatures::ScalarBlockLayout => {
                PhysicalDeviceScalarBlockLayoutFeatures::ScalarBlockLayout
            }
        }
    }
}
impl From<DeviceScalarBlockLayoutFeatures> for DeviceFeature {
    fn from(feature: DeviceScalarBlockLayoutFeatures) -> Self {
        DeviceFeature::DeviceScalarBlockLayoutFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceScalarBlockLayoutFeatures {
    type SubFeatureEnumTy = PhysicalDeviceScalarBlockLayoutFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.scalar_block_layout != 0 {
            set.insert(PhysicalDeviceScalarBlockLayoutFeatures::ScalarBlockLayout);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceScalarBlockLayoutFeatures {
    type VkStruct = ash::vk::PhysicalDeviceScalarBlockLayoutFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ScalarBlockLayout => vk_struct.scalar_block_layout = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    ShaderDemoteToHelperInvocation,
}
impl const From<PhysicalDeviceShaderDemoteToHelperInvocationFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceShaderDemoteToHelperInvocationFeatures) -> Self {
        FeatureType::DeviceShaderDemoteToHelperInvocationFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderDemoteToHelperInvocationFeatures {
    ShaderDemoteToHelperInvocation,
}
impl ToPhysicalFeature for DeviceShaderDemoteToHelperInvocationFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderDemoteToHelperInvocationFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderDemoteToHelperInvocationFeatures::ShaderDemoteToHelperInvocation => {
                PhysicalDeviceShaderDemoteToHelperInvocationFeatures::ShaderDemoteToHelperInvocation
            }
        }
    }
}
impl From<DeviceShaderDemoteToHelperInvocationFeatures> for DeviceFeature {
    fn from(feature: DeviceShaderDemoteToHelperInvocationFeatures) -> Self {
        DeviceFeature::DeviceShaderDemoteToHelperInvocationFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    type SubFeatureEnumTy = PhysicalDeviceShaderDemoteToHelperInvocationFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_demote_to_helper_invocation != 0 {
            set . insert (PhysicalDeviceShaderDemoteToHelperInvocationFeatures :: ShaderDemoteToHelperInvocation) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    type VkStruct = ash::vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderDemoteToHelperInvocation => {
                vk_struct.shader_demote_to_helper_invocation = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceSubgroupSizeControlFeatures {
    SubgroupSizeControl,
    ComputeFullSubgroups,
}
impl const From<PhysicalDeviceSubgroupSizeControlFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceSubgroupSizeControlFeatures) -> Self {
        FeatureType::DeviceSubgroupSizeControlFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceSubgroupSizeControlFeatures {
    SubgroupSizeControl,
    ComputeFullSubgroups,
}
impl ToPhysicalFeature for DeviceSubgroupSizeControlFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceSubgroupSizeControlFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceSubgroupSizeControlFeatures::SubgroupSizeControl => {
                PhysicalDeviceSubgroupSizeControlFeatures::SubgroupSizeControl
            }
            DeviceSubgroupSizeControlFeatures::ComputeFullSubgroups => {
                PhysicalDeviceSubgroupSizeControlFeatures::ComputeFullSubgroups
            }
        }
    }
}
impl From<DeviceSubgroupSizeControlFeatures> for DeviceFeature {
    fn from(feature: DeviceSubgroupSizeControlFeatures) -> Self {
        DeviceFeature::DeviceSubgroupSizeControlFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceSubgroupSizeControlFeatures {
    type SubFeatureEnumTy = PhysicalDeviceSubgroupSizeControlFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.subgroup_size_control != 0 {
            set.insert(PhysicalDeviceSubgroupSizeControlFeatures::SubgroupSizeControl);
        }
        if self.compute_full_subgroups != 0 {
            set.insert(PhysicalDeviceSubgroupSizeControlFeatures::ComputeFullSubgroups);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceSubgroupSizeControlFeatures {
    type VkStruct = ash::vk::PhysicalDeviceSubgroupSizeControlFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::SubgroupSizeControl => vk_struct.subgroup_size_control = 1,
            Self::ComputeFullSubgroups => vk_struct.compute_full_subgroups = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderAtomicInt64Features {
    ShaderBufferInt64Atomics,
    ShaderSharedInt64Atomics,
}
impl const From<PhysicalDeviceShaderAtomicInt64Features> for FeatureType {
    fn from(feature: PhysicalDeviceShaderAtomicInt64Features) -> Self {
        FeatureType::DeviceShaderAtomicInt64Features(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderAtomicInt64Features {
    ShaderBufferInt64Atomics,
    ShaderSharedInt64Atomics,
}
impl ToPhysicalFeature for DeviceShaderAtomicInt64Features {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderAtomicInt64Features;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderAtomicInt64Features::ShaderBufferInt64Atomics => {
                PhysicalDeviceShaderAtomicInt64Features::ShaderBufferInt64Atomics
            }
            DeviceShaderAtomicInt64Features::ShaderSharedInt64Atomics => {
                PhysicalDeviceShaderAtomicInt64Features::ShaderSharedInt64Atomics
            }
        }
    }
}
impl From<DeviceShaderAtomicInt64Features> for DeviceFeature {
    fn from(feature: DeviceShaderAtomicInt64Features) -> Self {
        DeviceFeature::DeviceShaderAtomicInt64Features(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderAtomicInt64Features {
    type SubFeatureEnumTy = PhysicalDeviceShaderAtomicInt64Features;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_buffer_int64_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicInt64Features::ShaderBufferInt64Atomics);
        }
        if self.shader_shared_int64_atomics != 0 {
            set.insert(PhysicalDeviceShaderAtomicInt64Features::ShaderSharedInt64Atomics);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderAtomicInt64Features {
    type VkStruct = ash::vk::PhysicalDeviceShaderAtomicInt64Features;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderBufferInt64Atomics => vk_struct.shader_buffer_int64_atomics = 1,
            Self::ShaderSharedInt64Atomics => vk_struct.shader_shared_int64_atomics = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceMeshShaderFeaturesNV {
    TaskShader,
    MeshShader,
}
impl const From<PhysicalDeviceMeshShaderFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceMeshShaderFeaturesNV) -> Self {
        FeatureType::DeviceMeshShaderFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceMeshShaderFeaturesNV {
    TaskShader,
    MeshShader,
}
impl ToPhysicalFeature for DeviceMeshShaderFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceMeshShaderFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceMeshShaderFeaturesNV::TaskShader => {
                PhysicalDeviceMeshShaderFeaturesNV::TaskShader
            }
            DeviceMeshShaderFeaturesNV::MeshShader => {
                PhysicalDeviceMeshShaderFeaturesNV::MeshShader
            }
        }
    }
}
impl From<DeviceMeshShaderFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceMeshShaderFeaturesNV) -> Self {
        DeviceFeature::DeviceMeshShaderFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceMeshShaderFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceMeshShaderFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.task_shader != 0 {
            set.insert(PhysicalDeviceMeshShaderFeaturesNV::TaskShader);
        }
        if self.mesh_shader != 0 {
            set.insert(PhysicalDeviceMeshShaderFeaturesNV::MeshShader);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceMeshShaderFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceMeshShaderFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::TaskShader => vk_struct.task_shader = 1,
            Self::MeshShader => vk_struct.mesh_shader = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderClockFeaturesKHR {
    ShaderSubgroupClock,
    ShaderDeviceClock,
}
impl const From<PhysicalDeviceShaderClockFeaturesKHR> for FeatureType {
    fn from(feature: PhysicalDeviceShaderClockFeaturesKHR) -> Self {
        FeatureType::DeviceShaderClockFeaturesKHR(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderClockFeaturesKHR {
    ShaderSubgroupClock,
    ShaderDeviceClock,
}
impl ToPhysicalFeature for DeviceShaderClockFeaturesKHR {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderClockFeaturesKHR;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderClockFeaturesKHR::ShaderSubgroupClock => {
                PhysicalDeviceShaderClockFeaturesKHR::ShaderSubgroupClock
            }
            DeviceShaderClockFeaturesKHR::ShaderDeviceClock => {
                PhysicalDeviceShaderClockFeaturesKHR::ShaderDeviceClock
            }
        }
    }
}
impl From<DeviceShaderClockFeaturesKHR> for DeviceFeature {
    fn from(feature: DeviceShaderClockFeaturesKHR) -> Self {
        DeviceFeature::DeviceShaderClockFeaturesKHR(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderClockFeaturesKHR {
    type SubFeatureEnumTy = PhysicalDeviceShaderClockFeaturesKHR;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_subgroup_clock != 0 {
            set.insert(PhysicalDeviceShaderClockFeaturesKHR::ShaderSubgroupClock);
        }
        if self.shader_device_clock != 0 {
            set.insert(PhysicalDeviceShaderClockFeaturesKHR::ShaderDeviceClock);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderClockFeaturesKHR {
    type VkStruct = ash::vk::PhysicalDeviceShaderClockFeaturesKHR;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderSubgroupClock => vk_struct.shader_subgroup_clock = 1,
            Self::ShaderDeviceClock => vk_struct.shader_device_clock = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    FragmentShaderSampleInterlock,
    FragmentShaderPixelInterlock,
    FragmentShaderShadingRateInterlock,
}
impl const From<PhysicalDeviceFragmentShaderInterlockFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceFragmentShaderInterlockFeaturesEXT) -> Self {
        FeatureType::DeviceFragmentShaderInterlockFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceFragmentShaderInterlockFeaturesEXT {
    FragmentShaderSampleInterlock,
    FragmentShaderPixelInterlock,
    FragmentShaderShadingRateInterlock,
}
impl ToPhysicalFeature for DeviceFragmentShaderInterlockFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceFragmentShaderInterlockFeaturesEXT::FragmentShaderSampleInterlock => {
                PhysicalDeviceFragmentShaderInterlockFeaturesEXT::FragmentShaderSampleInterlock
            }
            DeviceFragmentShaderInterlockFeaturesEXT::FragmentShaderPixelInterlock => {
                PhysicalDeviceFragmentShaderInterlockFeaturesEXT::FragmentShaderPixelInterlock
            }
            DeviceFragmentShaderInterlockFeaturesEXT::FragmentShaderShadingRateInterlock => {
                PhysicalDeviceFragmentShaderInterlockFeaturesEXT::FragmentShaderShadingRateInterlock
            }
        }
    }
}
impl From<DeviceFragmentShaderInterlockFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceFragmentShaderInterlockFeaturesEXT) -> Self {
        DeviceFeature::DeviceFragmentShaderInterlockFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.fragment_shader_sample_interlock != 0 {
            set.insert(
                PhysicalDeviceFragmentShaderInterlockFeaturesEXT::FragmentShaderSampleInterlock,
            );
        }
        if self.fragment_shader_pixel_interlock != 0 {
            set.insert(
                PhysicalDeviceFragmentShaderInterlockFeaturesEXT::FragmentShaderPixelInterlock,
            );
        }
        if self.fragment_shader_shading_rate_interlock != 0 {
            set . insert (PhysicalDeviceFragmentShaderInterlockFeaturesEXT :: FragmentShaderShadingRateInterlock) ;
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::FragmentShaderSampleInterlock => vk_struct.fragment_shader_sample_interlock = 1,
            Self::FragmentShaderPixelInterlock => vk_struct.fragment_shader_pixel_interlock = 1,
            Self::FragmentShaderShadingRateInterlock => {
                vk_struct.fragment_shader_shading_rate_interlock = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    RepresentativeFragmentTest,
}
impl const From<PhysicalDeviceRepresentativeFragmentTestFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceRepresentativeFragmentTestFeaturesNV) -> Self {
        FeatureType::DeviceRepresentativeFragmentTestFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceRepresentativeFragmentTestFeaturesNV {
    RepresentativeFragmentTest,
}
impl ToPhysicalFeature for DeviceRepresentativeFragmentTestFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceRepresentativeFragmentTestFeaturesNV::RepresentativeFragmentTest => {
                PhysicalDeviceRepresentativeFragmentTestFeaturesNV::RepresentativeFragmentTest
            }
        }
    }
}
impl From<DeviceRepresentativeFragmentTestFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceRepresentativeFragmentTestFeaturesNV) -> Self {
        DeviceFeature::DeviceRepresentativeFragmentTestFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.representative_fragment_test != 0 {
            set.insert(
                PhysicalDeviceRepresentativeFragmentTestFeaturesNV::RepresentativeFragmentTest,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::RepresentativeFragmentTest => vk_struct.representative_fragment_test = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    FragmentShadingRateEnums,
    SupersampleFragmentShadingRates,
    NoInvocationFragmentShadingRates,
}
impl const From<PhysicalDeviceFragmentShadingRateEnumsFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceFragmentShadingRateEnumsFeaturesNV) -> Self {
        FeatureType::DeviceFragmentShadingRateEnumsFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceFragmentShadingRateEnumsFeaturesNV {
    FragmentShadingRateEnums,
    SupersampleFragmentShadingRates,
    NoInvocationFragmentShadingRates,
}
impl ToPhysicalFeature for DeviceFragmentShadingRateEnumsFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceFragmentShadingRateEnumsFeaturesNV::FragmentShadingRateEnums => {
                PhysicalDeviceFragmentShadingRateEnumsFeaturesNV::FragmentShadingRateEnums
            }
            DeviceFragmentShadingRateEnumsFeaturesNV::SupersampleFragmentShadingRates => {
                PhysicalDeviceFragmentShadingRateEnumsFeaturesNV::SupersampleFragmentShadingRates
            }
            DeviceFragmentShadingRateEnumsFeaturesNV::NoInvocationFragmentShadingRates => {
                PhysicalDeviceFragmentShadingRateEnumsFeaturesNV::NoInvocationFragmentShadingRates
            }
        }
    }
}
impl From<DeviceFragmentShadingRateEnumsFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceFragmentShadingRateEnumsFeaturesNV) -> Self {
        DeviceFeature::DeviceFragmentShadingRateEnumsFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.fragment_shading_rate_enums != 0 {
            set.insert(PhysicalDeviceFragmentShadingRateEnumsFeaturesNV::FragmentShadingRateEnums);
        }
        if self.supersample_fragment_shading_rates != 0 {
            set.insert(
                PhysicalDeviceFragmentShadingRateEnumsFeaturesNV::SupersampleFragmentShadingRates,
            );
        }
        if self.no_invocation_fragment_shading_rates != 0 {
            set.insert(
                PhysicalDeviceFragmentShadingRateEnumsFeaturesNV::NoInvocationFragmentShadingRates,
            );
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::FragmentShadingRateEnums => vk_struct.fragment_shading_rate_enums = 1,
            Self::SupersampleFragmentShadingRates => {
                vk_struct.supersample_fragment_shading_rates = 1
            }
            Self::NoInvocationFragmentShadingRates => {
                vk_struct.no_invocation_fragment_shading_rates = 1
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    DeviceGeneratedCommands,
}
impl const From<PhysicalDeviceDeviceGeneratedCommandsFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceDeviceGeneratedCommandsFeaturesNV) -> Self {
        FeatureType::DeviceDeviceGeneratedCommandsFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceDeviceGeneratedCommandsFeaturesNV {
    DeviceGeneratedCommands,
}
impl ToPhysicalFeature for DeviceDeviceGeneratedCommandsFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceDeviceGeneratedCommandsFeaturesNV::DeviceGeneratedCommands => {
                PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::DeviceGeneratedCommands
            }
        }
    }
}
impl From<DeviceDeviceGeneratedCommandsFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceDeviceGeneratedCommandsFeaturesNV) -> Self {
        DeviceFeature::DeviceDeviceGeneratedCommandsFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.device_generated_commands != 0 {
            set.insert(PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::DeviceGeneratedCommands);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DeviceGeneratedCommands => vk_struct.device_generated_commands = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceExclusiveScissorFeaturesNV {
    ExclusiveScissor,
}
impl const From<PhysicalDeviceExclusiveScissorFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceExclusiveScissorFeaturesNV) -> Self {
        FeatureType::DeviceExclusiveScissorFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceExclusiveScissorFeaturesNV {
    ExclusiveScissor,
}
impl ToPhysicalFeature for DeviceExclusiveScissorFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceExclusiveScissorFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceExclusiveScissorFeaturesNV::ExclusiveScissor => {
                PhysicalDeviceExclusiveScissorFeaturesNV::ExclusiveScissor
            }
        }
    }
}
impl From<DeviceExclusiveScissorFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceExclusiveScissorFeaturesNV) -> Self {
        DeviceFeature::DeviceExclusiveScissorFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceExclusiveScissorFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceExclusiveScissorFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.exclusive_scissor != 0 {
            set.insert(PhysicalDeviceExclusiveScissorFeaturesNV::ExclusiveScissor);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceExclusiveScissorFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceExclusiveScissorFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ExclusiveScissor => vk_struct.exclusive_scissor = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceCornerSampledImageFeaturesNV {
    CornerSampledImage,
}
impl const From<PhysicalDeviceCornerSampledImageFeaturesNV> for FeatureType {
    fn from(feature: PhysicalDeviceCornerSampledImageFeaturesNV) -> Self {
        FeatureType::DeviceCornerSampledImageFeaturesNV(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceCornerSampledImageFeaturesNV {
    CornerSampledImage,
}
impl ToPhysicalFeature for DeviceCornerSampledImageFeaturesNV {
    type PhysicalDeviceFeatureTy = PhysicalDeviceCornerSampledImageFeaturesNV;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceCornerSampledImageFeaturesNV::CornerSampledImage => {
                PhysicalDeviceCornerSampledImageFeaturesNV::CornerSampledImage
            }
        }
    }
}
impl From<DeviceCornerSampledImageFeaturesNV> for DeviceFeature {
    fn from(feature: DeviceCornerSampledImageFeaturesNV) -> Self {
        DeviceFeature::DeviceCornerSampledImageFeaturesNV(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceCornerSampledImageFeaturesNV {
    type SubFeatureEnumTy = PhysicalDeviceCornerSampledImageFeaturesNV;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.corner_sampled_image != 0 {
            set.insert(PhysicalDeviceCornerSampledImageFeaturesNV::CornerSampledImage);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceCornerSampledImageFeaturesNV {
    type VkStruct = ash::vk::PhysicalDeviceCornerSampledImageFeaturesNV;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::CornerSampledImage => vk_struct.corner_sampled_image = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceDepthClipEnableFeaturesEXT {
    DepthClipEnable,
}
impl const From<PhysicalDeviceDepthClipEnableFeaturesEXT> for FeatureType {
    fn from(feature: PhysicalDeviceDepthClipEnableFeaturesEXT) -> Self {
        FeatureType::DeviceDepthClipEnableFeaturesEXT(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceDepthClipEnableFeaturesEXT {
    DepthClipEnable,
}
impl ToPhysicalFeature for DeviceDepthClipEnableFeaturesEXT {
    type PhysicalDeviceFeatureTy = PhysicalDeviceDepthClipEnableFeaturesEXT;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceDepthClipEnableFeaturesEXT::DepthClipEnable => {
                PhysicalDeviceDepthClipEnableFeaturesEXT::DepthClipEnable
            }
        }
    }
}
impl From<DeviceDepthClipEnableFeaturesEXT> for DeviceFeature {
    fn from(feature: DeviceDepthClipEnableFeaturesEXT) -> Self {
        DeviceFeature::DeviceDepthClipEnableFeaturesEXT(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceDepthClipEnableFeaturesEXT {
    type SubFeatureEnumTy = PhysicalDeviceDepthClipEnableFeaturesEXT;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.depth_clip_enable != 0 {
            set.insert(PhysicalDeviceDepthClipEnableFeaturesEXT::DepthClipEnable);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceDepthClipEnableFeaturesEXT {
    type VkStruct = ash::vk::PhysicalDeviceDepthClipEnableFeaturesEXT;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::DepthClipEnable => vk_struct.depth_clip_enable = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceShaderDrawParametersFeatures {
    ShaderDrawParameters,
}
impl const From<PhysicalDeviceShaderDrawParametersFeatures> for FeatureType {
    fn from(feature: PhysicalDeviceShaderDrawParametersFeatures) -> Self {
        FeatureType::DeviceShaderDrawParametersFeatures(feature)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceShaderDrawParametersFeatures {
    ShaderDrawParameters,
}
impl ToPhysicalFeature for DeviceShaderDrawParametersFeatures {
    type PhysicalDeviceFeatureTy = PhysicalDeviceShaderDrawParametersFeatures;
    fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
        match self {
            DeviceShaderDrawParametersFeatures::ShaderDrawParameters => {
                PhysicalDeviceShaderDrawParametersFeatures::ShaderDrawParameters
            }
        }
    }
}
impl From<DeviceShaderDrawParametersFeatures> for DeviceFeature {
    fn from(feature: DeviceShaderDrawParametersFeatures) -> Self {
        DeviceFeature::DeviceShaderDrawParametersFeatures(feature)
    }
}
impl VkDeviceFeature for ash::vk::PhysicalDeviceShaderDrawParametersFeatures {
    type SubFeatureEnumTy = PhysicalDeviceShaderDrawParametersFeatures;
    fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
        let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
        if self.shader_draw_parameters != 0 {
            set.insert(PhysicalDeviceShaderDrawParametersFeatures::ShaderDrawParameters);
        }
        set
    }
}
impl SubPhysicalFeature for PhysicalDeviceShaderDrawParametersFeatures {
    type VkStruct = ash::vk::PhysicalDeviceShaderDrawParametersFeatures;
    fn register(&self, vk_struct: &mut Self::VkStruct) {
        match self {
            Self::ShaderDrawParameters => vk_struct.shader_draw_parameters = 1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FeatureType {
    DeviceShaderAtomicFloat2FeaturesEXT(PhysicalDeviceShaderAtomicFloat2FeaturesEXT),
    DeviceCoverageReductionModeFeaturesNV(PhysicalDeviceCoverageReductionModeFeaturesNV),
    DeviceSynchronization2Features(PhysicalDeviceSynchronization2Features),
    DeviceVertexInputDynamicStateFeaturesEXT(PhysicalDeviceVertexInputDynamicStateFeaturesEXT),
    DeviceMultiviewFeatures(PhysicalDeviceMultiviewFeatures),
    DeviceShaderIntegerFunctions2FeaturesINTEL(PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL),
    DevicePrivateDataFeatures(PhysicalDevicePrivateDataFeatures),
    DevicePipelineCreationCacheControlFeatures(PhysicalDevicePipelineCreationCacheControlFeatures),
    DeviceMutableDescriptorTypeFeaturesVALVE(PhysicalDeviceMutableDescriptorTypeFeaturesVALVE),
    DeviceTimelineSemaphoreFeatures(PhysicalDeviceTimelineSemaphoreFeatures),
    DeviceRayTracingPipelineFeaturesKHR(PhysicalDeviceRayTracingPipelineFeaturesKHR),
    DeviceVulkanMemoryModelFeatures(PhysicalDeviceVulkanMemoryModelFeatures),
    DeviceExtendedDynamicState2FeaturesEXT(PhysicalDeviceExtendedDynamicState2FeaturesEXT),
    DeviceShaderImageAtomicInt64FeaturesEXT(PhysicalDeviceShaderImageAtomicInt64FeaturesEXT),
    DeviceInheritedViewportScissorFeaturesNV(PhysicalDeviceInheritedViewportScissorFeaturesNV),
    DeviceYcbcr2Plane444FormatsFeaturesEXT(PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT),
    DevicePresentWaitFeaturesKHR(PhysicalDevicePresentWaitFeaturesKHR),
    DeviceComputeShaderDerivativesFeaturesNV(PhysicalDeviceComputeShaderDerivativesFeaturesNV),
    DeviceVulkan13Features(PhysicalDeviceVulkan13Features),
    DeviceExternalMemoryRDMAFeaturesNV(PhysicalDeviceExternalMemoryRDMAFeaturesNV),
    DeviceProtectedMemoryFeatures(PhysicalDeviceProtectedMemoryFeatures),
    DeviceDynamicRenderingFeatures(PhysicalDeviceDynamicRenderingFeatures),
    DeviceFragmentDensityMapFeaturesEXT(PhysicalDeviceFragmentDensityMapFeaturesEXT),
    DeviceShaderFloat16Int8Features(PhysicalDeviceShaderFloat16Int8Features),
    DeviceInvocationMaskFeaturesHUAWEI(PhysicalDeviceInvocationMaskFeaturesHUAWEI),
    DeviceRayQueryFeaturesKHR(PhysicalDeviceRayQueryFeaturesKHR),
    DeviceSubpassShadingFeaturesHUAWEI(PhysicalDeviceSubpassShadingFeaturesHUAWEI),
    DeviceShaderSubgroupExtendedTypesFeatures(PhysicalDeviceShaderSubgroupExtendedTypesFeatures),
    DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR(
        PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR,
    ),
    DeviceRayTracingMotionBlurFeaturesNV(PhysicalDeviceRayTracingMotionBlurFeaturesNV),
    DeviceMaintenance4Features(PhysicalDeviceMaintenance4Features),
    Device4444FormatsFeaturesEXT(PhysicalDevice4444FormatsFeaturesEXT),
    DevicePrimitiveTopologyListRestartFeaturesEXT(
        PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT,
    ),
    DeviceShaderIntegerDotProductFeatures(PhysicalDeviceShaderIntegerDotProductFeatures),
    DeviceInlineUniformBlockFeatures(PhysicalDeviceInlineUniformBlockFeatures),
    DeviceSeparateDepthStencilLayoutsFeatures(PhysicalDeviceSeparateDepthStencilLayoutsFeatures),
    DeviceRGBA10X6FormatsFeaturesEXT(PhysicalDeviceRGBA10X6FormatsFeaturesEXT),
    DeviceShadingRateImageFeaturesNV(PhysicalDeviceShadingRateImageFeaturesNV),
    DeviceDescriptorSetHostMappingFeaturesVALVE(
        PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE,
    ),
    DeviceBufferDeviceAddressFeaturesEXT(PhysicalDeviceBufferDeviceAddressFeaturesEXT),
    DevicePipelineExecutablePropertiesFeaturesKHR(
        PhysicalDevicePipelineExecutablePropertiesFeaturesKHR,
    ),
    DeviceHostQueryResetFeatures(PhysicalDeviceHostQueryResetFeatures),
    DeviceRasterizationOrderAttachmentAccessFeaturesARM(
        PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM,
    ),
    DeviceFragmentDensityMapOffsetFeaturesQCOM(PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM),
    DeviceTransformFeedbackFeaturesEXT(PhysicalDeviceTransformFeedbackFeaturesEXT),
    DevicePerformanceQueryFeaturesKHR(PhysicalDevicePerformanceQueryFeaturesKHR),
    DeviceDedicatedAllocationImageAliasingFeaturesNV(
        PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV,
    ),
    DeviceImageViewMinLodFeaturesEXT(PhysicalDeviceImageViewMinLodFeaturesEXT),
    DeviceShaderSubgroupUniformControlFlowFeaturesKHR(
        PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR,
    ),
    DeviceFeatures(PhysicalDeviceFeatures),
    DeviceIndexTypeUint8FeaturesEXT(PhysicalDeviceIndexTypeUint8FeaturesEXT),
    DeviceVulkan12Features(PhysicalDeviceVulkan12Features),
    DeviceDiagnosticsConfigFeaturesNV(PhysicalDeviceDiagnosticsConfigFeaturesNV),
    DeviceFragmentShadingRateFeaturesKHR(PhysicalDeviceFragmentShadingRateFeaturesKHR),
    DeviceLineRasterizationFeaturesEXT(PhysicalDeviceLineRasterizationFeaturesEXT),
    DeviceProvokingVertexFeaturesEXT(PhysicalDeviceProvokingVertexFeaturesEXT),
    DeviceExtendedDynamicStateFeaturesEXT(PhysicalDeviceExtendedDynamicStateFeaturesEXT),
    DevicePresentIdFeaturesKHR(PhysicalDevicePresentIdFeaturesKHR),
    DeviceDeviceMemoryReportFeaturesEXT(PhysicalDeviceDeviceMemoryReportFeaturesEXT),
    Device16BitStorageFeatures(PhysicalDevice16BitStorageFeatures),
    DeviceBufferDeviceAddressFeatures(PhysicalDeviceBufferDeviceAddressFeatures),
    DeviceCoherentMemoryFeaturesAMD(PhysicalDeviceCoherentMemoryFeaturesAMD),
    DeviceColorWriteEnableFeaturesEXT(PhysicalDeviceColorWriteEnableFeaturesEXT),
    DeviceMultiDrawFeaturesEXT(PhysicalDeviceMultiDrawFeaturesEXT),
    DeviceSamplerYcbcrConversionFeatures(PhysicalDeviceSamplerYcbcrConversionFeatures),
    DeviceCustomBorderColorFeaturesEXT(PhysicalDeviceCustomBorderColorFeaturesEXT),
    DeviceBorderColorSwizzleFeaturesEXT(PhysicalDeviceBorderColorSwizzleFeaturesEXT),
    DeviceAccelerationStructureFeaturesKHR(PhysicalDeviceAccelerationStructureFeaturesKHR),
    DeviceBlendOperationAdvancedFeaturesEXT(PhysicalDeviceBlendOperationAdvancedFeaturesEXT),
    DeviceLinearColorAttachmentFeaturesNV(PhysicalDeviceLinearColorAttachmentFeaturesNV),
    DeviceConditionalRenderingFeaturesEXT(PhysicalDeviceConditionalRenderingFeaturesEXT),
    DeviceYcbcrImageArraysFeaturesEXT(PhysicalDeviceYcbcrImageArraysFeaturesEXT),
    DeviceImageRobustnessFeatures(PhysicalDeviceImageRobustnessFeatures),
    Device8BitStorageFeatures(PhysicalDevice8BitStorageFeatures),
    DeviceVulkan11Features(PhysicalDeviceVulkan11Features),
    DeviceDepthClipControlFeaturesEXT(PhysicalDeviceDepthClipControlFeaturesEXT),
    DeviceASTCDecodeFeaturesEXT(PhysicalDeviceASTCDecodeFeaturesEXT),
    DeviceCooperativeMatrixFeaturesNV(PhysicalDeviceCooperativeMatrixFeaturesNV),
    DeviceVariablePointersFeatures(PhysicalDeviceVariablePointersFeatures),
    DevicePortabilitySubsetFeaturesKHR(PhysicalDevicePortabilitySubsetFeaturesKHR),
    DeviceShaderImageFootprintFeaturesNV(PhysicalDeviceShaderImageFootprintFeaturesNV),
    DeviceTexelBufferAlignmentFeaturesEXT(PhysicalDeviceTexelBufferAlignmentFeaturesEXT),
    DeviceImagelessFramebufferFeatures(PhysicalDeviceImagelessFramebufferFeatures),
    DeviceShaderSMBuiltinsFeaturesNV(PhysicalDeviceShaderSMBuiltinsFeaturesNV),
    DeviceFragmentShaderBarycentricFeaturesNV(PhysicalDeviceFragmentShaderBarycentricFeaturesNV),
    DeviceTextureCompressionASTCHDRFeatures(PhysicalDeviceTextureCompressionASTCHDRFeatures),
    DevicePageableDeviceLocalMemoryFeaturesEXT(PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT),
    DeviceZeroInitializeWorkgroupMemoryFeatures(
        PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures,
    ),
    DeviceRobustness2FeaturesEXT(PhysicalDeviceRobustness2FeaturesEXT),
    DeviceShaderTerminateInvocationFeatures(PhysicalDeviceShaderTerminateInvocationFeatures),
    DeviceGlobalPriorityQueryFeaturesKHR(PhysicalDeviceGlobalPriorityQueryFeaturesKHR),
    DeviceShaderAtomicFloatFeaturesEXT(PhysicalDeviceShaderAtomicFloatFeaturesEXT),
    DeviceVertexAttributeDivisorFeaturesEXT(PhysicalDeviceVertexAttributeDivisorFeaturesEXT),
    DeviceDescriptorIndexingFeatures(PhysicalDeviceDescriptorIndexingFeatures),
    DeviceFragmentDensityMap2FeaturesEXT(PhysicalDeviceFragmentDensityMap2FeaturesEXT),
    DeviceUniformBufferStandardLayoutFeatures(PhysicalDeviceUniformBufferStandardLayoutFeatures),
    DeviceMemoryPriorityFeaturesEXT(PhysicalDeviceMemoryPriorityFeaturesEXT),
    DeviceScalarBlockLayoutFeatures(PhysicalDeviceScalarBlockLayoutFeatures),
    DeviceShaderDemoteToHelperInvocationFeatures(
        PhysicalDeviceShaderDemoteToHelperInvocationFeatures,
    ),
    DeviceSubgroupSizeControlFeatures(PhysicalDeviceSubgroupSizeControlFeatures),
    DeviceShaderAtomicInt64Features(PhysicalDeviceShaderAtomicInt64Features),
    DeviceMeshShaderFeaturesNV(PhysicalDeviceMeshShaderFeaturesNV),
    DeviceShaderClockFeaturesKHR(PhysicalDeviceShaderClockFeaturesKHR),
    DeviceFragmentShaderInterlockFeaturesEXT(PhysicalDeviceFragmentShaderInterlockFeaturesEXT),
    DeviceRepresentativeFragmentTestFeaturesNV(PhysicalDeviceRepresentativeFragmentTestFeaturesNV),
    DeviceFragmentShadingRateEnumsFeaturesNV(PhysicalDeviceFragmentShadingRateEnumsFeaturesNV),
    DeviceDeviceGeneratedCommandsFeaturesNV(PhysicalDeviceDeviceGeneratedCommandsFeaturesNV),
    DeviceExclusiveScissorFeaturesNV(PhysicalDeviceExclusiveScissorFeaturesNV),
    DeviceCornerSampledImageFeaturesNV(PhysicalDeviceCornerSampledImageFeaturesNV),
    DeviceDepthClipEnableFeaturesEXT(PhysicalDeviceDepthClipEnableFeaturesEXT),
    DeviceShaderDrawParametersFeatures(PhysicalDeviceShaderDrawParametersFeatures),
}
#[derive(Clone, PartialEq, Eq)]
pub enum DeviceFeature {
    DeviceShaderAtomicFloat2FeaturesEXT(DeviceShaderAtomicFloat2FeaturesEXT),
    DeviceCoverageReductionModeFeaturesNV(DeviceCoverageReductionModeFeaturesNV),
    DeviceSynchronization2Features(DeviceSynchronization2Features),
    DeviceVertexInputDynamicStateFeaturesEXT(DeviceVertexInputDynamicStateFeaturesEXT),
    DeviceMultiviewFeatures(DeviceMultiviewFeatures),
    DeviceShaderIntegerFunctions2FeaturesINTEL(DeviceShaderIntegerFunctions2FeaturesINTEL),
    DevicePrivateDataFeatures(DevicePrivateDataFeatures),
    DevicePipelineCreationCacheControlFeatures(DevicePipelineCreationCacheControlFeatures),
    DeviceMutableDescriptorTypeFeaturesVALVE(DeviceMutableDescriptorTypeFeaturesVALVE),
    DeviceTimelineSemaphoreFeatures(DeviceTimelineSemaphoreFeatures),
    DeviceRayTracingPipelineFeaturesKHR(DeviceRayTracingPipelineFeaturesKHR),
    DeviceVulkanMemoryModelFeatures(DeviceVulkanMemoryModelFeatures),
    DeviceExtendedDynamicState2FeaturesEXT(DeviceExtendedDynamicState2FeaturesEXT),
    DeviceShaderImageAtomicInt64FeaturesEXT(DeviceShaderImageAtomicInt64FeaturesEXT),
    DeviceInheritedViewportScissorFeaturesNV(DeviceInheritedViewportScissorFeaturesNV),
    DeviceYcbcr2Plane444FormatsFeaturesEXT(DeviceYcbcr2Plane444FormatsFeaturesEXT),
    DevicePresentWaitFeaturesKHR(DevicePresentWaitFeaturesKHR),
    DeviceComputeShaderDerivativesFeaturesNV(DeviceComputeShaderDerivativesFeaturesNV),
    DeviceVulkan13Features(DeviceVulkan13Features),
    DeviceExternalMemoryRDMAFeaturesNV(DeviceExternalMemoryRDMAFeaturesNV),
    DeviceProtectedMemoryFeatures(DeviceProtectedMemoryFeatures),
    DeviceDynamicRenderingFeatures(DeviceDynamicRenderingFeatures),
    DeviceFragmentDensityMapFeaturesEXT(DeviceFragmentDensityMapFeaturesEXT),
    DeviceShaderFloat16Int8Features(DeviceShaderFloat16Int8Features),
    DeviceInvocationMaskFeaturesHUAWEI(DeviceInvocationMaskFeaturesHUAWEI),
    DeviceRayQueryFeaturesKHR(DeviceRayQueryFeaturesKHR),
    DeviceSubpassShadingFeaturesHUAWEI(DeviceSubpassShadingFeaturesHUAWEI),
    DeviceShaderSubgroupExtendedTypesFeatures(DeviceShaderSubgroupExtendedTypesFeatures),
    DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR(DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR),
    DeviceRayTracingMotionBlurFeaturesNV(DeviceRayTracingMotionBlurFeaturesNV),
    DeviceMaintenance4Features(DeviceMaintenance4Features),
    Device4444FormatsFeaturesEXT(Device4444FormatsFeaturesEXT),
    DevicePrimitiveTopologyListRestartFeaturesEXT(DevicePrimitiveTopologyListRestartFeaturesEXT),
    DeviceShaderIntegerDotProductFeatures(DeviceShaderIntegerDotProductFeatures),
    DeviceInlineUniformBlockFeatures(DeviceInlineUniformBlockFeatures),
    DeviceSeparateDepthStencilLayoutsFeatures(DeviceSeparateDepthStencilLayoutsFeatures),
    DeviceRGBA10X6FormatsFeaturesEXT(DeviceRGBA10X6FormatsFeaturesEXT),
    DeviceShadingRateImageFeaturesNV(DeviceShadingRateImageFeaturesNV),
    DeviceDescriptorSetHostMappingFeaturesVALVE(DeviceDescriptorSetHostMappingFeaturesVALVE),
    DeviceBufferDeviceAddressFeaturesEXT(DeviceBufferDeviceAddressFeaturesEXT),
    DevicePipelineExecutablePropertiesFeaturesKHR(DevicePipelineExecutablePropertiesFeaturesKHR),
    DeviceHostQueryResetFeatures(DeviceHostQueryResetFeatures),
    DeviceRasterizationOrderAttachmentAccessFeaturesARM(
        DeviceRasterizationOrderAttachmentAccessFeaturesARM,
    ),
    DeviceFragmentDensityMapOffsetFeaturesQCOM(DeviceFragmentDensityMapOffsetFeaturesQCOM),
    DeviceTransformFeedbackFeaturesEXT(DeviceTransformFeedbackFeaturesEXT),
    DevicePerformanceQueryFeaturesKHR(DevicePerformanceQueryFeaturesKHR),
    DeviceDedicatedAllocationImageAliasingFeaturesNV(
        DeviceDedicatedAllocationImageAliasingFeaturesNV,
    ),
    DeviceImageViewMinLodFeaturesEXT(DeviceImageViewMinLodFeaturesEXT),
    DeviceShaderSubgroupUniformControlFlowFeaturesKHR(
        DeviceShaderSubgroupUniformControlFlowFeaturesKHR,
    ),
    DeviceFeatures(DeviceFeatures),
    DeviceIndexTypeUint8FeaturesEXT(DeviceIndexTypeUint8FeaturesEXT),
    DeviceVulkan12Features(DeviceVulkan12Features),
    DeviceDiagnosticsConfigFeaturesNV(DeviceDiagnosticsConfigFeaturesNV),
    DeviceFragmentShadingRateFeaturesKHR(DeviceFragmentShadingRateFeaturesKHR),
    DeviceLineRasterizationFeaturesEXT(DeviceLineRasterizationFeaturesEXT),
    DeviceProvokingVertexFeaturesEXT(DeviceProvokingVertexFeaturesEXT),
    DeviceExtendedDynamicStateFeaturesEXT(DeviceExtendedDynamicStateFeaturesEXT),
    DevicePresentIdFeaturesKHR(DevicePresentIdFeaturesKHR),
    DeviceDeviceMemoryReportFeaturesEXT(DeviceDeviceMemoryReportFeaturesEXT),
    Device16BitStorageFeatures(Device16BitStorageFeatures),
    DeviceBufferDeviceAddressFeatures(DeviceBufferDeviceAddressFeatures),
    DeviceCoherentMemoryFeaturesAMD(DeviceCoherentMemoryFeaturesAMD),
    DeviceColorWriteEnableFeaturesEXT(DeviceColorWriteEnableFeaturesEXT),
    DeviceMultiDrawFeaturesEXT(DeviceMultiDrawFeaturesEXT),
    DeviceSamplerYcbcrConversionFeatures(DeviceSamplerYcbcrConversionFeatures),
    DeviceCustomBorderColorFeaturesEXT(DeviceCustomBorderColorFeaturesEXT),
    DeviceBorderColorSwizzleFeaturesEXT(DeviceBorderColorSwizzleFeaturesEXT),
    DeviceAccelerationStructureFeaturesKHR(DeviceAccelerationStructureFeaturesKHR),
    DeviceBlendOperationAdvancedFeaturesEXT(DeviceBlendOperationAdvancedFeaturesEXT),
    DeviceLinearColorAttachmentFeaturesNV(DeviceLinearColorAttachmentFeaturesNV),
    DeviceConditionalRenderingFeaturesEXT(DeviceConditionalRenderingFeaturesEXT),
    DeviceYcbcrImageArraysFeaturesEXT(DeviceYcbcrImageArraysFeaturesEXT),
    DeviceImageRobustnessFeatures(DeviceImageRobustnessFeatures),
    Device8BitStorageFeatures(Device8BitStorageFeatures),
    DeviceVulkan11Features(DeviceVulkan11Features),
    DeviceDepthClipControlFeaturesEXT(DeviceDepthClipControlFeaturesEXT),
    DeviceASTCDecodeFeaturesEXT(DeviceASTCDecodeFeaturesEXT),
    DeviceCooperativeMatrixFeaturesNV(DeviceCooperativeMatrixFeaturesNV),
    DeviceVariablePointersFeatures(DeviceVariablePointersFeatures),
    DevicePortabilitySubsetFeaturesKHR(DevicePortabilitySubsetFeaturesKHR),
    DeviceShaderImageFootprintFeaturesNV(DeviceShaderImageFootprintFeaturesNV),
    DeviceTexelBufferAlignmentFeaturesEXT(DeviceTexelBufferAlignmentFeaturesEXT),
    DeviceImagelessFramebufferFeatures(DeviceImagelessFramebufferFeatures),
    DeviceShaderSMBuiltinsFeaturesNV(DeviceShaderSMBuiltinsFeaturesNV),
    DeviceFragmentShaderBarycentricFeaturesNV(DeviceFragmentShaderBarycentricFeaturesNV),
    DeviceTextureCompressionASTCHDRFeatures(DeviceTextureCompressionASTCHDRFeatures),
    DevicePageableDeviceLocalMemoryFeaturesEXT(DevicePageableDeviceLocalMemoryFeaturesEXT),
    DeviceZeroInitializeWorkgroupMemoryFeatures(DeviceZeroInitializeWorkgroupMemoryFeatures),
    DeviceRobustness2FeaturesEXT(DeviceRobustness2FeaturesEXT),
    DeviceShaderTerminateInvocationFeatures(DeviceShaderTerminateInvocationFeatures),
    DeviceGlobalPriorityQueryFeaturesKHR(DeviceGlobalPriorityQueryFeaturesKHR),
    DeviceShaderAtomicFloatFeaturesEXT(DeviceShaderAtomicFloatFeaturesEXT),
    DeviceVertexAttributeDivisorFeaturesEXT(DeviceVertexAttributeDivisorFeaturesEXT),
    DeviceDescriptorIndexingFeatures(DeviceDescriptorIndexingFeatures),
    DeviceFragmentDensityMap2FeaturesEXT(DeviceFragmentDensityMap2FeaturesEXT),
    DeviceUniformBufferStandardLayoutFeatures(DeviceUniformBufferStandardLayoutFeatures),
    DeviceMemoryPriorityFeaturesEXT(DeviceMemoryPriorityFeaturesEXT),
    DeviceScalarBlockLayoutFeatures(DeviceScalarBlockLayoutFeatures),
    DeviceShaderDemoteToHelperInvocationFeatures(DeviceShaderDemoteToHelperInvocationFeatures),
    DeviceSubgroupSizeControlFeatures(DeviceSubgroupSizeControlFeatures),
    DeviceShaderAtomicInt64Features(DeviceShaderAtomicInt64Features),
    DeviceMeshShaderFeaturesNV(DeviceMeshShaderFeaturesNV),
    DeviceShaderClockFeaturesKHR(DeviceShaderClockFeaturesKHR),
    DeviceFragmentShaderInterlockFeaturesEXT(DeviceFragmentShaderInterlockFeaturesEXT),
    DeviceRepresentativeFragmentTestFeaturesNV(DeviceRepresentativeFragmentTestFeaturesNV),
    DeviceFragmentShadingRateEnumsFeaturesNV(DeviceFragmentShadingRateEnumsFeaturesNV),
    DeviceDeviceGeneratedCommandsFeaturesNV(DeviceDeviceGeneratedCommandsFeaturesNV),
    DeviceExclusiveScissorFeaturesNV(DeviceExclusiveScissorFeaturesNV),
    DeviceCornerSampledImageFeaturesNV(DeviceCornerSampledImageFeaturesNV),
    DeviceDepthClipEnableFeaturesEXT(DeviceDepthClipEnableFeaturesEXT),
    DeviceShaderDrawParametersFeatures(DeviceShaderDrawParametersFeatures),
}
#[allow(dead_code)]
pub(crate) union VkFeatureUnion {
    pub(crate) vk_physical_device_shader_atomic_float2_features_ext:
        ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT,
    pub(crate) vk_physical_device_coverage_reduction_mode_features_nv:
        ash::vk::PhysicalDeviceCoverageReductionModeFeaturesNV,
    pub(crate) vk_physical_device_synchronization2_features:
        ash::vk::PhysicalDeviceSynchronization2Features,
    pub(crate) vk_physical_device_vertex_input_dynamic_state_features_ext:
        ash::vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT,
    pub(crate) vk_physical_device_multiview_features: ash::vk::PhysicalDeviceMultiviewFeatures,
    pub(crate) vk_physical_device_shader_integer_functions2_features_intel:
        ash::vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL,
    pub(crate) vk_physical_device_private_data_features: ash::vk::PhysicalDevicePrivateDataFeatures,
    pub(crate) vk_physical_device_pipeline_creation_cache_control_features:
        ash::vk::PhysicalDevicePipelineCreationCacheControlFeatures,
    pub(crate) vk_physical_device_mutable_descriptor_type_features_valve:
        ash::vk::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE,
    pub(crate) vk_physical_device_timeline_semaphore_features:
        ash::vk::PhysicalDeviceTimelineSemaphoreFeatures,
    pub(crate) vk_physical_device_ray_tracing_pipeline_features_khr:
        ash::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR,
    pub(crate) vk_physical_device_vulkan_memory_model_features:
        ash::vk::PhysicalDeviceVulkanMemoryModelFeatures,
    pub(crate) vk_physical_device_extended_dynamic_state2_features_ext:
        ash::vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT,
    pub(crate) vk_physical_device_shader_image_atomic_int64_features_ext:
        ash::vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT,
    pub(crate) vk_physical_device_inherited_viewport_scissor_features_nv:
        ash::vk::PhysicalDeviceInheritedViewportScissorFeaturesNV,
    pub(crate) vk_physical_device_ycbcr2_plane444_formats_features_ext:
        ash::vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT,
    pub(crate) vk_physical_device_present_wait_features_khr:
        ash::vk::PhysicalDevicePresentWaitFeaturesKHR,
    pub(crate) vk_physical_device_compute_shader_derivatives_features_nv:
        ash::vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV,
    pub(crate) vk_physical_device_vulkan13_features: ash::vk::PhysicalDeviceVulkan13Features,
    pub(crate) vk_physical_device_external_memory_rdma_features_nv:
        ash::vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV,
    pub(crate) vk_physical_device_protected_memory_features:
        ash::vk::PhysicalDeviceProtectedMemoryFeatures,
    pub(crate) vk_physical_device_dynamic_rendering_features:
        ash::vk::PhysicalDeviceDynamicRenderingFeatures,
    pub(crate) vk_physical_device_fragment_density_map_features_ext:
        ash::vk::PhysicalDeviceFragmentDensityMapFeaturesEXT,
    pub(crate) vk_physical_device_shader_float16_int8_features:
        ash::vk::PhysicalDeviceShaderFloat16Int8Features,
    pub(crate) vk_physical_device_invocation_mask_features_huawei:
        ash::vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI,
    pub(crate) vk_physical_device_ray_query_features_khr:
        ash::vk::PhysicalDeviceRayQueryFeaturesKHR,
    pub(crate) vk_physical_device_subpass_shading_features_huawei:
        ash::vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI,
    pub(crate) vk_physical_device_shader_subgroup_extended_types_features:
        ash::vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures,
    pub(crate) vk_physical_device_workgroup_memory_explicit_layout_features_khr:
        ash::vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR,
    pub(crate) vk_physical_device_ray_tracing_motion_blur_features_nv:
        ash::vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV,
    pub(crate) vk_physical_device_maintenance4_features:
        ash::vk::PhysicalDeviceMaintenance4Features,
    pub(crate) vk_physical_device4444_formats_features_ext:
        ash::vk::PhysicalDevice4444FormatsFeaturesEXT,
    pub(crate) vk_physical_device_primitive_topology_list_restart_features_ext:
        ash::vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT,
    pub(crate) vk_physical_device_shader_integer_dot_product_features:
        ash::vk::PhysicalDeviceShaderIntegerDotProductFeatures,
    pub(crate) vk_physical_device_inline_uniform_block_features:
        ash::vk::PhysicalDeviceInlineUniformBlockFeatures,
    pub(crate) vk_physical_device_separate_depth_stencil_layouts_features:
        ash::vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
    pub(crate) vk_physical_device_rgba10x6_formats_features_ext:
        ash::vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT,
    pub(crate) vk_physical_device_shading_rate_image_features_nv:
        ash::vk::PhysicalDeviceShadingRateImageFeaturesNV,
    pub(crate) vk_physical_device_descriptor_set_host_mapping_features_valve:
        ash::vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE,
    pub(crate) vk_physical_device_buffer_device_address_features_ext:
        ash::vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT,
    pub(crate) vk_physical_device_pipeline_executable_properties_features_khr:
        ash::vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR,
    pub(crate) vk_physical_device_host_query_reset_features:
        ash::vk::PhysicalDeviceHostQueryResetFeatures,
    pub(crate) vk_physical_device_rasterization_order_attachment_access_features_arm:
        ash::vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM,
    pub(crate) vk_physical_device_fragment_density_map_offset_features_qcom:
        ash::vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM,
    pub(crate) vk_physical_device_transform_feedback_features_ext:
        ash::vk::PhysicalDeviceTransformFeedbackFeaturesEXT,
    pub(crate) vk_physical_device_performance_query_features_khr:
        ash::vk::PhysicalDevicePerformanceQueryFeaturesKHR,
    pub(crate) vk_physical_device_dedicated_allocation_image_aliasing_features_nv:
        ash::vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV,
    pub(crate) vk_physical_device_image_view_min_lod_features_ext:
        ash::vk::PhysicalDeviceImageViewMinLodFeaturesEXT,
    pub(crate) vk_physical_device_shader_subgroup_uniform_control_flow_features_khr:
        ash::vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR,
    pub(crate) vk_physical_device_features: ash::vk::PhysicalDeviceFeatures,
    pub(crate) vk_physical_device_index_type_uint8_features_ext:
        ash::vk::PhysicalDeviceIndexTypeUint8FeaturesEXT,
    pub(crate) vk_physical_device_vulkan12_features: ash::vk::PhysicalDeviceVulkan12Features,
    pub(crate) vk_physical_device_diagnostics_config_features_nv:
        ash::vk::PhysicalDeviceDiagnosticsConfigFeaturesNV,
    pub(crate) vk_physical_device_fragment_shading_rate_features_khr:
        ash::vk::PhysicalDeviceFragmentShadingRateFeaturesKHR,
    pub(crate) vk_physical_device_line_rasterization_features_ext:
        ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT,
    pub(crate) vk_physical_device_provoking_vertex_features_ext:
        ash::vk::PhysicalDeviceProvokingVertexFeaturesEXT,
    pub(crate) vk_physical_device_extended_dynamic_state_features_ext:
        ash::vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT,
    pub(crate) vk_physical_device_present_id_features_khr:
        ash::vk::PhysicalDevicePresentIdFeaturesKHR,
    pub(crate) vk_physical_device_device_memory_report_features_ext:
        ash::vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT,
    pub(crate) vk_physical_device16_bit_storage_features:
        ash::vk::PhysicalDevice16BitStorageFeatures,
    pub(crate) vk_physical_device_buffer_device_address_features:
        ash::vk::PhysicalDeviceBufferDeviceAddressFeatures,
    pub(crate) vk_physical_device_coherent_memory_features_amd:
        ash::vk::PhysicalDeviceCoherentMemoryFeaturesAMD,
    pub(crate) vk_physical_device_color_write_enable_features_ext:
        ash::vk::PhysicalDeviceColorWriteEnableFeaturesEXT,
    pub(crate) vk_physical_device_multi_draw_features_ext:
        ash::vk::PhysicalDeviceMultiDrawFeaturesEXT,
    pub(crate) vk_physical_device_sampler_ycbcr_conversion_features:
        ash::vk::PhysicalDeviceSamplerYcbcrConversionFeatures,
    pub(crate) vk_physical_device_custom_border_color_features_ext:
        ash::vk::PhysicalDeviceCustomBorderColorFeaturesEXT,
    pub(crate) vk_physical_device_border_color_swizzle_features_ext:
        ash::vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT,
    pub(crate) vk_physical_device_acceleration_structure_features_khr:
        ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR,
    pub(crate) vk_physical_device_blend_operation_advanced_features_ext:
        ash::vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT,
    pub(crate) vk_physical_device_linear_color_attachment_features_nv:
        ash::vk::PhysicalDeviceLinearColorAttachmentFeaturesNV,
    pub(crate) vk_physical_device_conditional_rendering_features_ext:
        ash::vk::PhysicalDeviceConditionalRenderingFeaturesEXT,
    pub(crate) vk_physical_device_ycbcr_image_arrays_features_ext:
        ash::vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT,
    pub(crate) vk_physical_device_image_robustness_features:
        ash::vk::PhysicalDeviceImageRobustnessFeatures,
    pub(crate) vk_physical_device8_bit_storage_features: ash::vk::PhysicalDevice8BitStorageFeatures,
    pub(crate) vk_physical_device_vulkan11_features: ash::vk::PhysicalDeviceVulkan11Features,
    pub(crate) vk_physical_device_depth_clip_control_features_ext:
        ash::vk::PhysicalDeviceDepthClipControlFeaturesEXT,
    pub(crate) vk_physical_device_astc_decode_features_ext:
        ash::vk::PhysicalDeviceASTCDecodeFeaturesEXT,
    pub(crate) vk_physical_device_cooperative_matrix_features_nv:
        ash::vk::PhysicalDeviceCooperativeMatrixFeaturesNV,
    pub(crate) vk_physical_device_variable_pointers_features:
        ash::vk::PhysicalDeviceVariablePointersFeatures,
    pub(crate) vk_physical_device_portability_subset_features_khr:
        ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR,
    pub(crate) vk_physical_device_shader_image_footprint_features_nv:
        ash::vk::PhysicalDeviceShaderImageFootprintFeaturesNV,
    pub(crate) vk_physical_device_texel_buffer_alignment_features_ext:
        ash::vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT,
    pub(crate) vk_physical_device_imageless_framebuffer_features:
        ash::vk::PhysicalDeviceImagelessFramebufferFeatures,
    pub(crate) vk_physical_device_shader_sm_builtins_features_nv:
        ash::vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV,
    pub(crate) vk_physical_device_fragment_shader_barycentric_features_nv:
        ash::vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV,
    pub(crate) vk_physical_device_texture_compression_astchdr_features:
        ash::vk::PhysicalDeviceTextureCompressionASTCHDRFeatures,
    pub(crate) vk_physical_device_pageable_device_local_memory_features_ext:
        ash::vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT,
    pub(crate) vk_physical_device_zero_initialize_workgroup_memory_features:
        ash::vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures,
    pub(crate) vk_physical_device_robustness2_features_ext:
        ash::vk::PhysicalDeviceRobustness2FeaturesEXT,
    pub(crate) vk_physical_device_shader_terminate_invocation_features:
        ash::vk::PhysicalDeviceShaderTerminateInvocationFeatures,
    pub(crate) vk_physical_device_global_priority_query_features_khr:
        ash::vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR,
    pub(crate) vk_physical_device_shader_atomic_float_features_ext:
        ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT,
    pub(crate) vk_physical_device_vertex_attribute_divisor_features_ext:
        ash::vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT,
    pub(crate) vk_physical_device_descriptor_indexing_features:
        ash::vk::PhysicalDeviceDescriptorIndexingFeatures,
    pub(crate) vk_physical_device_fragment_density_map2_features_ext:
        ash::vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT,
    pub(crate) vk_physical_device_uniform_buffer_standard_layout_features:
        ash::vk::PhysicalDeviceUniformBufferStandardLayoutFeatures,
    pub(crate) vk_physical_device_memory_priority_features_ext:
        ash::vk::PhysicalDeviceMemoryPriorityFeaturesEXT,
    pub(crate) vk_physical_device_scalar_block_layout_features:
        ash::vk::PhysicalDeviceScalarBlockLayoutFeatures,
    pub(crate) vk_physical_device_shader_demote_to_helper_invocation_features:
        ash::vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures,
    pub(crate) vk_physical_device_subgroup_size_control_features:
        ash::vk::PhysicalDeviceSubgroupSizeControlFeatures,
    pub(crate) vk_physical_device_shader_atomic_int64_features:
        ash::vk::PhysicalDeviceShaderAtomicInt64Features,
    pub(crate) vk_physical_device_mesh_shader_features_nv:
        ash::vk::PhysicalDeviceMeshShaderFeaturesNV,
    pub(crate) vk_physical_device_shader_clock_features_khr:
        ash::vk::PhysicalDeviceShaderClockFeaturesKHR,
    pub(crate) vk_physical_device_fragment_shader_interlock_features_ext:
        ash::vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT,
    pub(crate) vk_physical_device_representative_fragment_test_features_nv:
        ash::vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV,
    pub(crate) vk_physical_device_fragment_shading_rate_enums_features_nv:
        ash::vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV,
    pub(crate) vk_physical_device_device_generated_commands_features_nv:
        ash::vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV,
    pub(crate) vk_physical_device_exclusive_scissor_features_nv:
        ash::vk::PhysicalDeviceExclusiveScissorFeaturesNV,
    pub(crate) vk_physical_device_corner_sampled_image_features_nv:
        ash::vk::PhysicalDeviceCornerSampledImageFeaturesNV,
    pub(crate) vk_physical_device_depth_clip_enable_features_ext:
        ash::vk::PhysicalDeviceDepthClipEnableFeaturesEXT,
    pub(crate) vk_physical_device_shader_draw_parameters_features:
        ash::vk::PhysicalDeviceShaderDrawParametersFeatures,
}
#[derive(Clone)]
pub struct Feature<const FEATURE: FeatureType> {
    pub device: std::sync::Arc<crate::device::Device>,
    pub(crate) _p: std::marker::PhantomData<usize>,
}
pub(crate) fn register_features(
    features: &rustc_hash::FxHashSet<FeatureType>,
) -> ash::vk::PhysicalDeviceFeatures2 {
    struct VkStructHeader {
        pub _s_type: ash::vk::StructureType,
        pub p_next: *mut std::ffi::c_void,
    }
    let mut feature2 = ash::vk::PhysicalDeviceFeatures2::default();
    let mut map: rustc_hash::FxHashMap<ash::vk::StructureType, VkFeatureUnion> =
        rustc_hash::FxHashMap::default();
    for feature in features {
        match feature {
            FeatureType::DeviceShaderAtomicFloat2FeaturesEXT(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shader_atomic_float2_features_ext:
                            ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceCoverageReductionModeFeaturesNV(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_coverage_reduction_mode_features_nv:
                            ash::vk::PhysicalDeviceCoverageReductionModeFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceSynchronization2Features(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_synchronization2_features:
                            ash::vk::PhysicalDeviceSynchronization2Features::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceVertexInputDynamicStateFeaturesEXT(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT) . or_insert (VkFeatureUnion { vk_physical_device_vertex_input_dynamic_state_features_ext : ash :: vk :: PhysicalDeviceVertexInputDynamicStateFeaturesEXT :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceMultiviewFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_multiview_features:
                            ash::vk::PhysicalDeviceMultiviewFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderIntegerFunctions2FeaturesINTEL(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL) . or_insert (VkFeatureUnion { vk_physical_device_shader_integer_functions2_features_intel : ash :: vk :: PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DevicePrivateDataFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_private_data_features:
                            ash::vk::PhysicalDevicePrivateDataFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DevicePipelineCreationCacheControlFeatures(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES) . or_insert (VkFeatureUnion { vk_physical_device_pipeline_creation_cache_control_features : ash :: vk :: PhysicalDevicePipelineCreationCacheControlFeatures :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceMutableDescriptorTypeFeaturesVALVE(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE) . or_insert (VkFeatureUnion { vk_physical_device_mutable_descriptor_type_features_valve : ash :: vk :: PhysicalDeviceMutableDescriptorTypeFeaturesVALVE :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceTimelineSemaphoreFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_timeline_semaphore_features:
                            ash::vk::PhysicalDeviceTimelineSemaphoreFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceRayTracingPipelineFeaturesKHR(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_ray_tracing_pipeline_features_khr:
                            ash::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceVulkanMemoryModelFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_vulkan_memory_model_features:
                            ash::vk::PhysicalDeviceVulkanMemoryModelFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceExtendedDynamicState2FeaturesEXT(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT) . or_insert (VkFeatureUnion { vk_physical_device_extended_dynamic_state2_features_ext : ash :: vk :: PhysicalDeviceExtendedDynamicState2FeaturesEXT :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderImageAtomicInt64FeaturesEXT(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT) . or_insert (VkFeatureUnion { vk_physical_device_shader_image_atomic_int64_features_ext : ash :: vk :: PhysicalDeviceShaderImageAtomicInt64FeaturesEXT :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceInheritedViewportScissorFeaturesNV(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV) . or_insert (VkFeatureUnion { vk_physical_device_inherited_viewport_scissor_features_nv : ash :: vk :: PhysicalDeviceInheritedViewportScissorFeaturesNV :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceYcbcr2Plane444FormatsFeaturesEXT(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT) . or_insert (VkFeatureUnion { vk_physical_device_ycbcr2_plane444_formats_features_ext : ash :: vk :: PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DevicePresentWaitFeaturesKHR(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_present_wait_features_khr:
                            ash::vk::PhysicalDevicePresentWaitFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceComputeShaderDerivativesFeaturesNV(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV) . or_insert (VkFeatureUnion { vk_physical_device_compute_shader_derivatives_features_nv : ash :: vk :: PhysicalDeviceComputeShaderDerivativesFeaturesNV :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceVulkan13Features(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_vulkan13_features:
                            ash::vk::PhysicalDeviceVulkan13Features::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceExternalMemoryRDMAFeaturesNV(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_external_memory_rdma_features_nv:
                            ash::vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceProtectedMemoryFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_protected_memory_features:
                            ash::vk::PhysicalDeviceProtectedMemoryFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceDynamicRenderingFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_dynamic_rendering_features:
                            ash::vk::PhysicalDeviceDynamicRenderingFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceFragmentDensityMapFeaturesEXT(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_fragment_density_map_features_ext:
                            ash::vk::PhysicalDeviceFragmentDensityMapFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderFloat16Int8Features(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shader_float16_int8_features:
                            ash::vk::PhysicalDeviceShaderFloat16Int8Features::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceInvocationMaskFeaturesHUAWEI(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_invocation_mask_features_huawei:
                            ash::vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceRayQueryFeaturesKHR(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_ray_query_features_khr:
                            ash::vk::PhysicalDeviceRayQueryFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceSubpassShadingFeaturesHUAWEI(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_subpass_shading_features_huawei:
                            ash::vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderSubgroupExtendedTypesFeatures(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES) . or_insert (VkFeatureUnion { vk_physical_device_shader_subgroup_extended_types_features : ash :: vk :: PhysicalDeviceShaderSubgroupExtendedTypesFeatures :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceWorkgroupMemoryExplicitLayoutFeaturesKHR(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR) . or_insert (VkFeatureUnion { vk_physical_device_workgroup_memory_explicit_layout_features_khr : ash :: vk :: PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceRayTracingMotionBlurFeaturesNV(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_ray_tracing_motion_blur_features_nv:
                            ash::vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceMaintenance4Features(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_maintenance4_features:
                            ash::vk::PhysicalDeviceMaintenance4Features::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::Device4444FormatsFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device4444_formats_features_ext:
                            ash::vk::PhysicalDevice4444FormatsFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DevicePrimitiveTopologyListRestartFeaturesEXT(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT) . or_insert (VkFeatureUnion { vk_physical_device_primitive_topology_list_restart_features_ext : ash :: vk :: PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderIntegerDotProductFeatures(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shader_integer_dot_product_features:
                            ash::vk::PhysicalDeviceShaderIntegerDotProductFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceInlineUniformBlockFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_inline_uniform_block_features:
                            ash::vk::PhysicalDeviceInlineUniformBlockFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceSeparateDepthStencilLayoutsFeatures(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES) . or_insert (VkFeatureUnion { vk_physical_device_separate_depth_stencil_layouts_features : ash :: vk :: PhysicalDeviceSeparateDepthStencilLayoutsFeatures :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceRGBA10X6FormatsFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_rgba10x6_formats_features_ext:
                            ash::vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShadingRateImageFeaturesNV(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shading_rate_image_features_nv:
                            ash::vk::PhysicalDeviceShadingRateImageFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceDescriptorSetHostMappingFeaturesVALVE(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE) . or_insert (VkFeatureUnion { vk_physical_device_descriptor_set_host_mapping_features_valve : ash :: vk :: PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceBufferDeviceAddressFeaturesEXT(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_buffer_device_address_features_ext:
                            ash::vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DevicePipelineExecutablePropertiesFeaturesKHR(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR) . or_insert (VkFeatureUnion { vk_physical_device_pipeline_executable_properties_features_khr : ash :: vk :: PhysicalDevicePipelineExecutablePropertiesFeaturesKHR :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceHostQueryResetFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_host_query_reset_features:
                            ash::vk::PhysicalDeviceHostQueryResetFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceRasterizationOrderAttachmentAccessFeaturesARM(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM) . or_insert (VkFeatureUnion { vk_physical_device_rasterization_order_attachment_access_features_arm : ash :: vk :: PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceFragmentDensityMapOffsetFeaturesQCOM(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM) . or_insert (VkFeatureUnion { vk_physical_device_fragment_density_map_offset_features_qcom : ash :: vk :: PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceTransformFeedbackFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_transform_feedback_features_ext:
                            ash::vk::PhysicalDeviceTransformFeedbackFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DevicePerformanceQueryFeaturesKHR(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_performance_query_features_khr:
                            ash::vk::PhysicalDevicePerformanceQueryFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceDedicatedAllocationImageAliasingFeaturesNV(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV) . or_insert (VkFeatureUnion { vk_physical_device_dedicated_allocation_image_aliasing_features_nv : ash :: vk :: PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceImageViewMinLodFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_image_view_min_lod_features_ext:
                            ash::vk::PhysicalDeviceImageViewMinLodFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderSubgroupUniformControlFlowFeaturesKHR(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR) . or_insert (VkFeatureUnion { vk_physical_device_shader_subgroup_uniform_control_flow_features_khr : ash :: vk :: PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceFeatures(feature) => {
                feature.register(&mut feature2.features);
            }
            FeatureType::DeviceIndexTypeUint8FeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_index_type_uint8_features_ext:
                            ash::vk::PhysicalDeviceIndexTypeUint8FeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceVulkan12Features(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_vulkan12_features:
                            ash::vk::PhysicalDeviceVulkan12Features::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceDiagnosticsConfigFeaturesNV(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_diagnostics_config_features_nv:
                            ash::vk::PhysicalDeviceDiagnosticsConfigFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceFragmentShadingRateFeaturesKHR(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_fragment_shading_rate_features_khr:
                            ash::vk::PhysicalDeviceFragmentShadingRateFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceLineRasterizationFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_line_rasterization_features_ext:
                            ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceProvokingVertexFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_provoking_vertex_features_ext:
                            ash::vk::PhysicalDeviceProvokingVertexFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceExtendedDynamicStateFeaturesEXT(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_extended_dynamic_state_features_ext:
                            ash::vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DevicePresentIdFeaturesKHR(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_present_id_features_khr:
                            ash::vk::PhysicalDevicePresentIdFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceDeviceMemoryReportFeaturesEXT(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_device_memory_report_features_ext:
                            ash::vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::Device16BitStorageFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device16_bit_storage_features:
                            ash::vk::PhysicalDevice16BitStorageFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceBufferDeviceAddressFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_buffer_device_address_features:
                            ash::vk::PhysicalDeviceBufferDeviceAddressFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceCoherentMemoryFeaturesAMD(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_coherent_memory_features_amd:
                            ash::vk::PhysicalDeviceCoherentMemoryFeaturesAMD::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceColorWriteEnableFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_color_write_enable_features_ext:
                            ash::vk::PhysicalDeviceColorWriteEnableFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceMultiDrawFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_multi_draw_features_ext:
                            ash::vk::PhysicalDeviceMultiDrawFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceSamplerYcbcrConversionFeatures(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_sampler_ycbcr_conversion_features:
                            ash::vk::PhysicalDeviceSamplerYcbcrConversionFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceCustomBorderColorFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_custom_border_color_features_ext:
                            ash::vk::PhysicalDeviceCustomBorderColorFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceBorderColorSwizzleFeaturesEXT(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_border_color_swizzle_features_ext:
                            ash::vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceAccelerationStructureFeaturesKHR(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_acceleration_structure_features_khr:
                            ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceBlendOperationAdvancedFeaturesEXT(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT) . or_insert (VkFeatureUnion { vk_physical_device_blend_operation_advanced_features_ext : ash :: vk :: PhysicalDeviceBlendOperationAdvancedFeaturesEXT :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceLinearColorAttachmentFeaturesNV(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_linear_color_attachment_features_nv:
                            ash::vk::PhysicalDeviceLinearColorAttachmentFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceConditionalRenderingFeaturesEXT(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_conditional_rendering_features_ext:
                            ash::vk::PhysicalDeviceConditionalRenderingFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceYcbcrImageArraysFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_ycbcr_image_arrays_features_ext:
                            ash::vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceImageRobustnessFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_image_robustness_features:
                            ash::vk::PhysicalDeviceImageRobustnessFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::Device8BitStorageFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device8_bit_storage_features:
                            ash::vk::PhysicalDevice8BitStorageFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceVulkan11Features(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_vulkan11_features:
                            ash::vk::PhysicalDeviceVulkan11Features::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceDepthClipControlFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_depth_clip_control_features_ext:
                            ash::vk::PhysicalDeviceDepthClipControlFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceASTCDecodeFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_astc_decode_features_ext:
                            ash::vk::PhysicalDeviceASTCDecodeFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceCooperativeMatrixFeaturesNV(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_cooperative_matrix_features_nv:
                            ash::vk::PhysicalDeviceCooperativeMatrixFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceVariablePointersFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_variable_pointers_features:
                            ash::vk::PhysicalDeviceVariablePointersFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DevicePortabilitySubsetFeaturesKHR(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_portability_subset_features_khr:
                            ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderImageFootprintFeaturesNV(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shader_image_footprint_features_nv:
                            ash::vk::PhysicalDeviceShaderImageFootprintFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceTexelBufferAlignmentFeaturesEXT(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_texel_buffer_alignment_features_ext:
                            ash::vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceImagelessFramebufferFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_imageless_framebuffer_features:
                            ash::vk::PhysicalDeviceImagelessFramebufferFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderSMBuiltinsFeaturesNV(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shader_sm_builtins_features_nv:
                            ash::vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceFragmentShaderBarycentricFeaturesNV(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV) . or_insert (VkFeatureUnion { vk_physical_device_fragment_shader_barycentric_features_nv : ash :: vk :: PhysicalDeviceFragmentShaderBarycentricFeaturesNV :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceTextureCompressionASTCHDRFeatures(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES) . or_insert (VkFeatureUnion { vk_physical_device_texture_compression_astchdr_features : ash :: vk :: PhysicalDeviceTextureCompressionASTCHDRFeatures :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DevicePageableDeviceLocalMemoryFeaturesEXT(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT) . or_insert (VkFeatureUnion { vk_physical_device_pageable_device_local_memory_features_ext : ash :: vk :: PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceZeroInitializeWorkgroupMemoryFeatures(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES) . or_insert (VkFeatureUnion { vk_physical_device_zero_initialize_workgroup_memory_features : ash :: vk :: PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceRobustness2FeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_robustness2_features_ext:
                            ash::vk::PhysicalDeviceRobustness2FeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderTerminateInvocationFeatures(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES) . or_insert (VkFeatureUnion { vk_physical_device_shader_terminate_invocation_features : ash :: vk :: PhysicalDeviceShaderTerminateInvocationFeatures :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceGlobalPriorityQueryFeaturesKHR(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_global_priority_query_features_khr:
                            ash::vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderAtomicFloatFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shader_atomic_float_features_ext:
                            ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceVertexAttributeDivisorFeaturesEXT(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT) . or_insert (VkFeatureUnion { vk_physical_device_vertex_attribute_divisor_features_ext : ash :: vk :: PhysicalDeviceVertexAttributeDivisorFeaturesEXT :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceDescriptorIndexingFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_descriptor_indexing_features:
                            ash::vk::PhysicalDeviceDescriptorIndexingFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceFragmentDensityMap2FeaturesEXT(feature) => {
                let u = map
                    .entry(
                        ash::vk::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT,
                    )
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_fragment_density_map2_features_ext:
                            ash::vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceUniformBufferStandardLayoutFeatures(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES) . or_insert (VkFeatureUnion { vk_physical_device_uniform_buffer_standard_layout_features : ash :: vk :: PhysicalDeviceUniformBufferStandardLayoutFeatures :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceMemoryPriorityFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_memory_priority_features_ext:
                            ash::vk::PhysicalDeviceMemoryPriorityFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceScalarBlockLayoutFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_scalar_block_layout_features:
                            ash::vk::PhysicalDeviceScalarBlockLayoutFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderDemoteToHelperInvocationFeatures(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES) . or_insert (VkFeatureUnion { vk_physical_device_shader_demote_to_helper_invocation_features : ash :: vk :: PhysicalDeviceShaderDemoteToHelperInvocationFeatures :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceSubgroupSizeControlFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_subgroup_size_control_features:
                            ash::vk::PhysicalDeviceSubgroupSizeControlFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderAtomicInt64Features(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shader_atomic_int64_features:
                            ash::vk::PhysicalDeviceShaderAtomicInt64Features::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceMeshShaderFeaturesNV(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_mesh_shader_features_nv:
                            ash::vk::PhysicalDeviceMeshShaderFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderClockFeaturesKHR(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shader_clock_features_khr:
                            ash::vk::PhysicalDeviceShaderClockFeaturesKHR::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceFragmentShaderInterlockFeaturesEXT(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT) . or_insert (VkFeatureUnion { vk_physical_device_fragment_shader_interlock_features_ext : ash :: vk :: PhysicalDeviceFragmentShaderInterlockFeaturesEXT :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceRepresentativeFragmentTestFeaturesNV(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV) . or_insert (VkFeatureUnion { vk_physical_device_representative_fragment_test_features_nv : ash :: vk :: PhysicalDeviceRepresentativeFragmentTestFeaturesNV :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceFragmentShadingRateEnumsFeaturesNV(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV) . or_insert (VkFeatureUnion { vk_physical_device_fragment_shading_rate_enums_features_nv : ash :: vk :: PhysicalDeviceFragmentShadingRateEnumsFeaturesNV :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceDeviceGeneratedCommandsFeaturesNV(feature) => {
                let u = map . entry (ash :: vk :: StructureType :: PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV) . or_insert (VkFeatureUnion { vk_physical_device_device_generated_commands_features_nv : ash :: vk :: PhysicalDeviceDeviceGeneratedCommandsFeaturesNV :: default () }) ;
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceExclusiveScissorFeaturesNV(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_exclusive_scissor_features_nv:
                            ash::vk::PhysicalDeviceExclusiveScissorFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceCornerSampledImageFeaturesNV(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_corner_sampled_image_features_nv:
                            ash::vk::PhysicalDeviceCornerSampledImageFeaturesNV::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceDepthClipEnableFeaturesEXT(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_depth_clip_enable_features_ext:
                            ash::vk::PhysicalDeviceDepthClipEnableFeaturesEXT::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
            FeatureType::DeviceShaderDrawParametersFeatures(feature) => {
                let u = map
                    .entry(ash::vk::StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES)
                    .or_insert(VkFeatureUnion {
                        vk_physical_device_shader_draw_parameters_features:
                            ash::vk::PhysicalDeviceShaderDrawParametersFeatures::default(),
                    });
                feature.register(unsafe { std::mem::transmute(u) });
            }
        }
    }
    map.into_iter().for_each(|(_, mut feature_union)| unsafe {
        let header = &mut feature_union as *mut _ as *mut VkStructHeader;
        (*header).p_next = feature2.p_next;
        feature2.p_next = header as *mut _;
    });
    feature2
}
