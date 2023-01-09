use std::sync::Arc;
use ash::vk::Handle;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::command::command_buffer::State::RECORDING;
use crate::descriptor_set_v2::descriptor_type::{
    DescriptorKind, DescriptorType, DescriptorTypeDetail,
};
use crate::descriptor_set_v2::private::{PrivateConstDescriptorSetValue, PrivateDescriptorSetValue};
use crate::pipeline::PipelineLayout;
use crate::pipeline::shader_stage::ShaderStage;

pub trait DescriptorSetValue: PrivateDescriptorSetValue + Send + Sync {
    type ConstDescriptorSetValue: PrivateConstDescriptorSetValue<Self>;
}

pub trait IDescriptorSet: Sync + Send {
    // TODO make this private
    fn raw(&self) -> ash::vk::DescriptorSet;
}

pub struct DescriptorSet<T: DescriptorSetValue> {
    pub(crate) ash_vk_descriptor_set: ash::vk::DescriptorSet,
    pub(crate) value: T,
}

impl<T: DescriptorSetValue> IDescriptorSet for DescriptorSet<T> {
    fn raw(&self) -> ash::vk::DescriptorSet {
        self.ash_vk_descriptor_set
    }
}

// TODO use variadic generics one day
// TODO use macro when test ready
// TODO wrap binding, kind, counts together when const generic feature is ready
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
    > const DescriptorSetValue
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
    > const DescriptorSetValue
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

// pub struct DescriptorSet3<
//     const BINDING_0: u32,
//     const DESCRIPTOR_KIND_0: DescriptorKind,
//     const DESCRIPTOR_COUNTS_0: usize,
//     const BINDING_1: u32,
//     const DESCRIPTOR_KIND_1: DescriptorKind,
//     const DESCRIPTOR_COUNTS_1: usize,
//     const BINDING_2: u32,
//     const DESCRIPTOR_KIND_2: DescriptorKind,
//     const DESCRIPTOR_COUNTS_2: usize,
// > {
//     ash_vk_descriptor_set: ash::vk::DescriptorSet,
//     t0: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_0 }> as DescriptorType>::ValueType;
//         DESCRIPTOR_COUNTS_0],
//     t1: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_1 }> as DescriptorType>::ValueType;
//         DESCRIPTOR_COUNTS_1],
//     t2: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_2 }> as DescriptorType>::ValueType;
//         DESCRIPTOR_COUNTS_2],
// }
//
// pub struct DescriptorSet4<
//     const BINDING_0: u32,
//     const DESCRIPTOR_KIND_0: DescriptorKind,
//     const DESCRIPTOR_COUNTS_0: usize,
//     const BINDING_1: u32,
//     const DESCRIPTOR_KIND_1: DescriptorKind,
//     const DESCRIPTOR_COUNTS_1: usize,
//     const BINDING_2: u32,
//     const DESCRIPTOR_KIND_2: DescriptorKind,
//     const DESCRIPTOR_COUNTS_2: usize,
//     const BINDING_3: u32,
//     const DESCRIPTOR_KIND_3: DescriptorKind,
//     const DESCRIPTOR_COUNTS_3: usize,
// > {
//     ash_vk_descriptor_set: ash::vk::DescriptorSet,
//     t0: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_0 }> as DescriptorType>::ValueType;
//         DESCRIPTOR_COUNTS_0],
//     t1: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_1 }> as DescriptorType>::ValueType;
//         DESCRIPTOR_COUNTS_1],
//     t2: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_2 }> as DescriptorType>::ValueType;
//         DESCRIPTOR_COUNTS_2],
//     t3: [<DescriptorTypeDetail<{ DESCRIPTOR_KIND_3 }> as DescriptorType>::ValueType;
//         DESCRIPTOR_COUNTS_3],
// }

impl<const LEVEL: Level, const SCOPE: RenderPassScope, const ONE_TIME_SUBMIT: bool>
CommandBuffer<LEVEL, { RECORDING }, SCOPE, ONE_TIME_SUBMIT>
{
    // DONE VUID-vkCmdBindDescriptorSets-commandBuffer-recording
    pub fn cmd_bind_descriptor_sets_v2<It: IntoIterator<Item = Arc<dyn IDescriptorSet>>>(
        &mut self,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        layout: Arc<PipelineLayout>,
        first_set: u32,
        descriptor_sets: It,
        dynamic_offsets: &[u32],
    ) {
        self.holding_resources
            .pipeline_layouts
            .insert(layout.ash_vk_pipeline_layout.as_raw(), layout.clone());
        let vk_descriptor_sets: Vec<_> = descriptor_sets
            .into_iter()
            .map(|ds| {
                let ash_vk_descriptor_set = ds.raw();
                self.holding_resources
                    .descriptor_sets_v2
                    .insert(ash_vk_descriptor_set.as_raw(), ds.clone());
                ash_vk_descriptor_set
            })
            .collect();
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_bind_descriptor_sets(
                self.vk_command_buffer,
                pipeline_bind_point,
                layout.ash_vk_pipeline_layout,
                first_set,
                vk_descriptor_sets.as_slice(),
                dynamic_offsets,
            )
        }
    }
}