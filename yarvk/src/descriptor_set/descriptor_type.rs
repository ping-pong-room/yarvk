use crate::descriptor_set::private::FnVkUpdateInfo;
use crate::image_view::ImageView;
use crate::sampler::Sampler;
use crate::{BufferView, IBuffer};
use std::sync::Arc;

#[derive(PartialEq, Eq)]
pub enum DescriptorKind {
    Sampler,
    SamplerImmutable,
    CombinedImageSampler,
    CombinedImageSamplerImmutable,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
    InlineUniformBlock,
    // TODO implement following, or should I?
    // AccelerationStructureKhr,
    // AccelerationStructureNv,
    // MutableValve,
}

impl DescriptorKind {
    pub(crate) fn to_ash(&self) -> ash::vk::DescriptorType {
        match self {
            DescriptorKind::Sampler => ash::vk::DescriptorType::SAMPLER,
            DescriptorKind::SamplerImmutable => ash::vk::DescriptorType::SAMPLER,
            DescriptorKind::CombinedImageSampler => ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            DescriptorKind::CombinedImageSamplerImmutable => {
                ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER
            }
            DescriptorKind::SampledImage => ash::vk::DescriptorType::SAMPLED_IMAGE,
            DescriptorKind::StorageImage => ash::vk::DescriptorType::STORAGE_IMAGE,
            DescriptorKind::UniformTexelBuffer => ash::vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
            DescriptorKind::StorageTexelBuffer => ash::vk::DescriptorType::STORAGE_TEXEL_BUFFER,
            DescriptorKind::UniformBuffer => ash::vk::DescriptorType::UNIFORM_BUFFER,
            DescriptorKind::StorageBuffer => ash::vk::DescriptorType::STORAGE_BUFFER,
            DescriptorKind::UniformBufferDynamic => ash::vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            DescriptorKind::StorageBufferDynamic => ash::vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            DescriptorKind::InputAttachment => ash::vk::DescriptorType::INPUT_ATTACHMENT,
            DescriptorKind::InlineUniformBlock => ash::vk::DescriptorType::INLINE_UNIFORM_BLOCK,
        }
    }
}

pub trait DescriptorType {
    type ValueType: FnVkUpdateInfo + Clone + PartialEq + Send + Sync;
    type ConstValueType;
}

pub struct DescriptorTypeDetail<const DESCRIPTOR_KIND: DescriptorKind> {}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::UniformTexelBuffer }> {
    type ValueType = Arc<BufferView>;
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::StorageTexelBuffer }> {
    type ValueType = Arc<BufferView>;
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::UniformBuffer }> {
    type ValueType = (Arc<IBuffer>, ash::vk::DeviceSize, ash::vk::DeviceSize);
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::StorageBuffer }> {
    type ValueType = (Arc<IBuffer>, ash::vk::DeviceSize, ash::vk::DeviceSize);
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::UniformBufferDynamic }> {
    type ValueType = (Arc<IBuffer>, ash::vk::DeviceSize, ash::vk::DeviceSize);
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::StorageBufferDynamic }> {
    type ValueType = (Arc<IBuffer>, ash::vk::DeviceSize, ash::vk::DeviceSize);
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::Sampler }> {
    type ValueType = Arc<Sampler>;
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::SamplerImmutable }> {
    type ValueType = ();
    type ConstValueType = Arc<Sampler>;
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::CombinedImageSampler }> {
    type ValueType = (Arc<Sampler>, Arc<ImageView>, ash::vk::ImageLayout);
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::CombinedImageSamplerImmutable }> {
    type ValueType = (Arc<ImageView>, ash::vk::ImageLayout);
    type ConstValueType = Arc<Sampler>;
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::SampledImage }> {
    type ValueType = (Arc<ImageView>, ash::vk::ImageLayout);
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::StorageImage }> {
    type ValueType = (Arc<ImageView>, ash::vk::ImageLayout);
    type ConstValueType = ();
}

impl DescriptorType for DescriptorTypeDetail<{ DescriptorKind::InputAttachment }> {
    type ValueType = (Arc<ImageView>, ash::vk::ImageLayout);
    type ConstValueType = ();
}

impl<const DESCRIPTOR_KIND: DescriptorKind> DescriptorType
    for DescriptorTypeDetail<{ DESCRIPTOR_KIND }>
{
    default type ValueType = ();
    default type ConstValueType = ();
}
