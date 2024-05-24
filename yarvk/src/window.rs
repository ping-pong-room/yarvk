#![warn(trivial_casts, trivial_numeric_casts)]

use crate::extensions::{
    ExtensionExtMetalSurface, ExtensionKhrAndroidSurface, ExtensionKhrSurface,
    ExtensionKhrWaylandSurface, ExtensionKhrWin32Surface, ExtensionKhrXcbSurface,
    ExtensionKhrXlibSurface,
};
use crate::instance::InstanceBuilder;
use ash::{
    extensions::{ext, khr},
    prelude::*,
    vk, Entry, Instance,
};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

/// Create a surface from a raw surface handle.
///
/// `instance` must have created with platform specific surface extensions enabled, acquired
/// through [`enumerate_required_extensions()`].
///
/// # Safety
///
/// In order for the created [`vk::SurfaceKHR`] to be valid for the duration of its
/// usage, the [`Instance`] this was called on must be dropped later than the
/// resulting [`vk::SurfaceKHR`].
///
/// The window represented by `window_handle` must be associated with the display connection
/// in `display_handle`.
///
/// `window_handle` and `display_handle` must be associated with a valid window and display
/// connection, which must not be destroyed for the lifetime of the returned [`vk::SurfaceKHR`].
pub unsafe fn create_surface(
    entry: &Entry,
    instance: &Instance,
    display_handle: RawDisplayHandle,
    window_handle: RawWindowHandle,
    allocation_callbacks: Option<&vk::AllocationCallbacks>,
) -> VkResult<vk::SurfaceKHR> {
    match (display_handle, window_handle) {
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::builder()
                .hinstance(window.hinstance)
                .hwnd(window.hwnd);
            let surface_fn = khr::Win32Surface::new(entry, instance);
            surface_fn.create_win32_surface(&surface_desc, allocation_callbacks)
        }

        (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
            let surface_desc = vk::WaylandSurfaceCreateInfoKHR::builder()
                .display(display.display)
                .surface(window.surface);
            let surface_fn = khr::WaylandSurface::new(entry, instance);
            surface_fn.create_wayland_surface(&surface_desc, allocation_callbacks)
        }

        (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
            let surface_desc = vk::XlibSurfaceCreateInfoKHR::builder()
                .dpy(display.display.cast())
                .window(window.window);
            let surface_fn = khr::XlibSurface::new(entry, instance);
            surface_fn.create_xlib_surface(&surface_desc, allocation_callbacks)
        }

        (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
            let surface_desc = vk::XcbSurfaceCreateInfoKHR::builder()
                .connection(display.connection)
                .window(window.window);
            let surface_fn = khr::XcbSurface::new(entry, instance);
            surface_fn.create_xcb_surface(&surface_desc, allocation_callbacks)
        }

        (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(window)) => {
            let surface_desc =
                vk::AndroidSurfaceCreateInfoKHR::builder().window(window.a_native_window);
            let surface_fn = khr::AndroidSurface::new(entry, instance);
            surface_fn.create_android_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(target_os = "macos")]
        (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
            use raw_window_metal::{appkit, Layer};

            let layer = match appkit::metal_layer_from_handle(window) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
            let surface_fn = ext::MetalSurface::new(entry, instance);
            surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(target_os = "ios")]
        (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
            use raw_window_metal::{uikit, Layer};

            let layer = match uikit::metal_layer_from_handle(window) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
            let surface_fn = ext::MetalSurface::new(entry, instance);
            surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
        }

        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}

/// Enable required instance extensions for creating a surface from a display handle.
///
/// This [`RawDisplayHandle`] can typically be acquired from a window, but is usually also
/// accessible earlier through an "event loop" concept to allow querying required instance
/// extensions and creation of a compatible Vulkan instance prior to creating a window.

pub fn enable_required_wsi_extensions(
    display_handle: RawDisplayHandle,
    instance_builder: InstanceBuilder,
) -> Result<InstanceBuilder, ash::vk::Result> {
    match display_handle {
        RawDisplayHandle::Windows(_) => {
            instance_builder
                // .add_extension::<ExtensionKhrSurface>()
                .add_extension::<ExtensionKhrWin32Surface>()
        }

        RawDisplayHandle::Wayland(_) => {
            instance_builder
                // .add_extension::<ExtensionKhrSurface>()
                .add_extension::<ExtensionKhrWaylandSurface>()
        }

        RawDisplayHandle::Xlib(_) => {
            instance_builder
                // .add_extension::<ExtensionKhrSurface>()
                .add_extension::<ExtensionKhrXlibSurface>()
        }

        RawDisplayHandle::Xcb(_) => {
            instance_builder
                // .add_extension::<ExtensionKhrSurface>()
                .add_extension::<ExtensionKhrXcbSurface>()
        }

        RawDisplayHandle::Android(_) => {
            instance_builder
                // .add_extension::<ExtensionKhrSurface>()
                .add_extension::<ExtensionKhrAndroidSurface>()
        }

        RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => {
            instance_builder
                // .add_extension::<ExtensionKhrSurface>()
                .add_extension::<ExtensionExtMetalSurface>()
        }

        // TODO Ref builders
        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}
