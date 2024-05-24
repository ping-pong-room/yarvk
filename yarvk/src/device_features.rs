pub trait Feature {
    type VkFeatureType: Default;
    type RequiredExtension: crate::extensions::DeviceExtension;
    const IS_FEATURE2: bool;
    fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool;
    fn register_feature(vk_feature: &mut Self::VkFeatureType);
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self;
}
#[derive(Clone)]
pub struct ExtensionNone {}
impl crate::extensions::Extension for ExtensionNone {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl crate::extensions::DeviceExtension for ExtensionNone {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(_device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {}
    }
}
pub mod physical_device_shader_atomic_float2_features_ext {
    pub struct FeatureShaderBufferFloat16Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferFloat16Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_float16_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_float16_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderBufferFloat16AtomicAdd {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferFloat16AtomicAdd {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_float16_atomic_add != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_float16_atomic_add = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderBufferFloat16AtomicMinMax {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferFloat16AtomicMinMax {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_float16_atomic_min_max != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_float16_atomic_min_max = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderBufferFloat32AtomicMinMax {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferFloat32AtomicMinMax {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_float32_atomic_min_max != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_float32_atomic_min_max = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderBufferFloat64AtomicMinMax {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferFloat64AtomicMinMax {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_float64_atomic_min_max != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_float64_atomic_min_max = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedFloat16Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedFloat16Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_float16_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_float16_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedFloat16AtomicAdd {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedFloat16AtomicAdd {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_float16_atomic_add != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_float16_atomic_add = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedFloat16AtomicMinMax {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedFloat16AtomicMinMax {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_float16_atomic_min_max != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_float16_atomic_min_max = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedFloat32AtomicMinMax {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedFloat32AtomicMinMax {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_float32_atomic_min_max != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_float32_atomic_min_max = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedFloat64AtomicMinMax {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedFloat64AtomicMinMax {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_float64_atomic_min_max != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_float64_atomic_min_max = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderImageFloat32AtomicMinMax {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderImageFloat32AtomicMinMax {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_image_float32_atomic_min_max != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_image_float32_atomic_min_max = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseImageFloat32AtomicMinMax {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseImageFloat32AtomicMinMax {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_image_float32_atomic_min_max != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_image_float32_atomic_min_max = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_coverage_reduction_mode_features_nv {
    pub struct FeatureCoverageReductionMode {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureCoverageReductionMode {
        type VkFeatureType = ash::vk::PhysicalDeviceCoverageReductionModeFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvCoverageReductionMode;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.coverage_reduction_mode != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.coverage_reduction_mode = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_synchronization2_features {
    pub struct FeatureSynchronization2 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSynchronization2 {
        type VkFeatureType = ash::vk::PhysicalDeviceSynchronization2Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.synchronization2 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.synchronization2 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_vertex_input_dynamic_state_features_ext {
    pub struct FeatureVertexInputDynamicState {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVertexInputDynamicState {
        type VkFeatureType = ash::vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtVertexInputDynamicState;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vertex_input_dynamic_state != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vertex_input_dynamic_state = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_multiview_features {
    pub struct FeatureMultiview {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultiview {
        type VkFeatureType = ash::vk::PhysicalDeviceMultiviewFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multiview != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multiview = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMultiviewGeometryShader {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultiviewGeometryShader {
        type VkFeatureType = ash::vk::PhysicalDeviceMultiviewFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multiview_geometry_shader != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multiview_geometry_shader = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMultiviewTessellationShader {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultiviewTessellationShader {
        type VkFeatureType = ash::vk::PhysicalDeviceMultiviewFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multiview_tessellation_shader != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multiview_tessellation_shader = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_integer_functions2_features_intel {
    pub struct FeatureShaderIntegerFunctions2 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderIntegerFunctions2 {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
        type RequiredExtension = crate::extensions::ExtensionIntelShaderIntegerFunctions2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_integer_functions2 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_integer_functions2 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_private_data_features {
    pub struct FeaturePrivateData {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePrivateData {
        type VkFeatureType = ash::vk::PhysicalDevicePrivateDataFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.private_data != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.private_data = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_pipeline_creation_cache_control_features {
    pub struct FeaturePipelineCreationCacheControl {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePipelineCreationCacheControl {
        type VkFeatureType = ash::vk::PhysicalDevicePipelineCreationCacheControlFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.pipeline_creation_cache_control != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.pipeline_creation_cache_control = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_mutable_descriptor_type_features_valve {
    pub struct FeatureMutableDescriptorType {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMutableDescriptorType {
        type VkFeatureType = ash::vk::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
        type RequiredExtension = crate::extensions::ExtensionValveMutableDescriptorType;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.mutable_descriptor_type != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.mutable_descriptor_type = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_timeline_semaphore_features {
    pub struct FeatureTimelineSemaphore {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTimelineSemaphore {
        type VkFeatureType = ash::vk::PhysicalDeviceTimelineSemaphoreFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.timeline_semaphore != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.timeline_semaphore = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_ray_tracing_pipeline_features_khr {
    pub struct FeatureRayTracingPipeline {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRayTracingPipeline {
        type VkFeatureType = ash::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrRayTracingPipeline;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ray_tracing_pipeline != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ray_tracing_pipeline = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRayTracingPipelineShaderGroupHandleCaptureReplay {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRayTracingPipelineShaderGroupHandleCaptureReplay {
        type VkFeatureType = ash::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrRayTracingPipeline;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ray_tracing_pipeline_shader_group_handle_capture_replay != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ray_tracing_pipeline_shader_group_handle_capture_replay = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRayTracingPipelineShaderGroupHandleCaptureReplayMixed {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature
        for FeatureRayTracingPipelineShaderGroupHandleCaptureReplayMixed
    {
        type VkFeatureType = ash::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrRayTracingPipeline;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRayTracingPipelineTraceRaysIndirect {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRayTracingPipelineTraceRaysIndirect {
        type VkFeatureType = ash::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrRayTracingPipeline;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ray_tracing_pipeline_trace_rays_indirect != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ray_tracing_pipeline_trace_rays_indirect = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRayTraversalPrimitiveCulling {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRayTraversalPrimitiveCulling {
        type VkFeatureType = ash::vk::PhysicalDeviceRayTracingPipelineFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrRayTracingPipeline;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ray_traversal_primitive_culling != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ray_traversal_primitive_culling = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_vulkan_memory_model_features {
    pub struct FeatureVulkanMemoryModel {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVulkanMemoryModel {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkanMemoryModelFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vulkan_memory_model != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vulkan_memory_model = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVulkanMemoryModelDeviceScope {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVulkanMemoryModelDeviceScope {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkanMemoryModelFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vulkan_memory_model_device_scope != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vulkan_memory_model_device_scope = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVulkanMemoryModelAvailabilityVisibilityChains {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVulkanMemoryModelAvailabilityVisibilityChains {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkanMemoryModelFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vulkan_memory_model_availability_visibility_chains != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vulkan_memory_model_availability_visibility_chains = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_extended_dynamic_state2_features_ext {
    pub struct FeatureExtendedDynamicState2 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureExtendedDynamicState2 {
        type VkFeatureType = ash::vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.extended_dynamic_state2 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.extended_dynamic_state2 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureExtendedDynamicState2LogicOp {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureExtendedDynamicState2LogicOp {
        type VkFeatureType = ash::vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.extended_dynamic_state2_logic_op != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.extended_dynamic_state2_logic_op = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureExtendedDynamicState2PatchControlPoints {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureExtendedDynamicState2PatchControlPoints {
        type VkFeatureType = ash::vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.extended_dynamic_state2_patch_control_points != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.extended_dynamic_state2_patch_control_points = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_image_atomic_int64_features_ext {
    pub struct FeatureShaderImageInt64Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderImageInt64Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderImageAtomicInt64;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_image_int64_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_image_int64_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseImageInt64Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseImageInt64Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderImageAtomicInt64;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_image_int64_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_image_int64_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_inherited_viewport_scissor_features_nv {
    pub struct FeatureInheritedViewportScissor2D {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureInheritedViewportScissor2D {
        type VkFeatureType = ash::vk::PhysicalDeviceInheritedViewportScissorFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvInheritedViewportScissor;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.inherited_viewport_scissor2_d != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.inherited_viewport_scissor2_d = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_ycbcr2_plane444_formats_features_ext {
    pub struct FeatureYcbcr2plane444Formats {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureYcbcr2plane444Formats {
        type VkFeatureType = ash::vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ycbcr2plane444_formats != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ycbcr2plane444_formats = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_present_wait_features_khr {
    pub struct FeaturePresentWait {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePresentWait {
        type VkFeatureType = ash::vk::PhysicalDevicePresentWaitFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPresentWait;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.present_wait != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.present_wait = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_compute_shader_derivatives_features_nv {
    pub struct FeatureComputeDerivativeGroupQuads {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureComputeDerivativeGroupQuads {
        type VkFeatureType = ash::vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvComputeShaderDerivatives;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.compute_derivative_group_quads != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.compute_derivative_group_quads = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureComputeDerivativeGroupLinear {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureComputeDerivativeGroupLinear {
        type VkFeatureType = ash::vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvComputeShaderDerivatives;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.compute_derivative_group_linear != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.compute_derivative_group_linear = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_vulkan13_features {
    pub struct FeatureRobustImageAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRobustImageAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.robust_image_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.robust_image_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureInlineUniformBlock {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureInlineUniformBlock {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.inline_uniform_block != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.inline_uniform_block = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingInlineUniformBlockUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingInlineUniformBlockUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_inline_uniform_block_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_inline_uniform_block_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeaturePipelineCreationCacheControl {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePipelineCreationCacheControl {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.pipeline_creation_cache_control != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.pipeline_creation_cache_control = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeaturePrivateData {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePrivateData {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.private_data != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.private_data = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderDemoteToHelperInvocation {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderDemoteToHelperInvocation {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_demote_to_helper_invocation != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_demote_to_helper_invocation = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderTerminateInvocation {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderTerminateInvocation {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_terminate_invocation != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_terminate_invocation = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSubgroupSizeControl {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSubgroupSizeControl {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.subgroup_size_control != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.subgroup_size_control = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureComputeFullSubgroups {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureComputeFullSubgroups {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.compute_full_subgroups != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.compute_full_subgroups = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSynchronization2 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSynchronization2 {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.synchronization2 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.synchronization2 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTextureCompressionAstcHdr {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTextureCompressionAstcHdr {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.texture_compression_astc_hdr != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.texture_compression_astc_hdr = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderZeroInitializeWorkgroupMemory {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderZeroInitializeWorkgroupMemory {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_zero_initialize_workgroup_memory != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_zero_initialize_workgroup_memory = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDynamicRendering {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDynamicRendering {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.dynamic_rendering != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.dynamic_rendering = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderIntegerDotProduct {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderIntegerDotProduct {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_integer_dot_product != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_integer_dot_product = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMaintenance4 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMaintenance4 {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan13Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.maintenance4 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.maintenance4 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_external_memory_rdma_features_nv {
    pub struct FeatureExternalMemoryRdma {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureExternalMemoryRdma {
        type VkFeatureType = ash::vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvExternalMemoryRdma;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.external_memory_rdma != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.external_memory_rdma = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_protected_memory_features {
    pub struct FeatureProtectedMemory {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureProtectedMemory {
        type VkFeatureType = ash::vk::PhysicalDeviceProtectedMemoryFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.protected_memory != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.protected_memory = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_dynamic_rendering_features {
    pub struct FeatureDynamicRendering {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDynamicRendering {
        type VkFeatureType = ash::vk::PhysicalDeviceDynamicRenderingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.dynamic_rendering != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.dynamic_rendering = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_fragment_density_map_features_ext {
    pub struct FeatureFragmentDensityMap {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentDensityMap {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentDensityMapFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtFragmentDensityMap;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_density_map != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_density_map = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureFragmentDensityMapDynamic {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentDensityMapDynamic {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentDensityMapFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtFragmentDensityMap;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_density_map_dynamic != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_density_map_dynamic = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureFragmentDensityMapNonSubsampledImages {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentDensityMapNonSubsampledImages {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentDensityMapFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtFragmentDensityMap;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_density_map_non_subsampled_images != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_density_map_non_subsampled_images = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_float16_int8_features {
    pub struct FeatureShaderFloat16 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderFloat16 {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderFloat16Int8Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_float16 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_float16 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderInt8 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderInt8 {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderFloat16Int8Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_int8 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_int8 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_invocation_mask_features_huawei {
    pub struct FeatureInvocationMask {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureInvocationMask {
        type VkFeatureType = ash::vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI;
        type RequiredExtension = crate::extensions::ExtensionHuaweiInvocationMask;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.invocation_mask != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.invocation_mask = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_ray_query_features_khr {
    pub struct FeatureRayQuery {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRayQuery {
        type VkFeatureType = ash::vk::PhysicalDeviceRayQueryFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrRayQuery;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ray_query != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ray_query = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_subpass_shading_features_huawei {
    pub struct FeatureSubpassShading {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSubpassShading {
        type VkFeatureType = ash::vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI;
        type RequiredExtension = crate::extensions::ExtensionHuaweiSubpassShading;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.subpass_shading != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.subpass_shading = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_subgroup_extended_types_features {
    pub struct FeatureShaderSubgroupExtendedTypes {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSubgroupExtendedTypes {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_subgroup_extended_types != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_subgroup_extended_types = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_workgroup_memory_explicit_layout_features_khr {
    pub struct FeatureWorkgroupMemoryExplicitLayout {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureWorkgroupMemoryExplicitLayout {
        type VkFeatureType = ash::vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrWorkgroupMemoryExplicitLayout;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.workgroup_memory_explicit_layout != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.workgroup_memory_explicit_layout = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureWorkgroupMemoryExplicitLayoutScalarBlockLayout {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureWorkgroupMemoryExplicitLayoutScalarBlockLayout {
        type VkFeatureType = ash::vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrWorkgroupMemoryExplicitLayout;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.workgroup_memory_explicit_layout_scalar_block_layout != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.workgroup_memory_explicit_layout_scalar_block_layout = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureWorkgroupMemoryExplicitLayout8BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureWorkgroupMemoryExplicitLayout8BitAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrWorkgroupMemoryExplicitLayout;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.workgroup_memory_explicit_layout8_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.workgroup_memory_explicit_layout8_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureWorkgroupMemoryExplicitLayout16BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureWorkgroupMemoryExplicitLayout16BitAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrWorkgroupMemoryExplicitLayout;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.workgroup_memory_explicit_layout16_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.workgroup_memory_explicit_layout16_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_ray_tracing_motion_blur_features_nv {
    pub struct FeatureRayTracingMotionBlur {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRayTracingMotionBlur {
        type VkFeatureType = ash::vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvRayTracingMotionBlur;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ray_tracing_motion_blur != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ray_tracing_motion_blur = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRayTracingMotionBlurPipelineTraceRaysIndirect {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRayTracingMotionBlurPipelineTraceRaysIndirect {
        type VkFeatureType = ash::vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvRayTracingMotionBlur;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ray_tracing_motion_blur_pipeline_trace_rays_indirect != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ray_tracing_motion_blur_pipeline_trace_rays_indirect = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_maintenance4_features {
    pub struct FeatureMaintenance4 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMaintenance4 {
        type VkFeatureType = ash::vk::PhysicalDeviceMaintenance4Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.maintenance4 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.maintenance4 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device4444_formats_features_ext {
    pub struct FeatureFormatA4r4g4b4 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFormatA4r4g4b4 {
        type VkFeatureType = ash::vk::PhysicalDevice4444FormatsFeaturesEXT;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.format_a4r4g4b4 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.format_a4r4g4b4 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureFormatA4b4g4r4 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFormatA4b4g4r4 {
        type VkFeatureType = ash::vk::PhysicalDevice4444FormatsFeaturesEXT;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.format_a4b4g4r4 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.format_a4b4g4r4 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_primitive_topology_list_restart_features_ext {
    pub struct FeaturePrimitiveTopologyListRestart {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePrimitiveTopologyListRestart {
        type VkFeatureType = ash::vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtPrimitiveTopologyListRestart;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.primitive_topology_list_restart != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.primitive_topology_list_restart = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeaturePrimitiveTopologyPatchListRestart {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePrimitiveTopologyPatchListRestart {
        type VkFeatureType = ash::vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtPrimitiveTopologyListRestart;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.primitive_topology_patch_list_restart != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.primitive_topology_patch_list_restart = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_integer_dot_product_features {
    pub struct FeatureShaderIntegerDotProduct {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderIntegerDotProduct {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderIntegerDotProductFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_integer_dot_product != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_integer_dot_product = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_inline_uniform_block_features {
    pub struct FeatureInlineUniformBlock {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureInlineUniformBlock {
        type VkFeatureType = ash::vk::PhysicalDeviceInlineUniformBlockFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.inline_uniform_block != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.inline_uniform_block = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingInlineUniformBlockUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingInlineUniformBlockUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceInlineUniformBlockFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_inline_uniform_block_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_inline_uniform_block_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_separate_depth_stencil_layouts_features {
    pub struct FeatureSeparateDepthStencilLayouts {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSeparateDepthStencilLayouts {
        type VkFeatureType = ash::vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.separate_depth_stencil_layouts != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.separate_depth_stencil_layouts = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_rgba10x6_formats_features_ext {
    pub struct FeatureFormatRgba10x6WithoutYCbCrSampler {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFormatRgba10x6WithoutYCbCrSampler {
        type VkFeatureType = ash::vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtRgba10x6Formats;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.format_rgba10x6_without_y_cb_cr_sampler != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.format_rgba10x6_without_y_cb_cr_sampler = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shading_rate_image_features_nv {
    pub struct FeatureShadingRateImage {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShadingRateImage {
        type VkFeatureType = ash::vk::PhysicalDeviceShadingRateImageFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvShadingRateImage;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shading_rate_image != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shading_rate_image = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShadingRateCoarseSampleOrder {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShadingRateCoarseSampleOrder {
        type VkFeatureType = ash::vk::PhysicalDeviceShadingRateImageFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvShadingRateImage;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shading_rate_coarse_sample_order != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shading_rate_coarse_sample_order = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_descriptor_set_host_mapping_features_valve {
    pub struct FeatureDescriptorSetHostMapping {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorSetHostMapping {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
        type RequiredExtension = crate::extensions::ExtensionValveDescriptorSetHostMapping;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_set_host_mapping != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_set_host_mapping = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_buffer_device_address_features_ext {
    pub struct FeatureBufferDeviceAddress {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBufferDeviceAddress {
        type VkFeatureType = ash::vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtBufferDeviceAddress;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.buffer_device_address != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.buffer_device_address = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureBufferDeviceAddressCaptureReplay {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBufferDeviceAddressCaptureReplay {
        type VkFeatureType = ash::vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtBufferDeviceAddress;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.buffer_device_address_capture_replay != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.buffer_device_address_capture_replay = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureBufferDeviceAddressMultiDevice {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBufferDeviceAddressMultiDevice {
        type VkFeatureType = ash::vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtBufferDeviceAddress;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.buffer_device_address_multi_device != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.buffer_device_address_multi_device = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_pipeline_executable_properties_features_khr {
    pub struct FeaturePipelineExecutableInfo {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePipelineExecutableInfo {
        type VkFeatureType = ash::vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPipelineExecutableProperties;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.pipeline_executable_info != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.pipeline_executable_info = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_host_query_reset_features {
    pub struct FeatureHostQueryReset {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureHostQueryReset {
        type VkFeatureType = ash::vk::PhysicalDeviceHostQueryResetFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.host_query_reset != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.host_query_reset = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_rasterization_order_attachment_access_features_arm {
    pub struct FeatureRasterizationOrderColorAttachmentAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRasterizationOrderColorAttachmentAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
        type RequiredExtension = crate::extensions::ExtensionArmRasterizationOrderAttachmentAccess;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.rasterization_order_color_attachment_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.rasterization_order_color_attachment_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRasterizationOrderDepthAttachmentAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRasterizationOrderDepthAttachmentAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
        type RequiredExtension = crate::extensions::ExtensionArmRasterizationOrderAttachmentAccess;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.rasterization_order_depth_attachment_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.rasterization_order_depth_attachment_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRasterizationOrderStencilAttachmentAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRasterizationOrderStencilAttachmentAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
        type RequiredExtension = crate::extensions::ExtensionArmRasterizationOrderAttachmentAccess;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.rasterization_order_stencil_attachment_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.rasterization_order_stencil_attachment_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_fragment_density_map_offset_features_qcom {
    pub struct FeatureFragmentDensityMapOffset {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentDensityMapOffset {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM;
        type RequiredExtension = crate::extensions::ExtensionQcomFragmentDensityMapOffset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_density_map_offset != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_density_map_offset = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_transform_feedback_features_ext {
    pub struct FeatureTransformFeedback {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTransformFeedback {
        type VkFeatureType = ash::vk::PhysicalDeviceTransformFeedbackFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtTransformFeedback;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.transform_feedback != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.transform_feedback = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureGeometryStreams {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureGeometryStreams {
        type VkFeatureType = ash::vk::PhysicalDeviceTransformFeedbackFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtTransformFeedback;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.geometry_streams != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.geometry_streams = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_performance_query_features_khr {
    pub struct FeaturePerformanceCounterQueryPools {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePerformanceCounterQueryPools {
        type VkFeatureType = ash::vk::PhysicalDevicePerformanceQueryFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPerformanceQuery;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.performance_counter_query_pools != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.performance_counter_query_pools = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeaturePerformanceCounterMultipleQueryPools {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePerformanceCounterMultipleQueryPools {
        type VkFeatureType = ash::vk::PhysicalDevicePerformanceQueryFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPerformanceQuery;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.performance_counter_multiple_query_pools != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.performance_counter_multiple_query_pools = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_dedicated_allocation_image_aliasing_features_nv {
    pub struct FeatureDedicatedAllocationImageAliasing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDedicatedAllocationImageAliasing {
        type VkFeatureType = ash::vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvDedicatedAllocationImageAliasing;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.dedicated_allocation_image_aliasing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.dedicated_allocation_image_aliasing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_image_view_min_lod_features_ext {
    pub struct FeatureMinLod {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMinLod {
        type VkFeatureType = ash::vk::PhysicalDeviceImageViewMinLodFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtImageViewMinLod;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.min_lod != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.min_lod = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_subgroup_uniform_control_flow_features_khr {
    pub struct FeatureShaderSubgroupUniformControlFlow {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSubgroupUniformControlFlow {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrShaderSubgroupUniformControlFlow;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_subgroup_uniform_control_flow != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_subgroup_uniform_control_flow = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_features {
    pub struct FeatureRobustBufferAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRobustBufferAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.robust_buffer_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.robust_buffer_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureFullDrawIndexUint32 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFullDrawIndexUint32 {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.full_draw_index_uint32 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.full_draw_index_uint32 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureImageCubeArray {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureImageCubeArray {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.image_cube_array != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.image_cube_array = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureIndependentBlend {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureIndependentBlend {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.independent_blend != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.independent_blend = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureGeometryShader {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureGeometryShader {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.geometry_shader != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.geometry_shader = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTessellationShader {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTessellationShader {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.tessellation_shader != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.tessellation_shader = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSampleRateShading {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSampleRateShading {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sample_rate_shading != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sample_rate_shading = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDualSrcBlend {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDualSrcBlend {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.dual_src_blend != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.dual_src_blend = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureLogicOp {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureLogicOp {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.logic_op != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.logic_op = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMultiDrawIndirect {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultiDrawIndirect {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multi_draw_indirect != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multi_draw_indirect = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDrawIndirectFirstInstance {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDrawIndirectFirstInstance {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.draw_indirect_first_instance != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.draw_indirect_first_instance = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDepthClamp {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDepthClamp {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.depth_clamp != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.depth_clamp = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDepthBiasClamp {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDepthBiasClamp {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.depth_bias_clamp != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.depth_bias_clamp = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureFillModeNonSolid {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFillModeNonSolid {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fill_mode_non_solid != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fill_mode_non_solid = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDepthBounds {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDepthBounds {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.depth_bounds != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.depth_bounds = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureWideLines {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureWideLines {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.wide_lines != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.wide_lines = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureLargePoints {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureLargePoints {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.large_points != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.large_points = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureAlphaToOne {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureAlphaToOne {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.alpha_to_one != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.alpha_to_one = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMultiViewport {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultiViewport {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multi_viewport != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multi_viewport = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSamplerAnisotropy {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSamplerAnisotropy {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sampler_anisotropy != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sampler_anisotropy = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTextureCompressionEtc2 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTextureCompressionEtc2 {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.texture_compression_etc2 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.texture_compression_etc2 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTextureCompressionAstcLdr {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTextureCompressionAstcLdr {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.texture_compression_astc_ldr != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.texture_compression_astc_ldr = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTextureCompressionBc {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTextureCompressionBc {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.texture_compression_bc != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.texture_compression_bc = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureOcclusionQueryPrecise {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureOcclusionQueryPrecise {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.occlusion_query_precise != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.occlusion_query_precise = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeaturePipelineStatisticsQuery {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePipelineStatisticsQuery {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.pipeline_statistics_query != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.pipeline_statistics_query = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVertexPipelineStoresAndAtomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVertexPipelineStoresAndAtomics {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vertex_pipeline_stores_and_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vertex_pipeline_stores_and_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureFragmentStoresAndAtomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentStoresAndAtomics {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_stores_and_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_stores_and_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderTessellationAndGeometryPointSize {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderTessellationAndGeometryPointSize {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_tessellation_and_geometry_point_size != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_tessellation_and_geometry_point_size = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderImageGatherExtended {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderImageGatherExtended {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_image_gather_extended != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_image_gather_extended = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageImageExtendedFormats {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageImageExtendedFormats {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_image_extended_formats != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_image_extended_formats = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageImageMultisample {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageImageMultisample {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_image_multisample != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_image_multisample = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageImageReadWithoutFormat {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageImageReadWithoutFormat {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_image_read_without_format != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_image_read_without_format = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageImageWriteWithoutFormat {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageImageWriteWithoutFormat {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_image_write_without_format != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_image_write_without_format = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderUniformBufferArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderUniformBufferArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_uniform_buffer_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_uniform_buffer_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSampledImageArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSampledImageArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_sampled_image_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_sampled_image_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageBufferArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageBufferArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_buffer_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_buffer_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageImageArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageImageArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_image_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_image_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderClipDistance {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderClipDistance {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_clip_distance != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_clip_distance = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderCullDistance {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderCullDistance {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_cull_distance != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_cull_distance = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderFloat64 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderFloat64 {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_float64 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_float64 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderInt64 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderInt64 {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_int64 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_int64 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderInt16 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderInt16 {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_int16 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_int16 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderResourceResidency {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderResourceResidency {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_resource_residency != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_resource_residency = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderResourceMinLod {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderResourceMinLod {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_resource_min_lod != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_resource_min_lod = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseBinding {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseBinding {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_binding != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_binding = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseResidencyBuffer {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseResidencyBuffer {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_residency_buffer != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_residency_buffer = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseResidencyImage2D {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseResidencyImage2D {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_residency_image2_d != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_residency_image2_d = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseResidencyImage3D {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseResidencyImage3D {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_residency_image3_d != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_residency_image3_d = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseResidency2Samples {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseResidency2Samples {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_residency2_samples != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_residency2_samples = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseResidency4Samples {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseResidency4Samples {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_residency4_samples != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_residency4_samples = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseResidency8Samples {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseResidency8Samples {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_residency8_samples != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_residency8_samples = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseResidency16Samples {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseResidency16Samples {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_residency16_samples != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_residency16_samples = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseResidencyAliased {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseResidencyAliased {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_residency_aliased != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_residency_aliased = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVariableMultisampleRate {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVariableMultisampleRate {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.variable_multisample_rate != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.variable_multisample_rate = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureInheritedQueries {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureInheritedQueries {
        type VkFeatureType = ash::vk::PhysicalDeviceFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = false;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.inherited_queries != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.inherited_queries = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_index_type_uint8_features_ext {
    pub struct FeatureIndexTypeUint8 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureIndexTypeUint8 {
        type VkFeatureType = ash::vk::PhysicalDeviceIndexTypeUint8FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtIndexTypeUint8;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.index_type_uint8 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.index_type_uint8 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_vulkan12_features {
    pub struct FeatureSamplerMirrorClampToEdge {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSamplerMirrorClampToEdge {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sampler_mirror_clamp_to_edge != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sampler_mirror_clamp_to_edge = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDrawIndirectCount {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDrawIndirectCount {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.draw_indirect_count != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.draw_indirect_count = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStorageBuffer8BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStorageBuffer8BitAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_buffer8_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_buffer8_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureUniformAndStorageBuffer8BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureUniformAndStorageBuffer8BitAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.uniform_and_storage_buffer8_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.uniform_and_storage_buffer8_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStoragePushConstant8 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStoragePushConstant8 {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_push_constant8 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_push_constant8 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderBufferInt64Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferInt64Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_int64_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_int64_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedInt64Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedInt64Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_int64_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_int64_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderFloat16 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderFloat16 {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_float16 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_float16 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderInt8 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderInt8 {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_int8 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_int8 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderInputAttachmentArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderInputAttachmentArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_input_attachment_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_input_attachment_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderUniformTexelBufferArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderUniformTexelBufferArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_uniform_texel_buffer_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_uniform_texel_buffer_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageTexelBufferArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageTexelBufferArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_texel_buffer_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_texel_buffer_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderUniformBufferArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderUniformBufferArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_uniform_buffer_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_uniform_buffer_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSampledImageArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSampledImageArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_sampled_image_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_sampled_image_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageBufferArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageBufferArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_buffer_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_buffer_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageImageArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageImageArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_image_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_image_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderInputAttachmentArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderInputAttachmentArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_input_attachment_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_input_attachment_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderUniformTexelBufferArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderUniformTexelBufferArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_uniform_texel_buffer_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_uniform_texel_buffer_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageTexelBufferArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageTexelBufferArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_texel_buffer_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_texel_buffer_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingUniformBufferUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingUniformBufferUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_uniform_buffer_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_uniform_buffer_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingSampledImageUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingSampledImageUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_sampled_image_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_sampled_image_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingStorageImageUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingStorageImageUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_storage_image_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_storage_image_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingStorageBufferUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingStorageBufferUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_storage_buffer_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_storage_buffer_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingUniformTexelBufferUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingUniformTexelBufferUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_uniform_texel_buffer_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_uniform_texel_buffer_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingStorageTexelBufferUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingStorageTexelBufferUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_storage_texel_buffer_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_storage_texel_buffer_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingUpdateUnusedWhilePending {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingUpdateUnusedWhilePending {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_update_unused_while_pending != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_update_unused_while_pending = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingPartiallyBound {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingPartiallyBound {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_partially_bound != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_partially_bound = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingVariableDescriptorCount {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingVariableDescriptorCount {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_variable_descriptor_count != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_variable_descriptor_count = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRuntimeDescriptorArray {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRuntimeDescriptorArray {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.runtime_descriptor_array != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.runtime_descriptor_array = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSamplerFilterMinmax {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSamplerFilterMinmax {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sampler_filter_minmax != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sampler_filter_minmax = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureScalarBlockLayout {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureScalarBlockLayout {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.scalar_block_layout != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.scalar_block_layout = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureImagelessFramebuffer {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureImagelessFramebuffer {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.imageless_framebuffer != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.imageless_framebuffer = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureUniformBufferStandardLayout {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureUniformBufferStandardLayout {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.uniform_buffer_standard_layout != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.uniform_buffer_standard_layout = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSubgroupExtendedTypes {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSubgroupExtendedTypes {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_subgroup_extended_types != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_subgroup_extended_types = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSeparateDepthStencilLayouts {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSeparateDepthStencilLayouts {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.separate_depth_stencil_layouts != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.separate_depth_stencil_layouts = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureHostQueryReset {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureHostQueryReset {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.host_query_reset != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.host_query_reset = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTimelineSemaphore {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTimelineSemaphore {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.timeline_semaphore != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.timeline_semaphore = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureBufferDeviceAddress {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBufferDeviceAddress {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.buffer_device_address != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.buffer_device_address = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureBufferDeviceAddressCaptureReplay {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBufferDeviceAddressCaptureReplay {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.buffer_device_address_capture_replay != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.buffer_device_address_capture_replay = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureBufferDeviceAddressMultiDevice {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBufferDeviceAddressMultiDevice {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.buffer_device_address_multi_device != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.buffer_device_address_multi_device = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVulkanMemoryModel {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVulkanMemoryModel {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vulkan_memory_model != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vulkan_memory_model = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVulkanMemoryModelDeviceScope {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVulkanMemoryModelDeviceScope {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vulkan_memory_model_device_scope != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vulkan_memory_model_device_scope = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVulkanMemoryModelAvailabilityVisibilityChains {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVulkanMemoryModelAvailabilityVisibilityChains {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vulkan_memory_model_availability_visibility_chains != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vulkan_memory_model_availability_visibility_chains = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderOutputViewportIndex {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderOutputViewportIndex {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_output_viewport_index != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_output_viewport_index = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderOutputLayer {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderOutputLayer {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_output_layer != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_output_layer = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSubgroupBroadcastDynamicId {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSubgroupBroadcastDynamicId {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan12Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.subgroup_broadcast_dynamic_id != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.subgroup_broadcast_dynamic_id = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_diagnostics_config_features_nv {
    pub struct FeatureDiagnosticsConfig {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDiagnosticsConfig {
        type VkFeatureType = ash::vk::PhysicalDeviceDiagnosticsConfigFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvDeviceDiagnosticsConfig;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.diagnostics_config != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.diagnostics_config = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_fragment_shading_rate_features_khr {
    pub struct FeaturePipelineFragmentShadingRate {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePipelineFragmentShadingRate {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShadingRateFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrFragmentShadingRate;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.pipeline_fragment_shading_rate != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.pipeline_fragment_shading_rate = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeaturePrimitiveFragmentShadingRate {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePrimitiveFragmentShadingRate {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShadingRateFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrFragmentShadingRate;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.primitive_fragment_shading_rate != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.primitive_fragment_shading_rate = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureAttachmentFragmentShadingRate {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureAttachmentFragmentShadingRate {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShadingRateFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrFragmentShadingRate;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.attachment_fragment_shading_rate != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.attachment_fragment_shading_rate = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_line_rasterization_features_ext {
    pub struct FeatureRectangularLines {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRectangularLines {
        type VkFeatureType = ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtLineRasterization;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.rectangular_lines != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.rectangular_lines = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureBresenhamLines {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBresenhamLines {
        type VkFeatureType = ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtLineRasterization;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.bresenham_lines != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.bresenham_lines = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSmoothLines {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSmoothLines {
        type VkFeatureType = ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtLineRasterization;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.smooth_lines != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.smooth_lines = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStippledRectangularLines {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStippledRectangularLines {
        type VkFeatureType = ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtLineRasterization;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.stippled_rectangular_lines != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.stippled_rectangular_lines = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStippledBresenhamLines {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStippledBresenhamLines {
        type VkFeatureType = ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtLineRasterization;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.stippled_bresenham_lines != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.stippled_bresenham_lines = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStippledSmoothLines {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStippledSmoothLines {
        type VkFeatureType = ash::vk::PhysicalDeviceLineRasterizationFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtLineRasterization;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.stippled_smooth_lines != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.stippled_smooth_lines = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_provoking_vertex_features_ext {
    pub struct FeatureProvokingVertexLast {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureProvokingVertexLast {
        type VkFeatureType = ash::vk::PhysicalDeviceProvokingVertexFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtProvokingVertex;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.provoking_vertex_last != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.provoking_vertex_last = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTransformFeedbackPreservesProvokingVertex {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTransformFeedbackPreservesProvokingVertex {
        type VkFeatureType = ash::vk::PhysicalDeviceProvokingVertexFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtProvokingVertex;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.transform_feedback_preserves_provoking_vertex != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.transform_feedback_preserves_provoking_vertex = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_extended_dynamic_state_features_ext {
    pub struct FeatureExtendedDynamicState {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureExtendedDynamicState {
        type VkFeatureType = ash::vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.extended_dynamic_state != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.extended_dynamic_state = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_present_id_features_khr {
    pub struct FeaturePresentId {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePresentId {
        type VkFeatureType = ash::vk::PhysicalDevicePresentIdFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPresentId;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.present_id != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.present_id = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_device_memory_report_features_ext {
    pub struct FeatureDeviceMemoryReport {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDeviceMemoryReport {
        type VkFeatureType = ash::vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtDeviceMemoryReport;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.device_memory_report != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.device_memory_report = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device16_bit_storage_features {
    pub struct FeatureStorageBuffer16BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStorageBuffer16BitAccess {
        type VkFeatureType = ash::vk::PhysicalDevice16BitStorageFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_buffer16_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_buffer16_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureUniformAndStorageBuffer16BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureUniformAndStorageBuffer16BitAccess {
        type VkFeatureType = ash::vk::PhysicalDevice16BitStorageFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.uniform_and_storage_buffer16_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.uniform_and_storage_buffer16_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStoragePushConstant16 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStoragePushConstant16 {
        type VkFeatureType = ash::vk::PhysicalDevice16BitStorageFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_push_constant16 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_push_constant16 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStorageInputOutput16 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStorageInputOutput16 {
        type VkFeatureType = ash::vk::PhysicalDevice16BitStorageFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_input_output16 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_input_output16 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_buffer_device_address_features {
    pub struct FeatureBufferDeviceAddress {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBufferDeviceAddress {
        type VkFeatureType = ash::vk::PhysicalDeviceBufferDeviceAddressFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.buffer_device_address != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.buffer_device_address = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureBufferDeviceAddressCaptureReplay {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBufferDeviceAddressCaptureReplay {
        type VkFeatureType = ash::vk::PhysicalDeviceBufferDeviceAddressFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.buffer_device_address_capture_replay != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.buffer_device_address_capture_replay = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureBufferDeviceAddressMultiDevice {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBufferDeviceAddressMultiDevice {
        type VkFeatureType = ash::vk::PhysicalDeviceBufferDeviceAddressFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.buffer_device_address_multi_device != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.buffer_device_address_multi_device = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_coherent_memory_features_amd {
    pub struct FeatureDeviceCoherentMemory {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDeviceCoherentMemory {
        type VkFeatureType = ash::vk::PhysicalDeviceCoherentMemoryFeaturesAMD;
        type RequiredExtension = crate::extensions::ExtensionAmdDeviceCoherentMemory;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.device_coherent_memory != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.device_coherent_memory = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_color_write_enable_features_ext {
    pub struct FeatureColorWriteEnable {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureColorWriteEnable {
        type VkFeatureType = ash::vk::PhysicalDeviceColorWriteEnableFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtColorWriteEnable;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.color_write_enable != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.color_write_enable = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_multi_draw_features_ext {
    pub struct FeatureMultiDraw {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultiDraw {
        type VkFeatureType = ash::vk::PhysicalDeviceMultiDrawFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtMultiDraw;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multi_draw != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multi_draw = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_sampler_ycbcr_conversion_features {
    pub struct FeatureSamplerYcbcrConversion {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSamplerYcbcrConversion {
        type VkFeatureType = ash::vk::PhysicalDeviceSamplerYcbcrConversionFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sampler_ycbcr_conversion != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sampler_ycbcr_conversion = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_custom_border_color_features_ext {
    pub struct FeatureCustomBorderColors {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureCustomBorderColors {
        type VkFeatureType = ash::vk::PhysicalDeviceCustomBorderColorFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtCustomBorderColor;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.custom_border_colors != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.custom_border_colors = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureCustomBorderColorWithoutFormat {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureCustomBorderColorWithoutFormat {
        type VkFeatureType = ash::vk::PhysicalDeviceCustomBorderColorFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtCustomBorderColor;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.custom_border_color_without_format != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.custom_border_color_without_format = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_border_color_swizzle_features_ext {
    pub struct FeatureBorderColorSwizzle {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBorderColorSwizzle {
        type VkFeatureType = ash::vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtBorderColorSwizzle;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.border_color_swizzle != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.border_color_swizzle = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureBorderColorSwizzleFromImage {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureBorderColorSwizzleFromImage {
        type VkFeatureType = ash::vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtBorderColorSwizzle;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.border_color_swizzle_from_image != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.border_color_swizzle_from_image = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_acceleration_structure_features_khr {
    pub struct FeatureAccelerationStructure {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureAccelerationStructure {
        type VkFeatureType = ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrAccelerationStructure;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.acceleration_structure != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.acceleration_structure = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureAccelerationStructureCaptureReplay {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureAccelerationStructureCaptureReplay {
        type VkFeatureType = ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrAccelerationStructure;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.acceleration_structure_capture_replay != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.acceleration_structure_capture_replay = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureAccelerationStructureIndirectBuild {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureAccelerationStructureIndirectBuild {
        type VkFeatureType = ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrAccelerationStructure;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.acceleration_structure_indirect_build != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.acceleration_structure_indirect_build = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureAccelerationStructureHostCommands {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureAccelerationStructureHostCommands {
        type VkFeatureType = ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrAccelerationStructure;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.acceleration_structure_host_commands != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.acceleration_structure_host_commands = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingAccelerationStructureUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature
        for FeatureDescriptorBindingAccelerationStructureUpdateAfterBind
    {
        type VkFeatureType = ash::vk::PhysicalDeviceAccelerationStructureFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrAccelerationStructure;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_acceleration_structure_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_acceleration_structure_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_blend_operation_advanced_features_ext {
    pub struct FeatureAdvancedBlendCoherentOperations {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureAdvancedBlendCoherentOperations {
        type VkFeatureType = ash::vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtBlendOperationAdvanced;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.advanced_blend_coherent_operations != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.advanced_blend_coherent_operations = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_linear_color_attachment_features_nv {
    pub struct FeatureLinearColorAttachment {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureLinearColorAttachment {
        type VkFeatureType = ash::vk::PhysicalDeviceLinearColorAttachmentFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvLinearColorAttachment;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.linear_color_attachment != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.linear_color_attachment = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_conditional_rendering_features_ext {
    pub struct FeatureConditionalRendering {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureConditionalRendering {
        type VkFeatureType = ash::vk::PhysicalDeviceConditionalRenderingFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtConditionalRendering;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.conditional_rendering != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.conditional_rendering = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureInheritedConditionalRendering {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureInheritedConditionalRendering {
        type VkFeatureType = ash::vk::PhysicalDeviceConditionalRenderingFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtConditionalRendering;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.inherited_conditional_rendering != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.inherited_conditional_rendering = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_ycbcr_image_arrays_features_ext {
    pub struct FeatureYcbcrImageArrays {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureYcbcrImageArrays {
        type VkFeatureType = ash::vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtYcbcrImageArrays;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.ycbcr_image_arrays != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.ycbcr_image_arrays = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_image_robustness_features {
    pub struct FeatureRobustImageAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRobustImageAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceImageRobustnessFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.robust_image_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.robust_image_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device8_bit_storage_features {
    pub struct FeatureStorageBuffer8BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStorageBuffer8BitAccess {
        type VkFeatureType = ash::vk::PhysicalDevice8BitStorageFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_buffer8_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_buffer8_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureUniformAndStorageBuffer8BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureUniformAndStorageBuffer8BitAccess {
        type VkFeatureType = ash::vk::PhysicalDevice8BitStorageFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.uniform_and_storage_buffer8_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.uniform_and_storage_buffer8_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStoragePushConstant8 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStoragePushConstant8 {
        type VkFeatureType = ash::vk::PhysicalDevice8BitStorageFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_push_constant8 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_push_constant8 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_vulkan11_features {
    pub struct FeatureStorageBuffer16BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStorageBuffer16BitAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_buffer16_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_buffer16_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureUniformAndStorageBuffer16BitAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureUniformAndStorageBuffer16BitAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.uniform_and_storage_buffer16_bit_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.uniform_and_storage_buffer16_bit_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStoragePushConstant16 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStoragePushConstant16 {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_push_constant16 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_push_constant16 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureStorageInputOutput16 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureStorageInputOutput16 {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.storage_input_output16 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.storage_input_output16 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMultiview {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultiview {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multiview != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multiview = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMultiviewGeometryShader {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultiviewGeometryShader {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multiview_geometry_shader != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multiview_geometry_shader = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMultiviewTessellationShader {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultiviewTessellationShader {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multiview_tessellation_shader != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multiview_tessellation_shader = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVariablePointersStorageBuffer {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVariablePointersStorageBuffer {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.variable_pointers_storage_buffer != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.variable_pointers_storage_buffer = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVariablePointers {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVariablePointers {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.variable_pointers != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.variable_pointers = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureProtectedMemory {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureProtectedMemory {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.protected_memory != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.protected_memory = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSamplerYcbcrConversion {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSamplerYcbcrConversion {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sampler_ycbcr_conversion != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sampler_ycbcr_conversion = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderDrawParameters {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderDrawParameters {
        type VkFeatureType = ash::vk::PhysicalDeviceVulkan11Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_draw_parameters != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_draw_parameters = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_depth_clip_control_features_ext {
    pub struct FeatureDepthClipControl {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDepthClipControl {
        type VkFeatureType = ash::vk::PhysicalDeviceDepthClipControlFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtDepthClipControl;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.depth_clip_control != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.depth_clip_control = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_astc_decode_features_ext {
    pub struct FeatureDecodeModeSharedExponent {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDecodeModeSharedExponent {
        type VkFeatureType = ash::vk::PhysicalDeviceASTCDecodeFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtAstcDecodeMode;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.decode_mode_shared_exponent != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.decode_mode_shared_exponent = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_cooperative_matrix_features_nv {
    pub struct FeatureCooperativeMatrix {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureCooperativeMatrix {
        type VkFeatureType = ash::vk::PhysicalDeviceCooperativeMatrixFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvCooperativeMatrix;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.cooperative_matrix != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.cooperative_matrix = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureCooperativeMatrixRobustBufferAccess {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureCooperativeMatrixRobustBufferAccess {
        type VkFeatureType = ash::vk::PhysicalDeviceCooperativeMatrixFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvCooperativeMatrix;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.cooperative_matrix_robust_buffer_access != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.cooperative_matrix_robust_buffer_access = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_variable_pointers_features {
    pub struct FeatureVariablePointersStorageBuffer {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVariablePointersStorageBuffer {
        type VkFeatureType = ash::vk::PhysicalDeviceVariablePointersFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.variable_pointers_storage_buffer != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.variable_pointers_storage_buffer = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVariablePointers {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVariablePointers {
        type VkFeatureType = ash::vk::PhysicalDeviceVariablePointersFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.variable_pointers != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.variable_pointers = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_portability_subset_features_khr {
    pub struct FeatureConstantAlphaColorBlendFactors {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureConstantAlphaColorBlendFactors {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.constant_alpha_color_blend_factors != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.constant_alpha_color_blend_factors = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureEvents {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureEvents {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.events != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.events = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureImageViewFormatReinterpretation {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureImageViewFormatReinterpretation {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.image_view_format_reinterpretation != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.image_view_format_reinterpretation = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureImageViewFormatSwizzle {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureImageViewFormatSwizzle {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.image_view_format_swizzle != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.image_view_format_swizzle = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureImageView2DOn3DImage {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureImageView2DOn3DImage {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.image_view2_d_on3_d_image != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.image_view2_d_on3_d_image = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMultisampleArrayImage {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMultisampleArrayImage {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.multisample_array_image != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.multisample_array_image = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMutableComparisonSamplers {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMutableComparisonSamplers {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.mutable_comparison_samplers != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.mutable_comparison_samplers = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeaturePointPolygons {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePointPolygons {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.point_polygons != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.point_polygons = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSamplerMipLodBias {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSamplerMipLodBias {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sampler_mip_lod_bias != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sampler_mip_lod_bias = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSeparateStencilMaskRef {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSeparateStencilMaskRef {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.separate_stencil_mask_ref != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.separate_stencil_mask_ref = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSampleRateInterpolationFunctions {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSampleRateInterpolationFunctions {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_sample_rate_interpolation_functions != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_sample_rate_interpolation_functions = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTessellationIsolines {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTessellationIsolines {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.tessellation_isolines != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.tessellation_isolines = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTessellationPointMode {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTessellationPointMode {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.tessellation_point_mode != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.tessellation_point_mode = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureTriangleFans {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTriangleFans {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.triangle_fans != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.triangle_fans = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVertexAttributeAccessBeyondStride {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVertexAttributeAccessBeyondStride {
        type VkFeatureType = ash::vk::PhysicalDevicePortabilitySubsetFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrPortabilitySubset;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vertex_attribute_access_beyond_stride != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vertex_attribute_access_beyond_stride = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_image_footprint_features_nv {
    pub struct FeatureImageFootprint {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureImageFootprint {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderImageFootprintFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvShaderImageFootprint;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.image_footprint != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.image_footprint = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_texel_buffer_alignment_features_ext {
    pub struct FeatureTexelBufferAlignment {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTexelBufferAlignment {
        type VkFeatureType = ash::vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.texel_buffer_alignment != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.texel_buffer_alignment = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_imageless_framebuffer_features {
    pub struct FeatureImagelessFramebuffer {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureImagelessFramebuffer {
        type VkFeatureType = ash::vk::PhysicalDeviceImagelessFramebufferFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.imageless_framebuffer != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.imageless_framebuffer = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_sm_builtins_features_nv {
    pub struct FeatureShaderSmBuiltins {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSmBuiltins {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvShaderSmBuiltins;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_sm_builtins != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_sm_builtins = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_fragment_shader_barycentric_features_nv {
    pub struct FeatureFragmentShaderBarycentric {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentShaderBarycentric {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvFragmentShaderBarycentric;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_shader_barycentric != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_shader_barycentric = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_texture_compression_astchdr_features {
    pub struct FeatureTextureCompressionAstcHdr {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTextureCompressionAstcHdr {
        type VkFeatureType = ash::vk::PhysicalDeviceTextureCompressionASTCHDRFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.texture_compression_astc_hdr != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.texture_compression_astc_hdr = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_pageable_device_local_memory_features_ext {
    pub struct FeaturePageableDeviceLocalMemory {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeaturePageableDeviceLocalMemory {
        type VkFeatureType = ash::vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtPageableDeviceLocalMemory;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.pageable_device_local_memory != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.pageable_device_local_memory = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_zero_initialize_workgroup_memory_features {
    pub struct FeatureShaderZeroInitializeWorkgroupMemory {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderZeroInitializeWorkgroupMemory {
        type VkFeatureType = ash::vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_zero_initialize_workgroup_memory != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_zero_initialize_workgroup_memory = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_robustness2_features_ext {
    pub struct FeatureRobustBufferAccess2 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRobustBufferAccess2 {
        type VkFeatureType = ash::vk::PhysicalDeviceRobustness2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtRobustness2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.robust_buffer_access2 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.robust_buffer_access2 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRobustImageAccess2 {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRobustImageAccess2 {
        type VkFeatureType = ash::vk::PhysicalDeviceRobustness2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtRobustness2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.robust_image_access2 != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.robust_image_access2 = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureNullDescriptor {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureNullDescriptor {
        type VkFeatureType = ash::vk::PhysicalDeviceRobustness2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtRobustness2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.null_descriptor != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.null_descriptor = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_terminate_invocation_features {
    pub struct FeatureShaderTerminateInvocation {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderTerminateInvocation {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderTerminateInvocationFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_terminate_invocation != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_terminate_invocation = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_global_priority_query_features_khr {
    pub struct FeatureGlobalPriorityQuery {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureGlobalPriorityQuery {
        type VkFeatureType = ash::vk::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrGlobalPriority;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.global_priority_query != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.global_priority_query = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_atomic_float_features_ext {
    pub struct FeatureShaderBufferFloat32Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferFloat32Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_float32_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_float32_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderBufferFloat32AtomicAdd {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferFloat32AtomicAdd {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_float32_atomic_add != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_float32_atomic_add = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderBufferFloat64Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferFloat64Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_float64_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_float64_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderBufferFloat64AtomicAdd {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferFloat64AtomicAdd {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_float64_atomic_add != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_float64_atomic_add = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedFloat32Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedFloat32Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_float32_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_float32_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedFloat32AtomicAdd {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedFloat32AtomicAdd {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_float32_atomic_add != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_float32_atomic_add = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedFloat64Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedFloat64Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_float64_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_float64_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedFloat64AtomicAdd {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedFloat64AtomicAdd {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_float64_atomic_add != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_float64_atomic_add = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderImageFloat32Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderImageFloat32Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_image_float32_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_image_float32_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderImageFloat32AtomicAdd {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderImageFloat32AtomicAdd {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_image_float32_atomic_add != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_image_float32_atomic_add = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseImageFloat32Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseImageFloat32Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_image_float32_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_image_float32_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSparseImageFloat32AtomicAdd {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSparseImageFloat32AtomicAdd {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtShaderAtomicFloat;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.sparse_image_float32_atomic_add != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.sparse_image_float32_atomic_add = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_vertex_attribute_divisor_features_ext {
    pub struct FeatureVertexAttributeInstanceRateDivisor {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVertexAttributeInstanceRateDivisor {
        type VkFeatureType = ash::vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtVertexAttributeDivisor;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vertex_attribute_instance_rate_divisor != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vertex_attribute_instance_rate_divisor = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureVertexAttributeInstanceRateZeroDivisor {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureVertexAttributeInstanceRateZeroDivisor {
        type VkFeatureType = ash::vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtVertexAttributeDivisor;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.vertex_attribute_instance_rate_zero_divisor != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.vertex_attribute_instance_rate_zero_divisor = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_descriptor_indexing_features {
    pub struct FeatureShaderInputAttachmentArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderInputAttachmentArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_input_attachment_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_input_attachment_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderUniformTexelBufferArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderUniformTexelBufferArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_uniform_texel_buffer_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_uniform_texel_buffer_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageTexelBufferArrayDynamicIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageTexelBufferArrayDynamicIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_texel_buffer_array_dynamic_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_texel_buffer_array_dynamic_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderUniformBufferArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderUniformBufferArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_uniform_buffer_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_uniform_buffer_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSampledImageArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSampledImageArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_sampled_image_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_sampled_image_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageBufferArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageBufferArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_buffer_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_buffer_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageImageArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageImageArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_image_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_image_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderInputAttachmentArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderInputAttachmentArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_input_attachment_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_input_attachment_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderUniformTexelBufferArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderUniformTexelBufferArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_uniform_texel_buffer_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_uniform_texel_buffer_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderStorageTexelBufferArrayNonUniformIndexing {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderStorageTexelBufferArrayNonUniformIndexing {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_storage_texel_buffer_array_non_uniform_indexing != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_storage_texel_buffer_array_non_uniform_indexing = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingUniformBufferUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingUniformBufferUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_uniform_buffer_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_uniform_buffer_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingSampledImageUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingSampledImageUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_sampled_image_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_sampled_image_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingStorageImageUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingStorageImageUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_storage_image_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_storage_image_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingStorageBufferUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingStorageBufferUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_storage_buffer_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_storage_buffer_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingUniformTexelBufferUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingUniformTexelBufferUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_uniform_texel_buffer_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_uniform_texel_buffer_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingStorageTexelBufferUpdateAfterBind {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingStorageTexelBufferUpdateAfterBind {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_storage_texel_buffer_update_after_bind != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_storage_texel_buffer_update_after_bind = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingUpdateUnusedWhilePending {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingUpdateUnusedWhilePending {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_update_unused_while_pending != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_update_unused_while_pending = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingPartiallyBound {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingPartiallyBound {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_partially_bound != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_partially_bound = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureDescriptorBindingVariableDescriptorCount {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDescriptorBindingVariableDescriptorCount {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.descriptor_binding_variable_descriptor_count != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.descriptor_binding_variable_descriptor_count = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureRuntimeDescriptorArray {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRuntimeDescriptorArray {
        type VkFeatureType = ash::vk::PhysicalDeviceDescriptorIndexingFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.runtime_descriptor_array != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.runtime_descriptor_array = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_fragment_density_map2_features_ext {
    pub struct FeatureFragmentDensityMapDeferred {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentDensityMapDeferred {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtFragmentDensityMap2;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_density_map_deferred != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_density_map_deferred = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_uniform_buffer_standard_layout_features {
    pub struct FeatureUniformBufferStandardLayout {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureUniformBufferStandardLayout {
        type VkFeatureType = ash::vk::PhysicalDeviceUniformBufferStandardLayoutFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.uniform_buffer_standard_layout != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.uniform_buffer_standard_layout = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_memory_priority_features_ext {
    pub struct FeatureMemoryPriority {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMemoryPriority {
        type VkFeatureType = ash::vk::PhysicalDeviceMemoryPriorityFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtMemoryPriority;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.memory_priority != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.memory_priority = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_scalar_block_layout_features {
    pub struct FeatureScalarBlockLayout {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureScalarBlockLayout {
        type VkFeatureType = ash::vk::PhysicalDeviceScalarBlockLayoutFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.scalar_block_layout != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.scalar_block_layout = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_demote_to_helper_invocation_features {
    pub struct FeatureShaderDemoteToHelperInvocation {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderDemoteToHelperInvocation {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_demote_to_helper_invocation != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_demote_to_helper_invocation = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_subgroup_size_control_features {
    pub struct FeatureSubgroupSizeControl {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSubgroupSizeControl {
        type VkFeatureType = ash::vk::PhysicalDeviceSubgroupSizeControlFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.subgroup_size_control != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.subgroup_size_control = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureComputeFullSubgroups {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureComputeFullSubgroups {
        type VkFeatureType = ash::vk::PhysicalDeviceSubgroupSizeControlFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.compute_full_subgroups != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.compute_full_subgroups = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_atomic_int64_features {
    pub struct FeatureShaderBufferInt64Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderBufferInt64Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicInt64Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_buffer_int64_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_buffer_int64_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderSharedInt64Atomics {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSharedInt64Atomics {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderAtomicInt64Features;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_shared_int64_atomics != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_shared_int64_atomics = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_mesh_shader_features_nv {
    pub struct FeatureTaskShader {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureTaskShader {
        type VkFeatureType = ash::vk::PhysicalDeviceMeshShaderFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvMeshShader;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.task_shader != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.task_shader = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureMeshShader {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureMeshShader {
        type VkFeatureType = ash::vk::PhysicalDeviceMeshShaderFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvMeshShader;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.mesh_shader != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.mesh_shader = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_clock_features_khr {
    pub struct FeatureShaderSubgroupClock {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderSubgroupClock {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderClockFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrShaderClock;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_subgroup_clock != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_subgroup_clock = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureShaderDeviceClock {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderDeviceClock {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderClockFeaturesKHR;
        type RequiredExtension = crate::extensions::ExtensionKhrShaderClock;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_device_clock != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_device_clock = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_fragment_shader_interlock_features_ext {
    pub struct FeatureFragmentShaderSampleInterlock {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentShaderSampleInterlock {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtFragmentShaderInterlock;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_shader_sample_interlock != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_shader_sample_interlock = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureFragmentShaderPixelInterlock {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentShaderPixelInterlock {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtFragmentShaderInterlock;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_shader_pixel_interlock != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_shader_pixel_interlock = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureFragmentShaderShadingRateInterlock {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentShaderShadingRateInterlock {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtFragmentShaderInterlock;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_shader_shading_rate_interlock != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_shader_shading_rate_interlock = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_representative_fragment_test_features_nv {
    pub struct FeatureRepresentativeFragmentTest {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureRepresentativeFragmentTest {
        type VkFeatureType = ash::vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvRepresentativeFragmentTest;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.representative_fragment_test != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.representative_fragment_test = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_fragment_shading_rate_enums_features_nv {
    pub struct FeatureFragmentShadingRateEnums {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureFragmentShadingRateEnums {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvFragmentShadingRateEnums;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.fragment_shading_rate_enums != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.fragment_shading_rate_enums = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureSupersampleFragmentShadingRates {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureSupersampleFragmentShadingRates {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvFragmentShadingRateEnums;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.supersample_fragment_shading_rates != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.supersample_fragment_shading_rates = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
    pub struct FeatureNoInvocationFragmentShadingRates {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureNoInvocationFragmentShadingRates {
        type VkFeatureType = ash::vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvFragmentShadingRateEnums;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.no_invocation_fragment_shading_rates != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.no_invocation_fragment_shading_rates = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_device_generated_commands_features_nv {
    pub struct FeatureDeviceGeneratedCommands {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDeviceGeneratedCommands {
        type VkFeatureType = ash::vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvDeviceGeneratedCommands;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.device_generated_commands != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.device_generated_commands = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_exclusive_scissor_features_nv {
    pub struct FeatureExclusiveScissor {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureExclusiveScissor {
        type VkFeatureType = ash::vk::PhysicalDeviceExclusiveScissorFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvScissorExclusive;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.exclusive_scissor != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.exclusive_scissor = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_corner_sampled_image_features_nv {
    pub struct FeatureCornerSampledImage {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureCornerSampledImage {
        type VkFeatureType = ash::vk::PhysicalDeviceCornerSampledImageFeaturesNV;
        type RequiredExtension = crate::extensions::ExtensionNvCornerSampledImage;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.corner_sampled_image != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.corner_sampled_image = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_depth_clip_enable_features_ext {
    pub struct FeatureDepthClipEnable {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureDepthClipEnable {
        type VkFeatureType = ash::vk::PhysicalDeviceDepthClipEnableFeaturesEXT;
        type RequiredExtension = crate::extensions::ExtensionExtDepthClipEnable;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.depth_clip_enable != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.depth_clip_enable = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
pub mod physical_device_shader_draw_parameters_features {
    pub struct FeatureShaderDrawParameters {
        pub device: std::sync::Arc<crate::device::Device>,
    }
    impl crate::device_features::Feature for FeatureShaderDrawParameters {
        type VkFeatureType = ash::vk::PhysicalDeviceShaderDrawParametersFeatures;
        type RequiredExtension = crate::device_features::ExtensionNone;
        const IS_FEATURE2: bool = true;
        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
            vk_feature.shader_draw_parameters != 0
        }
        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
            vk_feature.shader_draw_parameters = 1;
        }
        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
            Self {
                device: device.clone(),
            }
        }
    }
}
