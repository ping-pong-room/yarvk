use crate::debug_utils_messenger::DebugUtilsMessengerCreateInfoEXT;
use crate::entry::Entry;
use crate::extensions::{
    Extension, ExtensionExtDebugUtils, ExtensionKhrGetSurfaceCapabilities2,
    ExtensionKhrPortabilityEnumeration, InstanceExtension,
};
use crate::physical_device::PhysicalDevice;
use ash::vk::ExtensionProperties;
use rustc_hash::FxHashSet;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Arc;

pub struct ApplicationInfo {
    application_name: CString,
    application_version: u32,
    engine_name: CString,
    engine_version: u32,
}

impl ApplicationInfo {
    pub(crate) fn ash_builder(&self) -> ash::vk::ApplicationInfoBuilder {
        ash::vk::ApplicationInfo::builder()
            .application_name(self.application_name.as_c_str())
            .application_version(self.application_version)
            .engine_name(self.engine_name.as_c_str())
            .engine_version(self.engine_version)
            // DONE VUID-VkApplicationInfo-apiVersion-04010
            .api_version(ash::vk::API_VERSION_1_3)
    }
}

impl ApplicationInfo {
    pub fn builder() -> ApplicationInfoBuilder {
        ApplicationInfoBuilder::default()
    }
}

pub struct ApplicationInfoBuilder {
    inner: ApplicationInfo,
}

impl Default for ApplicationInfoBuilder {
    fn default() -> Self {
        Self {
            inner: ApplicationInfo {
                application_name: unsafe { CString::new("yarvk").unwrap_unchecked() },
                application_version: 0,
                engine_name: unsafe { CString::new("yarvk_engine").unwrap_unchecked() },
                engine_version: 0,
            },
        }
    }
}

impl ApplicationInfoBuilder {
    fn remove_zero_byte<T: Into<String>>(t: T) -> String {
        let mut string = t.into();
        string.retain(|char| char != '\0');
        string
    }
    pub fn app_name<S: Into<String>>(mut self, app_name: S) -> Self {
        self.inner.application_name =
            unsafe { CString::new(Self::remove_zero_byte(app_name)).unwrap_unchecked() };
        self
    }
    pub fn app_version(mut self, app_version: u32) -> Self {
        self.inner.application_version = app_version;
        self
    }
    pub fn engine_name<S: Into<String>>(mut self, engine_name: S) -> Self {
        self.inner.engine_name =
            unsafe { CString::new(Self::remove_zero_byte(engine_name)).unwrap_unchecked() };
        self
    }
    pub fn engine_version(mut self, engine_version: u32) -> Self {
        self.inner.engine_version = engine_version;
        self
    }

    pub fn build(self) -> ApplicationInfo {
        self.inner
    }
}

pub struct InstanceBuilder {
    entry: Arc<Entry>,
    flags: ash::vk::InstanceCreateFlags,
    application_info: ApplicationInfo,
    enabled_layers: FxHashSet<&'static CStr>,
    enabled_extensions: FxHashSet<&'static CStr>,
    debug_utils_messenger_create_info_exts: Vec<DebugUtilsMessengerCreateInfoEXT>,
}

impl InstanceBuilder {
    pub fn flags(mut self, flags: ash::vk::InstanceCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn application_info(mut self, application_info: ApplicationInfo) -> Self {
        self.application_info = application_info;
        self
    }
    pub fn add_layer(mut self, layer: &'static CStr) -> Self {
        self.enabled_layers.insert(layer);
        self
    }
    fn add_extension_inner<EXT: InstanceExtension>(&mut self) -> &mut Self {
        // SILENCE VUID-vkCreateInstance-ppEnabledExtensionNames-01388
        for dep in EXT::DEPENDENCIES {
            self.enabled_extensions.insert(*dep);
        }
        self.enabled_extensions.insert(EXT::NAME);
        self
    }
    pub fn add_extension<EXT: InstanceExtension>(mut self) -> Result<Self, ash::vk::Result> {
        if !self.entry.supports_extension::<EXT>(None) {
            return Err(ash::vk::Result::ERROR_EXTENSION_NOT_PRESENT);
        }
        self.add_extension_inner::<EXT>();
        Ok(self)
    }
    pub fn debug_utils_messenger_exts(
        mut self,
        exts: Vec<DebugUtilsMessengerCreateInfoEXT>,
    ) -> Self {
        self.debug_utils_messenger_create_info_exts = exts;
        self
    }
    pub fn build(mut self) -> Result<Arc<Instance>, ash::vk::Result> {
        // SILENCE EXTENSION: VK_KHR_get_surface_capabilities2 by default,
        // function vkGetPhysicalDeviceSurfaceCapabilities2KHR relies on it.
        if self
            .entry
            .supports_extension::<ExtensionKhrGetSurfaceCapabilities2>(None)
        {
            self.add_extension_inner::<ExtensionKhrGetSurfaceCapabilities2>();
        }

        // TODO SILENCE VUID-VkInstanceCreateInfo-pNext-04925
        // if self.debug_report_callback_create_info_ext.is_some() {
        //     if !self.enabled_extensions.contains(&Extension::ExtDebugReport) {
        //         self.add_extension(&Extension::ExtDebugReport);
        //     }
        // }
        // SILENCE VUID-VkInstanceCreateInfo-pNext-04926
        if !self.debug_utils_messenger_create_info_exts.is_empty()
            && !self
                .enabled_extensions
                .contains(ExtensionExtDebugUtils::NAME)
        {
            self.add_extension_inner::<ExtensionExtDebugUtils>();
        }
        // SILENCE VUID-VkInstanceCreateInfo-flags-06559
        if self
            .flags
            .contains(ash::vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
        {
            self.add_extension_inner::<ExtensionKhrPortabilityEnumeration>();
        }

        let enabled_layer_names_raw: Vec<*const c_char> = self
            .enabled_layers
            .iter()
            .map(|layer| layer.as_ptr())
            .collect();
        let enabled_extension_names_raw: Vec<*const c_char> = self
            .enabled_extensions
            .iter()
            .map(|extension| extension.as_ptr())
            .collect();

        let ash_vk_application_info = self.application_info.ash_builder().build();
        let mut builder = ash::vk::InstanceCreateInfo::builder()
            .application_info(&ash_vk_application_info)
            .enabled_layer_names(enabled_layer_names_raw.as_slice())
            .enabled_extension_names(enabled_extension_names_raw.as_slice());
        let mut ash_vk_debug_utils = self
            .debug_utils_messenger_create_info_exts
            .iter()
            .map(|ext| ext.ash_builder().build())
            .collect::<Vec<_>>();
        for debug_utils in &mut ash_vk_debug_utils {
            builder = builder.push_next(debug_utils);
        }
        let create_info = builder.build();
        // Host Synchronization: none
        let ash_instance = unsafe { self.entry.ash_entry.create_instance(&create_info, None)? };
        Ok(Arc::new(Instance {
            entry: self.entry,
            ash_instance,
            _debug_utils_messenger_create_info_exts: self.debug_utils_messenger_create_info_exts,
            // enabled_layers: self.enabled_layers,
            enabled_extensions: self.enabled_extensions,
        }))
    }
}

pub struct Instance {
    pub(crate) entry: Arc<Entry>,
    pub(crate) ash_instance: ash::Instance,
    _debug_utils_messenger_create_info_exts: Vec<DebugUtilsMessengerCreateInfoEXT>,
    // pub(crate) enabled_layers: FxHashSet<&'static CStr>,
    pub(crate) enabled_extensions: FxHashSet<&'static CStr>,
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.ash_instance.handle() == other.ash_instance.handle()
    }
}

impl Eq for Instance {}

impl Instance {
    pub fn builder(entry: Arc<Entry>) -> InstanceBuilder {
        InstanceBuilder {
            entry,
            flags: Default::default(),
            application_info: ApplicationInfoBuilder::default().build(),
            enabled_layers: Default::default(),
            enabled_extensions: Default::default(),
            debug_utils_messenger_create_info_exts: vec![],
        }
    }
    pub fn enumerate_physical_devices(
        self: &Arc<Self>,
    ) -> Result<Vec<Arc<PhysicalDevice>>, ash::vk::Result> {
        // Host Synchronization: none
        let devices = unsafe { self.ash_instance.enumerate_physical_devices()? };
        let mut arc_devices: Vec<Arc<PhysicalDevice>> = Vec::new();
        for pdevice in devices {
            arc_devices.push(PhysicalDevice::new(self.clone(), pdevice)?);
        }
        Ok(arc_devices)
    }
    pub fn get_extension<EXT: InstanceExtension>(self: &Arc<Self>) -> Option<EXT> {
        if self.enabled_extensions.contains(EXT::NAME) {
            Some(EXT::new(self))
        } else {
            None
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            // Done VUID-vkDestroyInstance-instance-00629
            // TODO VUID-vkDestroyInstance-instance-00630
            // TODO VUID-vkDestroyInstance-instance-00631
            // Host Synchronization: instance, PhysicalDevice objects
            self.ash_instance.destroy_instance(None);
        }
    }
}
