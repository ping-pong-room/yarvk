use crate::descriptor_set::descriptor_set::DescriptorSetValue;
use crate::descriptor_set::private::PrivateConstDescriptorSetValue;
use crate::device::Device;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait IDescriptorSetLayout {
    fn raw(&self) -> ash::vk::DescriptorSetLayout;
}

pub struct DescriptorSetLayout<T: DescriptorSetValue> {
    pub device: Arc<Device>,
    pub(crate) ash_vk_descriptor_set_layout: ash::vk::DescriptorSetLayout,
    pub(crate) _phantom_data: PhantomData<T>,
}

impl<T: DescriptorSetValue> IDescriptorSetLayout for DescriptorSetLayout<T> {
    fn raw(&self) -> ash::vk::DescriptorSetLayout {
        self.ash_vk_descriptor_set_layout
    }
}

impl<T: DescriptorSetValue> DescriptorSetLayout<T> {
    pub fn new(
        device: &Arc<Device>,
        init_value: T::ConstDescriptorSetValue,
    ) -> Result<Arc<Self>, ash::vk::Result> {
        init_value.new_descriptor_layout(device, ash::vk::DescriptorSetLayoutCreateFlags::default())
    }
}

impl<T: DescriptorSetValue> Drop for DescriptorSetLayout<T> {
    fn drop(&mut self) {
        unsafe {
            // Host Synchronization: descriptorSetLayout
            self.device
                .ash_device
                .destroy_descriptor_set_layout(self.ash_vk_descriptor_set_layout, None);
        }
    }
}
