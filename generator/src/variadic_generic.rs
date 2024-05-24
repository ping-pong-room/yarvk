use quote::__private::TokenStream;
use quote::{format_ident, quote};

pub fn descriptor_set_variadic_generics(n: usize) -> (TokenStream, TokenStream) {
    let mut public_generics = Vec::with_capacity(n);
    let mut private_generics = Vec::with_capacity(n);
    for n in 0..n {
        let mut generic_parameters = Vec::with_capacity(n);
        let mut generic_arguments = Vec::with_capacity(n);
        let mut descriptor_set_value_fields = Vec::with_capacity(n);
        let mut const_descriptor_set_value_fields = Vec::with_capacity(n);
        let mut vk_layout_create_info_bindings = Vec::with_capacity(n);
        let mut update_info_collection_fields = Vec::with_capacity(n);
        let mut collection_init_fields = Vec::with_capacity(n);
        let mut diff_stats = Vec::with_capacity(n);
        let mut push_to_init_stats = Vec::with_capacity(n);
        let mut put_pool_size_stats = Vec::with_capacity(n);
        for i in 0..=n {
            let arg_binding_i = format_ident!("BINDING_{}", i);
            let arg_descriptor_kind_i = format_ident!("DESCRIPTOR_KIND_{}", i);
            let arg_descriptor_counts_i = format_ident!("DESCRIPTOR_COUNTS_{}", i);
            generic_arguments
                .push(quote!(#arg_binding_i, #arg_descriptor_kind_i, #arg_descriptor_counts_i));

            let para_binding_i = quote!(const #arg_binding_i: u32);
            let para_descriptor_kind_i = quote!(const #arg_descriptor_kind_i: DescriptorKind);
            let para_descriptor_counts_i = quote!(const #arg_descriptor_counts_i: usize);
            generic_parameters
                .push(quote!(#para_binding_i,#para_descriptor_kind_i,#para_descriptor_counts_i));
            let field_name_t_i = format_ident!("t{}", i);
            let descriptor_set_value_field = quote!(pub #field_name_t_i: [<DescriptorTypeDetail<{ #arg_descriptor_kind_i }> as DescriptorType>::ValueType;#arg_descriptor_counts_i]);
            descriptor_set_value_fields.push(quote!(#descriptor_set_value_field));
            let const_descriptor_set_value_field = quote!(pub #field_name_t_i: ([<DescriptorTypeDetail<{ #arg_descriptor_kind_i }> as DescriptorType>::ConstValueType;#arg_descriptor_counts_i],ShaderStage));
            const_descriptor_set_value_fields.push(quote!(#const_descriptor_set_value_field));
            let immutable_samplers_ident = format_ident!("immutable_samplers_{}", i);
            let builder_ident = format_ident!("builder_{}", i);
            let update_info_collection_field = quote!(pub(crate) #field_name_t_i: [<<DescriptorTypeDetail<{ #arg_descriptor_kind_i }> as DescriptorType>::ValueType as FnVkUpdateInfo>::VkValueType;#arg_descriptor_counts_i]);
            vk_layout_create_info_bindings.push(quote!(
                let #immutable_samplers_ident = self.#field_name_t_i.0.try_get_immutable_samplers();
                        let mut #builder_ident = ash::vk::DescriptorSetLayoutBinding::builder()
                            .binding(#arg_binding_i)
                            .descriptor_type(#arg_descriptor_kind_i.to_ash())
                            .descriptor_count(#arg_descriptor_counts_i as _)
                            .stage_flags(self.#field_name_t_i.1.to_ash());
                        if !#immutable_samplers_ident.is_empty() {
                            #builder_ident = #builder_ident.immutable_samplers(#immutable_samplers_ident.as_slice());
                        };
                        ash_vk_bindings.push(#builder_ident.build());
            ));
            update_info_collection_fields.push(quote!(#update_info_collection_field,));
            collection_init_fields.push (quote!(#field_name_t_i: std::array::from_fn(|i| self.#field_name_t_i[i].to_vk_update_ty())));
            diff_stats.push(quote!(
                diff(&self.#field_name_t_i, &new_value.#field_name_t_i, |dst_array_element, counts| {
                        let slice = &vk_value.#field_name_t_i[dst_array_element..counts];
                        let builder = ash::vk::WriteDescriptorSet::builder()
                            .dst_set(ash_vk_descriptor_set)
                            .dst_binding(#arg_binding_i)
                            .dst_array_element(dst_array_element as _)
                            .descriptor_type(#arg_descriptor_kind_i.to_ash())
                            .update_info(slice);
                        vec.fix_push(builder);
                    });
            ));
            push_to_init_stats.push(quote!(
                {
                    let slice = &vk_value.#field_name_t_i;
                    let builder = ash::vk::WriteDescriptorSet::builder()
                        .dst_set(ash_vk_descriptor_set)
                        .dst_binding(#arg_binding_i)
                        .dst_array_element(0)
                        .descriptor_type(#arg_descriptor_kind_i.to_ash())
                        .update_info(slice);
                    vec.fix_push(builder);
                }
            ));
            put_pool_size_stats.push(quote!(
                vec.push(ash::vk::DescriptorPoolSize {
                    ty: #arg_descriptor_kind_i.to_ash(),
                    descriptor_count: #arg_descriptor_counts_i as u32,
                });
            ))
        }
        let descriptor_set_value_ident = format_ident!("DescriptorSetValue{}", n + 1);
        let const_descriptor_set_value_ident = format_ident!("ConstDescriptorSetValue{}", n + 1);
        let vk_update_info_collection_ident = format_ident!("VkUpdateInfoCollection{}", n + 1);
        public_generics.push(quote!(
            #[derive(Clone)]
            pub struct #descriptor_set_value_ident<
                 #(#generic_parameters,)*
            > {
                #(#descriptor_set_value_fields,)*
            }
            impl<
                #(#generic_parameters,)*
            > DescriptorSetValue
            for #descriptor_set_value_ident<#(#generic_arguments,)*>
            {
                type ConstDescriptorSetValue =
                #const_descriptor_set_value_ident<#(#generic_arguments,)*>;
            }

            pub struct #const_descriptor_set_value_ident<
                #(#generic_parameters,)*
            > {
                #(#const_descriptor_set_value_fields,)*
            }
        ));
        private_generics.push(quote!(
            use crate::descriptor_set::descriptor_variadic_generics::{#const_descriptor_set_value_ident, #descriptor_set_value_ident};
            impl<
                #(#generic_parameters,)*
            > PrivateConstDescriptorSetValue<
                #descriptor_set_value_ident<
                    #(#generic_arguments,)*
                >
            > for #const_descriptor_set_value_ident<
                #(#generic_arguments,)*
            > {
                fn new_descriptor_layout(
                        self,
                        device: &Arc<Device>,
                        flags: ash::vk::DescriptorSetLayoutCreateFlags,
                    ) -> Result<
                        Arc<
                            DescriptorSetLayout<
                                #descriptor_set_value_ident<#(#generic_arguments,)*>,
                            >,
                        >,
                        ash::vk::Result,
                    > {
                        let mut ash_vk_bindings = Vec::with_capacity(#n + 1);
                        #(#vk_layout_create_info_bindings)*
                        let create_info = ash::vk::DescriptorSetLayoutCreateInfo::builder()
                            .flags(flags)
                            .bindings(ash_vk_bindings.as_slice())
                            .build();
                        unsafe {
                            // Host Synchronization: none
                            let ash_vk_descriptor_set_layout = device
                                .ash_device
                                .create_descriptor_set_layout(&create_info, None)?;
                            Ok(Arc::new(DescriptorSetLayout {
                                device: device.clone(),
                                ash_vk_descriptor_set_layout,
                                _phantom_data: Default::default(),
                            }))
                        }
                    }
            }
            pub struct #vk_update_info_collection_ident<
                 #(#generic_parameters,)*
            > {
                #(#update_info_collection_fields)*
            }

            impl<
                #(#generic_parameters,)*
            > PrivateDescriptorSetValue
            for #descriptor_set_value_ident <
                #(#generic_arguments,)*
            >
            {
                type VkType = #vk_update_info_collection_ident<
                    #(#generic_arguments,)*
                >;

                fn to_vk_ty(&self) -> Self::VkType {
                    #vk_update_info_collection_ident {
                        #(#collection_init_fields,)*
                    }
                }

                fn push_to_update(
                    &self,
                    new_value: &Self,
                    vk_value: &mut Self::VkType,
                    ash_vk_descriptor_set: ash::vk::DescriptorSet,
                    vec: &mut ParallelSplitWriteDescriptorSets,
                ) {
                    #(#diff_stats)*
                    vec.focus_next();
                }

                fn push_to_init(
                    vk_value: &mut Self::VkType,
                    ash_vk_descriptor_set: DescriptorSet,
                    vec: &mut ParallelSplitWriteDescriptorSets,
                ) {
                    #(#push_to_init_stats)*
                }

                fn put_pool_size(vec: &mut Vec<ash::vk::DescriptorPoolSize>) {
                    #(#put_pool_size_stats)*
                }
            }
        ));
    }
    (
        quote!(
            use crate::descriptor_set::descriptor_set::DescriptorSetValue;
            use crate::descriptor_set::descriptor_type::{DescriptorKind, DescriptorType, DescriptorTypeDetail};
            use crate::pipeline::shader_stage::ShaderStage;
            #(#public_generics)*
        ),
        quote!(
            use crate::descriptor_set::descriptor_set_layout::DescriptorSetLayout;
            use crate::descriptor_set::descriptor_type::{
                DescriptorKind, DescriptorType, DescriptorTypeDetail,
            };
            use crate::descriptor_set::diff;
            use crate::descriptor_set::private::{
                DescriptorBuilderUpdate, FnVkUpdateInfo, ParallelSplitWriteDescriptorSets,
                PrivateConstDescriptorSetValue, PrivateDescriptorSetValue, TryGetImmutableSamplers,
            };
            use crate::device::Device;
            use ash::vk::DescriptorSet;
            use std::sync::Arc;
            #(#private_generics)*
        ),
    )
}
