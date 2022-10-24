use std::sync::Arc;
use crate::descriptor::descriptor_set_layout::DescriptorSetLayout;

pub struct DescriptorSet {
    pub descriptor_set_layout: Arc<DescriptorSetLayout>,
    pub(crate) ash_vk_descriptor_set: ash::vk::DescriptorSet,
}