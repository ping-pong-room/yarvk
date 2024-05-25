pub trait Extension: Clone {
    const NAME: &'static std::ffi::CStr;
    const DEPENDENCIES: &'static [&'static std::ffi::CStr];
}
pub trait InstanceExtension: Extension {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self;
}
pub trait DeviceExtension: Extension {
    type InstanceDependenciesTy;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self;
}
#[derive(Clone)]
pub struct ExtensionKhrWin32Surface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrWin32Surface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionKhrWin32Surface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrAndroidSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrAndroidSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_android_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionKhrAndroidSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionGoogleSurfacelessQuery {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionGoogleSurfacelessQuery {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_surfaceless_query\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionGoogleSurfacelessQuery {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDebugReport {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtDebugReport {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_report\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl InstanceExtension for ExtensionExtDebugReport {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDebugUtils {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtDebugUtils {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_utils\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl InstanceExtension for ExtensionExtDebugUtils {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtValidationFlags {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtValidationFlags {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_flags\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl InstanceExtension for ExtensionExtValidationFlags {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrWaylandSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrWaylandSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_wayland_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionKhrWaylandSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrPortabilityEnumeration {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrPortabilityEnumeration {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_enumeration\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl InstanceExtension for ExtensionKhrPortabilityEnumeration {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDisplaySurfaceCounter {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtDisplaySurfaceCounter {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_surface_counter\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionKhrDisplay::NAME, ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionExtDisplaySurfaceCounter {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtValidationFeatures {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtValidationFeatures {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_features\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl InstanceExtension for ExtensionExtValidationFeatures {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionMvkMacosSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionMvkMacosSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MVK_macos_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionMvkMacosSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrXcbSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrXcbSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xcb_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionKhrXcbSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionMvkIosSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionMvkIosSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MVK_ios_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionMvkIosSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionGgpStreamDescriptorSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionGgpStreamDescriptorSurface {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_stream_descriptor_surface\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionGgpStreamDescriptorSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDirectModeDisplay {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtDirectModeDisplay {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_direct_mode_display\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionKhrDisplay::NAME, ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionExtDirectModeDisplay {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrSurfaceProtectedCapabilities {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrSurfaceProtectedCapabilities {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface_protected_capabilities\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrGetSurfaceCapabilities2::NAME,
        ExtensionKhrSurface::NAME,
    ];
}
impl InstanceExtension for ExtensionKhrSurfaceProtectedCapabilities {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrXlibSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrXlibSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xlib_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionKhrXlibSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDirectfbSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtDirectfbSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_directfb_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionExtDirectfbSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvExternalMemoryCapabilities {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionNvExternalMemoryCapabilities {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_capabilities\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl InstanceExtension for ExtensionNvExternalMemoryCapabilities {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl InstanceExtension for ExtensionKhrSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNnViSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionNnViSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NN_vi_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionNnViSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtMetalSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtMetalSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_metal_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionExtMetalSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtAcquireXlibDisplay {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtAcquireXlibDisplay {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_acquire_xlib_display\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrSurface::NAME,
        ExtensionExtDirectModeDisplay::NAME,
        ExtensionKhrDisplay::NAME,
    ];
}
impl InstanceExtension for ExtensionExtAcquireXlibDisplay {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtSwapchainColorspace {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtSwapchainColorspace {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_swapchain_colorspace\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionExtSwapchainColorspace {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrGetDisplayProperties2 {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrGetDisplayProperties2 {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_display_properties2\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionKhrDisplay::NAME, ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionKhrGetDisplayProperties2 {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionQnxScreenSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionQnxScreenSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QNX_screen_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionQnxScreenSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtHeadlessSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtHeadlessSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_headless_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionExtHeadlessSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrDisplay {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrDisplay {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionKhrDisplay {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionFuchsiaImagepipeSurface {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionFuchsiaImagepipeSurface {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_imagepipe_surface\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionFuchsiaImagepipeSurface {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtAcquireDrmDisplay {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionExtAcquireDrmDisplay {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_acquire_drm_display\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrSurface::NAME,
        ExtensionExtDirectModeDisplay::NAME,
        ExtensionKhrDisplay::NAME,
    ];
}
impl InstanceExtension for ExtensionExtAcquireDrmDisplay {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrGetSurfaceCapabilities2 {
    pub instance: std::sync::Arc<crate::instance::Instance>,
}
impl Extension for ExtensionKhrGetSurfaceCapabilities2 {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_surface_capabilities2\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
}
impl InstanceExtension for ExtensionKhrGetSurfaceCapabilities2 {
    fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
        Self {
            instance: instance.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvCoverageReductionMode {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvCoverageReductionMode {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_coverage_reduction_mode\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionNvFramebufferMixedSamples::NAME];
}
impl DeviceExtension for ExtensionNvCoverageReductionMode {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrIncrementalPresent {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrIncrementalPresent {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_incremental_present\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionKhrIncrementalPresent {
    type InstanceDependenciesTy = ExtensionKhrSurface;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrVideoEncodeQueue {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrVideoEncodeQueue {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_encode_queue\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrVideoQueue::NAME];
}
impl DeviceExtension for ExtensionKhrVideoEncodeQueue {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtMemoryBudget {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtMemoryBudget {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_budget\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtMemoryBudget {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvAcquireWinrtDisplay {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvAcquireWinrtDisplay {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_acquire_winrt_display\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvAcquireWinrtDisplay {
    type InstanceDependenciesTy = ExtensionExtDirectModeDisplay;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionExtDirectModeDisplay::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrSwapchainMutableFormat {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrSwapchainMutableFormat {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain_mutable_format\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionKhrSwapchainMutableFormat {
    type InstanceDependenciesTy = ExtensionKhrSurface;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionHuaweiSubpassShading {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionHuaweiSubpassShading {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_subpass_shading\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionHuaweiSubpassShading {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdShaderBallot {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdShaderBallot {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_ballot\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdShaderBallot {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdDisplayNativeHdr {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdDisplayNativeHdr {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_display_native_hdr\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionAmdDisplayNativeHdr {
    type InstanceDependenciesTy = ((ExtensionKhrGetSurfaceCapabilities2, ExtensionKhrSurface));
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrGetSurfaceCapabilities2::NAME,
        ExtensionKhrSurface::NAME,
    ];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtPhysicalDeviceDrm {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtPhysicalDeviceDrm {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_physical_device_drm\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtPhysicalDeviceDrm {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtMultiDraw {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtMultiDraw {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_multi_draw\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtMultiDraw {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionImgFilterCubic {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionImgFilterCubic {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_filter_cubic\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionImgFilterCubic {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvxImageViewHandle {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvxImageViewHandle {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_image_view_handle\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvxImageViewHandle {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvDeviceDiagnosticCheckpoints {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvDeviceDiagnosticCheckpoints {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_diagnostic_checkpoints\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvDeviceDiagnosticCheckpoints {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDepthRangeUnrestricted {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtDepthRangeUnrestricted {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_range_unrestricted\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtDepthRangeUnrestricted {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrExternalSemaphoreWin32 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrExternalSemaphoreWin32 {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_win32\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrExternalSemaphoreWin32 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionQcomFragmentDensityMapOffset {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionQcomFragmentDensityMapOffset {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_fragment_density_map_offset\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionExtFragmentDensityMap::NAME];
}
impl DeviceExtension for ExtensionQcomFragmentDensityMapOffset {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrExternalSemaphoreFd {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrExternalSemaphoreFd {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_fd\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrExternalSemaphoreFd {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvComputeShaderDerivatives {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvComputeShaderDerivatives {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_compute_shader_derivatives\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvComputeShaderDerivatives {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtCalibratedTimestamps {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtCalibratedTimestamps {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_calibrated_timestamps\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtCalibratedTimestamps {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtMemoryPriority {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtMemoryPriority {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_priority\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtMemoryPriority {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdMemoryOverallocationBehavior {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdMemoryOverallocationBehavior {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_memory_overallocation_behavior\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdMemoryOverallocationBehavior {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvClipSpaceWScaling {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvClipSpaceWScaling {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_clip_space_w_scaling\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvClipSpaceWScaling {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrExternalMemoryWin32 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrExternalMemoryWin32 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_win32\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrExternalMemoryWin32 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdShaderExplicitVertexParameter {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdShaderExplicitVertexParameter {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_explicit_vertex_parameter\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdShaderExplicitVertexParameter {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdMixedAttachmentSamples {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdMixedAttachmentSamples {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_mixed_attachment_samples\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdMixedAttachmentSamples {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvShaderImageFootprint {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvShaderImageFootprint {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_image_footprint\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvShaderImageFootprint {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtShaderSubgroupBallot {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtShaderSubgroupBallot {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_ballot\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtShaderSubgroupBallot {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdGpuShaderHalfFloat {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdGpuShaderHalfFloat {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_half_float\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdGpuShaderHalfFloat {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDiscardRectangles {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtDiscardRectangles {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_discard_rectangles\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtDiscardRectangles {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdPipelineCompilerControl {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdPipelineCompilerControl {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_pipeline_compiler_control\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdPipelineCompilerControl {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrPresentWait {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrPresentWait {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_present_wait\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionKhrSwapchain::NAME, ExtensionKhrPresentId::NAME];
}
impl DeviceExtension for ExtensionKhrPresentWait {
    type InstanceDependenciesTy = ExtensionKhrSurface;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvViewportArray2 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvViewportArray2 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_array2\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvViewportArray2 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtPostDepthCoverage {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtPostDepthCoverage {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_post_depth_coverage\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtPostDepthCoverage {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrDisplaySwapchain {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrDisplaySwapchain {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display_swapchain\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionKhrDisplaySwapchain {
    type InstanceDependenciesTy = ((ExtensionKhrDisplay, ExtensionKhrSurface));
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionKhrDisplay::NAME, ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdShaderInfo {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdShaderInfo {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_info\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdShaderInfo {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrVideoDecodeQueue {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrVideoDecodeQueue {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_decode_queue\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrVideoQueue::NAME];
}
impl DeviceExtension for ExtensionKhrVideoDecodeQueue {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvFillRectangle {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvFillRectangle {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fill_rectangle\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvFillRectangle {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdGcnShader {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdGcnShader {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gcn_shader\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdGcnShader {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtLoadStoreOpNone {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtLoadStoreOpNone {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_load_store_op_none\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtLoadStoreOpNone {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvSampleMaskOverrideCoverage {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvSampleMaskOverrideCoverage {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_sample_mask_override_coverage\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvSampleMaskOverrideCoverage {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvGeometryShaderPassthrough {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvGeometryShaderPassthrough {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_geometry_shader_passthrough\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvGeometryShaderPassthrough {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtQueueFamilyForeign {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtQueueFamilyForeign {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_queue_family_foreign\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtQueueFamilyForeign {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtImageDrmFormatModifier {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtImageDrmFormatModifier {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_drm_format_modifier\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtImageDrmFormatModifier {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdShaderFragmentMask {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdShaderFragmentMask {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_fragment_mask\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdShaderFragmentMask {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtHdrMetadata {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtHdrMetadata {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_hdr_metadata\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionExtHdrMetadata {
    type InstanceDependenciesTy = ExtensionKhrSurface;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtFilterCubic {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtFilterCubic {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_filter_cubic\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtFilterCubic {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtPageableDeviceLocalMemory {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtPageableDeviceLocalMemory {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pageable_device_local_memory\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionExtMemoryPriority::NAME];
}
impl DeviceExtension for ExtensionExtPageableDeviceLocalMemory {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdTextureGatherBiasLod {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdTextureGatherBiasLod {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_texture_gather_bias_lod\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdTextureGatherBiasLod {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtTransformFeedback {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtTransformFeedback {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_transform_feedback\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtTransformFeedback {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrShaderSubgroupUniformControlFlow {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrShaderSubgroupUniformControlFlow {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_KHR_shader_subgroup_uniform_control_flow\0",
        )
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrShaderSubgroupUniformControlFlow {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrDeferredHostOperations {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrDeferredHostOperations {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_deferred_host_operations\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrDeferredHostOperations {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDepthClipControl {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtDepthClipControl {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clip_control\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtDepthClipControl {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionValveDescriptorSetHostMapping {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionValveDescriptorSetHostMapping {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_VALVE_descriptor_set_host_mapping\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionValveDescriptorSetHostMapping {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtIndexTypeUint8 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtIndexTypeUint8 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_index_type_uint8\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtIndexTypeUint8 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrPushDescriptor {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrPushDescriptor {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_push_descriptor\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrPushDescriptor {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDepthClipEnable {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtDepthClipEnable {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clip_enable\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtDepthClipEnable {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvInheritedViewportScissor {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvInheritedViewportScissor {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_inherited_viewport_scissor\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvInheritedViewportScissor {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAndroidExternalMemoryAndroidHardwareBuffer {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAndroidExternalMemoryAndroidHardwareBuffer {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_ANDROID_external_memory_android_hardware_buffer\0",
        )
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionExtQueueFamilyForeign::NAME];
}
impl DeviceExtension for ExtensionAndroidExternalMemoryAndroidHardwareBuffer {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionGoogleHlslFunctionality1 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionGoogleHlslFunctionality1 {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_hlsl_functionality1\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionGoogleHlslFunctionality1 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtShaderImageAtomicInt64 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtShaderImageAtomicInt64 {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_image_atomic_int64\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtShaderImageAtomicInt64 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtShaderAtomicFloat {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtShaderAtomicFloat {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_atomic_float\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtShaderAtomicFloat {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionFuchsiaExternalMemory {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionFuchsiaExternalMemory {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_memory\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionFuchsiaExternalMemory {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrFragmentShadingRate {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrFragmentShadingRate {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_fragment_shading_rate\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrFragmentShadingRate {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionFuchsiaExternalSemaphore {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionFuchsiaExternalSemaphore {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_semaphore\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionFuchsiaExternalSemaphore {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdShaderCoreProperties {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdShaderCoreProperties {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdShaderCoreProperties {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionGgpFrameToken {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionGgpFrameToken {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_frame_token\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionGgpFrameToken {
    type InstanceDependenciesTy = ((ExtensionGgpStreamDescriptorSurface, ExtensionKhrSurface));
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionGgpStreamDescriptorSurface::NAME,
        ExtensionKhrSurface::NAME,
    ];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdShaderImageLoadStoreLod {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdShaderImageLoadStoreLod {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_image_load_store_lod\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdShaderImageLoadStoreLod {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtShaderSubgroupVote {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtShaderSubgroupVote {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_vote\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtShaderSubgroupVote {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtLineRasterization {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtLineRasterization {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_line_rasterization\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtLineRasterization {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrGlobalPriority {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrGlobalPriority {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_global_priority\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrGlobalPriority {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrPresentId {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrPresentId {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_present_id\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionKhrPresentId {
    type InstanceDependenciesTy = ExtensionKhrSurface;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrVideoQueue {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrVideoQueue {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_queue\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrVideoQueue {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrShaderClock {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrShaderClock {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_clock\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrShaderClock {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvFragmentCoverageToColor {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvFragmentCoverageToColor {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_coverage_to_color\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvFragmentCoverageToColor {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDisplayControl {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtDisplayControl {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_control\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionExtDisplayControl {
    type InstanceDependenciesTy = ((ExtensionExtDisplaySurfaceCounter, ExtensionKhrSurface));
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionExtDisplaySurfaceCounter::NAME,
        ExtensionKhrSurface::NAME,
    ];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvShaderSmBuiltins {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvShaderSmBuiltins {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_sm_builtins\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvShaderSmBuiltins {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrSharedPresentableImage {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrSharedPresentableImage {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shared_presentable_image\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionKhrSharedPresentableImage {
    type InstanceDependenciesTy = ((ExtensionKhrGetSurfaceCapabilities2, ExtensionKhrSurface));
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrGetSurfaceCapabilities2::NAME,
        ExtensionKhrSurface::NAME,
    ];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtVideoDecodeH264 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtVideoDecodeH264 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_decode_h264\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrVideoDecodeQueue::NAME,
        ExtensionKhrVideoQueue::NAME,
    ];
}
impl DeviceExtension for ExtensionExtVideoDecodeH264 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionQcomRenderPassTransform {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionQcomRenderPassTransform {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_transform\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionQcomRenderPassTransform {
    type InstanceDependenciesTy = ExtensionKhrSurface;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionGoogleUserType {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionGoogleUserType {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_user_type\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionGoogleUserType {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdRasterizationOrder {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdRasterizationOrder {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_rasterization_order\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdRasterizationOrder {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtBufferDeviceAddress {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtBufferDeviceAddress {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_buffer_device_address\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtBufferDeviceAddress {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtConservativeRasterization {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtConservativeRasterization {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_conservative_rasterization\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtConservativeRasterization {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionFuchsiaBufferCollection {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionFuchsiaBufferCollection {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_buffer_collection\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionFuchsiaExternalMemory::NAME];
}
impl DeviceExtension for ExtensionFuchsiaBufferCollection {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionIntelShaderIntegerFunctions2 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionIntelShaderIntegerFunctions2 {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_shader_integer_functions2\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionIntelShaderIntegerFunctions2 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrWorkgroupMemoryExplicitLayout {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrWorkgroupMemoryExplicitLayout {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_workgroup_memory_explicit_layout\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrWorkgroupMemoryExplicitLayout {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtVertexInputDynamicState {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtVertexInputDynamicState {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_vertex_input_dynamic_state\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtVertexInputDynamicState {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvRepresentativeFragmentTest {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvRepresentativeFragmentTest {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_representative_fragment_test\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvRepresentativeFragmentTest {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdBufferMarker {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdBufferMarker {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_buffer_marker\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdBufferMarker {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdShaderTrinaryMinmax {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdShaderTrinaryMinmax {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_trinary_minmax\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdShaderTrinaryMinmax {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrExternalFenceFd {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrExternalFenceFd {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_fd\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrExternalFenceFd {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvShadingRateImage {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvShadingRateImage {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shading_rate_image\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvShadingRateImage {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvScissorExclusive {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvScissorExclusive {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_scissor_exclusive\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvScissorExclusive {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvCooperativeMatrix {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvCooperativeMatrix {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_cooperative_matrix\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvCooperativeMatrix {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvDeviceGeneratedCommands {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvDeviceGeneratedCommands {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_generated_commands\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvDeviceGeneratedCommands {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrRayQuery {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrRayQuery {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_query\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrDeferredHostOperations::NAME,
        ExtensionKhrAccelerationStructure::NAME,
    ];
}
impl DeviceExtension for ExtensionKhrRayQuery {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtPciBusInfo {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtPciBusInfo {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pci_bus_info\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtPciBusInfo {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtFragmentDensityMap2 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtFragmentDensityMap2 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map2\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionExtFragmentDensityMap::NAME];
}
impl DeviceExtension for ExtensionExtFragmentDensityMap2 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionArmRasterizationOrderAttachmentAccess {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionArmRasterizationOrderAttachmentAccess {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_ARM_rasterization_order_attachment_access\0",
        )
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionArmRasterizationOrderAttachmentAccess {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionValveMutableDescriptorType {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionValveMutableDescriptorType {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_VALVE_mutable_descriptor_type\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionValveMutableDescriptorType {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtConditionalRendering {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtConditionalRendering {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_conditional_rendering\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtConditionalRendering {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvFragmentShadingRateEnums {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvFragmentShadingRateEnums {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_shading_rate_enums\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionKhrFragmentShadingRate::NAME];
}
impl DeviceExtension for ExtensionNvFragmentShadingRateEnums {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrExternalFenceWin32 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrExternalFenceWin32 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_win32\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrExternalFenceWin32 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrPerformanceQuery {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrPerformanceQuery {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_performance_query\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrPerformanceQuery {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionImgFormatPvrtc {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionImgFormatPvrtc {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_format_pvrtc\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionImgFormatPvrtc {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrPortabilitySubset {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrPortabilitySubset {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_subset\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrPortabilitySubset {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionQcomRenderPassShaderResolve {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionQcomRenderPassShaderResolve {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_shader_resolve\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionQcomRenderPassShaderResolve {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvxMultiviewPerViewAttributes {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvxMultiviewPerViewAttributes {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_multiview_per_view_attributes\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvxMultiviewPerViewAttributes {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtValidationCache {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtValidationCache {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_cache\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtValidationCache {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvxBinaryImport {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvxBinaryImport {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_binary_import\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvxBinaryImport {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtVideoEncodeH265 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtVideoEncodeH265 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_encode_h265\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrVideoQueue::NAME,
        ExtensionKhrVideoEncodeQueue::NAME,
    ];
}
impl DeviceExtension for ExtensionExtVideoEncodeH265 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtShaderStencilExport {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtShaderStencilExport {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_stencil_export\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtShaderStencilExport {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrAccelerationStructure {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrAccelerationStructure {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_acceleration_structure\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionKhrDeferredHostOperations::NAME];
}
impl DeviceExtension for ExtensionKhrAccelerationStructure {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrSwapchain {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrSwapchain {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrSwapchain {
    type InstanceDependenciesTy = ExtensionKhrSurface;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvRayTracing {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvRayTracing {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvRayTracing {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtAstcDecodeMode {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtAstcDecodeMode {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_astc_decode_mode\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtAstcDecodeMode {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtFragmentShaderInterlock {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtFragmentShaderInterlock {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_shader_interlock\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtFragmentShaderInterlock {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionQcomRotatedCopyCommands {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionQcomRotatedCopyCommands {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_rotated_copy_commands\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionQcomRotatedCopyCommands {
    type InstanceDependenciesTy = ExtensionKhrSurface;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtVideoDecodeH265 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtVideoDecodeH265 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_decode_h265\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrVideoDecodeQueue::NAME,
        ExtensionKhrVideoQueue::NAME,
    ];
}
impl DeviceExtension for ExtensionExtVideoDecodeH265 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtShaderAtomicFloat2 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtShaderAtomicFloat2 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_atomic_float2\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionExtShaderAtomicFloat::NAME];
}
impl DeviceExtension for ExtensionExtShaderAtomicFloat2 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvGlslShader {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvGlslShader {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_glsl_shader\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvGlslShader {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtFullScreenExclusive {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtFullScreenExclusive {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_full_screen_exclusive\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionExtFullScreenExclusive {
    type InstanceDependenciesTy = ((ExtensionKhrGetSurfaceCapabilities2, ExtensionKhrSurface));
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrGetSurfaceCapabilities2::NAME,
        ExtensionKhrSurface::NAME,
    ];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtYcbcrImageArrays {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtYcbcrImageArrays {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_ycbcr_image_arrays\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtYcbcrImageArrays {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvRayTracingMotionBlur {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvRayTracingMotionBlur {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing_motion_blur\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrRayTracingPipeline::NAME,
        ExtensionKhrDeferredHostOperations::NAME,
        ExtensionKhrAccelerationStructure::NAME,
    ];
}
impl DeviceExtension for ExtensionNvRayTracingMotionBlur {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtVertexAttributeDivisor {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtVertexAttributeDivisor {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_vertex_attribute_divisor\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtVertexAttributeDivisor {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrExternalMemoryFd {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrExternalMemoryFd {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_fd\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrExternalMemoryFd {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrWin32KeyedMutex {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrWin32KeyedMutex {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_keyed_mutex\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionKhrExternalMemoryWin32::NAME];
}
impl DeviceExtension for ExtensionKhrWin32KeyedMutex {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvExternalMemoryWin32 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvExternalMemoryWin32 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_win32\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionNvExternalMemory::NAME];
}
impl DeviceExtension for ExtensionNvExternalMemoryWin32 {
    type InstanceDependenciesTy = ExtensionNvExternalMemoryCapabilities;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionNvExternalMemoryCapabilities::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvShaderSubgroupPartitioned {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvShaderSubgroupPartitioned {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_subgroup_partitioned\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvShaderSubgroupPartitioned {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtExternalMemoryHost {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtExternalMemoryHost {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_host\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtExternalMemoryHost {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtExternalMemoryDmaBuf {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtExternalMemoryDmaBuf {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_dma_buf\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrExternalMemoryFd::NAME];
}
impl DeviceExtension for ExtensionExtExternalMemoryDmaBuf {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionGoogleDisplayTiming {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionGoogleDisplayTiming {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_display_timing\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSwapchain::NAME];
}
impl DeviceExtension for ExtensionGoogleDisplayTiming {
    type InstanceDependenciesTy = ExtensionKhrSurface;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionKhrSurface::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvExternalMemory {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvExternalMemory {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvExternalMemory {
    type InstanceDependenciesTy = ExtensionNvExternalMemoryCapabilities;
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionNvExternalMemoryCapabilities::NAME];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvExternalMemoryRdma {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvExternalMemoryRdma {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_rdma\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvExternalMemoryRdma {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionQcomRenderPassStoreOps {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionQcomRenderPassStoreOps {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_store_ops\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionQcomRenderPassStoreOps {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtFragmentDensityMap {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtFragmentDensityMap {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtFragmentDensityMap {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtPrimitiveTopologyListRestart {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtPrimitiveTopologyListRestart {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_primitive_topology_list_restart\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtPrimitiveTopologyListRestart {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrPipelineExecutableProperties {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrPipelineExecutableProperties {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_pipeline_executable_properties\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrPipelineExecutableProperties {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvViewportSwizzle {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvViewportSwizzle {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_swizzle\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvViewportSwizzle {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvLinearColorAttachment {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvLinearColorAttachment {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_linear_color_attachment\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvLinearColorAttachment {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtImageViewMinLod {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtImageViewMinLod {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_view_min_lod\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtImageViewMinLod {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtCustomBorderColor {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtCustomBorderColor {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_custom_border_color\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtCustomBorderColor {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionHuaweiInvocationMask {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionHuaweiInvocationMask {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_invocation_mask\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrRayTracingPipeline::NAME,
        ExtensionKhrDeferredHostOperations::NAME,
        ExtensionKhrAccelerationStructure::NAME,
    ];
}
impl DeviceExtension for ExtensionHuaweiInvocationMask {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionIntelPerformanceQuery {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionIntelPerformanceQuery {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_performance_query\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionIntelPerformanceQuery {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtProvokingVertex {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtProvokingVertex {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_provoking_vertex\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtProvokingVertex {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrPipelineLibrary {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrPipelineLibrary {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_pipeline_library\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionKhrPipelineLibrary {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtVideoEncodeH264 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtVideoEncodeH264 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_video_encode_h264\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrVideoQueue::NAME,
        ExtensionKhrVideoEncodeQueue::NAME,
    ];
}
impl DeviceExtension for ExtensionExtVideoEncodeH264 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdDeviceCoherentMemory {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdDeviceCoherentMemory {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_device_coherent_memory\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdDeviceCoherentMemory {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtSampleLocations {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtSampleLocations {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_sample_locations\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtSampleLocations {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvFramebufferMixedSamples {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvFramebufferMixedSamples {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_framebuffer_mixed_samples\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvFramebufferMixedSamples {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvDedicatedAllocation {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvDedicatedAllocation {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_dedicated_allocation\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvDedicatedAllocation {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtBlendOperationAdvanced {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtBlendOperationAdvanced {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_blend_operation_advanced\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtBlendOperationAdvanced {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionKhrRayTracingPipeline {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionKhrRayTracingPipeline {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_tracing_pipeline\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
        ExtensionKhrDeferredHostOperations::NAME,
        ExtensionKhrAccelerationStructure::NAME,
    ];
}
impl DeviceExtension for ExtensionKhrRayTracingPipeline {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvDedicatedAllocationImageAliasing {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvDedicatedAllocationImageAliasing {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_NV_dedicated_allocation_image_aliasing\0",
        )
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvDedicatedAllocationImageAliasing {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionGoogleDecorateString {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionGoogleDecorateString {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_decorate_string\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionGoogleDecorateString {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtColorWriteEnable {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtColorWriteEnable {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_color_write_enable\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtColorWriteEnable {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtBorderColorSwizzle {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtBorderColorSwizzle {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_border_color_swizzle\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[ExtensionExtCustomBorderColor::NAME];
}
impl DeviceExtension for ExtensionExtBorderColorSwizzle {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdNegativeViewportHeight {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdNegativeViewportHeight {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_negative_viewport_height\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdNegativeViewportHeight {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtRgba10x6Formats {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtRgba10x6Formats {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_rgba10x6_formats\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtRgba10x6Formats {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvCornerSampledImage {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvCornerSampledImage {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_corner_sampled_image\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvCornerSampledImage {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtRobustness2 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtRobustness2 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_robustness2\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtRobustness2 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvFragmentShaderBarycentric {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvFragmentShaderBarycentric {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_shader_barycentric\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvFragmentShaderBarycentric {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvDeviceDiagnosticsConfig {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvDeviceDiagnosticsConfig {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_diagnostics_config\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvDeviceDiagnosticsConfig {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionNvMeshShader {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionNvMeshShader {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_mesh_shader\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionNvMeshShader {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdGpuShaderInt16 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdGpuShaderInt16 {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_int16\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionAmdGpuShaderInt16 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionExtDeviceMemoryReport {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionExtDeviceMemoryReport {
    const NAME: &'static std::ffi::CStr =
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_device_memory_report\0") };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
}
impl DeviceExtension for ExtensionExtDeviceMemoryReport {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
#[derive(Clone)]
pub struct ExtensionAmdShaderCoreProperties2 {
    pub device: std::sync::Arc<crate::device::Device>,
}
impl Extension for ExtensionAmdShaderCoreProperties2 {
    const NAME: &'static std::ffi::CStr = unsafe {
        std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties2\0")
    };
    const DEPENDENCIES: &'static [&'static std::ffi::CStr] =
        &[ExtensionAmdShaderCoreProperties::NAME];
}
impl DeviceExtension for ExtensionAmdShaderCoreProperties2 {
    type InstanceDependenciesTy = ();
    const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
    fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
        Self {
            device: device.clone(),
        }
    }
}
