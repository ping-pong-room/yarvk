use crate::device_features::Feature;
use crate::extensions::DeviceExtension;
use crate::instance::Instance;
use crate::physical_device::memory_properties::PhysicalDeviceMemoryProperties;
use crate::physical_device::queue_family_properties::QueueFamilyProperties;
use rustc_hash::FxHashSet;
use std::ffi::CStr;
use std::sync::Arc;

pub mod memory_properties;
pub mod physical_device_properties;
pub mod queue_family_properties;

#[derive(Clone)]
pub enum SharingMode {
    EXCLUSIVE,
    CONCURRENT(FxHashSet<QueueFamilyProperties>),
}

impl Default for SharingMode {
    fn default() -> Self {
        SharingMode::EXCLUSIVE
    }
}

pub struct PhysicalDevice {
    pub instance: Arc<Instance>,
    pub(crate) vk_physical_device: ash::vk::PhysicalDevice,
    pub(crate) memory_properties: PhysicalDeviceMemoryProperties,
    pub(super) physical_device_properties: ash::vk::PhysicalDeviceProperties,
    supported_extensions: Vec<ash::vk::ExtensionProperties>,
}

impl PhysicalDevice {
    pub(crate) fn new(
        instance: Arc<Instance>,
        vk_physical_device: ash::vk::PhysicalDevice,
    ) -> Result<Arc<Self>, ash::vk::Result> {
        let memory_properties =
            Self::memory_properties_inner(&instance.ash_instance, vk_physical_device);
        let physical_device_properties = unsafe {
            // Host Synchronization: none
            instance
                .ash_instance
                .get_physical_device_properties(vk_physical_device)
        };
        let supported_extensions = unsafe {
            // Host Synchronization: none
            instance
                .ash_instance
                .enumerate_device_extension_properties(vk_physical_device)
                .unwrap()
        };
        Ok(Arc::new(Self {
            instance,
            vk_physical_device,
            memory_properties,
            physical_device_properties,
            supported_extensions,
        }))
    }

    pub fn supported_feature<T: Feature>(&self) -> bool {
        unsafe {
            let mut feature_holder = ash::vk::PhysicalDeviceFeatures2::default();
            let mut feature2 = T::VkFeatureType::default();
            if T::IS_FEATURE2 {
                feature_holder.p_next = <*mut T::VkFeatureType>::cast(&mut feature2);
            }
            self.instance
                .ash_instance
                .get_physical_device_features2(self.vk_physical_device, &mut feature_holder);
            if T::IS_FEATURE2 {
                T::is_enabled(&feature2)
            } else {
                T::is_enabled(std::mem::transmute(&feature_holder.features))
            }
        }
    }

    pub fn supported_extension<EXT: DeviceExtension>(&self) -> bool {
        self.supported_extensions
            .iter()
            .find(|ext| unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) == EXT::NAME })
            .is_some()
    }
}
