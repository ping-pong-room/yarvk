use crate::device_features::{PhysicalDeviceFeatures, SubPhysicalFeature, VkDeviceFeature};
use crate::extensions::PhysicalDeviceExtensionType;
use crate::instance::Instance;
use crate::physical_device::memory_properties::PhysicalDeviceMemoryProperties;
use crate::physical_device::queue_falmily_properties::QueueFamilyProperties;
use rustc_hash::FxHashSet;
use std::ffi::CStr;
use std::sync::Arc;

pub mod memory_properties;
pub mod physical_device_properties;
pub mod queue_falmily_properties;

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
    pub(crate) supported_extensions: FxHashSet<PhysicalDeviceExtensionType>,
    pub(crate) memory_properties: PhysicalDeviceMemoryProperties,
}

impl PhysicalDevice {
    pub(crate) fn new(
        instance: Arc<Instance>,
        vk_physical_device: ash::vk::PhysicalDevice,
    ) -> Result<Arc<Self>, ash::vk::Result> {
        let supported_extensions =
            // Host Synchronization: none
            unsafe { instance.ash_instance.enumerate_device_extension_properties(vk_physical_device)? }
                .iter()
                .filter_map(|ext_props| {
                    PhysicalDeviceExtensionType::from_cstr(
                        unsafe { CStr::from_ptr(ext_props.extension_name.as_ptr()) }
                    )
                }).collect();
        let memory_properties =
            Self::memory_properties_inner(&instance.ash_instance, vk_physical_device);
        Ok(Arc::new(Self {
            instance,
            vk_physical_device,
            supported_extensions,
            memory_properties,
        }))
    }

    pub fn get_physical_device_features(&self) -> FxHashSet<PhysicalDeviceFeatures> {
        unsafe {
            // Host Synchronization: none
            self.instance
                .ash_instance
                .get_physical_device_features(self.vk_physical_device)
                .collect_feature()
        }
    }

    pub fn get_physical_device_features2<T: SubPhysicalFeature>(
        &self,
    ) -> FxHashSet<<T::VkStruct as VkDeviceFeature>::SubFeatureEnumTy> {
        let mut t = T::VkStruct::default();
        let mut feature2 = ash::vk::PhysicalDeviceFeatures2::builder()
            .push_next(&mut t)
            .build();
        unsafe {
            // Host Synchronization: none
            self.instance
                .ash_instance
                .get_physical_device_features2(self.vk_physical_device, &mut feature2);
        }
        t.collect_feature()
    }

    pub fn enumerate_device_extension_properties(&self) -> &FxHashSet<PhysicalDeviceExtensionType> {
        &self.supported_extensions
    }
}
