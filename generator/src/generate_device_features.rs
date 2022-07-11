use crate::generate_extensions::{ExtensionInfo, ExtensionType};
use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use quote::__private::TokenStream;
use quote::{format_ident, quote};
use rustc_hash::FxHashMap;
use syn::Ident;
use vk_parse::TypeMember::Definition;
use vk_parse::TypeSpec::Members;
use vk_parse::{Registry, Type, TypeMemberMarkup, TypesChild};

struct FeatureDetail<'a> {
    struct_type: Option<&'a str>,
    feature_members: Vec<&'a str>,
    required_extension: Option<&'a ExtensionInfo<'a>>,
}

fn required_extension<'a>(
    extensions: &'a FxHashMap<String, ExtensionInfo<'a>>,
    ty: &Type,
) -> Option<&'a ExtensionInfo<'a>> {
    for (vk_name, ext_info) in extensions {
        if let Some(target_name) = &ty.name {
            if !ext_info.promoted && vk_name == target_name {
                return Some(&ext_info);
            }
        }
    }
    None
}

fn get_enum_item_tuple(extension: Option<&ExtensionInfo>) -> TokenStream {
    if let Some(extension) = extension {
        let ident = &extension.name;
        return match &extension.extension_type {
            ExtensionType::Instance => {
                quote!((crate::extensions::InstanceExtension<{ crate::extensions::PhysicalInstanceExtensionType::#ident}>))
            }
            ExtensionType::Device => {
                quote!((crate::extensions::DeviceExtension<{ crate::extensions::PhysicalDeviceExtensionType::#ident}>))
            }
        };
    }
    return quote!();
}

fn handle_feature_member<'a>(
    ty: &'a Type,
    extensions: &'a FxHashMap<String, ExtensionInfo<'a>>,
) -> FeatureDetail<'a> {
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

pub fn generate_device_features(
    spec2: &Registry,
    extensions: &FxHashMap<String, ExtensionInfo>,
) -> TokenStream {
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
                                        handle_feature_member(&ty, extensions),
                                    );
                                } else if let Some(extends) = &ty.structextends {
                                    if extends.as_str().contains("VkPhysicalDeviceFeatures2") {
                                        total_features.insert(
                                            name.as_str(),
                                            handle_feature_member(&ty, extensions),
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

    let mut sub_feature_enum_definitions = Vec::with_capacity(total_features.len());
    let mut sub_feature_enum_names = Vec::with_capacity(total_features.len());
    // This is also used as feature enum variants
    let mut sub_feature_logical_enum_names = Vec::with_capacity(total_features.len());
    let mut feature_name_idents_lower_case = Vec::with_capacity(total_features.len());
    let mut vk_struct_name_idents = Vec::with_capacity(total_features.len());
    // match arms for function feature registration
    let mut register_feature_match_arms = Vec::with_capacity(total_features.len());

    for (feature_name, feature_detail) in total_features {
        let ash_ty_name = format_ident!(
            "{}",
            feature_name.strip_prefix("Vk").unwrap_or(feature_name)
        );
        let feature_name_ident = format_ident!("{}", feature_name.strip_prefix("Vk").unwrap());
        let feature_name_ident_logical =
            format_ident!("{}", feature_name.strip_prefix("VkPhysical").unwrap());
        let feature_variants: Vec<Ident> = feature_detail
            .feature_members
            .iter()
            .map(|feature| format_ident!("{}", feature.to_upper_camel_case()))
            .collect();
        let feature_member_tuple = get_enum_item_tuple(feature_detail.required_extension);
        let pattern = match feature_detail.required_extension {
            None => {
                quote!()
            }
            Some(_) => {
                quote!((..))
            }
        };
        let rust_style_feature_idents: Vec<Ident> = feature_detail
            .feature_members
            .iter()
            .map(|feature| format_ident!("{}", feature.to_snake_case()))
            .collect();
        let feature_name_ident_lower_case = format_ident!("{}", feature_name.to_snake_case());
        let mut sub_feature_enum_definition = quote! {
            #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
            pub enum #feature_name_ident {
                #(#feature_variants,)*
            }
            impl const From<#feature_name_ident> for FeatureType {
                fn from(feature: #feature_name_ident) -> Self {
                    FeatureType::#feature_name_ident_logical(feature)
                }
            }
            #[derive(Clone, PartialEq, Eq)]
            pub enum #feature_name_ident_logical {
                #(#feature_variants #feature_member_tuple, )*
            }
            impl ToPhysicalFeature for #feature_name_ident_logical {
                type PhysicalDeviceFeatureTy = #feature_name_ident;
                fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy {
                    match self {
                        #(#feature_name_ident_logical::#feature_variants #pattern => #feature_name_ident::#feature_variants, )*
                    }
                }
            }
            impl From<#feature_name_ident_logical> for DeviceFeature {
                fn from(feature: #feature_name_ident_logical) -> Self {
                    DeviceFeature::#feature_name_ident_logical(feature)
                }
            }
            impl VkDeviceFeature for ash::vk::#ash_ty_name {
                type SubFeatureEnumTy = #feature_name_ident;
                fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy> {
                    let mut set = rustc_hash::FxHashSet::<Self::SubFeatureEnumTy>::default();
                    #(if self.#rust_style_feature_idents != 0 {
                        set.insert(#feature_name_ident::#feature_variants);
                    })*
                    set
                }
            }
        };
        if feature_name == "VkPhysicalDeviceFeatures" {
            sub_feature_enum_definition = quote!(
                #sub_feature_enum_definition
                impl #feature_name_ident {
                    fn register(&self, vk_struct: &mut ash::vk::PhysicalDeviceFeatures) {
                        match self {
                             #(Self::#feature_variants => vk_struct.#rust_style_feature_idents = 1,)*
                        }
                    }
                }
            );
            register_feature_match_arms.push(quote!(
                FeatureType::DeviceFeatures(feature) => {
                    feature.register(&mut feature2.features);
                }
            ));
        } else {
            sub_feature_enum_definition = quote!(
                #sub_feature_enum_definition
                impl SubPhysicalFeature for #feature_name_ident {
                    type VkStruct = ash::vk::#ash_ty_name;
                    fn register(&self, vk_struct: &mut Self::VkStruct) {
                        match self {
                             #(Self::#feature_variants => vk_struct.#rust_style_feature_idents = 1,)*
                        }
                    }
                }
            );
            let vk_structure_type = format_ident!(
                "{}",
                feature_detail
                    .struct_type
                    .unwrap()
                    .strip_prefix("VK_STRUCTURE_TYPE_")
                    .unwrap()
            );
            register_feature_match_arms.push(quote!(
                FeatureType::#feature_name_ident_logical(feature) => {
                    let u = map.entry(ash::vk::StructureType::#vk_structure_type).or_insert(
                        VkFeatureUnion {
                            #feature_name_ident_lower_case: ash::vk::#ash_ty_name ::default()
                        }
                    );
                    feature.register(unsafe { std::mem::transmute(u) });
                }
            ));
        }
        sub_feature_enum_definitions.push(sub_feature_enum_definition);
        vk_struct_name_idents.push(ash_ty_name);
        sub_feature_enum_names.push(feature_name_ident);
        sub_feature_logical_enum_names.push(feature_name_ident_logical);
        feature_name_idents_lower_case.push(feature_name_ident_lower_case);
    }

    let res = quote! {
        pub trait VkDeviceFeature {
            type SubFeatureEnumTy;
            fn collect_feature(&self) -> rustc_hash::FxHashSet<Self::SubFeatureEnumTy>;
        }
        pub trait SubPhysicalFeature {
            type VkStruct: Default + ash::vk::ExtendsPhysicalDeviceFeatures2 + VkDeviceFeature;
            fn register(&self, vk_struct: &mut Self::VkStruct);
        }
        pub trait ToPhysicalFeature {
            type PhysicalDeviceFeatureTy;
            fn to_physical(&self) -> Self::PhysicalDeviceFeatureTy;
        }
        #(#sub_feature_enum_definitions)*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum FeatureType {
            #(#sub_feature_logical_enum_names(#sub_feature_enum_names),)*
        }
        #[derive(Clone, PartialEq, Eq)]
        pub enum DeviceFeature {
            #(#sub_feature_logical_enum_names(#sub_feature_logical_enum_names),)*
        }
        #[allow(dead_code)]
        pub(crate) union VkFeatureUnion {
            #(pub(crate) #feature_name_idents_lower_case: ash::vk::#vk_struct_name_idents,)*
        }
        #[derive(Clone)]
        pub struct Feature<const FEATURE: FeatureType> {
            pub device: std::sync::Arc<crate::device::Device>,
            pub(crate) _p: std::marker::PhantomData<usize>,
        }
        pub(crate) fn register_features(features: &rustc_hash::FxHashSet<FeatureType>) -> ash::vk::PhysicalDeviceFeatures2 {
            struct VkStructHeader {
                pub _s_type: ash::vk::StructureType,
                pub p_next: *mut std::ffi::c_void,
            }
            let mut feature2 = ash::vk::PhysicalDeviceFeatures2::default();
            let mut map: rustc_hash::FxHashMap<ash::vk::StructureType, VkFeatureUnion> = rustc_hash::FxHashMap::default();
            for feature in features {
                match feature {
                    #(#register_feature_match_arms)*
                }
            };

            map.into_iter().for_each(|(_, mut feature_union)| {
                unsafe {
                    let header = &mut feature_union as *mut _ as *mut VkStructHeader;
                    (*header).p_next = feature2.p_next;
                    feature2.p_next = header as *mut _;
                }
            });
            feature2
        }
    };
    res
}
