use crate::extensions::InstanceExtension;
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
    pub fn supports_extension<EXT: InstanceExtension>(&self, layer_name: Option<&CStr>) -> bool {
        self.ash_entry
            .enumerate_instance_extension_properties(layer_name)
            .unwrap()
            .iter()
            .find(|ext_props| unsafe {
                CStr::from_ptr(ext_props.extension_name.as_ptr()) == EXT::NAME
            })
            .is_some()
    }
}
