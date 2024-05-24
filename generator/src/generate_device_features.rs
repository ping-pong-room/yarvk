use crate::generate_extensions::{ExtensionInfo, ExtensionMap};
use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use quote::__private::TokenStream;
use quote::{format_ident, quote};
use rustc_hash::FxHashMap;
use vk_parse::TypeMember::Definition;
use vk_parse::TypeSpec::Members;
use vk_parse::{Registry, Type, TypeMemberMarkup, TypesChild};

struct FeatureDetail<'a> {
    struct_type: Option<&'a str>,
    feature_members: Vec<&'a str>,
    required_extension: Option<&'a ExtensionInfo>,
}

fn required_extension<'a>(extensions: &'a ExtensionMap, ty: &Type) -> Option<&'a ExtensionInfo> {
    for ext_info in extensions.0.values() {
        if let Some(target_name) = &ty.name {
            for required_struct in &ext_info.required_structs {
                if !ext_info.promoted && required_struct == target_name {
                    return Some(ext_info);
                }
            }
        }
    }
    None
}

fn handle_feature_member<'a>(ty: &'a Type, extensions: &'a ExtensionMap) -> FeatureDetail<'a> {
    let mut feature_members = Vec::new();
    let mut struct_type: Option<&str> = None;
    if let Members(members) = &ty.spec {
        feature_members.reserve(members.len());
        for member in members {
            if let Definition(definition) = member {
                for markup in &definition.markup {
                    match markup {
                        TypeMemberMarkup::Name(markup_name) => {
                            feature_members.push(markup_name.as_str());
                        }
                        TypeMemberMarkup::Type(markup_ty) => {
                            if markup_ty.as_str() == "VkBool32" {
                            } else if markup_ty.as_str() == "VkStructureType" {
                                if let Some(type_name) = &definition.values {
                                    struct_type = Some(type_name.as_str())
                                };
                                break;
                            } else {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    let required_extension = required_extension(extensions, ty);
    FeatureDetail {
        struct_type,
        feature_members,
        required_extension,
    }
}

pub fn generate_device_features(spec2: &Registry, extensions: &ExtensionMap) -> TokenStream {
    let mut total_features: FxHashMap<&str, FeatureDetail> = FxHashMap::default();

    spec2.0.iter().for_each(|item| {
        if let vk_parse::RegistryChild::Types(types) = item {
            types.children.iter().for_each(|item| {
                if let TypesChild::Type(ty) = item {
                    if let Some(category) = &ty.category {
                        if category.as_str() == "struct" {
                            if let Some(name) = &ty.name {
                                if name.as_str() == "VkPhysicalDeviceFeatures" {
                                    total_features.insert(
                                        name.as_str(),
                                        handle_feature_member(ty, extensions),
                                    );
                                } else if let Some(extends) = &ty.structextends {
                                    if extends.as_str().contains("VkPhysicalDeviceFeatures2") {
                                        total_features.insert(
                                            name.as_str(),
                                            handle_feature_member(ty, extensions),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    let mut feature_definitions = Vec::with_capacity(total_features.len());

    for (feature_name, feature_detail) in total_features {
        // this is used as mod name
        let feature_group_name = format_ident!(
            "{}",
            feature_name
                .strip_prefix("Vk")
                .unwrap_or(feature_name)
                .to_snake_case()
        );
        let vk_feature_name = format_ident!(
            "{}",
            feature_name.strip_prefix("Vk").unwrap_or(feature_name)
        );

        let required_extension = match feature_detail.required_extension {
            None => {
                quote!(crate::device_features::ExtensionNone)
            }
            Some(ext) => {
                let ext_name = format_ident!("Extension{}", &ext.name);
                quote!(crate::extensions::#ext_name)
            }
        };
        let is_feature2 = feature_detail.struct_type.is_some();
        let feature_struct_definitions: Vec<_> = feature_detail
            .feature_members
            .iter()
            .map(|feature| {
                let feature_name_ident = format_ident!("Feature{}", feature.to_upper_camel_case());
                let vk_field_ident = format_ident!("{}", feature.to_snake_case());
                quote! {
                    pub struct #feature_name_ident {
                       pub device: std::sync::Arc<crate::device::Device>,
                    }
                    impl crate::device_features::Feature for #feature_name_ident {
                        type VkFeatureType = ash::vk::#vk_feature_name;
                        type RequiredExtension = #required_extension;
                        const IS_FEATURE2: bool = #is_feature2;
                        fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool {
                            vk_feature.#vk_field_ident != 0
                        }
                        fn register_feature(vk_feature: &mut Self::VkFeatureType) {
                            vk_feature.#vk_field_ident = 1;
                        }
                        fn new(device: &std::sync::Arc<crate::device::Device>) -> Self {
                            Self {
                                device: device.clone(),
                            }
                        }
                    }
                }
            })
            .collect();

        feature_definitions.push(quote! {
            pub mod #feature_group_name {
              #(#feature_struct_definitions)*
            }
        });
    }

    let res = quote! {
        pub trait Feature {
            type VkFeatureType: Default;
            type RequiredExtension: crate::extensions::DeviceExtension;
            const IS_FEATURE2: bool;
            fn is_enabled(vk_feature: &Self::VkFeatureType) -> bool;
            fn register_feature(vk_feature: &mut Self::VkFeatureType);
            fn new(device: &std::sync::Arc<crate::device::Device>) -> Self;
        }

        #[derive(Clone)]
        pub struct ExtensionNone {}
        impl crate::extensions::Extension for ExtensionNone {
            const NAME: &'static std::ffi::CStr = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"\0") };
            const DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
        }
        impl crate::extensions::DeviceExtension for ExtensionNone {
            type InstanceDependenciesTy = ();
            const INSTANCE_DEPENDENCIES: &'static [&'static std::ffi::CStr] = &[];
            fn new(_device: &std::sync::Arc<crate::device::Device>) -> Self {
                Self {}
            }
        }

        #(#feature_definitions)*
    };
    res
}
