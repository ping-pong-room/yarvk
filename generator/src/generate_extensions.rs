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
    pub fn get_all_dependencies<'a>(&'a self, dependency_info: &mut DependencyInfo<'a>) {
        for dep_ext_info in &self.dependency_info {
            if dep_ext_info.promoted {
                continue;
            }
            if self.extension_type == ExtensionType::Device
                && dep_ext_info.extension_type == ExtensionType::Instance
            {
                dependency_info
                    .top_level_instance
                    .insert(&dep_ext_info.name);
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
        .unwrap_or(name)
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

    let mut instance_extension_struct_definations = Vec::new();
    let mut device_extension_struct_definations = Vec::new();

    for (vk_name, ext_info) in &extension_infos.0 {
        if ext_info.promoted {
            continue;
        }
        let cstr_name = format!("{vk_name}\0");
        let cstr_ident = LitByteStr::new(cstr_name.as_bytes(), Span::call_site());
        match ext_info.extension_type {
            ExtensionType::Instance => {
                let struct_name = format_ident!("Extension{}", &ext_info.name);
                let mut dependencies_info = DependencyInfo::default();
                ext_info.get_all_dependencies(&mut dependencies_info);
                let DependencyInfo {
                    dependencies,
                    top_level_instance,
                } = dependencies_info;
                let dependencies: Vec<Ident> = dependencies
                    .into_iter()
                    .map(|ident| format_ident!("Extension{}", &ident))
                    .collect();
                let _instance_dependencies: Vec<&Ident> = top_level_instance.into_iter().collect();
                instance_extension_struct_definations.push(
                    quote!(
                        #[derive(Clone)]
                        pub struct #struct_name {
                            pub instance: std::sync::Arc<crate::instance::Instance>,
                        }
                        impl Extension for #struct_name {
                            const NAME: &'static std::ffi::CStr =  unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(#cstr_ident) };
                            const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
                                #(#dependencies::NAME,)*
                            ];
                        }
                        impl InstanceExtension for #struct_name {
                            fn new(instance: &std::sync::Arc<crate::instance::Instance>) -> Self {
                                Self {
                                    instance: instance.clone(),
                                }
                            }
                        }
                    )
                );
            }
            ExtensionType::Device => {
                let struct_name = format_ident!("Extension{}", &ext_info.name);
                let mut dependencies_info = DependencyInfo::default();
                ext_info.get_all_dependencies(&mut dependencies_info);
                let DependencyInfo {
                    dependencies,
                    top_level_instance,
                } = dependencies_info;
                let dependencies: Vec<Ident> = dependencies
                    .into_iter()
                    .map(|ident| format_ident!("Extension{}", &ident))
                    .collect();
                let instance_dependencies: Vec<Ident> = top_level_instance
                    .into_iter()
                    .map(|ident| format_ident!("Extension{}", &ident))
                    .collect();
                device_extension_struct_definations.push(
                    quote!(
                        #[derive(Clone)]
                        pub struct #struct_name {
                            pub device: std::sync::Arc<crate::device::Device>,
                        }
                        impl Extension for #struct_name {
                            const NAME: &'static std::ffi::CStr =  unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(#cstr_ident) };
                            const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
                                #(#dependencies::NAME,)*
                            ];
                        }
                        impl DeviceExtension for #struct_name {
                            type InstanceDependenciesTy = (#(#instance_dependencies,)*);
                            const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[
                                #(#instance_dependencies::NAME,)*
                            ];
                            fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
                                Self {
                                    device: device.clone(),
                                }
                            }
                        }
                    )
                );
            }
        }
    }
    let res = quote! {
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

        #(#instance_extension_struct_definations)*
        #(#device_extension_struct_definations)*
    };
    (res, extension_infos)
}
