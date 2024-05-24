use crate::descriptor_set::descriptor_set::DescriptorSetValue;
use crate::descriptor_set::descriptor_type::{
    DescriptorKind, DescriptorType, DescriptorTypeDetail,
};
use crate::pipeline::shader_stage::ShaderStage;
#[derive(Clone)]
pub struct DescriptorSetValue1<
    const BINDING_0: u32,
    const DESCRIPTOR_KIND_0: DescriptorKind,
    const DESCRIPTOR_COUNTS_0: usize,
> {
    pub t0: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_0 }> as DescriptorType>::ValueType;
        DESCRIPTOR_COUNTS_0],
}
impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
    > DescriptorSetValue
    for DescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>
{
    type ConstDescriptorSetValue =
        ConstDescriptorSetValue1<BINDING_0, DESCRIPTOR_KIND_0, DESCRIPTOR_COUNTS_0>;
}
pub struct ConstDescriptorSetValue1<
    const BINDING_0: u32,
    const DESCRIPTOR_KIND_0: DescriptorKind,
    const DESCRIPTOR_COUNTS_0: usize,
> {
    pub t0: (
        [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_0 }> as DescriptorType>::ConstValueType;
            DESCRIPTOR_COUNTS_0],
        ShaderStage,
    ),
}
#[derive(Clone)]
pub struct DescriptorSetValue2<
    const BINDING_0: u32,
    const DESCRIPTOR_KIND_0: DescriptorKind,
    const DESCRIPTOR_COUNTS_0: usize,
    const BINDING_1: u32,
    const DESCRIPTOR_KIND_1: DescriptorKind,
    const DESCRIPTOR_COUNTS_1: usize,
> {
    pub t0: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_0 }> as DescriptorType>::ValueType;
        DESCRIPTOR_COUNTS_0],
    pub t1: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_1 }> as DescriptorType>::ValueType;
        DESCRIPTOR_COUNTS_1],
}
impl<
        const BINDING_0: u32,
        const DESCRIPTOR_KIND_0: DescriptorKind,
        const DESCRIPTOR_COUNTS_0: usize,
        const BINDING_1: u32,
        const DESCRIPTOR_KIND_1: DescriptorKind,
        const DESCRIPTOR_COUNTS_1: usize,
    > DescriptorSetValue
    for DescriptorSetValue2<
        BINDING_0,
        DESCRIPTOR_KIND_0,
        DESCRIPTOR_COUNTS_0,
        BINDING_1,
        DESCRIPTOR_KIND_1,
        DESCRIPTOR_COUNTS_1,
    >
{
    type ConstDescriptorSetValue = ConstDescriptorSetValue2<
        BINDING_0,
        DESCRIPTOR_KIND_0,
        DESCRIPTOR_COUNTS_0,
        BINDING_1,
        DESCRIPTOR_KIND_1,
        DESCRIPTOR_COUNTS_1,
    >;
}
pub struct ConstDescriptorSetValue2<
    const BINDING_0: u32,
    const DESCRIPTOR_KIND_0: DescriptorKind,
    const DESCRIPTOR_COUNTS_0: usize,
    const BINDING_1: u32,
    const DESCRIPTOR_KIND_1: DescriptorKind,
    const DESCRIPTOR_COUNTS_1: usize,
> {
    pub t0: (
        [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_0 }> as DescriptorType>::ConstValueType;
            DESCRIPTOR_COUNTS_0],
        ShaderStage,
    ),
    pub t1: (
        [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_1 }> as DescriptorType>::ConstValueType;
            DESCRIPTOR_COUNTS_1],
        ShaderStage,
    ),
}
