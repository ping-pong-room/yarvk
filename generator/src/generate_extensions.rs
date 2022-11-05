use heck::ToUpperCamelCase;
use quote::__private::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::Arc;
use syn::__private::Span;
use syn::{Ident, LitByteStr};
use vk_parse::{ExtensionChild, InterfaceItem, Registry};

#[derive(PartialEq, Eq)]
pub enum ExtensionType {
    Instance,
    Device,
}

#[derive(Default)]
pub struct DependencyInfo<'a> {
    pub dependencies: FxHashSet<&'a Ident>,
    // only used for device extensions
    pub top_level_instance: FxHashSet<&'a Ident>,
}

pub struct ExtensionInfo {
    pub name: Ident,
    pub extension_type: ExtensionType,
    pub promoted: bool,
    pub required_structs: Vec<String>,
    pub dependency_info: Vec<Arc<ExtensionInfo>>,
}

impl Default for ExtensionInfo {
    fn default() -> Self {
        Self {
            name: format_ident!("empty"),
            extension_type: ExtensionType::Instance,
            promoted: false,
            required_structs: vec![],
            dependency_info: vec![],
        }
    }
}

impl ExtensionInfo {
    pub fn get_all_dependencies<'a>(
        &'a self,
        dependency_info: &mut DependencyInfo<'a>,
    ) {
        for dep_ext_info in &self.dependency_info {
            if dep_ext_info.promoted {
                continue;
            }
            if self.extension_type == ExtensionType::Device
                && dep_ext_info.extension_type == ExtensionType::Instance
            {
                dependency_info.top_level_instance.insert(&dep_ext_info.name);
                continue;
            } else {
                dependency_info.dependencies.insert(&dep_ext_info.name);
                dep_ext_info.get_all_dependencies(dependency_info);
            }
        }
    }
}

#[derive(Default)]
pub struct ExtensionMap(pub FxHashMap<String, Arc<ExtensionInfo>>);

impl ExtensionMap {
    fn get(&mut self, vk_name: &str) -> Arc<ExtensionInfo> {
        if let Some(extension_info) = self.0.get(vk_name) {
            return extension_info.clone();
        }
        self.0
            .insert(vk_name.to_string(), Arc::new(ExtensionInfo::default()));
        self.0.get(vk_name).unwrap().clone()
    }
}

fn to_yarvk_extension_name(name: &str) -> String {
    name.strip_prefix("VK_")
        .unwrap_or(&name)
        .to_upper_camel_case()
}

pub fn generate_extensions(spec2: &Registry) -> (TokenStream, ExtensionMap) {
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
    let re = Regex::new(r"([A-Za-z0-9_]+)").unwrap();
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
                        required_structs.push(name.clone());
                    }
                }
            }
        }
        let vk_name = extension.name.as_str();
        let mut extension_info = extension_infos.get(vk_name);
        let extension_info = unsafe { Arc::get_mut_unchecked(&mut extension_info) };
        extension_info.name = format_ident!("{}", to_yarvk_extension_name(vk_name));
        let extension_type = match extension.ext_type.as_ref().expect(vk_name).as_str() {
            "instance" => ExtensionType::Instance,
            "device" => ExtensionType::Device,
            _ => panic!("unknown extension type"),
        };
        extension_info.extension_type = extension_type;
        if let Some(requires) = &extension.requires {
            for cap in re.captures_iter(requires.as_str()) {
                let vk_name = cap.get(1).unwrap().as_str();
                let dep_info = extension_infos.get(vk_name);
                extension_info.dependency_info.push(dep_info);
            }
        }
        extension_info.promoted = extension.promotedto.is_some();
        extension_info.required_structs = required_structs;
    }
    let mut instance_extension_enum_variants = Vec::<&Ident>::new();
    let mut instance_extension_cstr_names = Vec::<LitByteStr>::new();
    let mut instance_extensions_dependencies = Vec::<Vec<&Ident>>::new();

    let mut device_extension_enum_variants = Vec::<&Ident>::new();
    let mut device_extension_cstr_names = Vec::<LitByteStr>::new();
    let mut device_extension_dependencies = Vec::<Vec<&Ident>>::new();
    let mut device_extension_instance_dependencies = Vec::<TokenStream>::new();
    let mut device_extension_match_pattern = Vec::<TokenStream>::new();

    for (vk_name, ext_info) in &extension_infos.0 {
        if ext_info.promoted {
            continue;
        }
        let cstr_name = format!("{}\0", vk_name);
        let cstr_ident = LitByteStr::new(cstr_name.as_bytes(), Span::call_site());
        match ext_info.extension_type {
            ExtensionType::Instance => {
                instance_extension_enum_variants.push(&ext_info.name);
                instance_extension_cstr_names.push(cstr_ident);
                let mut dependencies_info = DependencyInfo::default();
                ext_info.get_all_dependencies(&mut dependencies_info);
                instance_extensions_dependencies
                    .push(dependencies_info.dependencies.into_iter().collect());
            }
            ExtensionType::Device => {
                device_extension_enum_variants.push(&ext_info.name);
                device_extension_cstr_names.push(cstr_ident);
                let mut dependencies_info = DependencyInfo::default();
                ext_info.get_all_dependencies(&mut dependencies_info);
                device_extension_dependencies
                    .push(dependencies_info.dependencies.into_iter().collect());
                let tuples = dependencies_info
                    .top_level_instance
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
                    #(Self::#instance_extension_enum_variants => &[#(Self::#instance_extensions_dependencies,)*],)*
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
    (res, extension_infos)
}
