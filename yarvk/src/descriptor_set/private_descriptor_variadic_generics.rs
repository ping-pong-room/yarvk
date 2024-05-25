use crate::descriptor_set::descriptor_set_layout::DescriptorSetLayout;
use crate::descriptor_set::descriptor_type::{
    DescriptorKind, DescriptorType, DescriptorTypeDetail,
};
use crate::descriptor_set::descriptor_variadic_generics::{
    ConstDescriptorSetValue1, DescriptorSetValue1,
};
use crate::descriptor_set::diff;
use crate::descriptor_set::private::{
    DescriptorBuilderUpdate, FnVkUpdateInfo, ParallelSplitWriteDescriptorSets,
    PrivateConstDescriptorSetValue, PrivateDescriptorSetValue, TryGetImmutableSamplers,
};
use crate::device::Device;
use ash::vk::DescriptorSet;
use std::sync::Arc;
impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
    >
    PrivateConstDescriptorSetValue<
        DescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>,
    > for ConstDescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>
{
    fn new_descriptor_layout(
        self,
        device: &Arc<Device>,
        flags: ash::vk::DescriptorSetLayoutCreateFlags,
    ) -> Result<
        Arc<
            DescriptorSetLayout<
                DescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>,
            >,
        >,
        ash::vk::Result,
    > {
        let mut ash_vk_bindings = Vec::with_capacity(0usize + 1);
        let immutable_samplers_0 = self.t0.0.try_get_immutable_samplers();
        let mut builder_0 = ash::vk::DescriptorSetLayoutBinding::builder()
            .binding(BINDING_0)
            .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
            .descriptor_count(DESCRIPTOR_COUNTS_0 as _)
            .stage_flags(self.t0.1.to_ash());
        if !immutable_samplers_0.is_empty() {
            builder_0 = builder_0.immutable_samplers(immutable_samplers_0.as_slice());
        };
        ash_vk_bindings.push(builder_0.build());
        let create_info = ash::vk::DescriptorSetLayoutCreateInfo::builder()
            .flags(flags)
            .bindings(ash_vk_bindings.as_slice())
            .build();
        unsafe {
            let ash_vk_descriptor_set_layout = device
                .ash_device
                .create_descriptor_set_layout(&create_info, None)?;
            Ok(Arc::new(DescriptorSetLayout {
                device: device.clone(),
                _init_value: self,
                ash_vk_descriptor_set_layout,
                _phantom_data: Default::default(),
            }))
        }
    }
}
pub struct VkUpdateInfoCollection1 < const BINDING_0 : u32 , const DESCRIPTOR_KIND_0 : DescriptorKind , const DESCRIPTOR_COUNTS_0 : usize , > { pub (crate) t0 : [<< DescriptorTypeDetail < { DESCRIPTOR_KIND_0 } > as DescriptorType > :: ValueType as FnVkUpdateInfo > :: VkValueType ; DESCRIPTOR_COUNTS_0] , }
impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
    > PrivateDescriptorSetValue
    for DescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>
{
    type VkType = VkUpdateInfoCollection1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>;
    fn to_vk_ty(&self) -> Self::VkType {
        VkUpdateInfoCollection1 {
            t0: std::array::from_fn(|i| self.t0[i].to_vk_update_ty()),
        }
    }
    fn push_to_update(
        &self,
        new_value: &Self,
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: ash::vk::DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    ) {
        diff(&self.t0, &new_value.t0, |dst_array_element, counts| {
            let slice = &vk_value.t0[dst_array_element..counts];
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_0)
                .dst_array_element(dst_array_element as _)
                .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        });
        vec.focus_next();
    }
    fn push_to_init(
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    ) {
        {
            let slice = &vk_value.t0;
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_0)
                .dst_array_element(0)
                .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        }
    }
    fn put_pool_size(vec: &mut Vec<ash::vk::DescriptorPoolSize>) {
        vec.push(ash::vk::DescriptorPoolSize {
            ty: DESCRIPTOR_KIND_0.to_ash(),
            descriptor_count: DESCRIPTOR_COUNTS_0 as u32,
        });
    }
}
use crate::descriptor_set::descriptor_variadic_generics::{
    ConstDescriptorSetValue2, DescriptorSetValue2,
};
impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
        const BINDING_1: u32,
        const DESCRIPTOR_KIND_1: DescriptorKind,
        const DESCRIPTOR_COUNTS_1: usize,
    >
    PrivateConstDescriptorSetValue<
        DescriptorSetValue2<
            BINDING_0,
            DESCRIPTOR_KIND_0,
            DESCRIPTOR_COUNTS_0,
            BINDING_1,
            DESCRIPTOR_KIND_1,
            DESCRIPTOR_COUNTS_1,
        >,
    >
    for ConstDescriptorSetValue2<
        BINDING_0,
        DESCRIPTOR_KIND_0,
        DESCRIPTOR_COUNTS_0,
        BINDING_1,
        DESCRIPTOR_KIND_1,
        DESCRIPTOR_COUNTS_1,
    >
{
    fn new_descriptor_layout(
        self,
        device: &Arc<Device>,
        flags: ash::vk::DescriptorSetLayoutCreateFlags,
    ) -> Result<
        Arc<
            DescriptorSetLayout<
                DescriptorSetValue2<
                    BINDING_0,
                    DESCRIPTOR_KIND_0,
                    DESCRIPTOR_COUNTS_0,
                    BINDING_1,
                    DESCRIPTOR_KIND_1,
                    DESCRIPTOR_COUNTS_1,
                >,
            >,
        >,
        ash::vk::Result,
    > {
        let mut ash_vk_bindings = Vec::with_capacity(1usize + 1);
        let immutable_samplers_0 = self.t0.0.try_get_immutable_samplers();
        let mut builder_0 = ash::vk::DescriptorSetLayoutBinding::builder()
            .binding(BINDING_0)
            .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
            .descriptor_count(DESCRIPTOR_COUNTS_0 as _)
            .stage_flags(self.t0.1.to_ash());
        if !immutable_samplers_0.is_empty() {
            builder_0 = builder_0.immutable_samplers(immutable_samplers_0.as_slice());
        };
        ash_vk_bindings.push(builder_0.build());
        let immutable_samplers_1 = self.t1.0.try_get_immutable_samplers();
        let mut builder_1 = ash::vk::DescriptorSetLayoutBinding::builder()
            .binding(BINDING_1)
            .descriptor_type(DESCRIPTOR_KIND_1.to_ash())
            .descriptor_count(DESCRIPTOR_COUNTS_1 as _)
            .stage_flags(self.t1.1.to_ash());
        if !immutable_samplers_1.is_empty() {
            builder_1 = builder_1.immutable_samplers(immutable_samplers_1.as_slice());
        };
        ash_vk_bindings.push(builder_1.build());
        let create_info = ash::vk::DescriptorSetLayoutCreateInfo::builder()
            .flags(flags)
            .bindings(ash_vk_bindings.as_slice())
            .build();
        unsafe {
            let ash_vk_descriptor_set_layout = device
                .ash_device
                .create_descriptor_set_layout(&create_info, None)?;
            Ok(Arc::new(DescriptorSetLayout {
                device: device.clone(),
                _init_value: self,
                ash_vk_descriptor_set_layout,
                _phantom_data: Default::default(),
            }))
        }
    }
}
pub struct VkUpdateInfoCollection2 < const BINDING_0 : u32 , const DESCRIPTOR_KIND_0 : DescriptorKind , const DESCRIPTOR_COUNTS_0 : usize , const BINDING_1 : u32 , const DESCRIPTOR_KIND_1 : DescriptorKind , const DESCRIPTOR_COUNTS_1 : usize , > { pub (crate) t0 : [<< DescriptorTypeDetail < { DESCRIPTOR_KIND_0 } > as DescriptorType > :: ValueType as FnVkUpdateInfo > :: VkValueType ; DESCRIPTOR_COUNTS_0] , pub (crate) t1 : [<< DescriptorTypeDetail < { DESCRIPTOR_KIND_1 } > as DescriptorType > :: ValueType as FnVkUpdateInfo > :: VkValueType ; DESCRIPTOR_COUNTS_1] , }
impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
        const BINDING_1: u32,
        const DESCRIPTOR_KIND_1: DescriptorKind,
        const DESCRIPTOR_COUNTS_1: usize,
    > PrivateDescriptorSetValue
    for DescriptorSetValue2<
        BINDING_0,
        DESCRIPTOR_KIND_0,
        DESCRIPTOR_COUNTS_0,
        BINDING_1,
        DESCRIPTOR_KIND_1,
        DESCRIPTOR_COUNTS_1,
    >
{
    type VkType = VkUpdateInfoCollection2<
        BINDING_0,
        DESCRIPTOR_KIND_0,
        DESCRIPTOR_COUNTS_0,
        BINDING_1,
        DESCRIPTOR_KIND_1,
        DESCRIPTOR_COUNTS_1,
    >;
    fn to_vk_ty(&self) -> Self::VkType {
        VkUpdateInfoCollection2 {
            t0: std::array::from_fn(|i| self.t0[i].to_vk_update_ty()),
            t1: std::array::from_fn(|i| self.t1[i].to_vk_update_ty()),
        }
    }
    fn push_to_update(
        &self,
        new_value: &Self,
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: ash::vk::DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    ) {
        diff(&self.t0, &new_value.t0, |dst_array_element, counts| {
            let slice = &vk_value.t0[dst_array_element..counts];
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_0)
                .dst_array_element(dst_array_element as _)
                .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        });
        diff(&self.t1, &new_value.t1, |dst_array_element, counts| {
            let slice = &vk_value.t1[dst_array_element..counts];
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_1)
                .dst_array_element(dst_array_element as _)
                .descriptor_type(DESCRIPTOR_KIND_1.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        });
        vec.focus_next();
    }
    fn push_to_init(
        vk_value: &mut Self::VkType,
        ash_vk_descriptor_set: DescriptorSet,
        vec: &mut ParallelSplitWriteDescriptorSets,
    ) {
        {
            let slice = &vk_value.t0;
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_0)
                .dst_array_element(0)
                .descriptor_type(DESCRIPTOR_KIND_0.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        }
        {
            let slice = &vk_value.t1;
            let builder = ash::vk::WriteDescriptorSet::builder()
                .dst_set(ash_vk_descriptor_set)
                .dst_binding(BINDING_1)
                .dst_array_element(0)
                .descriptor_type(DESCRIPTOR_KIND_1.to_ash())
                .update_info(slice);
            vec.fix_push(builder);
        }
    }
    fn put_pool_size(vec: &mut Vec<ash::vk::DescriptorPoolSize>) {
        vec.push(ash::vk::DescriptorPoolSize {
            ty: DESCRIPTOR_KIND_0.to_ash(),
            descriptor_count: DESCRIPTOR_COUNTS_0 as u32,
        });
        vec.push(ash::vk::DescriptorPoolSize {
            ty: DESCRIPTOR_KIND_1.to_ash(),
            descriptor_count: DESCRIPTOR_COUNTS_1 as u32,
        });
    }
}
