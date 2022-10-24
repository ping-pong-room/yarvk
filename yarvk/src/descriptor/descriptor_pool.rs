use crate::descriptor::descriptor_set::DescriptorSet;
use crate::descriptor::descriptor_set_layout::DescriptorSetLayout;
use crate::descriptor::write_descriptor_sets::WriteDescriptorSets;
use crate::descriptor::DescriptorType;
use crate::device::Device;
use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceMutableDescriptorTypeFeaturesVALVE::MutableDescriptorType;
use ash::vk::Handle;
use rustc_hash::FxHashMap;
use std::sync::Arc;

pub enum DescriptorPoolCreateFlag {
    DescriptorPoolCreateFreeDescriptorSet,
    VkDescriptorPoolCreateUpdateAfterBind,
    // DONE VUID-VkDescriptorPoolCreateInfo-flags-04609
    VkDescriptorPoolCreateHostOnlyValve(Feature<{ MutableDescriptorType.into() }>),
}

impl DescriptorPoolCreateFlag {
    pub fn to_ash(&self) -> ash::vk::DescriptorPoolCreateFlags {
        match self {
            DescriptorPoolCreateFlag::DescriptorPoolCreateFreeDescriptorSet => {
                ash::vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET
            }
            DescriptorPoolCreateFlag::VkDescriptorPoolCreateUpdateAfterBind => {
                ash::vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND
            }
            DescriptorPoolCreateFlag::VkDescriptorPoolCreateHostOnlyValve(_) => {
                ash::vk::DescriptorPoolCreateFlags::HOST_ONLY_VALVE
            }
        }
    }
}

pub struct Allocatable<'a> {
    descriptor_pool: &'a mut DescriptorPool,
    descriptor_set_layouts: FxHashMap<usize, Arc<DescriptorSetLayout>>,
}

impl<'a> Allocatable<'a> {
    pub fn add_descriptor_set_layout(
        mut self,
        index: usize,
        descriptor_set_layout: Arc<DescriptorSetLayout>,
    ) -> Self {
        self.descriptor_set_layouts
            .insert(index, descriptor_set_layout);
        self
    }
    pub fn allocate(self) -> Result<(), ash::vk::Result> {
        let vk_layouts = self
            .descriptor_set_layouts
            .iter()
            .map(|(_, layout)| layout.ash_vk_descriptor_set_layout)
            .collect::<Vec<_>>();
        let create_info = ash::vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool.ash_vk_descriptor_pool)
            .set_layouts(vk_layouts.as_slice())
            .build();
        let ash_vk_descriptor_sets = unsafe {
            // Host Synchronization: pAllocateInfo->descriptorPool
            self.descriptor_pool
                .device
                .ash_device
                .allocate_descriptor_sets(&create_info)?
        };

        self.descriptor_set_layouts
            .iter()
            .enumerate()
            .for_each(|(i, (index, layout))| {
                self.descriptor_pool.allocated_descriptor_sets.insert(
                    *index,
                    DescriptorSet {
                        ash_vk_descriptor_set: ash_vk_descriptor_sets[i],
                        descriptor_set_layout: layout.clone(),
                    },
                );
            });
        Ok(())
    }
}

pub struct DescriptorPool {
    pub device: Arc<Device>,
    pub(crate) allocated_descriptor_sets: FxHashMap<usize, DescriptorSet>,
    pub(crate) ash_vk_descriptor_pool: ash::vk::DescriptorPool,
    free_descriptor_set: bool,
}

impl crate::Handle for DescriptorPool {
    fn handle(&self) -> u64 {
        self.ash_vk_descriptor_pool.as_raw()
    }
}

impl DescriptorPool {
    pub fn can_free_descriptor_set(&self) -> bool {
        self.free_descriptor_set
    }
    pub fn builder(device: Arc<Device>) -> DescriptorPoolBuilder {
        DescriptorPoolBuilder {
            device,
            flags: Default::default(),
            max_sets: 1,
            descriptor_pool_sizes: Default::default(),
            descriptor_set_layouts: Default::default(),
        }
    }

    pub fn get_descriptor_set(&self, index: &usize) -> Option<&DescriptorSet> {
        self.allocated_descriptor_sets.get(index)
    }

    pub fn allocatable(&mut self) -> Allocatable {
        Allocatable {
            descriptor_pool: self,
            descriptor_set_layouts: Default::default(),
        }
    }

    pub fn free_descriptor_sets(
        &mut self,
        descriptor_sets: &[usize],
    ) -> Result<(), ash::vk::Result> {
        if self.free_descriptor_set {
            let ash_vk_descriptor_sets = descriptor_sets
                .iter()
                .filter_map(|index| match self.allocated_descriptor_sets.remove(index) {
                    None => None,
                    Some(descriptor_set) => Some(descriptor_set.ash_vk_descriptor_set),
                })
                .collect::<Vec<_>>();
            unsafe {
                // TODO VUID-vkFreeDescriptorSets-pDescriptorSets-00309
                // Host Synchronization descriptorPool, pDescriptorSets

                self.device.ash_device.free_descriptor_sets(
                    self.ash_vk_descriptor_pool,
                    ash_vk_descriptor_sets.as_slice(),
                )
            }
        } else {
            // MUST VUID-vkFreeDescriptorSets-descriptorPool-00312
            panic!("descriptorPool must have been created with the VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT flag")
        }
    }

    pub fn write_descriptor_sets(&mut self) -> WriteDescriptorSets {
        WriteDescriptorSets::new(self)
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        if self.free_descriptor_set {
            let _result = self.free_descriptor_sets(
                self.allocated_descriptor_sets
                    .iter()
                    .map(|(index, _)| *index)
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
        }
        unsafe {
            // DONE VUID-vkDestroyDescriptorPool-descriptorPool-00303
            // Host Synchronization: descriptorPool
            self.device
                .ash_device
                .destroy_descriptor_pool(self.ash_vk_descriptor_pool, None);
        }
    }
}

pub struct DescriptorPoolBuilder {
    device: Arc<Device>,
    flags: ash::vk::DescriptorPoolCreateFlags,
    max_sets: u32,
    descriptor_pool_sizes: Vec<ash::vk::DescriptorPoolSize>,
    descriptor_set_layouts: FxHashMap<usize, Arc<DescriptorSetLayout>>,
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
    pub fn add_descriptor_pool_size(mut self, ty: &DescriptorType, descriptor_count: u32) -> Self {
        self.descriptor_pool_sizes
            .push(ash::vk::DescriptorPoolSize {
                ty: ty.to_ash(),
                descriptor_count,
            });
        self
    }

    pub fn build(self) -> Result<DescriptorPool, ash::vk::Result> {
        let free_descriptor_set = self
            .flags
            .contains(ash::vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET);
        let info = ash::vk::DescriptorPoolCreateInfo::builder()
            .flags(self.flags)
            .max_sets(self.max_sets)
            .pool_sizes(self.descriptor_pool_sizes.as_slice())
            .build();
        unsafe {
            // Host Synchronization: none
            let ash_vk_descriptor_pool =
                self.device.ash_device.create_descriptor_pool(&info, None)?;

            let mut descriptor_pool = DescriptorPool {
                device: self.device,
                allocated_descriptor_sets: Default::default(),
                ash_vk_descriptor_pool,
                free_descriptor_set,
            };
            // if there are descriptor set layouts pending, we allocate them as well.
            if !self.descriptor_set_layouts.is_empty() {
                let mut allocatable = descriptor_pool.allocatable();
                allocatable.descriptor_set_layouts = self.descriptor_set_layouts;
                allocatable.allocate()?;
            }
            Ok(descriptor_pool)
        }
    }
}
