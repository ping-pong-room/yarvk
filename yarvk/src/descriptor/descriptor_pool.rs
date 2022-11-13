use crate::descriptor::descriptor_set_layout::DescriptorSetLayout;
use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE::MutableDescriptorType;
use ash::vk::{DescriptorPoolSize, Handle};
use std::sync::Arc;
use crate::descriptor::write_descriptor_sets::WriteDescriptorSets;

pub enum DescriptorPoolCreateFlag {
    VkDescriptorPoolCreateUpdateAfterBind,
    // DONE VUID-VkDescriptorPoolCreateInfo-flags-04609
    VkDescriptorPoolCreateHostOnlyValve(Feature<{ MutableDescriptorType.into() }>),
}

impl DescriptorPoolCreateFlag {
    pub fn to_ash(&self) -> ash::vk::DescriptorPoolCreateFlags {
        match self {
            DescriptorPoolCreateFlag::VkDescriptorPoolCreateUpdateAfterBind => {
                ash::vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND
            }
            DescriptorPoolCreateFlag::VkDescriptorPoolCreateHostOnlyValve(_) => {
                ash::vk::DescriptorPoolCreateFlags::HOST_ONLY_VALVE
            }
        }
    }
}

/// Boring descriptor pool
/// 1. Only one type of descriptor set layout are hold.
/// 2. allocate max_set allocators when created.
/// 3. cannot free, update descriptor set instead.
pub struct DescriptorPool {
    pub max_sets: u32,
    pub descriptor_set_layout: Arc<DescriptorSetLayout>,
    pub(crate) ash_vk_descriptor_pool: ash::vk::DescriptorPool,
    unused_descriptor_sets: scc::Queue<ash::vk::DescriptorSet>,
}

impl crate::Handle for DescriptorPool {
    fn handle(&self) -> u64 {
        self.ash_vk_descriptor_pool.as_raw()
    }
}

impl DescriptorPool {
    pub fn builder(descriptor_set_layout: Arc<DescriptorSetLayout>) -> DescriptorPoolBuilder {
        DescriptorPoolBuilder {
            descriptor_set_layout,
            flags: Default::default(),
            max_sets: 0,
        }
    }

    pub fn allocate(self: &Arc<Self>) -> Option<Arc<DescriptorSet>> {
        match self.unused_descriptor_sets.pop() {
            None => None,
            Some(ash_vk_descriptor_set) => Some(Arc::new(DescriptorSet {
                pool: self.clone(),
                ash_vk_descriptor_set: **ash_vk_descriptor_set,
            })),
        }
    }

    pub fn write_descriptor_sets<'a>(self: &'a Arc<Self>) -> WriteDescriptorSets<'a> {
        WriteDescriptorSets::new(self.clone())
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        unsafe {
            // DONE VUID-vkDestroyDescriptorPool-descriptorPool-00303
            // Host Synchronization: descriptorPool
            self.descriptor_set_layout.device
                .ash_device
                .destroy_descriptor_pool(self.ash_vk_descriptor_pool, None);
        }
    }
}

pub struct DescriptorPoolBuilder {
    descriptor_set_layout: Arc<DescriptorSetLayout>,
    flags: ash::vk::DescriptorPoolCreateFlags,
    max_sets: u32,
}

impl DescriptorPoolBuilder {
    pub fn add_flag(mut self, flag: DescriptorPoolCreateFlag) -> Self {
        self.flags |= flag.to_ash();
        self
    }
    pub fn max_sets(mut self, max_sets: u32) -> Self {
        self.max_sets += max_sets;
        self
    }

    pub fn build(self) -> Result<Arc<DescriptorPool>, ash::vk::Result> {
        let device = self.descriptor_set_layout.device.clone();
        let mut pool_sizes =
            Vec::with_capacity(self.max_sets as usize * self.descriptor_set_layout.bindings.len());
        for _ in 0..self.max_sets {
            for (_, binding) in &self.descriptor_set_layout.bindings {
                pool_sizes.push(DescriptorPoolSize {
                    ty: binding.descriptor_type().to_ash(),
                    descriptor_count: binding.descriptor_count(),
                });
            }
        }
        let info = ash::vk::DescriptorPoolCreateInfo::builder()
            .flags(self.flags)
            .max_sets(self.max_sets)
            .pool_sizes(pool_sizes.as_slice())
            .build();
        unsafe {
            // Host Synchronization: none
            let ash_vk_descriptor_pool = device.ash_device.create_descriptor_pool(&info, None)?;

            // pre-allocate all descroptor sets
            let vk_layouts = (0..self.max_sets)
                .map(|_| self.descriptor_set_layout.ash_vk_descriptor_set_layout)
                .collect::<Vec<_>>();
            let create_info = ash::vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(ash_vk_descriptor_pool)
                .set_layouts(vk_layouts.as_slice())
                .build();
            let unused_descriptor_sets = scc::Queue::default();
            // Host Synchronization: pAllocateInfo->descriptorPool
            for vk_descriptor_set in device.ash_device.allocate_descriptor_sets(&create_info)? {
                unused_descriptor_sets.push(vk_descriptor_set);
            }

            let descriptor_pool = DescriptorPool {
                descriptor_set_layout: self.descriptor_set_layout.clone(),
                ash_vk_descriptor_pool,
                unused_descriptor_sets,
                max_sets: self.max_sets,
            };

            Ok(Arc::new(descriptor_pool))
        }
    }
}

pub struct DescriptorSet {
    pub pool: Arc<DescriptorPool>,
    pub(crate) ash_vk_descriptor_set: ash::vk::DescriptorSet,
}

impl Drop for DescriptorSet {
    fn drop(&mut self) {
        self.pool
            .unused_descriptor_sets
            .push(self.ash_vk_descriptor_set);
    }
}
