use crate::extensions::{InstanceExtension, PhysicalInstanceExtensionType};
use crate::physical_device::queue_family_properties::QueueFamilyProperties;
use crate::physical_device::PhysicalDevice;
use raw_window_handle::RawWindowHandle;
use raw_window_handle::RawDisplayHandle;
use std::sync::Arc;

pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    pub local_dimming_support: bool,
}

pub struct SharedPresentSurfaceCapabilitiesKHR {
    pub shared_present_supported_usage_flags: ash::vk::ImageUsageFlags,
}

pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    pub full_screen_exclusive_supported: bool,
}

pub struct SurfaceProtectedCapabilitiesKHR {
    pub supports_protected: bool,
}

pub struct Surface {
    pub physical_device: Arc<PhysicalDevice>,
    loader: ash::extensions::khr::Surface,
    pub(crate) vk_surface_khr: ash::vk::SurfaceKHR,
    pub(crate) supported_formats: Vec<ash::vk::SurfaceFormatKHR>,
    pub(crate) capabilities: ash::vk::SurfaceCapabilitiesKHR,
    pub(crate) present_modes: Vec<ash::vk::PresentModeKHR>,
}

impl Surface {
    pub fn get_physical_device_surface_support(
        _surface_ext: InstanceExtension<{ PhysicalInstanceExtensionType::KhrSurface }>,
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
        queue_family: &QueueFamilyProperties,
    ) -> Result<Option<Arc<Self>>, ash::vk::Result> {
        let physical_device = &queue_family.physical_device;
        let instance = &physical_device.instance;
        let vk_surface_khr = unsafe {
            crate::window::create_surface(
                &instance.entry.ash_entry,
                &instance.ash_instance,
                display_handle,
                window_handle,
                None,
            )?
        };
        // Done VUID-vkGetPhysicalDeviceSurfaceSupportKHR-physicalDevice-parameter
        // Done VUID-vkGetPhysicalDeviceSurfaceSupportKHR-surface-parameter
        // Done VUID-vkGetPhysicalDeviceSurfaceSupportKHR-pSupported-parameter
        // Done VUID-vkGetPhysicalDeviceSurfaceSupportKHR-commonparent

        let loader =
            ash::extensions::khr::Surface::new(&instance.entry.ash_entry, &instance.ash_instance);

        // TODO VUID-vkGetPhysicalDeviceSurfaceFormatsKHR-surface-06524
        // Done VUID-vkGetPhysicalDeviceSurfaceFormatsKHR-surface-06525
        // Host Synchronization: none
        let supported_formats = unsafe {
            loader.get_physical_device_surface_formats(
                physical_device.vk_physical_device,
                vk_surface_khr,
            )?
        };

        // Done VUID-vkGetPhysicalDeviceSurfaceCapabilitiesKHR-surface-06523
        // Done VUID-vkGetPhysicalDeviceSurfaceCapabilitiesKHR-surface-06211
        // Host Synchronization: none
        let capabilities = unsafe {
            loader.get_physical_device_surface_capabilities(
                physical_device.vk_physical_device,
                vk_surface_khr,
            )?
        };

        // TODO VUID-vkGetPhysicalDeviceSurfacePresentModesKHR-surface-06524
        // DONE VUID-vkGetPhysicalDeviceSurfacePresentModesKHR-surface-06525
        // Host Synchronization: none
        let present_modes = unsafe {
            loader.get_physical_device_surface_present_modes(
                physical_device.vk_physical_device,
                vk_surface_khr,
            )?
        };

        unsafe {
            // Host Synchronization: none
            if loader.get_physical_device_surface_support(
                physical_device.vk_physical_device,
                queue_family.index,
                vk_surface_khr,
            )? {
                Ok(Some(Arc::new(Self {
                    physical_device: physical_device.clone(),
                    loader,
                    vk_surface_khr,
                    supported_formats,
                    capabilities,
                    present_modes,
                })))
            } else {
                Ok(None)
            }
        }
    }

    pub fn get_physical_device_surface_formats(&self) -> &[ash::vk::SurfaceFormatKHR] {
        self.supported_formats.as_slice()
    }
    pub fn get_physical_device_surface_capabilities(&self) -> &ash::vk::SurfaceCapabilitiesKHR {
        &self.capabilities
    }
    pub fn get_physical_device_capabilities2<
        T: ash::vk::ExtendsPhysicalDeviceSurfaceInfo2KHR + Default,
    >(
        &self,
    ) -> Result<T, ash::vk::Result> {
        unsafe {
            // Host Synchronization: none
            let mut t = T::default();
            let info = ash::vk::PhysicalDeviceSurfaceInfo2KHR::builder().push_next(&mut t);
            ash::extensions::khr::GetSurfaceCapabilities2::new(
                &self.physical_device.instance.entry.ash_entry,
                &self.physical_device.instance.ash_instance,
            )
            .get_physical_device_surface_capabilities2(
                self.physical_device.vk_physical_device,
                &info,
            )?;
            Ok(t)
        }
    }
    pub fn get_physical_device_surface_present_modes(&self) -> &[ash::vk::PresentModeKHR] {
        self.present_modes.as_slice()
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        // TODO VUID-vkDestroySurfaceKHR-surface-01267
        // TODO VUID-vkDestroySurfaceKHR-surface-01268
        // Host Synchronization: surface
        unsafe {
            self.loader.destroy_surface(self.vk_surface_khr, None);
        }
    }
}
