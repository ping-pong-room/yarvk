use crate::descriptor_pool::{ChangedDescriptorSet, DescriptorSet, UpdateHolder};
use crate::device_features::{register_features, Feature, FeatureType};
use crate::extensions::{DeviceExtension, DeviceExtensionType, PhysicalDeviceExtensionType};
use crate::physical_device::queue_falmily_properties::QueueFamilyProperties;
use crate::physical_device::PhysicalDevice;
use crate::queue::Queue;
use rustc_hash::{FxHashMap, FxHashSet};
use std::mem::ManuallyDrop;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

pub struct DeviceQueueCreateInfoBuilder {
    queue_family: QueueFamilyProperties,
    priorities: Vec<f32>,
}

impl DeviceQueueCreateInfoBuilder {
    pub fn add_priority(mut self, priority: f32) -> Self {
        self.priorities.push(priority);
        self
    }

    pub fn build(self) -> DeviceQueueCreateInfo {
        DeviceQueueCreateInfo {
            queue_family: self.queue_family,
            priorities: self.priorities,
        }
    }
}

pub struct DeviceQueueCreateInfo {
    queue_family: QueueFamilyProperties,
    priorities: Vec<f32>,
}

impl DeviceQueueCreateInfo {
    pub fn builder(queue_family: QueueFamilyProperties) -> DeviceQueueCreateInfoBuilder {
        // DONE VUID-VkDeviceQueueCreateInfo-queueFamilyIndex-00381
        DeviceQueueCreateInfoBuilder {
            queue_family,
            priorities: Default::default(),
        }
    }
    fn ash_builder(&self) -> ash::vk::DeviceQueueCreateInfoBuilder {
        ash::vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(self.queue_family.index)
            .queue_priorities(self.priorities.as_slice())
    }
}

pub struct DeviceBuilder {
    physical_device: Arc<PhysicalDevice>,
    device_queue_create_infos: Vec<DeviceQueueCreateInfo>,
    enabled_features: FxHashSet<FeatureType>,
    enabled_extensions: FxHashSet<PhysicalDeviceExtensionType>,
}

impl DeviceBuilder {
    pub fn add_queue_info(mut self, queue_create_info: DeviceQueueCreateInfo) -> Self {
        self.device_queue_create_infos.push(queue_create_info);
        self
    }

    pub fn add_feature<T: Into<FeatureType>>(mut self, feature: T) -> Self {
        self.enabled_features.insert(feature.into());
        self
    }

    fn add_extension_inner(&mut self, extension: &DeviceExtensionType) {
        let physical_ext_ty = extension.into();
        self.enabled_extensions.insert(physical_ext_ty);
        //SILENCE VUID-vkCreateDevice-ppEnabledExtensionNames-01387
        // It's safe to do this, the instance extension required for dependencies is also need by
        // the extension it self.
        let deps = physical_ext_ty.get_dependencies();
        for dep in deps {
            self.enabled_extensions.insert(*dep);
        }
    }

    pub fn add_extension(mut self, extension: &DeviceExtensionType) -> Self {
        self.add_extension_inner(extension);
        self
    }

    pub fn build(
        mut self,
    ) -> Result<(Arc<Device>, FxHashMap<QueueFamilyProperties, Vec<Queue>>), ash::vk::Result> {
        //SILENCE VUID-VkDeviceCreateInfo-pProperties-04451
        if self
            .physical_device
            .supported_extensions
            .contains(&PhysicalDeviceExtensionType::KhrPortabilitySubset)
        {
            self.add_extension_inner(&DeviceExtensionType::KhrPortabilitySubset);
        }

        //TODO VUID-VkDeviceCreateInfo-pNext-pNext
        //TODO VUID-VkDeviceCreateInfo-sType-unique
        //MUST VUID-VkDeviceCreateInfo-queueCreateInfoCount-arraylength
        if self.device_queue_create_infos.is_empty() {
            panic!("VUID-VkDeviceCreateInfo-queueCreateInfoCount-arraylength");
        }

        let mut device_create_info = ash::vk::DeviceCreateInfo::builder();
        unsafe {
            let feature2 = register_features(&self.enabled_features);
            // SILENCE VUID-VkDeviceCreateInfo-pNext-00373
            device_create_info.p_next = std::mem::transmute(&feature2);
        }

        let queue_create_infos: Vec<ash::vk::DeviceQueueCreateInfo> = self
            .device_queue_create_infos
            .iter()
            .map(|q| q.ash_builder().build())
            .collect();
        let extensions: Vec<*const i8> = self
            .enabled_extensions
            .iter()
            .map(|extension| extension.to_cstr().as_ptr())
            .collect();
        let device_create_info = device_create_info
            .queue_create_infos(unsafe { std::mem::transmute(queue_create_infos.as_slice()) })
            .enabled_extension_names(extensions.as_slice());
        let device_create_info = device_create_info.build();
        //TODO VUID-vkCreateDevice-pAllocator-parameter
        // Host Synchronization: none
        let vk_device: ash::Device = unsafe {
            self.physical_device.instance.ash_instance.create_device(
                self.physical_device.vk_physical_device,
                &device_create_info,
                None,
            )?
        };

        let device = Arc::new(Device {
            physical_device: self.physical_device,
            ash_device: vk_device,
            enabled_extensions: self.enabled_extensions,
            enabled_features: self.enabled_features,
            allocations: AtomicU32::default(),
        });

        let mut queues_map = FxHashMap::default();
        self.device_queue_create_infos.into_iter().for_each(|q| {
            let queue_family = q.queue_family;
            let queues = q
                .priorities
                .iter()
                .enumerate()
                .map(|(index, _)| unsafe {
                    // Host Synchronization: none
                    let queue = device
                        .ash_device
                        .get_device_queue(queue_family.index, index as u32);
                    Queue {
                        device: device.clone(),
                        vk_queue: queue,
                        // queue_family_property: queue_family.property,
                    }
                })
                .collect();
            queues_map.insert(queue_family, queues);
        });

        Ok((device, queues_map))
    }
}

pub struct Device {
    pub physical_device: Arc<PhysicalDevice>,
    pub(crate) ash_device: ash::Device,
    pub(crate) enabled_extensions: FxHashSet<PhysicalDeviceExtensionType>,
    pub(crate) enabled_features: FxHashSet<FeatureType>,
    pub(crate) allocations: AtomicU32,
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.ash_device.handle() == other.ash_device.handle()
    }
}

impl Eq for Device {}

impl Device {
    pub fn builder(physical_device: Arc<PhysicalDevice>) -> DeviceBuilder {
        DeviceBuilder {
            physical_device,
            device_queue_create_infos: vec![],
            enabled_features: Default::default(),
            enabled_extensions: Default::default(),
        }
    }
    pub fn get_extension<const EXT: PhysicalDeviceExtensionType>(
        self: &Arc<Self>,
    ) -> Option<DeviceExtension<EXT>> {
        if self.enabled_extensions.contains(&EXT) {
            Some(DeviceExtension::<EXT> {
                device: self.clone(),
                _p: Default::default(),
            })
        } else {
            None
        }
    }
    pub fn get_feature<const F: FeatureType>(self: &Arc<Self>) -> Option<Feature<F>> {
        if self.enabled_features.contains(&F) {
            Some(Feature::<F> {
                device: self.clone(),
                _p: Default::default(),
            })
        } else {
            None
        }
    }
    // TODO implement copy
    // TODO VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT
    pub fn update_descriptor_sets(
        &self,
        descriptor_writes: &mut [ChangedDescriptorSet],
        // descriptor_copies: &[CopyDescriptorSet],
    ) {
        let mut images_counts = 0;
        let mut buffers_counts = 0;
        let mut buffer_views_counts = 0;

        for ds in descriptor_writes.iter_mut() {
            images_counts += ds.images_counts;
            buffers_counts += ds.buffers_counts;
            buffer_views_counts += ds.buffer_views_counts;
        }
        let mut holder = UpdateHolder {
            vk_image_infos: Vec::with_capacity(images_counts),
            vk_buffer_infos: Vec::with_capacity(buffers_counts),
            vk_texel_buffer_views: Vec::with_capacity(buffer_views_counts),
            vk_write_descriptor_sets: Vec::with_capacity(
                images_counts + buffers_counts + buffer_views_counts,
            ),
        };
        for ds in descriptor_writes.iter_mut() {
            ds.to_ash(&mut holder);
        }
        unsafe {
            // Host Synchronization: VUID-vkUpdateDescriptorSets-pDescriptorWrites-06993
            //  pDescriptorWrites[i].dstSet pDescriptorCopies[i].dstSet
            self.ash_device
                .update_descriptor_sets(holder.vk_write_descriptor_sets.as_slice(), &[]);
        }
    }

    pub fn free_descriptor_sets(&self, ds: Vec<DescriptorSet>) -> Result<(), ash::vk::Result> {
        let ash_vk_descriptor_sets: Vec<_> = ds.iter().map(|ds| ds.ash_vk_descriptor_set).collect();
        let descriptor_pool = ds.first().unwrap().descriptor_pool.clone();
        unsafe {
            let _ = ManuallyDrop::new(ds);
            // TODO VUID-vkFreeDescriptorSets-pDescriptorSets-00309
            // Host Synchronization descriptorPool, pDescriptorSets
            let ash_vk_descriptor_pool = descriptor_pool.ash_vk_descriptor_pool.write();
            self.ash_device
                .free_descriptor_sets(*ash_vk_descriptor_pool, ash_vk_descriptor_sets.as_slice())
        }
    }

    pub fn wait_idle(&self) {
        panic!(
            "due to performance reason, device_wait_idle is not implemented. \
        vkDeviceWaitIdle is equivalent to calling vkQueueWaitIdle for all queues owned by device. \
        Manage queues and wait on them manually for best performance"
        );
    }
}

impl Drop for Device {
    // DONE VUID-vkDestroyDevice-device-00378
    // TODO VUID-vkDestroyDevice-device-00379
    // TODO VUID-vkDestroyDevice-device-00380
    // Host Synchronization: device all VkQueue objects
    fn drop(&mut self) {
        unsafe {
            self.ash_device.destroy_device(None);
        }
    }
}
