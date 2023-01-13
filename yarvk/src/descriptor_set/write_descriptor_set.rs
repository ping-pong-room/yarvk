use crate::descriptor_set::descriptor_set::{DescriptorSet, DescriptorSetValue};
use crate::descriptor_set::private::ParallelSplitWriteDescriptorSets;
use crate::device::Device;
use std::any::Any;
use std::sync::Arc;

pub struct WriteDescriptorSets<'a, T: DescriptorSetValue> {
    pub(crate) descriptor_set: &'a mut DescriptorSet<T>,
    pub(crate) new_value: T,
    pub(crate) vk_value: T::VkType,
}

impl<'a, T: DescriptorSetValue> WriteDescriptorSets<'a, T> {
    pub fn set_value(&mut self, f: impl FnOnce(&mut T)) {
        let mut t = self.descriptor_set.value.clone();
        f(&mut t);
        self.new_value = t;
    }
}

impl<T: DescriptorSetValue> DescriptorSet<T> {
    pub fn update(&mut self, f: impl FnOnce(&mut T)) -> WriteDescriptorSets<T> {
        let mut new_value = self.value.clone();
        f(&mut new_value);
        let vk_value = new_value.to_vk_ty();
        WriteDescriptorSets {
            descriptor_set: self,
            new_value,
            vk_value,
        }
    }
    pub(crate) fn init(&mut self) -> WriteDescriptorSets<T> {
        let new_value = self.value.clone();
        let vk_value = new_value.to_vk_ty();
        WriteDescriptorSets {
            descriptor_set: self,
            new_value,
            vk_value,
        }
    }
}

pub struct Updatable<'a> {
    device: Arc<Device>,
    vk_values: Vec<Box<dyn Any>>,
    update_vec: ParallelSplitWriteDescriptorSets<'a>,
}

impl<'a> Updatable<'a> {
    pub fn add<T: DescriptorSetValue>(
        &mut self,
        write_descriptor_sets: WriteDescriptorSets<'a, T>,
    ) {
        self.vk_values
            .push(Box::new(write_descriptor_sets.vk_value));
        let vk_value: &mut T::VkType = self.vk_values.last_mut().unwrap().downcast_mut().unwrap();
        write_descriptor_sets.descriptor_set.value.push_to_update(
            &write_descriptor_sets.new_value,
            vk_value,
            write_descriptor_sets.descriptor_set.ash_vk_descriptor_set,
            &mut self.update_vec,
        );
        write_descriptor_sets.descriptor_set.value = write_descriptor_sets.new_value.clone()
    }

    pub(crate) fn add_to_init<T: DescriptorSetValue>(
        &mut self,
        write_descriptor_sets: WriteDescriptorSets<'a, T>,
    ) {
        self.vk_values
            .push(Box::new(write_descriptor_sets.vk_value));
        let vk_value: &mut T::VkType = self.vk_values.last_mut().unwrap().downcast_mut().unwrap();
        write_descriptor_sets.descriptor_set.value.push_to_init(
            vk_value,
            write_descriptor_sets.descriptor_set.ash_vk_descriptor_set,
            &mut self.update_vec,
        );
        write_descriptor_sets.descriptor_set.value = write_descriptor_sets.new_value.clone()
    }

    pub fn update(self) {
        self.update_vec.update(&self.device)
    }
}

impl Device {
    pub fn update_descriptor_sets<'a>(self: &Arc<Self>) -> Updatable<'a> {
        Updatable {
            device: self.clone(),
            vk_values: vec![],
            update_vec: ParallelSplitWriteDescriptorSets::new(),
        }
    }
}
