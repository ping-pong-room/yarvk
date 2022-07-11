use crate::extensions::PhysicalInstanceExtensionType;
use std::ffi::CStr;
use std::sync::Arc;

pub struct Entry {
    pub(crate) ash_entry: ash::Entry,
}

impl Entry {
    pub fn load() -> Result<Arc<Self>, ash::LoadingError> {
        Ok(Arc::new(Self {
            ash_entry: unsafe { ash::Entry::load()? },
        }))
    }
    pub fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&CStr>,
    ) -> Result<Vec<PhysicalInstanceExtensionType>, ash::vk::Result> {
        Ok(self
            .ash_entry
            .enumerate_instance_extension_properties(layer_name)?
            .iter()
            .filter_map(|ext_props| {
                PhysicalInstanceExtensionType::from_cstr(unsafe {
                    CStr::from_ptr(ext_props.extension_name.as_ptr())
                })
            })
            .collect())
    }
}
