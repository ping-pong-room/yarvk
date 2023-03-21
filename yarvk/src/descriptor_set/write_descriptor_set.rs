use crate::descriptor_set::descriptor_set::{DescriptorSet, DescriptorSetValue};
use crate::descriptor_set::private::ParallelSplitWriteDescriptorSets;
use crate::device::Device;
use std::any::Any;
use std::sync::Arc;

struct WriteDescriptorSets<'a, T: DescriptorSetValue> {
    descriptor_set: &'a mut DescriptorSet<T>,
    new_value: T,
    vk_value: T::VkType,
}

pub struct Updatable<'a> {
    device: Arc<Device>,
    vk_values: Vec<Box<dyn Any>>,
    update_vec: ParallelSplitWriteDescriptorSets<'a>,
}

impl<'a> Updatable<'a> {
    pub fn add<T: DescriptorSetValue>(
        &mut self,
        descriptor_set: &'a mut DescriptorSet<T>,
        f: impl FnOnce(Option<T>) -> T,
    ) {
        let new_value = descriptor_set.value.clone();
        let new_value = f(new_value);
        let vk_value = new_value.to_vk_ty();
        let write_descriptor_sets = WriteDescriptorSets {
            descriptor_set,
            new_value,
            vk_value,
        };
        self.vk_values
            .push(Box::new(write_descriptor_sets.vk_value));
        let vk_value: &mut T::VkType = self.vk_values.last_mut().unwrap().downcast_mut().unwrap();
        match &write_descriptor_sets.descriptor_set.value {
            None => {
                T::push_to_init(
                    vk_value,
                    write_descriptor_sets.descriptor_set.ash_vk_descriptor_set,
                    &mut self.update_vec,
                );
            }
            Some(value) => {
                value.push_to_update(
                    &write_descriptor_sets.new_value,
                    vk_value,
                    write_descriptor_sets.descriptor_set.ash_vk_descriptor_set,
                    &mut self.update_vec,
                );
            }
        }
        write_descriptor_sets.descriptor_set.value = Some(write_descriptor_sets.new_value.clone())
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
