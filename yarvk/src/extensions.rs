#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalInstanceExtensionType {
    KhrWin32Surface,
    KhrAndroidSurface,
    GoogleSurfacelessQuery,
    ExtDebugReport,
    ExtDebugUtils,
    ExtValidationFlags,
    KhrWaylandSurface,
    KhrPortabilityEnumeration,
    ExtDisplaySurfaceCounter,
    ExtValidationFeatures,
    MvkMacosSurface,
    KhrXcbSurface,
    MvkIosSurface,
    GgpStreamDescriptorSurface,
    ExtDirectModeDisplay,
    KhrSurfaceProtectedCapabilities,
    KhrXlibSurface,
    ExtDirectfbSurface,
    NvExternalMemoryCapabilities,
    KhrSurface,
    NnViSurface,
    ExtMetalSurface,
    ExtAcquireXlibDisplay,
    ExtSwapchainColorspace,
    KhrGetDisplayProperties2,
    QnxScreenSurface,
    ExtHeadlessSurface,
    KhrDisplay,
    FuchsiaImagepipeSurface,
    ExtAcquireDrmDisplay,
    KhrGetSurfaceCapabilities2,
}
impl PhysicalInstanceExtensionType {
    pub fn to_cstr(&self) -> &'static std::ffi::CStr {
        match self {
            Self::KhrWin32Surface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_surface\0")
            },
            Self::KhrAndroidSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_android_surface\0")
            },
            Self::GoogleSurfacelessQuery => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_surfaceless_query\0")
            },
            Self::ExtDebugReport => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_report\0")
            },
            Self::ExtDebugUtils => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_utils\0")
            },
            Self::ExtValidationFlags => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_flags\0")
            },
            Self::KhrWaylandSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_wayland_surface\0")
            },
            Self::KhrPortabilityEnumeration => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_enumeration\0")
            },
            Self::ExtDisplaySurfaceCounter => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_surface_counter\0")
            },
            Self::ExtValidationFeatures => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_features\0")
            },
            Self::MvkMacosSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MVK_macos_surface\0")
            },
            Self::KhrXcbSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xcb_surface\0")
            },
            Self::MvkIosSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MVK_ios_surface\0")
            },
            Self::GgpStreamDescriptorSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_stream_descriptor_surface\0")
            },
            Self::ExtDirectModeDisplay => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_direct_mode_display\0")
            },
            Self::KhrSurfaceProtectedCapabilities => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_KHR_surface_protected_capabilities\0",
                )
            },
            Self::KhrXlibSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xlib_surface\0")
            },
            Self::ExtDirectfbSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_directfb_surface\0")
            },
            Self::NvExternalMemoryCapabilities => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NV_external_memory_capabilities\0",
                )
            },
            Self::KhrSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface\0")
            },
            Self::NnViSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NN_vi_surface\0")
            },
            Self::ExtMetalSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_metal_surface\0")
            },
            Self::ExtAcquireXlibDisplay => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_acquire_xlib_display\0")
            },
            Self::ExtSwapchainColorspace => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_swapchain_colorspace\0")
            },
            Self::KhrGetDisplayProperties2 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_display_properties2\0")
            },
            Self::QnxScreenSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QNX_screen_surface\0")
            },
            Self::ExtHeadlessSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_headless_surface\0")
            },
            Self::KhrDisplay => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display\0")
            },
            Self::FuchsiaImagepipeSurface => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_imagepipe_surface\0")
            },
            Self::ExtAcquireDrmDisplay => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_acquire_drm_display\0")
            },
            Self::KhrGetSurfaceCapabilities2 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_surface_capabilities2\0")
            },
        }
    }
    pub fn get_dependencies(&self) -> &[Self] {
        match self {
            Self::KhrWin32Surface => &[Self::KhrSurface],
            Self::KhrAndroidSurface => &[Self::KhrSurface],
            Self::GoogleSurfacelessQuery => &[Self::KhrSurface],
            Self::ExtDebugReport => &[],
            Self::ExtDebugUtils => &[],
            Self::ExtValidationFlags => &[],
            Self::KhrWaylandSurface => &[Self::KhrSurface],
            Self::KhrPortabilityEnumeration => &[],
            Self::ExtDisplaySurfaceCounter => &[Self::KhrDisplay, Self::KhrSurface],
            Self::ExtValidationFeatures => &[],
            Self::MvkMacosSurface => &[Self::KhrSurface],
            Self::KhrXcbSurface => &[Self::KhrSurface],
            Self::MvkIosSurface => &[Self::KhrSurface],
            Self::GgpStreamDescriptorSurface => &[Self::KhrSurface],
            Self::ExtDirectModeDisplay => &[Self::KhrDisplay, Self::KhrSurface],
            Self::KhrSurfaceProtectedCapabilities => {
                &[Self::KhrGetSurfaceCapabilities2, Self::KhrSurface]
            }
            Self::KhrXlibSurface => &[Self::KhrSurface],
            Self::ExtDirectfbSurface => &[Self::KhrSurface],
            Self::NvExternalMemoryCapabilities => &[],
            Self::KhrSurface => &[],
            Self::NnViSurface => &[Self::KhrSurface],
            Self::ExtMetalSurface => &[Self::KhrSurface],
            Self::ExtAcquireXlibDisplay => &[
                Self::KhrSurface,
                Self::ExtDirectModeDisplay,
                Self::KhrDisplay,
            ],
            Self::ExtSwapchainColorspace => &[Self::KhrSurface],
            Self::KhrGetDisplayProperties2 => &[Self::KhrDisplay, Self::KhrSurface],
            Self::QnxScreenSurface => &[Self::KhrSurface],
            Self::ExtHeadlessSurface => &[Self::KhrSurface],
            Self::KhrDisplay => &[Self::KhrSurface],
            Self::FuchsiaImagepipeSurface => &[Self::KhrSurface],
            Self::ExtAcquireDrmDisplay => &[
                Self::KhrSurface,
                Self::ExtDirectModeDisplay,
                Self::KhrDisplay,
            ],
            Self::KhrGetSurfaceCapabilities2 => &[Self::KhrSurface],
        }
    }
    pub fn from_cstr(vk_name: &'static std::ffi::CStr) -> Option<Self> {
        let bytes = vk_name.to_bytes_with_nul();
        match bytes {
            b"VK_KHR_win32_surface\0" => Some(Self::KhrWin32Surface),
            b"VK_KHR_android_surface\0" => Some(Self::KhrAndroidSurface),
            b"VK_GOOGLE_surfaceless_query\0" => Some(Self::GoogleSurfacelessQuery),
            b"VK_EXT_debug_report\0" => Some(Self::ExtDebugReport),
            b"VK_EXT_debug_utils\0" => Some(Self::ExtDebugUtils),
            b"VK_EXT_validation_flags\0" => Some(Self::ExtValidationFlags),
            b"VK_KHR_wayland_surface\0" => Some(Self::KhrWaylandSurface),
            b"VK_KHR_portability_enumeration\0" => Some(Self::KhrPortabilityEnumeration),
            b"VK_EXT_display_surface_counter\0" => Some(Self::ExtDisplaySurfaceCounter),
            b"VK_EXT_validation_features\0" => Some(Self::ExtValidationFeatures),
            b"VK_MVK_macos_surface\0" => Some(Self::MvkMacosSurface),
            b"VK_KHR_xcb_surface\0" => Some(Self::KhrXcbSurface),
            b"VK_MVK_ios_surface\0" => Some(Self::MvkIosSurface),
            b"VK_GGP_stream_descriptor_surface\0" => Some(Self::GgpStreamDescriptorSurface),
            b"VK_EXT_direct_mode_display\0" => Some(Self::ExtDirectModeDisplay),
            b"VK_KHR_surface_protected_capabilities\0" => {
                Some(Self::KhrSurfaceProtectedCapabilities)
            }
            b"VK_KHR_xlib_surface\0" => Some(Self::KhrXlibSurface),
            b"VK_EXT_directfb_surface\0" => Some(Self::ExtDirectfbSurface),
            b"VK_NV_external_memory_capabilities\0" => Some(Self::NvExternalMemoryCapabilities),
            b"VK_KHR_surface\0" => Some(Self::KhrSurface),
            b"VK_NN_vi_surface\0" => Some(Self::NnViSurface),
            b"VK_EXT_metal_surface\0" => Some(Self::ExtMetalSurface),
            b"VK_EXT_acquire_xlib_display\0" => Some(Self::ExtAcquireXlibDisplay),
            b"VK_EXT_swapchain_colorspace\0" => Some(Self::ExtSwapchainColorspace),
            b"VK_KHR_get_display_properties2\0" => Some(Self::KhrGetDisplayProperties2),
            b"VK_QNX_screen_surface\0" => Some(Self::QnxScreenSurface),
            b"VK_EXT_headless_surface\0" => Some(Self::ExtHeadlessSurface),
            b"VK_KHR_display\0" => Some(Self::KhrDisplay),
            b"VK_FUCHSIA_imagepipe_surface\0" => Some(Self::FuchsiaImagepipeSurface),
            b"VK_EXT_acquire_drm_display\0" => Some(Self::ExtAcquireDrmDisplay),
            b"VK_KHR_get_surface_capabilities2\0" => Some(Self::KhrGetSurfaceCapabilities2),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalDeviceExtensionType {
    NvCoverageReductionMode,
    KhrIncrementalPresent,
    KhrVideoEncodeQueue,
    ExtMemoryBudget,
    NvAcquireWinrtDisplay,
    KhrSwapchainMutableFormat,
    HuaweiSubpassShading,
    AmdShaderBallot,
    AmdDisplayNativeHdr,
    ExtPhysicalDeviceDrm,
    ExtMultiDraw,
    ImgFilterCubic,
    NvxImageViewHandle,
    NvDeviceDiagnosticCheckpoints,
    ExtDepthRangeUnrestricted,
    KhrExternalSemaphoreWin32,
    QcomFragmentDensityMapOffset,
    KhrExternalSemaphoreFd,
    NvComputeShaderDerivatives,
    ExtCalibratedTimestamps,
    ExtMemoryPriority,
    AmdMemoryOverallocationBehavior,
    NvClipSpaceWScaling,
    KhrExternalMemoryWin32,
    AmdShaderExplicitVertexParameter,
    AmdMixedAttachmentSamples,
    NvShaderImageFootprint,
    ExtShaderSubgroupBallot,
    AmdGpuShaderHalfFloat,
    ExtDiscardRectangles,
    AmdPipelineCompilerControl,
    KhrPresentWait,
    NvViewportArray2,
    ExtPostDepthCoverage,
    KhrDisplaySwapchain,
    AmdShaderInfo,
    KhrVideoDecodeQueue,
    NvFillRectangle,
    AmdGcnShader,
    ExtLoadStoreOpNone,
    NvSampleMaskOverrideCoverage,
    NvGeometryShaderPassthrough,
    ExtQueueFamilyForeign,
    ExtImageDrmFormatModifier,
    AmdShaderFragmentMask,
    ExtHdrMetadata,
    ExtFilterCubic,
    ExtPageableDeviceLocalMemory,
    AmdTextureGatherBiasLod,
    ExtTransformFeedback,
    KhrShaderSubgroupUniformControlFlow,
    KhrDeferredHostOperations,
    ExtDepthClipControl,
    ValveDescriptorSetHostMapping,
    ExtIndexTypeUint8,
    KhrPushDescriptor,
    ExtDepthClipEnable,
    NvInheritedViewportScissor,
    AndroidExternalMemoryAndroidHardwareBuffer,
    GoogleHlslFunctionality1,
    ExtShaderImageAtomicInt64,
    ExtShaderAtomicFloat,
    FuchsiaExternalMemory,
    KhrFragmentShadingRate,
    FuchsiaExternalSemaphore,
    AmdShaderCoreProperties,
    GgpFrameToken,
    AmdShaderImageLoadStoreLod,
    ExtShaderSubgroupVote,
    ExtLineRasterization,
    KhrGlobalPriority,
    KhrPresentId,
    KhrVideoQueue,
    KhrShaderClock,
    NvFragmentCoverageToColor,
    ExtDisplayControl,
    NvShaderSmBuiltins,
    KhrSharedPresentableImage,
    ExtVideoDecodeH264,
    QcomRenderPassTransform,
    GoogleUserType,
    AmdRasterizationOrder,
    ExtBufferDeviceAddress,
    ExtConservativeRasterization,
    FuchsiaBufferCollection,
    IntelShaderIntegerFunctions2,
    KhrWorkgroupMemoryExplicitLayout,
    ExtVertexInputDynamicState,
    NvRepresentativeFragmentTest,
    AmdBufferMarker,
    AmdShaderTrinaryMinmax,
    KhrExternalFenceFd,
    NvShadingRateImage,
    NvScissorExclusive,
    NvCooperativeMatrix,
    NvDeviceGeneratedCommands,
    KhrRayQuery,
    ExtPciBusInfo,
    ExtFragmentDensityMap2,
    ArmRasterizationOrderAttachmentAccess,
    ValveMutableDescriptorType,
    ExtConditionalRendering,
    NvFragmentShadingRateEnums,
    KhrExternalFenceWin32,
    KhrPerformanceQuery,
    ImgFormatPvrtc,
    KhrPortabilitySubset,
    QcomRenderPassShaderResolve,
    NvxMultiviewPerViewAttributes,
    ExtValidationCache,
    NvxBinaryImport,
    ExtVideoEncodeH265,
    ExtShaderStencilExport,
    KhrAccelerationStructure,
    KhrSwapchain,
    NvRayTracing,
    ExtAstcDecodeMode,
    ExtFragmentShaderInterlock,
    QcomRotatedCopyCommands,
    ExtVideoDecodeH265,
    ExtShaderAtomicFloat2,
    NvGlslShader,
    ExtFullScreenExclusive,
    ExtYcbcrImageArrays,
    NvRayTracingMotionBlur,
    ExtVertexAttributeDivisor,
    KhrExternalMemoryFd,
    KhrWin32KeyedMutex,
    NvExternalMemoryWin32,
    NvShaderSubgroupPartitioned,
    ExtExternalMemoryHost,
    ExtExternalMemoryDmaBuf,
    GoogleDisplayTiming,
    NvExternalMemory,
    NvExternalMemoryRdma,
    QcomRenderPassStoreOps,
    ExtFragmentDensityMap,
    ExtPrimitiveTopologyListRestart,
    KhrPipelineExecutableProperties,
    NvViewportSwizzle,
    NvLinearColorAttachment,
    ExtImageViewMinLod,
    ExtCustomBorderColor,
    HuaweiInvocationMask,
    IntelPerformanceQuery,
    ExtProvokingVertex,
    KhrPipelineLibrary,
    ExtVideoEncodeH264,
    AmdDeviceCoherentMemory,
    ExtSampleLocations,
    NvFramebufferMixedSamples,
    NvDedicatedAllocation,
    ExtBlendOperationAdvanced,
    KhrRayTracingPipeline,
    NvDedicatedAllocationImageAliasing,
    GoogleDecorateString,
    ExtColorWriteEnable,
    ExtBorderColorSwizzle,
    AmdNegativeViewportHeight,
    ExtRgba10x6Formats,
    NvCornerSampledImage,
    ExtRobustness2,
    NvFragmentShaderBarycentric,
    NvDeviceDiagnosticsConfig,
    NvMeshShader,
    AmdGpuShaderInt16,
    ExtDeviceMemoryReport,
    AmdShaderCoreProperties2,
}
impl PhysicalDeviceExtensionType {
    pub fn to_cstr(&self) -> &'static std::ffi::CStr {
        match self {
            Self::NvCoverageReductionMode => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_coverage_reduction_mode\0")
            },
            Self::KhrIncrementalPresent => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_incremental_present\0")
            },
            Self::KhrVideoEncodeQueue => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_encode_queue\0")
            },
            Self::ExtMemoryBudget => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_budget\0")
            },
            Self::NvAcquireWinrtDisplay => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_acquire_winrt_display\0")
            },
            Self::KhrSwapchainMutableFormat => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain_mutable_format\0")
            },
            Self::HuaweiSubpassShading => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_subpass_shading\0")
            },
            Self::AmdShaderBallot => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_ballot\0")
            },
            Self::AmdDisplayNativeHdr => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_display_native_hdr\0")
            },
            Self::ExtPhysicalDeviceDrm => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_physical_device_drm\0")
            },
            Self::ExtMultiDraw => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_multi_draw\0")
            },
            Self::ImgFilterCubic => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_filter_cubic\0")
            },
            Self::NvxImageViewHandle => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_image_view_handle\0")
            },
            Self::NvDeviceDiagnosticCheckpoints => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NV_device_diagnostic_checkpoints\0",
                )
            },
            Self::ExtDepthRangeUnrestricted => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_range_unrestricted\0")
            },
            Self::KhrExternalSemaphoreWin32 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_win32\0")
            },
            Self::QcomFragmentDensityMapOffset => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_QCOM_fragment_density_map_offset\0",
                )
            },
            Self::KhrExternalSemaphoreFd => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_fd\0")
            },
            Self::NvComputeShaderDerivatives => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_compute_shader_derivatives\0")
            },
            Self::ExtCalibratedTimestamps => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_calibrated_timestamps\0")
            },
            Self::ExtMemoryPriority => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_priority\0")
            },
            Self::AmdMemoryOverallocationBehavior => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_AMD_memory_overallocation_behavior\0",
                )
            },
            Self::NvClipSpaceWScaling => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_clip_space_w_scaling\0")
            },
            Self::KhrExternalMemoryWin32 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_win32\0")
            },
            Self::AmdShaderExplicitVertexParameter => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_AMD_shader_explicit_vertex_parameter\0",
                )
            },
            Self::AmdMixedAttachmentSamples => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_mixed_attachment_samples\0")
            },
            Self::NvShaderImageFootprint => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_image_footprint\0")
            },
            Self::ExtShaderSubgroupBallot => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_ballot\0")
            },
            Self::AmdGpuShaderHalfFloat => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_half_float\0")
            },
            Self::ExtDiscardRectangles => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_discard_rectangles\0")
            },
            Self::AmdPipelineCompilerControl => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_pipeline_compiler_control\0")
            },
            Self::KhrPresentWait => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_present_wait\0")
            },
            Self::NvViewportArray2 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_array2\0")
            },
            Self::ExtPostDepthCoverage => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_post_depth_coverage\0")
            },
            Self::KhrDisplaySwapchain => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display_swapchain\0")
            },
            Self::AmdShaderInfo => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_info\0")
            },
            Self::KhrVideoDecodeQueue => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_decode_queue\0")
            },
            Self::NvFillRectangle => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fill_rectangle\0")
            },
            Self::AmdGcnShader => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gcn_shader\0")
            },
            Self::ExtLoadStoreOpNone => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_load_store_op_none\0")
            },
            Self::NvSampleMaskOverrideCoverage => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NV_sample_mask_override_coverage\0",
                )
            },
            Self::NvGeometryShaderPassthrough => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NV_geometry_shader_passthrough\0",
                )
            },
            Self::ExtQueueFamilyForeign => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_queue_family_foreign\0")
            },
            Self::ExtImageDrmFormatModifier => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_drm_format_modifier\0")
            },
            Self::AmdShaderFragmentMask => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_fragment_mask\0")
            },
            Self::ExtHdrMetadata => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_hdr_metadata\0")
            },
            Self::ExtFilterCubic => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_filter_cubic\0")
            },
            Self::ExtPageableDeviceLocalMemory => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_EXT_pageable_device_local_memory\0",
                )
            },
            Self::AmdTextureGatherBiasLod => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_texture_gather_bias_lod\0")
            },
            Self::ExtTransformFeedback => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_transform_feedback\0")
            },
            Self::KhrShaderSubgroupUniformControlFlow => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_KHR_shader_subgroup_uniform_control_flow\0",
                )
            },
            Self::KhrDeferredHostOperations => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_deferred_host_operations\0")
            },
            Self::ExtDepthClipControl => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clip_control\0")
            },
            Self::ValveDescriptorSetHostMapping => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_VALVE_descriptor_set_host_mapping\0",
                )
            },
            Self::ExtIndexTypeUint8 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_index_type_uint8\0")
            },
            Self::KhrPushDescriptor => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_push_descriptor\0")
            },
            Self::ExtDepthClipEnable => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clip_enable\0")
            },
            Self::NvInheritedViewportScissor => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_inherited_viewport_scissor\0")
            },
            Self::AndroidExternalMemoryAndroidHardwareBuffer => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_ANDROID_external_memory_android_hardware_buffer\0",
                )
            },
            Self::GoogleHlslFunctionality1 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_hlsl_functionality1\0")
            },
            Self::ExtShaderImageAtomicInt64 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_image_atomic_int64\0")
            },
            Self::ExtShaderAtomicFloat => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_atomic_float\0")
            },
            Self::FuchsiaExternalMemory => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_memory\0")
            },
            Self::KhrFragmentShadingRate => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_fragment_shading_rate\0")
            },
            Self::FuchsiaExternalSemaphore => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_semaphore\0")
            },
            Self::AmdShaderCoreProperties => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties\0")
            },
            Self::GgpFrameToken => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_frame_token\0")
            },
            Self::AmdShaderImageLoadStoreLod => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_AMD_shader_image_load_store_lod\0",
                )
            },
            Self::ExtShaderSubgroupVote => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_vote\0")
            },
            Self::ExtLineRasterization => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_line_rasterization\0")
            },
            Self::KhrGlobalPriority => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_global_priority\0")
            },
            Self::KhrPresentId => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_present_id\0")
            },
            Self::KhrVideoQueue => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_queue\0")
            },
            Self::KhrShaderClock => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_clock\0")
            },
            Self::NvFragmentCoverageToColor => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_coverage_to_color\0")
            },
            Self::ExtDisplayControl => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_control\0")
            },
            Self::NvShaderSmBuiltins => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_sm_builtins\0")
            },
            Self::KhrSharedPresentableImage => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shared_presentable_image\0")
            },
            Self::ExtVideoDecodeH264 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_decode_h264\0")
            },
            Self::QcomRenderPassTransform => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_transform\0")
            },
            Self::GoogleUserType => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_user_type\0")
            },
            Self::AmdRasterizationOrder => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_rasterization_order\0")
            },
            Self::ExtBufferDeviceAddress => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_buffer_device_address\0")
            },
            Self::ExtConservativeRasterization => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_EXT_conservative_rasterization\0",
                )
            },
            Self::FuchsiaBufferCollection => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_buffer_collection\0")
            },
            Self::IntelShaderIntegerFunctions2 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_INTEL_shader_integer_functions2\0",
                )
            },
            Self::KhrWorkgroupMemoryExplicitLayout => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_KHR_workgroup_memory_explicit_layout\0",
                )
            },
            Self::ExtVertexInputDynamicState => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_EXT_vertex_input_dynamic_state\0",
                )
            },
            Self::NvRepresentativeFragmentTest => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NV_representative_fragment_test\0",
                )
            },
            Self::AmdBufferMarker => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_buffer_marker\0")
            },
            Self::AmdShaderTrinaryMinmax => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_trinary_minmax\0")
            },
            Self::KhrExternalFenceFd => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_fd\0")
            },
            Self::NvShadingRateImage => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shading_rate_image\0")
            },
            Self::NvScissorExclusive => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_scissor_exclusive\0")
            },
            Self::NvCooperativeMatrix => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_cooperative_matrix\0")
            },
            Self::NvDeviceGeneratedCommands => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_generated_commands\0")
            },
            Self::KhrRayQuery => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_query\0")
            },
            Self::ExtPciBusInfo => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pci_bus_info\0")
            },
            Self::ExtFragmentDensityMap2 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map2\0")
            },
            Self::ArmRasterizationOrderAttachmentAccess => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_ARM_rasterization_order_attachment_access\0",
                )
            },
            Self::ValveMutableDescriptorType => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_VALVE_mutable_descriptor_type\0")
            },
            Self::ExtConditionalRendering => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_conditional_rendering\0")
            },
            Self::NvFragmentShadingRateEnums => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NV_fragment_shading_rate_enums\0",
                )
            },
            Self::KhrExternalFenceWin32 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_win32\0")
            },
            Self::KhrPerformanceQuery => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_performance_query\0")
            },
            Self::ImgFormatPvrtc => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_format_pvrtc\0")
            },
            Self::KhrPortabilitySubset => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_subset\0")
            },
            Self::QcomRenderPassShaderResolve => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_QCOM_render_pass_shader_resolve\0",
                )
            },
            Self::NvxMultiviewPerViewAttributes => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NVX_multiview_per_view_attributes\0",
                )
            },
            Self::ExtValidationCache => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_cache\0")
            },
            Self::NvxBinaryImport => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_binary_import\0")
            },
            Self::ExtVideoEncodeH265 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_encode_h265\0")
            },
            Self::ExtShaderStencilExport => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_stencil_export\0")
            },
            Self::KhrAccelerationStructure => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_acceleration_structure\0")
            },
            Self::KhrSwapchain => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0")
            },
            Self::NvRayTracing => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing\0")
            },
            Self::ExtAstcDecodeMode => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_astc_decode_mode\0")
            },
            Self::ExtFragmentShaderInterlock => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_shader_interlock\0")
            },
            Self::QcomRotatedCopyCommands => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_rotated_copy_commands\0")
            },
            Self::ExtVideoDecodeH265 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_decode_h265\0")
            },
            Self::ExtShaderAtomicFloat2 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_atomic_float2\0")
            },
            Self::NvGlslShader => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_glsl_shader\0")
            },
            Self::ExtFullScreenExclusive => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_full_screen_exclusive\0")
            },
            Self::ExtYcbcrImageArrays => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_ycbcr_image_arrays\0")
            },
            Self::NvRayTracingMotionBlur => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing_motion_blur\0")
            },
            Self::ExtVertexAttributeDivisor => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_vertex_attribute_divisor\0")
            },
            Self::KhrExternalMemoryFd => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_fd\0")
            },
            Self::KhrWin32KeyedMutex => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_keyed_mutex\0")
            },
            Self::NvExternalMemoryWin32 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_win32\0")
            },
            Self::NvShaderSubgroupPartitioned => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NV_shader_subgroup_partitioned\0",
                )
            },
            Self::ExtExternalMemoryHost => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_host\0")
            },
            Self::ExtExternalMemoryDmaBuf => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_dma_buf\0")
            },
            Self::GoogleDisplayTiming => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_display_timing\0")
            },
            Self::NvExternalMemory => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory\0")
            },
            Self::NvExternalMemoryRdma => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_rdma\0")
            },
            Self::QcomRenderPassStoreOps => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_store_ops\0")
            },
            Self::ExtFragmentDensityMap => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map\0")
            },
            Self::ExtPrimitiveTopologyListRestart => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_EXT_primitive_topology_list_restart\0",
                )
            },
            Self::KhrPipelineExecutableProperties => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_KHR_pipeline_executable_properties\0",
                )
            },
            Self::NvViewportSwizzle => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_swizzle\0")
            },
            Self::NvLinearColorAttachment => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_linear_color_attachment\0")
            },
            Self::ExtImageViewMinLod => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_view_min_lod\0")
            },
            Self::ExtCustomBorderColor => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_custom_border_color\0")
            },
            Self::HuaweiInvocationMask => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_invocation_mask\0")
            },
            Self::IntelPerformanceQuery => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_performance_query\0")
            },
            Self::ExtProvokingVertex => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_provoking_vertex\0")
            },
            Self::KhrPipelineLibrary => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_pipeline_library\0")
            },
            Self::ExtVideoEncodeH264 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_encode_h264\0")
            },
            Self::AmdDeviceCoherentMemory => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_device_coherent_memory\0")
            },
            Self::ExtSampleLocations => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_sample_locations\0")
            },
            Self::NvFramebufferMixedSamples => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_framebuffer_mixed_samples\0")
            },
            Self::NvDedicatedAllocation => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_dedicated_allocation\0")
            },
            Self::ExtBlendOperationAdvanced => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_blend_operation_advanced\0")
            },
            Self::KhrRayTracingPipeline => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_tracing_pipeline\0")
            },
            Self::NvDedicatedAllocationImageAliasing => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NV_dedicated_allocation_image_aliasing\0",
                )
            },
            Self::GoogleDecorateString => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_decorate_string\0")
            },
            Self::ExtColorWriteEnable => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_color_write_enable\0")
            },
            Self::ExtBorderColorSwizzle => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_border_color_swizzle\0")
            },
            Self::AmdNegativeViewportHeight => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_negative_viewport_height\0")
            },
            Self::ExtRgba10x6Formats => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_rgba10x6_formats\0")
            },
            Self::NvCornerSampledImage => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_corner_sampled_image\0")
            },
            Self::ExtRobustness2 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_robustness2\0")
            },
            Self::NvFragmentShaderBarycentric => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"VK_NV_fragment_shader_barycentric\0",
                )
            },
            Self::NvDeviceDiagnosticsConfig => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_diagnostics_config\0")
            },
            Self::NvMeshShader => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_mesh_shader\0")
            },
            Self::AmdGpuShaderInt16 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_int16\0")
            },
            Self::ExtDeviceMemoryReport => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_device_memory_report\0")
            },
            Self::AmdShaderCoreProperties2 => unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties2\0")
            },
        }
    }
    pub fn get_dependencies(&self) -> &[Self] {
        match self {
            Self::NvCoverageReductionMode => &[Self::NvFramebufferMixedSamples],
            Self::KhrIncrementalPresent => &[Self::KhrSwapchain],
            Self::KhrVideoEncodeQueue => &[Self::KhrVideoQueue],
            Self::ExtMemoryBudget => &[],
            Self::NvAcquireWinrtDisplay => &[],
            Self::KhrSwapchainMutableFormat => &[Self::KhrSwapchain],
            Self::HuaweiSubpassShading => &[],
            Self::AmdShaderBallot => &[],
            Self::AmdDisplayNativeHdr => &[Self::KhrSwapchain],
            Self::ExtPhysicalDeviceDrm => &[],
            Self::ExtMultiDraw => &[],
            Self::ImgFilterCubic => &[],
            Self::NvxImageViewHandle => &[],
            Self::NvDeviceDiagnosticCheckpoints => &[],
            Self::ExtDepthRangeUnrestricted => &[],
            Self::KhrExternalSemaphoreWin32 => &[],
            Self::QcomFragmentDensityMapOffset => &[Self::ExtFragmentDensityMap],
            Self::KhrExternalSemaphoreFd => &[],
            Self::NvComputeShaderDerivatives => &[],
            Self::ExtCalibratedTimestamps => &[],
            Self::ExtMemoryPriority => &[],
            Self::AmdMemoryOverallocationBehavior => &[],
            Self::NvClipSpaceWScaling => &[],
            Self::KhrExternalMemoryWin32 => &[],
            Self::AmdShaderExplicitVertexParameter => &[],
            Self::AmdMixedAttachmentSamples => &[],
            Self::NvShaderImageFootprint => &[],
            Self::ExtShaderSubgroupBallot => &[],
            Self::AmdGpuShaderHalfFloat => &[],
            Self::ExtDiscardRectangles => &[],
            Self::AmdPipelineCompilerControl => &[],
            Self::KhrPresentWait => &[Self::KhrSwapchain, Self::KhrPresentId],
            Self::NvViewportArray2 => &[],
            Self::ExtPostDepthCoverage => &[],
            Self::KhrDisplaySwapchain => &[Self::KhrSwapchain],
            Self::AmdShaderInfo => &[],
            Self::KhrVideoDecodeQueue => &[Self::KhrVideoQueue],
            Self::NvFillRectangle => &[],
            Self::AmdGcnShader => &[],
            Self::ExtLoadStoreOpNone => &[],
            Self::NvSampleMaskOverrideCoverage => &[],
            Self::NvGeometryShaderPassthrough => &[],
            Self::ExtQueueFamilyForeign => &[],
            Self::ExtImageDrmFormatModifier => &[],
            Self::AmdShaderFragmentMask => &[],
            Self::ExtHdrMetadata => &[Self::KhrSwapchain],
            Self::ExtFilterCubic => &[],
            Self::ExtPageableDeviceLocalMemory => &[Self::ExtMemoryPriority],
            Self::AmdTextureGatherBiasLod => &[],
            Self::ExtTransformFeedback => &[],
            Self::KhrShaderSubgroupUniformControlFlow => &[],
            Self::KhrDeferredHostOperations => &[],
            Self::ExtDepthClipControl => &[],
            Self::ValveDescriptorSetHostMapping => &[],
            Self::ExtIndexTypeUint8 => &[],
            Self::KhrPushDescriptor => &[],
            Self::ExtDepthClipEnable => &[],
            Self::NvInheritedViewportScissor => &[],
            Self::AndroidExternalMemoryAndroidHardwareBuffer => &[Self::ExtQueueFamilyForeign],
            Self::GoogleHlslFunctionality1 => &[],
            Self::ExtShaderImageAtomicInt64 => &[],
            Self::ExtShaderAtomicFloat => &[],
            Self::FuchsiaExternalMemory => &[],
            Self::KhrFragmentShadingRate => &[],
            Self::FuchsiaExternalSemaphore => &[],
            Self::AmdShaderCoreProperties => &[],
            Self::GgpFrameToken => &[Self::KhrSwapchain],
            Self::AmdShaderImageLoadStoreLod => &[],
            Self::ExtShaderSubgroupVote => &[],
            Self::ExtLineRasterization => &[],
            Self::KhrGlobalPriority => &[],
            Self::KhrPresentId => &[Self::KhrSwapchain],
            Self::KhrVideoQueue => &[],
            Self::KhrShaderClock => &[],
            Self::NvFragmentCoverageToColor => &[],
            Self::ExtDisplayControl => &[Self::KhrSwapchain],
            Self::NvShaderSmBuiltins => &[],
            Self::KhrSharedPresentableImage => &[Self::KhrSwapchain],
            Self::ExtVideoDecodeH264 => &[Self::KhrVideoDecodeQueue, Self::KhrVideoQueue],
            Self::QcomRenderPassTransform => &[Self::KhrSwapchain],
            Self::GoogleUserType => &[],
            Self::AmdRasterizationOrder => &[],
            Self::ExtBufferDeviceAddress => &[],
            Self::ExtConservativeRasterization => &[],
            Self::FuchsiaBufferCollection => &[Self::FuchsiaExternalMemory],
            Self::IntelShaderIntegerFunctions2 => &[],
            Self::KhrWorkgroupMemoryExplicitLayout => &[],
            Self::ExtVertexInputDynamicState => &[],
            Self::NvRepresentativeFragmentTest => &[],
            Self::AmdBufferMarker => &[],
            Self::AmdShaderTrinaryMinmax => &[],
            Self::KhrExternalFenceFd => &[],
            Self::NvShadingRateImage => &[],
            Self::NvScissorExclusive => &[],
            Self::NvCooperativeMatrix => &[],
            Self::NvDeviceGeneratedCommands => &[],
            Self::KhrRayQuery => &[
                Self::KhrDeferredHostOperations,
                Self::KhrAccelerationStructure,
            ],
            Self::ExtPciBusInfo => &[],
            Self::ExtFragmentDensityMap2 => &[Self::ExtFragmentDensityMap],
            Self::ArmRasterizationOrderAttachmentAccess => &[],
            Self::ValveMutableDescriptorType => &[],
            Self::ExtConditionalRendering => &[],
            Self::NvFragmentShadingRateEnums => &[Self::KhrFragmentShadingRate],
            Self::KhrExternalFenceWin32 => &[],
            Self::KhrPerformanceQuery => &[],
            Self::ImgFormatPvrtc => &[],
            Self::KhrPortabilitySubset => &[],
            Self::QcomRenderPassShaderResolve => &[],
            Self::NvxMultiviewPerViewAttributes => &[],
            Self::ExtValidationCache => &[],
            Self::NvxBinaryImport => &[],
            Self::ExtVideoEncodeH265 => &[Self::KhrVideoQueue, Self::KhrVideoEncodeQueue],
            Self::ExtShaderStencilExport => &[],
            Self::KhrAccelerationStructure => &[Self::KhrDeferredHostOperations],
            Self::KhrSwapchain => &[],
            Self::NvRayTracing => &[],
            Self::ExtAstcDecodeMode => &[],
            Self::ExtFragmentShaderInterlock => &[],
            Self::QcomRotatedCopyCommands => &[Self::KhrSwapchain],
            Self::ExtVideoDecodeH265 => &[Self::KhrVideoDecodeQueue, Self::KhrVideoQueue],
            Self::ExtShaderAtomicFloat2 => &[Self::ExtShaderAtomicFloat],
            Self::NvGlslShader => &[],
            Self::ExtFullScreenExclusive => &[Self::KhrSwapchain],
            Self::ExtYcbcrImageArrays => &[],
            Self::NvRayTracingMotionBlur => &[
                Self::KhrRayTracingPipeline,
                Self::KhrDeferredHostOperations,
                Self::KhrAccelerationStructure,
            ],
            Self::ExtVertexAttributeDivisor => &[],
            Self::KhrExternalMemoryFd => &[],
            Self::KhrWin32KeyedMutex => &[Self::KhrExternalMemoryWin32],
            Self::NvExternalMemoryWin32 => &[Self::NvExternalMemory],
            Self::NvShaderSubgroupPartitioned => &[],
            Self::ExtExternalMemoryHost => &[],
            Self::ExtExternalMemoryDmaBuf => &[Self::KhrExternalMemoryFd],
            Self::GoogleDisplayTiming => &[Self::KhrSwapchain],
            Self::NvExternalMemory => &[],
            Self::NvExternalMemoryRdma => &[],
            Self::QcomRenderPassStoreOps => &[],
            Self::ExtFragmentDensityMap => &[],
            Self::ExtPrimitiveTopologyListRestart => &[],
            Self::KhrPipelineExecutableProperties => &[],
            Self::NvViewportSwizzle => &[],
            Self::NvLinearColorAttachment => &[],
            Self::ExtImageViewMinLod => &[],
            Self::ExtCustomBorderColor => &[],
            Self::HuaweiInvocationMask => &[
                Self::KhrRayTracingPipeline,
                Self::KhrDeferredHostOperations,
                Self::KhrAccelerationStructure,
            ],
            Self::IntelPerformanceQuery => &[],
            Self::ExtProvokingVertex => &[],
            Self::KhrPipelineLibrary => &[],
            Self::ExtVideoEncodeH264 => &[Self::KhrVideoQueue, Self::KhrVideoEncodeQueue],
            Self::AmdDeviceCoherentMemory => &[],
            Self::ExtSampleLocations => &[],
            Self::NvFramebufferMixedSamples => &[],
            Self::NvDedicatedAllocation => &[],
            Self::ExtBlendOperationAdvanced => &[],
            Self::KhrRayTracingPipeline => &[
                Self::KhrDeferredHostOperations,
                Self::KhrAccelerationStructure,
            ],
            Self::NvDedicatedAllocationImageAliasing => &[],
            Self::GoogleDecorateString => &[],
            Self::ExtColorWriteEnable => &[],
            Self::ExtBorderColorSwizzle => &[Self::ExtCustomBorderColor],
            Self::AmdNegativeViewportHeight => &[],
            Self::ExtRgba10x6Formats => &[],
            Self::NvCornerSampledImage => &[],
            Self::ExtRobustness2 => &[],
            Self::NvFragmentShaderBarycentric => &[],
            Self::NvDeviceDiagnosticsConfig => &[],
            Self::NvMeshShader => &[],
            Self::AmdGpuShaderInt16 => &[],
            Self::ExtDeviceMemoryReport => &[],
            Self::AmdShaderCoreProperties2 => &[Self::AmdShaderCoreProperties],
        }
    }
    pub fn from_cstr(vk_name: &'static std::ffi::CStr) -> Option<Self> {
        let bytes = vk_name.to_bytes_with_nul();
        match bytes {
            b"VK_NV_coverage_reduction_mode\0" => Some(Self::NvCoverageReductionMode),
            b"VK_KHR_incremental_present\0" => Some(Self::KhrIncrementalPresent),
            b"VK_KHR_video_encode_queue\0" => Some(Self::KhrVideoEncodeQueue),
            b"VK_EXT_memory_budget\0" => Some(Self::ExtMemoryBudget),
            b"VK_NV_acquire_winrt_display\0" => Some(Self::NvAcquireWinrtDisplay),
            b"VK_KHR_swapchain_mutable_format\0" => Some(Self::KhrSwapchainMutableFormat),
            b"VK_HUAWEI_subpass_shading\0" => Some(Self::HuaweiSubpassShading),
            b"VK_AMD_shader_ballot\0" => Some(Self::AmdShaderBallot),
            b"VK_AMD_display_native_hdr\0" => Some(Self::AmdDisplayNativeHdr),
            b"VK_EXT_physical_device_drm\0" => Some(Self::ExtPhysicalDeviceDrm),
            b"VK_EXT_multi_draw\0" => Some(Self::ExtMultiDraw),
            b"VK_IMG_filter_cubic\0" => Some(Self::ImgFilterCubic),
            b"VK_NVX_image_view_handle\0" => Some(Self::NvxImageViewHandle),
            b"VK_NV_device_diagnostic_checkpoints\0" => Some(Self::NvDeviceDiagnosticCheckpoints),
            b"VK_EXT_depth_range_unrestricted\0" => Some(Self::ExtDepthRangeUnrestricted),
            b"VK_KHR_external_semaphore_win32\0" => Some(Self::KhrExternalSemaphoreWin32),
            b"VK_QCOM_fragment_density_map_offset\0" => Some(Self::QcomFragmentDensityMapOffset),
            b"VK_KHR_external_semaphore_fd\0" => Some(Self::KhrExternalSemaphoreFd),
            b"VK_NV_compute_shader_derivatives\0" => Some(Self::NvComputeShaderDerivatives),
            b"VK_EXT_calibrated_timestamps\0" => Some(Self::ExtCalibratedTimestamps),
            b"VK_EXT_memory_priority\0" => Some(Self::ExtMemoryPriority),
            b"VK_AMD_memory_overallocation_behavior\0" => {
                Some(Self::AmdMemoryOverallocationBehavior)
            }
            b"VK_NV_clip_space_w_scaling\0" => Some(Self::NvClipSpaceWScaling),
            b"VK_KHR_external_memory_win32\0" => Some(Self::KhrExternalMemoryWin32),
            b"VK_AMD_shader_explicit_vertex_parameter\0" => {
                Some(Self::AmdShaderExplicitVertexParameter)
            }
            b"VK_AMD_mixed_attachment_samples\0" => Some(Self::AmdMixedAttachmentSamples),
            b"VK_NV_shader_image_footprint\0" => Some(Self::NvShaderImageFootprint),
            b"VK_EXT_shader_subgroup_ballot\0" => Some(Self::ExtShaderSubgroupBallot),
            b"VK_AMD_gpu_shader_half_float\0" => Some(Self::AmdGpuShaderHalfFloat),
            b"VK_EXT_discard_rectangles\0" => Some(Self::ExtDiscardRectangles),
            b"VK_AMD_pipeline_compiler_control\0" => Some(Self::AmdPipelineCompilerControl),
            b"VK_KHR_present_wait\0" => Some(Self::KhrPresentWait),
            b"VK_NV_viewport_array2\0" => Some(Self::NvViewportArray2),
            b"VK_EXT_post_depth_coverage\0" => Some(Self::ExtPostDepthCoverage),
            b"VK_KHR_display_swapchain\0" => Some(Self::KhrDisplaySwapchain),
            b"VK_AMD_shader_info\0" => Some(Self::AmdShaderInfo),
            b"VK_KHR_video_decode_queue\0" => Some(Self::KhrVideoDecodeQueue),
            b"VK_NV_fill_rectangle\0" => Some(Self::NvFillRectangle),
            b"VK_AMD_gcn_shader\0" => Some(Self::AmdGcnShader),
            b"VK_EXT_load_store_op_none\0" => Some(Self::ExtLoadStoreOpNone),
            b"VK_NV_sample_mask_override_coverage\0" => Some(Self::NvSampleMaskOverrideCoverage),
            b"VK_NV_geometry_shader_passthrough\0" => Some(Self::NvGeometryShaderPassthrough),
            b"VK_EXT_queue_family_foreign\0" => Some(Self::ExtQueueFamilyForeign),
            b"VK_EXT_image_drm_format_modifier\0" => Some(Self::ExtImageDrmFormatModifier),
            b"VK_AMD_shader_fragment_mask\0" => Some(Self::AmdShaderFragmentMask),
            b"VK_EXT_hdr_metadata\0" => Some(Self::ExtHdrMetadata),
            b"VK_EXT_filter_cubic\0" => Some(Self::ExtFilterCubic),
            b"VK_EXT_pageable_device_local_memory\0" => Some(Self::ExtPageableDeviceLocalMemory),
            b"VK_AMD_texture_gather_bias_lod\0" => Some(Self::AmdTextureGatherBiasLod),
            b"VK_EXT_transform_feedback\0" => Some(Self::ExtTransformFeedback),
            b"VK_KHR_shader_subgroup_uniform_control_flow\0" => {
                Some(Self::KhrShaderSubgroupUniformControlFlow)
            }
            b"VK_KHR_deferred_host_operations\0" => Some(Self::KhrDeferredHostOperations),
            b"VK_EXT_depth_clip_control\0" => Some(Self::ExtDepthClipControl),
            b"VK_VALVE_descriptor_set_host_mapping\0" => Some(Self::ValveDescriptorSetHostMapping),
            b"VK_EXT_index_type_uint8\0" => Some(Self::ExtIndexTypeUint8),
            b"VK_KHR_push_descriptor\0" => Some(Self::KhrPushDescriptor),
            b"VK_EXT_depth_clip_enable\0" => Some(Self::ExtDepthClipEnable),
            b"VK_NV_inherited_viewport_scissor\0" => Some(Self::NvInheritedViewportScissor),
            b"VK_ANDROID_external_memory_android_hardware_buffer\0" => {
                Some(Self::AndroidExternalMemoryAndroidHardwareBuffer)
            }
            b"VK_GOOGLE_hlsl_functionality1\0" => Some(Self::GoogleHlslFunctionality1),
            b"VK_EXT_shader_image_atomic_int64\0" => Some(Self::ExtShaderImageAtomicInt64),
            b"VK_EXT_shader_atomic_float\0" => Some(Self::ExtShaderAtomicFloat),
            b"VK_FUCHSIA_external_memory\0" => Some(Self::FuchsiaExternalMemory),
            b"VK_KHR_fragment_shading_rate\0" => Some(Self::KhrFragmentShadingRate),
            b"VK_FUCHSIA_external_semaphore\0" => Some(Self::FuchsiaExternalSemaphore),
            b"VK_AMD_shader_core_properties\0" => Some(Self::AmdShaderCoreProperties),
            b"VK_GGP_frame_token\0" => Some(Self::GgpFrameToken),
            b"VK_AMD_shader_image_load_store_lod\0" => Some(Self::AmdShaderImageLoadStoreLod),
            b"VK_EXT_shader_subgroup_vote\0" => Some(Self::ExtShaderSubgroupVote),
            b"VK_EXT_line_rasterization\0" => Some(Self::ExtLineRasterization),
            b"VK_KHR_global_priority\0" => Some(Self::KhrGlobalPriority),
            b"VK_KHR_present_id\0" => Some(Self::KhrPresentId),
            b"VK_KHR_video_queue\0" => Some(Self::KhrVideoQueue),
            b"VK_KHR_shader_clock\0" => Some(Self::KhrShaderClock),
            b"VK_NV_fragment_coverage_to_color\0" => Some(Self::NvFragmentCoverageToColor),
            b"VK_EXT_display_control\0" => Some(Self::ExtDisplayControl),
            b"VK_NV_shader_sm_builtins\0" => Some(Self::NvShaderSmBuiltins),
            b"VK_KHR_shared_presentable_image\0" => Some(Self::KhrSharedPresentableImage),
            b"VK_EXT_video_decode_h264\0" => Some(Self::ExtVideoDecodeH264),
            b"VK_QCOM_render_pass_transform\0" => Some(Self::QcomRenderPassTransform),
            b"VK_GOOGLE_user_type\0" => Some(Self::GoogleUserType),
            b"VK_AMD_rasterization_order\0" => Some(Self::AmdRasterizationOrder),
            b"VK_EXT_buffer_device_address\0" => Some(Self::ExtBufferDeviceAddress),
            b"VK_EXT_conservative_rasterization\0" => Some(Self::ExtConservativeRasterization),
            b"VK_FUCHSIA_buffer_collection\0" => Some(Self::FuchsiaBufferCollection),
            b"VK_INTEL_shader_integer_functions2\0" => Some(Self::IntelShaderIntegerFunctions2),
            b"VK_KHR_workgroup_memory_explicit_layout\0" => {
                Some(Self::KhrWorkgroupMemoryExplicitLayout)
            }
            b"VK_EXT_vertex_input_dynamic_state\0" => Some(Self::ExtVertexInputDynamicState),
            b"VK_NV_representative_fragment_test\0" => Some(Self::NvRepresentativeFragmentTest),
            b"VK_AMD_buffer_marker\0" => Some(Self::AmdBufferMarker),
            b"VK_AMD_shader_trinary_minmax\0" => Some(Self::AmdShaderTrinaryMinmax),
            b"VK_KHR_external_fence_fd\0" => Some(Self::KhrExternalFenceFd),
            b"VK_NV_shading_rate_image\0" => Some(Self::NvShadingRateImage),
            b"VK_NV_scissor_exclusive\0" => Some(Self::NvScissorExclusive),
            b"VK_NV_cooperative_matrix\0" => Some(Self::NvCooperativeMatrix),
            b"VK_NV_device_generated_commands\0" => Some(Self::NvDeviceGeneratedCommands),
            b"VK_KHR_ray_query\0" => Some(Self::KhrRayQuery),
            b"VK_EXT_pci_bus_info\0" => Some(Self::ExtPciBusInfo),
            b"VK_EXT_fragment_density_map2\0" => Some(Self::ExtFragmentDensityMap2),
            b"VK_ARM_rasterization_order_attachment_access\0" => {
                Some(Self::ArmRasterizationOrderAttachmentAccess)
            }
            b"VK_VALVE_mutable_descriptor_type\0" => Some(Self::ValveMutableDescriptorType),
            b"VK_EXT_conditional_rendering\0" => Some(Self::ExtConditionalRendering),
            b"VK_NV_fragment_shading_rate_enums\0" => Some(Self::NvFragmentShadingRateEnums),
            b"VK_KHR_external_fence_win32\0" => Some(Self::KhrExternalFenceWin32),
            b"VK_KHR_performance_query\0" => Some(Self::KhrPerformanceQuery),
            b"VK_IMG_format_pvrtc\0" => Some(Self::ImgFormatPvrtc),
            b"VK_KHR_portability_subset\0" => Some(Self::KhrPortabilitySubset),
            b"VK_QCOM_render_pass_shader_resolve\0" => Some(Self::QcomRenderPassShaderResolve),
            b"VK_NVX_multiview_per_view_attributes\0" => Some(Self::NvxMultiviewPerViewAttributes),
            b"VK_EXT_validation_cache\0" => Some(Self::ExtValidationCache),
            b"VK_NVX_binary_import\0" => Some(Self::NvxBinaryImport),
            b"VK_EXT_video_encode_h265\0" => Some(Self::ExtVideoEncodeH265),
            b"VK_EXT_shader_stencil_export\0" => Some(Self::ExtShaderStencilExport),
            b"VK_KHR_acceleration_structure\0" => Some(Self::KhrAccelerationStructure),
            b"VK_KHR_swapchain\0" => Some(Self::KhrSwapchain),
            b"VK_NV_ray_tracing\0" => Some(Self::NvRayTracing),
            b"VK_EXT_astc_decode_mode\0" => Some(Self::ExtAstcDecodeMode),
            b"VK_EXT_fragment_shader_interlock\0" => Some(Self::ExtFragmentShaderInterlock),
            b"VK_QCOM_rotated_copy_commands\0" => Some(Self::QcomRotatedCopyCommands),
            b"VK_EXT_video_decode_h265\0" => Some(Self::ExtVideoDecodeH265),
            b"VK_EXT_shader_atomic_float2\0" => Some(Self::ExtShaderAtomicFloat2),
            b"VK_NV_glsl_shader\0" => Some(Self::NvGlslShader),
            b"VK_EXT_full_screen_exclusive\0" => Some(Self::ExtFullScreenExclusive),
            b"VK_EXT_ycbcr_image_arrays\0" => Some(Self::ExtYcbcrImageArrays),
            b"VK_NV_ray_tracing_motion_blur\0" => Some(Self::NvRayTracingMotionBlur),
            b"VK_EXT_vertex_attribute_divisor\0" => Some(Self::ExtVertexAttributeDivisor),
            b"VK_KHR_external_memory_fd\0" => Some(Self::KhrExternalMemoryFd),
            b"VK_KHR_win32_keyed_mutex\0" => Some(Self::KhrWin32KeyedMutex),
            b"VK_NV_external_memory_win32\0" => Some(Self::NvExternalMemoryWin32),
            b"VK_NV_shader_subgroup_partitioned\0" => Some(Self::NvShaderSubgroupPartitioned),
            b"VK_EXT_external_memory_host\0" => Some(Self::ExtExternalMemoryHost),
            b"VK_EXT_external_memory_dma_buf\0" => Some(Self::ExtExternalMemoryDmaBuf),
            b"VK_GOOGLE_display_timing\0" => Some(Self::GoogleDisplayTiming),
            b"VK_NV_external_memory\0" => Some(Self::NvExternalMemory),
            b"VK_NV_external_memory_rdma\0" => Some(Self::NvExternalMemoryRdma),
            b"VK_QCOM_render_pass_store_ops\0" => Some(Self::QcomRenderPassStoreOps),
            b"VK_EXT_fragment_density_map\0" => Some(Self::ExtFragmentDensityMap),
            b"VK_EXT_primitive_topology_list_restart\0" => {
                Some(Self::ExtPrimitiveTopologyListRestart)
            }
            b"VK_KHR_pipeline_executable_properties\0" => {
                Some(Self::KhrPipelineExecutableProperties)
            }
            b"VK_NV_viewport_swizzle\0" => Some(Self::NvViewportSwizzle),
            b"VK_NV_linear_color_attachment\0" => Some(Self::NvLinearColorAttachment),
            b"VK_EXT_image_view_min_lod\0" => Some(Self::ExtImageViewMinLod),
            b"VK_EXT_custom_border_color\0" => Some(Self::ExtCustomBorderColor),
            b"VK_HUAWEI_invocation_mask\0" => Some(Self::HuaweiInvocationMask),
            b"VK_INTEL_performance_query\0" => Some(Self::IntelPerformanceQuery),
            b"VK_EXT_provoking_vertex\0" => Some(Self::ExtProvokingVertex),
            b"VK_KHR_pipeline_library\0" => Some(Self::KhrPipelineLibrary),
            b"VK_EXT_video_encode_h264\0" => Some(Self::ExtVideoEncodeH264),
            b"VK_AMD_device_coherent_memory\0" => Some(Self::AmdDeviceCoherentMemory),
            b"VK_EXT_sample_locations\0" => Some(Self::ExtSampleLocations),
            b"VK_NV_framebuffer_mixed_samples\0" => Some(Self::NvFramebufferMixedSamples),
            b"VK_NV_dedicated_allocation\0" => Some(Self::NvDedicatedAllocation),
            b"VK_EXT_blend_operation_advanced\0" => Some(Self::ExtBlendOperationAdvanced),
            b"VK_KHR_ray_tracing_pipeline\0" => Some(Self::KhrRayTracingPipeline),
            b"VK_NV_dedicated_allocation_image_aliasing\0" => {
                Some(Self::NvDedicatedAllocationImageAliasing)
            }
            b"VK_GOOGLE_decorate_string\0" => Some(Self::GoogleDecorateString),
            b"VK_EXT_color_write_enable\0" => Some(Self::ExtColorWriteEnable),
            b"VK_EXT_border_color_swizzle\0" => Some(Self::ExtBorderColorSwizzle),
            b"VK_AMD_negative_viewport_height\0" => Some(Self::AmdNegativeViewportHeight),
            b"VK_EXT_rgba10x6_formats\0" => Some(Self::ExtRgba10x6Formats),
            b"VK_NV_corner_sampled_image\0" => Some(Self::NvCornerSampledImage),
            b"VK_EXT_robustness2\0" => Some(Self::ExtRobustness2),
            b"VK_NV_fragment_shader_barycentric\0" => Some(Self::NvFragmentShaderBarycentric),
            b"VK_NV_device_diagnostics_config\0" => Some(Self::NvDeviceDiagnosticsConfig),
            b"VK_NV_mesh_shader\0" => Some(Self::NvMeshShader),
            b"VK_AMD_gpu_shader_int16\0" => Some(Self::AmdGpuShaderInt16),
            b"VK_EXT_device_memory_report\0" => Some(Self::ExtDeviceMemoryReport),
            b"VK_AMD_shader_core_properties2\0" => Some(Self::AmdShaderCoreProperties2),
            _ => None,
        }
    }
}
pub enum DeviceExtensionType {
    NvCoverageReductionMode,
    KhrIncrementalPresent(InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>),
    KhrVideoEncodeQueue,
    ExtMemoryBudget,
    NvAcquireWinrtDisplay(
        InstanceExtension<{ PhysicalInstanceExtensionType::ExtDirectModeDisplay }>,
    ),
    KhrSwapchainMutableFormat(InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>),
    HuaweiSubpassShading,
    AmdShaderBallot,
    AmdDisplayNativeHdr(
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrGetSurfaceCapabilities2 }>,
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>,
    ),
    ExtPhysicalDeviceDrm,
    ExtMultiDraw,
    ImgFilterCubic,
    NvxImageViewHandle,
    NvDeviceDiagnosticCheckpoints,
    ExtDepthRangeUnrestricted,
    KhrExternalSemaphoreWin32,
    QcomFragmentDensityMapOffset,
    KhrExternalSemaphoreFd,
    NvComputeShaderDerivatives,
    ExtCalibratedTimestamps,
    ExtMemoryPriority,
    AmdMemoryOverallocationBehavior,
    NvClipSpaceWScaling,
    KhrExternalMemoryWin32,
    AmdShaderExplicitVertexParameter,
    AmdMixedAttachmentSamples,
    NvShaderImageFootprint,
    ExtShaderSubgroupBallot,
    AmdGpuShaderHalfFloat,
    ExtDiscardRectangles,
    AmdPipelineCompilerControl,
    KhrPresentWait(InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>),
    NvViewportArray2,
    ExtPostDepthCoverage,
    KhrDisplaySwapchain(
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrDisplay }>,
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>,
    ),
    AmdShaderInfo,
    KhrVideoDecodeQueue,
    NvFillRectangle,
    AmdGcnShader,
    ExtLoadStoreOpNone,
    NvSampleMaskOverrideCoverage,
    NvGeometryShaderPassthrough,
    ExtQueueFamilyForeign,
    ExtImageDrmFormatModifier,
    AmdShaderFragmentMask,
    ExtHdrMetadata(InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>),
    ExtFilterCubic,
    ExtPageableDeviceLocalMemory,
    AmdTextureGatherBiasLod,
    ExtTransformFeedback,
    KhrShaderSubgroupUniformControlFlow,
    KhrDeferredHostOperations,
    ExtDepthClipControl,
    ValveDescriptorSetHostMapping,
    ExtIndexTypeUint8,
    KhrPushDescriptor,
    ExtDepthClipEnable,
    NvInheritedViewportScissor,
    AndroidExternalMemoryAndroidHardwareBuffer,
    GoogleHlslFunctionality1,
    ExtShaderImageAtomicInt64,
    ExtShaderAtomicFloat,
    FuchsiaExternalMemory,
    KhrFragmentShadingRate,
    FuchsiaExternalSemaphore,
    AmdShaderCoreProperties,
    GgpFrameToken(
        InstanceExtension<{ PhysicalInstanceExtensionType::GgpStreamDescriptorSurface }>,
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>,
    ),
    AmdShaderImageLoadStoreLod,
    ExtShaderSubgroupVote,
    ExtLineRasterization,
    KhrGlobalPriority,
    KhrPresentId(InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>),
    KhrVideoQueue,
    KhrShaderClock,
    NvFragmentCoverageToColor,
    ExtDisplayControl(
        InstanceExtension<{ PhysicalInstanceExtensionType::ExtDisplaySurfaceCounter }>,
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>,
    ),
    NvShaderSmBuiltins,
    KhrSharedPresentableImage(
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrGetSurfaceCapabilities2 }>,
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>,
    ),
    ExtVideoDecodeH264,
    QcomRenderPassTransform(InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>),
    GoogleUserType,
    AmdRasterizationOrder,
    ExtBufferDeviceAddress,
    ExtConservativeRasterization,
    FuchsiaBufferCollection,
    IntelShaderIntegerFunctions2,
    KhrWorkgroupMemoryExplicitLayout,
    ExtVertexInputDynamicState,
    NvRepresentativeFragmentTest,
    AmdBufferMarker,
    AmdShaderTrinaryMinmax,
    KhrExternalFenceFd,
    NvShadingRateImage,
    NvScissorExclusive,
    NvCooperativeMatrix,
    NvDeviceGeneratedCommands,
    KhrRayQuery,
    ExtPciBusInfo,
    ExtFragmentDensityMap2,
    ArmRasterizationOrderAttachmentAccess,
    ValveMutableDescriptorType,
    ExtConditionalRendering,
    NvFragmentShadingRateEnums,
    KhrExternalFenceWin32,
    KhrPerformanceQuery,
    ImgFormatPvrtc,
    KhrPortabilitySubset,
    QcomRenderPassShaderResolve,
    NvxMultiviewPerViewAttributes,
    ExtValidationCache,
    NvxBinaryImport,
    ExtVideoEncodeH265,
    ExtShaderStencilExport,
    KhrAccelerationStructure,
    KhrSwapchain(InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>),
    NvRayTracing,
    ExtAstcDecodeMode,
    ExtFragmentShaderInterlock,
    QcomRotatedCopyCommands(InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>),
    ExtVideoDecodeH265,
    ExtShaderAtomicFloat2,
    NvGlslShader,
    ExtFullScreenExclusive(
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrGetSurfaceCapabilities2 }>,
        InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>,
    ),
    ExtYcbcrImageArrays,
    NvRayTracingMotionBlur,
    ExtVertexAttributeDivisor,
    KhrExternalMemoryFd,
    KhrWin32KeyedMutex,
    NvExternalMemoryWin32(
        InstanceExtension<{ PhysicalInstanceExtensionType::NvExternalMemoryCapabilities }>,
    ),
    NvShaderSubgroupPartitioned,
    ExtExternalMemoryHost,
    ExtExternalMemoryDmaBuf,
    GoogleDisplayTiming(InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>),
    NvExternalMemory(
        InstanceExtension<{ PhysicalInstanceExtensionType::NvExternalMemoryCapabilities }>,
    ),
    NvExternalMemoryRdma,
    QcomRenderPassStoreOps,
    ExtFragmentDensityMap,
    ExtPrimitiveTopologyListRestart,
    KhrPipelineExecutableProperties,
    NvViewportSwizzle,
    NvLinearColorAttachment,
    ExtImageViewMinLod,
    ExtCustomBorderColor,
    HuaweiInvocationMask,
    IntelPerformanceQuery,
    ExtProvokingVertex,
    KhrPipelineLibrary,
    ExtVideoEncodeH264,
    AmdDeviceCoherentMemory,
    ExtSampleLocations,
    NvFramebufferMixedSamples,
    NvDedicatedAllocation,
    ExtBlendOperationAdvanced,
    KhrRayTracingPipeline,
    NvDedicatedAllocationImageAliasing,
    GoogleDecorateString,
    ExtColorWriteEnable,
    ExtBorderColorSwizzle,
    AmdNegativeViewportHeight,
    ExtRgba10x6Formats,
    NvCornerSampledImage,
    ExtRobustness2,
    NvFragmentShaderBarycentric,
    NvDeviceDiagnosticsConfig,
    NvMeshShader,
    AmdGpuShaderInt16,
    ExtDeviceMemoryReport,
    AmdShaderCoreProperties2,
}
impl From<&DeviceExtensionType> for PhysicalDeviceExtensionType {
    fn from(device_ext_type: &DeviceExtensionType) -> PhysicalDeviceExtensionType {
        match device_ext_type {
            DeviceExtensionType::NvCoverageReductionMode => Self::NvCoverageReductionMode,
            DeviceExtensionType::KhrIncrementalPresent(..) => Self::KhrIncrementalPresent,
            DeviceExtensionType::KhrVideoEncodeQueue => Self::KhrVideoEncodeQueue,
            DeviceExtensionType::ExtMemoryBudget => Self::ExtMemoryBudget,
            DeviceExtensionType::NvAcquireWinrtDisplay(..) => Self::NvAcquireWinrtDisplay,
            DeviceExtensionType::KhrSwapchainMutableFormat(..) => Self::KhrSwapchainMutableFormat,
            DeviceExtensionType::HuaweiSubpassShading => Self::HuaweiSubpassShading,
            DeviceExtensionType::AmdShaderBallot => Self::AmdShaderBallot,
            DeviceExtensionType::AmdDisplayNativeHdr(..) => Self::AmdDisplayNativeHdr,
            DeviceExtensionType::ExtPhysicalDeviceDrm => Self::ExtPhysicalDeviceDrm,
            DeviceExtensionType::ExtMultiDraw => Self::ExtMultiDraw,
            DeviceExtensionType::ImgFilterCubic => Self::ImgFilterCubic,
            DeviceExtensionType::NvxImageViewHandle => Self::NvxImageViewHandle,
            DeviceExtensionType::NvDeviceDiagnosticCheckpoints => {
                Self::NvDeviceDiagnosticCheckpoints
            }
            DeviceExtensionType::ExtDepthRangeUnrestricted => Self::ExtDepthRangeUnrestricted,
            DeviceExtensionType::KhrExternalSemaphoreWin32 => Self::KhrExternalSemaphoreWin32,
            DeviceExtensionType::QcomFragmentDensityMapOffset => Self::QcomFragmentDensityMapOffset,
            DeviceExtensionType::KhrExternalSemaphoreFd => Self::KhrExternalSemaphoreFd,
            DeviceExtensionType::NvComputeShaderDerivatives => Self::NvComputeShaderDerivatives,
            DeviceExtensionType::ExtCalibratedTimestamps => Self::ExtCalibratedTimestamps,
            DeviceExtensionType::ExtMemoryPriority => Self::ExtMemoryPriority,
            DeviceExtensionType::AmdMemoryOverallocationBehavior => {
                Self::AmdMemoryOverallocationBehavior
            }
            DeviceExtensionType::NvClipSpaceWScaling => Self::NvClipSpaceWScaling,
            DeviceExtensionType::KhrExternalMemoryWin32 => Self::KhrExternalMemoryWin32,
            DeviceExtensionType::AmdShaderExplicitVertexParameter => {
                Self::AmdShaderExplicitVertexParameter
            }
            DeviceExtensionType::AmdMixedAttachmentSamples => Self::AmdMixedAttachmentSamples,
            DeviceExtensionType::NvShaderImageFootprint => Self::NvShaderImageFootprint,
            DeviceExtensionType::ExtShaderSubgroupBallot => Self::ExtShaderSubgroupBallot,
            DeviceExtensionType::AmdGpuShaderHalfFloat => Self::AmdGpuShaderHalfFloat,
            DeviceExtensionType::ExtDiscardRectangles => Self::ExtDiscardRectangles,
            DeviceExtensionType::AmdPipelineCompilerControl => Self::AmdPipelineCompilerControl,
            DeviceExtensionType::KhrPresentWait(..) => Self::KhrPresentWait,
            DeviceExtensionType::NvViewportArray2 => Self::NvViewportArray2,
            DeviceExtensionType::ExtPostDepthCoverage => Self::ExtPostDepthCoverage,
            DeviceExtensionType::KhrDisplaySwapchain(..) => Self::KhrDisplaySwapchain,
            DeviceExtensionType::AmdShaderInfo => Self::AmdShaderInfo,
            DeviceExtensionType::KhrVideoDecodeQueue => Self::KhrVideoDecodeQueue,
            DeviceExtensionType::NvFillRectangle => Self::NvFillRectangle,
            DeviceExtensionType::AmdGcnShader => Self::AmdGcnShader,
            DeviceExtensionType::ExtLoadStoreOpNone => Self::ExtLoadStoreOpNone,
            DeviceExtensionType::NvSampleMaskOverrideCoverage => Self::NvSampleMaskOverrideCoverage,
            DeviceExtensionType::NvGeometryShaderPassthrough => Self::NvGeometryShaderPassthrough,
            DeviceExtensionType::ExtQueueFamilyForeign => Self::ExtQueueFamilyForeign,
            DeviceExtensionType::ExtImageDrmFormatModifier => Self::ExtImageDrmFormatModifier,
            DeviceExtensionType::AmdShaderFragmentMask => Self::AmdShaderFragmentMask,
            DeviceExtensionType::ExtHdrMetadata(..) => Self::ExtHdrMetadata,
            DeviceExtensionType::ExtFilterCubic => Self::ExtFilterCubic,
            DeviceExtensionType::ExtPageableDeviceLocalMemory => Self::ExtPageableDeviceLocalMemory,
            DeviceExtensionType::AmdTextureGatherBiasLod => Self::AmdTextureGatherBiasLod,
            DeviceExtensionType::ExtTransformFeedback => Self::ExtTransformFeedback,
            DeviceExtensionType::KhrShaderSubgroupUniformControlFlow => {
                Self::KhrShaderSubgroupUniformControlFlow
            }
            DeviceExtensionType::KhrDeferredHostOperations => Self::KhrDeferredHostOperations,
            DeviceExtensionType::ExtDepthClipControl => Self::ExtDepthClipControl,
            DeviceExtensionType::ValveDescriptorSetHostMapping => {
                Self::ValveDescriptorSetHostMapping
            }
            DeviceExtensionType::ExtIndexTypeUint8 => Self::ExtIndexTypeUint8,
            DeviceExtensionType::KhrPushDescriptor => Self::KhrPushDescriptor,
            DeviceExtensionType::ExtDepthClipEnable => Self::ExtDepthClipEnable,
            DeviceExtensionType::NvInheritedViewportScissor => Self::NvInheritedViewportScissor,
            DeviceExtensionType::AndroidExternalMemoryAndroidHardwareBuffer => {
                Self::AndroidExternalMemoryAndroidHardwareBuffer
            }
            DeviceExtensionType::GoogleHlslFunctionality1 => Self::GoogleHlslFunctionality1,
            DeviceExtensionType::ExtShaderImageAtomicInt64 => Self::ExtShaderImageAtomicInt64,
            DeviceExtensionType::ExtShaderAtomicFloat => Self::ExtShaderAtomicFloat,
            DeviceExtensionType::FuchsiaExternalMemory => Self::FuchsiaExternalMemory,
            DeviceExtensionType::KhrFragmentShadingRate => Self::KhrFragmentShadingRate,
            DeviceExtensionType::FuchsiaExternalSemaphore => Self::FuchsiaExternalSemaphore,
            DeviceExtensionType::AmdShaderCoreProperties => Self::AmdShaderCoreProperties,
            DeviceExtensionType::GgpFrameToken(..) => Self::GgpFrameToken,
            DeviceExtensionType::AmdShaderImageLoadStoreLod => Self::AmdShaderImageLoadStoreLod,
            DeviceExtensionType::ExtShaderSubgroupVote => Self::ExtShaderSubgroupVote,
            DeviceExtensionType::ExtLineRasterization => Self::ExtLineRasterization,
            DeviceExtensionType::KhrGlobalPriority => Self::KhrGlobalPriority,
            DeviceExtensionType::KhrPresentId(..) => Self::KhrPresentId,
            DeviceExtensionType::KhrVideoQueue => Self::KhrVideoQueue,
            DeviceExtensionType::KhrShaderClock => Self::KhrShaderClock,
            DeviceExtensionType::NvFragmentCoverageToColor => Self::NvFragmentCoverageToColor,
            DeviceExtensionType::ExtDisplayControl(..) => Self::ExtDisplayControl,
            DeviceExtensionType::NvShaderSmBuiltins => Self::NvShaderSmBuiltins,
            DeviceExtensionType::KhrSharedPresentableImage(..) => Self::KhrSharedPresentableImage,
            DeviceExtensionType::ExtVideoDecodeH264 => Self::ExtVideoDecodeH264,
            DeviceExtensionType::QcomRenderPassTransform(..) => Self::QcomRenderPassTransform,
            DeviceExtensionType::GoogleUserType => Self::GoogleUserType,
            DeviceExtensionType::AmdRasterizationOrder => Self::AmdRasterizationOrder,
            DeviceExtensionType::ExtBufferDeviceAddress => Self::ExtBufferDeviceAddress,
            DeviceExtensionType::ExtConservativeRasterization => Self::ExtConservativeRasterization,
            DeviceExtensionType::FuchsiaBufferCollection => Self::FuchsiaBufferCollection,
            DeviceExtensionType::IntelShaderIntegerFunctions2 => Self::IntelShaderIntegerFunctions2,
            DeviceExtensionType::KhrWorkgroupMemoryExplicitLayout => {
                Self::KhrWorkgroupMemoryExplicitLayout
            }
            DeviceExtensionType::ExtVertexInputDynamicState => Self::ExtVertexInputDynamicState,
            DeviceExtensionType::NvRepresentativeFragmentTest => Self::NvRepresentativeFragmentTest,
            DeviceExtensionType::AmdBufferMarker => Self::AmdBufferMarker,
            DeviceExtensionType::AmdShaderTrinaryMinmax => Self::AmdShaderTrinaryMinmax,
            DeviceExtensionType::KhrExternalFenceFd => Self::KhrExternalFenceFd,
            DeviceExtensionType::NvShadingRateImage => Self::NvShadingRateImage,
            DeviceExtensionType::NvScissorExclusive => Self::NvScissorExclusive,
            DeviceExtensionType::NvCooperativeMatrix => Self::NvCooperativeMatrix,
            DeviceExtensionType::NvDeviceGeneratedCommands => Self::NvDeviceGeneratedCommands,
            DeviceExtensionType::KhrRayQuery => Self::KhrRayQuery,
            DeviceExtensionType::ExtPciBusInfo => Self::ExtPciBusInfo,
            DeviceExtensionType::ExtFragmentDensityMap2 => Self::ExtFragmentDensityMap2,
            DeviceExtensionType::ArmRasterizationOrderAttachmentAccess => {
                Self::ArmRasterizationOrderAttachmentAccess
            }
            DeviceExtensionType::ValveMutableDescriptorType => Self::ValveMutableDescriptorType,
            DeviceExtensionType::ExtConditionalRendering => Self::ExtConditionalRendering,
            DeviceExtensionType::NvFragmentShadingRateEnums => Self::NvFragmentShadingRateEnums,
            DeviceExtensionType::KhrExternalFenceWin32 => Self::KhrExternalFenceWin32,
            DeviceExtensionType::KhrPerformanceQuery => Self::KhrPerformanceQuery,
            DeviceExtensionType::ImgFormatPvrtc => Self::ImgFormatPvrtc,
            DeviceExtensionType::KhrPortabilitySubset => Self::KhrPortabilitySubset,
            DeviceExtensionType::QcomRenderPassShaderResolve => Self::QcomRenderPassShaderResolve,
            DeviceExtensionType::NvxMultiviewPerViewAttributes => {
                Self::NvxMultiviewPerViewAttributes
            }
            DeviceExtensionType::ExtValidationCache => Self::ExtValidationCache,
            DeviceExtensionType::NvxBinaryImport => Self::NvxBinaryImport,
            DeviceExtensionType::ExtVideoEncodeH265 => Self::ExtVideoEncodeH265,
            DeviceExtensionType::ExtShaderStencilExport => Self::ExtShaderStencilExport,
            DeviceExtensionType::KhrAccelerationStructure => Self::KhrAccelerationStructure,
            DeviceExtensionType::KhrSwapchain(..) => Self::KhrSwapchain,
            DeviceExtensionType::NvRayTracing => Self::NvRayTracing,
            DeviceExtensionType::ExtAstcDecodeMode => Self::ExtAstcDecodeMode,
            DeviceExtensionType::ExtFragmentShaderInterlock => Self::ExtFragmentShaderInterlock,
            DeviceExtensionType::QcomRotatedCopyCommands(..) => Self::QcomRotatedCopyCommands,
            DeviceExtensionType::ExtVideoDecodeH265 => Self::ExtVideoDecodeH265,
            DeviceExtensionType::ExtShaderAtomicFloat2 => Self::ExtShaderAtomicFloat2,
            DeviceExtensionType::NvGlslShader => Self::NvGlslShader,
            DeviceExtensionType::ExtFullScreenExclusive(..) => Self::ExtFullScreenExclusive,
            DeviceExtensionType::ExtYcbcrImageArrays => Self::ExtYcbcrImageArrays,
            DeviceExtensionType::NvRayTracingMotionBlur => Self::NvRayTracingMotionBlur,
            DeviceExtensionType::ExtVertexAttributeDivisor => Self::ExtVertexAttributeDivisor,
            DeviceExtensionType::KhrExternalMemoryFd => Self::KhrExternalMemoryFd,
            DeviceExtensionType::KhrWin32KeyedMutex => Self::KhrWin32KeyedMutex,
            DeviceExtensionType::NvExternalMemoryWin32(..) => Self::NvExternalMemoryWin32,
            DeviceExtensionType::NvShaderSubgroupPartitioned => Self::NvShaderSubgroupPartitioned,
            DeviceExtensionType::ExtExternalMemoryHost => Self::ExtExternalMemoryHost,
            DeviceExtensionType::ExtExternalMemoryDmaBuf => Self::ExtExternalMemoryDmaBuf,
            DeviceExtensionType::GoogleDisplayTiming(..) => Self::GoogleDisplayTiming,
            DeviceExtensionType::NvExternalMemory(..) => Self::NvExternalMemory,
            DeviceExtensionType::NvExternalMemoryRdma => Self::NvExternalMemoryRdma,
            DeviceExtensionType::QcomRenderPassStoreOps => Self::QcomRenderPassStoreOps,
            DeviceExtensionType::ExtFragmentDensityMap => Self::ExtFragmentDensityMap,
            DeviceExtensionType::ExtPrimitiveTopologyListRestart => {
                Self::ExtPrimitiveTopologyListRestart
            }
            DeviceExtensionType::KhrPipelineExecutableProperties => {
                Self::KhrPipelineExecutableProperties
            }
            DeviceExtensionType::NvViewportSwizzle => Self::NvViewportSwizzle,
            DeviceExtensionType::NvLinearColorAttachment => Self::NvLinearColorAttachment,
            DeviceExtensionType::ExtImageViewMinLod => Self::ExtImageViewMinLod,
            DeviceExtensionType::ExtCustomBorderColor => Self::ExtCustomBorderColor,
            DeviceExtensionType::HuaweiInvocationMask => Self::HuaweiInvocationMask,
            DeviceExtensionType::IntelPerformanceQuery => Self::IntelPerformanceQuery,
            DeviceExtensionType::ExtProvokingVertex => Self::ExtProvokingVertex,
            DeviceExtensionType::KhrPipelineLibrary => Self::KhrPipelineLibrary,
            DeviceExtensionType::ExtVideoEncodeH264 => Self::ExtVideoEncodeH264,
            DeviceExtensionType::AmdDeviceCoherentMemory => Self::AmdDeviceCoherentMemory,
            DeviceExtensionType::ExtSampleLocations => Self::ExtSampleLocations,
            DeviceExtensionType::NvFramebufferMixedSamples => Self::NvFramebufferMixedSamples,
            DeviceExtensionType::NvDedicatedAllocation => Self::NvDedicatedAllocation,
            DeviceExtensionType::ExtBlendOperationAdvanced => Self::ExtBlendOperationAdvanced,
            DeviceExtensionType::KhrRayTracingPipeline => Self::KhrRayTracingPipeline,
            DeviceExtensionType::NvDedicatedAllocationImageAliasing => {
                Self::NvDedicatedAllocationImageAliasing
            }
            DeviceExtensionType::GoogleDecorateString => Self::GoogleDecorateString,
            DeviceExtensionType::ExtColorWriteEnable => Self::ExtColorWriteEnable,
            DeviceExtensionType::ExtBorderColorSwizzle => Self::ExtBorderColorSwizzle,
            DeviceExtensionType::AmdNegativeViewportHeight => Self::AmdNegativeViewportHeight,
            DeviceExtensionType::ExtRgba10x6Formats => Self::ExtRgba10x6Formats,
            DeviceExtensionType::NvCornerSampledImage => Self::NvCornerSampledImage,
            DeviceExtensionType::ExtRobustness2 => Self::ExtRobustness2,
            DeviceExtensionType::NvFragmentShaderBarycentric => Self::NvFragmentShaderBarycentric,
            DeviceExtensionType::NvDeviceDiagnosticsConfig => Self::NvDeviceDiagnosticsConfig,
            DeviceExtensionType::NvMeshShader => Self::NvMeshShader,
            DeviceExtensionType::AmdGpuShaderInt16 => Self::AmdGpuShaderInt16,
            DeviceExtensionType::ExtDeviceMemoryReport => Self::ExtDeviceMemoryReport,
            DeviceExtensionType::AmdShaderCoreProperties2 => Self::AmdShaderCoreProperties2,
        }
    }
}
#[derive(Clone)]
pub struct InstanceExtension<const EXT: PhysicalInstanceExtensionType> {
    pub instance: std::sync::Arc<crate::instance::Instance>,
    pub(crate) _p: std::marker::PhantomData<usize>,
}
#[derive(Clone)]
pub struct DeviceExtension<const EXT: PhysicalDeviceExtensionType> {
    pub device: std::sync::Arc<crate::device::Device>,
    pub(crate) _p: std::marker::PhantomData<usize>,
}
