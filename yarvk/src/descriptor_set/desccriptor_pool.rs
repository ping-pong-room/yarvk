use crate::descriptor_set::descriptor_set::{DescriptorSet, DescriptorSetValue};
use crate::descriptor_set::descriptor_set_layout::DescriptorSetLayout;
use crate::device::Device;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct DescriptorPool<T: DescriptorSetValue> {
    pub device: Arc<Device>,
    pub max_sets: usize,
    pub(crate) ash_vk_descriptor_pool: ash::vk::DescriptorPool,
    unused_descriptor_sets: crossbeam_queue::ArrayQueue<ash::vk::DescriptorSet>,
    _phantom_data: PhantomData<T>,
}

impl<T: DescriptorSetValue> DescriptorPool<T> {
    pub fn new(
        device: &Arc<Device>,
        max_sets: usize,
        layout: &Arc<DescriptorSetLayout<T>>,
    ) -> Result<Arc<Self>, ash::vk::Result> {
        let mut pool_sizes = Vec::new();
        for _ in 0..max_sets {
            T::put_pool_size(&mut pool_sizes);
        }
        let info = ash::vk::DescriptorPoolCreateInfo::builder()
            .max_sets(max_sets as _)
            .pool_sizes(pool_sizes.as_slice())
            .build();
        unsafe {
            // Host Synchronization: none
            let ash_vk_descriptor_pool = device.ash_device.create_descriptor_pool(&info, None)?;

            // pre-allocate all descriptor sets
            let vk_layouts = (0..max_sets)
                .map(|_| layout.ash_vk_descriptor_set_layout)
                .collect::<Vec<_>>();
            let create_info = ash::vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(ash_vk_descriptor_pool)
                .set_layouts(vk_layouts.as_slice())
                .build();
            let unused_descriptor_sets = crossbeam_queue::ArrayQueue::new(max_sets as _);
            // Host Synchronization: pAllocateInfo->descriptorPool
            for vk_descriptor_set in device.ash_device.allocate_descriptor_sets(&create_info)? {
                unused_descriptor_sets.push(vk_descriptor_set).unwrap();
            }

            let descriptor_pool = DescriptorPool {
                device: device.clone(),
                ash_vk_descriptor_pool,
                unused_descriptor_sets,
                max_sets,
                _phantom_data: Default::default(),
            };

            Ok(Arc::new(descriptor_pool))
        }
    }

    // TODO this is not the best solution, since we cannot batching different pool allocations, but
    // I won't go any further, while waiting descriptor buffer extension ready to land instead.
    pub fn allocate<It: IntoIterator<Item = T>>(
        self: &Arc<Self>,
        values: It,
    ) -> Vec<DescriptorSet<T>> {
        let it = values.into_iter();
        let mut descriptor_sets = Vec::with_capacity(it.size_hint().0);
        for value in it {
            if let Some(ash_vk_descriptor_set) = self.unused_descriptor_sets.pop() {
                descriptor_sets.push(DescriptorSet {
                    ash_vk_descriptor_set,
                    value,
                });
            } else {
                break;
            }
        }
        let mut update = self.device.update_descriptor_sets();
        descriptor_sets.iter_mut().for_each(|descriptor_set| {
            let write_descriptor_sets = descriptor_set.init();
            update.add_to_init(write_descriptor_sets);
        });
        update.update();
        descriptor_sets
    }
}

impl<T: DescriptorSetValue> Drop for DescriptorPool<T> {
    fn drop(&mut self) {
        unsafe {
            // DONE VUID-vkDestroyDescriptorPool-descriptorPool-00303
            // Host Synchronization: descriptorPool
            self.device
                .ash_device
                .destroy_descriptor_pool(self.ash_vk_descriptor_pool, None);
        }
    }
}
