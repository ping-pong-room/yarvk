use heck::ToUpperCamelCase;
use quote::__private::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::{Deref, DerefMut};
use syn::__private::Span;
use syn::{Ident, LitByteStr};
use vk_parse::{ExtensionChild, InterfaceItem, Registry};

#[derive(PartialEq, Eq)]
pub enum ExtensionType {
    Instance,
    Device,
}

pub struct ExtensionInfo<'a> {
    pub name: Ident,
    pub extension_type: ExtensionType,
    pub dependencies: Vec<&'a str>,
    pub promoted: bool,
    pub required_structs: Vec<&'a str>,
}

#[derive(Default)]
struct ExtensionMap<'a>(FxHashMap<String, ExtensionInfo<'a>>);

impl<'a> Deref for ExtensionMap<'a> {
    type Target = FxHashMap<String, ExtensionInfo<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for ExtensionMap<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> ExtensionMap<'a> {
    fn get_all_dependencies(
        &self,
        vk_name: &str,
    ) -> (
        FxHashSet<&Ident>, /*instance_dep*/
        FxHashSet<&Ident>, /*device_dep*/
    ) {
        let mut instance_dep = FxHashSet::default();
        let mut device_dep = FxHashSet::default();
        let ext_info = self.get(vk_name).unwrap();
        for dep in &ext_info.dependencies {
            let dep_info = self.get(*dep).unwrap();
            if dep_info.promoted {
                continue;
            }
            match dep_info.extension_type {
                ExtensionType::Instance => {
                    instance_dep.insert(&dep_info.name);
                }
                ExtensionType::Device => {
                    device_dep.insert(&dep_info.name);
                }
            }

            let (sub_instance_dep, sub_device_dep) = self.get_all_dependencies(dep);
            instance_dep.extend(sub_instance_dep);
            device_dep.extend(sub_device_dep);
        }
        (instance_dep, device_dep)
    }
}

fn to_yarvk_extension_name(name: &str) -> String {
    name.strip_prefix("VK_")
        .unwrap_or(&name)
        .to_upper_camel_case()
}

pub fn generate_extensions(spec2: &Registry) -> (TokenStream, FxHashMap<String, ExtensionInfo>) {
    let mut extension_infos = ExtensionMap::default();
    let extensions: &Vec<vk_parse::Extension> = spec2
        .0
        .iter()
        .filter_map(|item| match item {
            vk_parse::RegistryChild::Extensions(ref ext) => Some(&ext.children),
            _ => None,
        })
        .next()
        .expect("extension");
    for extension in extensions {
        if let Some(supported) = &extension.supported {
            if supported.as_str() == "disabled" {
                continue;
            }
        }
        let mut required_structs = Vec::new();
        for child in &extension.children {
            if let ExtensionChild::Require { items, .. } = child {
                for item in items {
                    if let InterfaceItem::Type { name, .. } = item {
                        required_structs.push(name.as_str());
                    }
                }
            }
        }
        let vk_name = extension.name.as_str();
        let extension_type = match extension.ext_type.as_ref().expect(vk_name).as_str() {
            "instance" => ExtensionType::Instance,
            "device" => ExtensionType::Device,
            _ => panic!("unknown extension type"),
        };
        let mut dependencies = Vec::new();
        let re = Regex::new(r"([A-Za-z0-9_]+)").unwrap();
        if let Some(requires) = &extension.requires {
            for cap in re.captures_iter(requires.as_str()) {
                let vk_name = cap.get(1).unwrap().as_str();
                dependencies.push(vk_name);
            }
        }
        let promoted = extension.promotedto.is_some();
        extension_infos.insert(
            vk_name.to_string(),
            ExtensionInfo {
                name: format_ident!("{}", to_yarvk_extension_name(vk_name)),
                extension_type,
                dependencies,
                promoted,
                required_structs,
            },
        );
    }
    let mut instance_extension_enum_variants = Vec::<&Ident>::new();
    let mut instance_extension_cstr_names = Vec::<LitByteStr>::new();
    let mut instance_extension_dependencies = Vec::<Vec<&Ident>>::new();

    let mut device_extension_enum_variants = Vec::<&Ident>::new();
    let mut device_extension_cstr_names = Vec::<LitByteStr>::new();
    let mut device_extension_dependencies = Vec::<Vec<&Ident>>::new();
    let mut device_extension_instance_dependencies = Vec::<TokenStream>::new();
    let mut device_extension_match_pattern = Vec::<TokenStream>::new();

    for (vk_name, ext_info) in extension_infos.iter() {
        if ext_info.promoted {
            continue;
        }
        let cstr_name = format!("{}\0", vk_name);
        let cstr_ident = LitByteStr::new(cstr_name.as_bytes(), Span::call_site());
        match ext_info.extension_type {
            ExtensionType::Instance => {
                instance_extension_enum_variants.push(&ext_info.name);
                instance_extension_cstr_names.push(cstr_ident);
                instance_extension_dependencies.push(
                    extension_infos
                        .get_all_dependencies(vk_name)
                        .0
                        .into_iter()
                        .collect(),
                );
            }
            ExtensionType::Device => {
                device_extension_enum_variants.push(&ext_info.name);
                device_extension_cstr_names.push(cstr_ident);
                device_extension_dependencies.push(
                    extension_infos
                        .get_all_dependencies(vk_name)
                        .1
                        .into_iter()
                        .collect(),
                );
                let tuples = extension_infos
                    .get_all_dependencies(vk_name)
                    .0
                    .into_iter()
                    .map(
                        |ident| quote!(InstanceExtension<{PhysicalInstanceExtensionType::#ident}>,),
                    )
                    .collect::<Vec<_>>();
                let tuples_tokens = if tuples.is_empty() {
                    quote!()
                } else {
                    quote!((#(#tuples)*))
                };
                device_extension_instance_dependencies.push(tuples_tokens);
                let pattern = if tuples.is_empty() {
                    quote!()
                } else {
                    quote!((..))
                };
                device_extension_match_pattern.push(pattern);
            }
        }
    }
    let res = quote! {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum PhysicalInstanceExtensionType {
            #(#instance_extension_enum_variants,)*
        }
        impl PhysicalInstanceExtensionType {
            pub fn to_cstr(&self) -> &'static std::ffi::CStr {
                match self {
                    #(Self::#instance_extension_enum_variants => unsafe {std::ffi::CStr::from_bytes_with_nul_unchecked(#instance_extension_cstr_names)})*
                }
            }
            pub fn get_dependencies(&self) -> &[Self] {
                match self {
                    #(Self::#instance_extension_enum_variants => &[#(Self::#instance_extension_dependencies,)*],)*
                }
            }
            pub fn from_cstr(vk_name: &'static std::ffi::CStr) -> Option<Self> {
                let bytes = vk_name.to_bytes_with_nul();
                match bytes {
                    #(#instance_extension_cstr_names => {Some(Self::#instance_extension_enum_variants)})*
                    _ => {None}
                }
            }
        }
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum PhysicalDeviceExtensionType {
            #(#device_extension_enum_variants,)*
        }
        impl PhysicalDeviceExtensionType {
            pub fn to_cstr(&self) -> &'static std::ffi::CStr {
                match self {
                    #(Self::#device_extension_enum_variants => unsafe {std::ffi::CStr::from_bytes_with_nul_unchecked(#device_extension_cstr_names)})*
                }
            }
            pub fn get_dependencies(&self) -> &[Self] {
                match self {
                    #(Self::#device_extension_enum_variants => &[#(Self::#device_extension_dependencies,)*],)*
                }
            }
            pub fn from_cstr(vk_name: &'static std::ffi::CStr) -> Option<Self> {
                let bytes = vk_name.to_bytes_with_nul();
                match bytes {
                    #(#device_extension_cstr_names => {Some(Self::#device_extension_enum_variants)})*
                    _ => {None}
                }
            }
        }
        pub enum DeviceExtensionType {
            #(#device_extension_enum_variants #device_extension_instance_dependencies,)*
        }
        impl From<&DeviceExtensionType> for PhysicalDeviceExtensionType {
            fn from(device_ext_type: &DeviceExtensionType) -> PhysicalDeviceExtensionType {
                match device_ext_type {
                    #(DeviceExtensionType::#device_extension_enum_variants #device_extension_match_pattern => Self::#device_extension_enum_variants,)*
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
    };
    (res, extension_infos.0)
}
