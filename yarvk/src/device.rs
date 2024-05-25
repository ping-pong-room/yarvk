use crate::device_features::Feature;
use crate::extensions::DeviceExtension;
use crate::extensions::Extension;
use crate::physical_device::queue_family_properties::QueueFamilyProperties;
use crate::physical_device::PhysicalDevice;
use crate::queue::Queue;
use ash::vk;
use rustc_hash::{FxHashMap, FxHashSet};
use std::any::TypeId;
use std::collections::hash_map::Entry;
use std::ffi::CStr;
#[cfg(feature = "max_memory_allocation_count_check")]
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

struct VkFeatureUnion {
    raw: Vec<usize>,
}

impl VkFeatureUnion {
    pub unsafe fn new<T: Feature>() -> Self {
        let size = std::mem::size_of::<T::VkFeatureType>();
        let mut raw = Vec::with_capacity(size);
        let ptr = raw.as_mut_ptr() as *mut T::VkFeatureType;
        *ptr = T::VkFeatureType::default();
        T::register_feature(&mut *ptr);
        Self { raw }
    }

    pub unsafe fn add<T: Feature>(&mut self) {
        let ptr = self.raw.as_mut_ptr() as *mut T::VkFeatureType;
        *ptr = T::VkFeatureType::default();
        T::register_feature(&mut *ptr);
    }

    pub unsafe fn get_mut<T>(&mut self) -> &mut T {
        let ptr = self.raw.as_mut_ptr() as *mut T;
        &mut *ptr
    }
}

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
    vk_enabled_feature1: ash::vk::PhysicalDeviceFeatures,
    enabled_features1: FxHashSet<TypeId>,
    enabled_features2: FxHashMap<TypeId, VkFeatureUnion>,
    vk_enabled_extensions: FxHashSet<&'static CStr>,
    enabled_extensions: FxHashSet<TypeId>,
}

impl DeviceBuilder {
    pub fn add_queue_info(mut self, queue_create_info: DeviceQueueCreateInfo) -> Self {
        self.device_queue_create_infos.push(queue_create_info);
        self
    }

    pub fn add_feature<FT: Feature + 'static>(mut self, _instance_extension_dependency: <FT::RequiredExtension as DeviceExtension>::InstanceDependenciesTy) -> Result<Self, ash::vk::Result> {
        if !self.physical_device.supported_feature::<FT>() {
            return Err(ash::vk::Result::ERROR_FEATURE_NOT_PRESENT);
        }

        if FT::RequiredExtension::NAME.to_bytes().len() != 0 {
            self.add_extension_inner::<FT::RequiredExtension>();
        }

        unsafe {
            if !FT::IS_FEATURE2 {
                FT::register_feature(std::mem::transmute(&mut self.vk_enabled_feature1));
                self.enabled_features1.insert(TypeId::of::<FT>());
            } else {
                match self.enabled_features2.entry(TypeId::of::<FT>()) {
                    Entry::Occupied(entry) => {
                        entry.into_mut().add::<FT>();
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(VkFeatureUnion::new::<FT>());
                    }
                }
            }
            Ok(self)
        }
    }

    fn add_extension_inner<EXT: DeviceExtension + 'static>(&mut self) {
        //SILENCE VUID-vkCreateDevice-ppEnabledExtensionNames-01387
        // It's safe to do this, the instance extension required for dependencies is also need by
        // the extension itself.
        for dep in EXT::DEPENDENCIES {
            self.vk_enabled_extensions.insert(*dep);
            self.enabled_extensions.insert(TypeId::of::<EXT>());
        }
        self.vk_enabled_extensions.insert(EXT::NAME);
        self.enabled_extensions.insert(TypeId::of::<EXT>());
    }

    pub fn add_extension<EXT: DeviceExtension + 'static>(
        mut self,
        _instance_extension_dependency: EXT::InstanceDependenciesTy,
    ) -> Result<Self, vk::Result> {
        if !self.physical_device.supported_extension::<EXT>() {
            return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT);
        }
        self.add_extension_inner::<EXT>();
        Ok(self)
    }

    pub fn build(
        mut self,
    ) -> Result<(Arc<Device>, FxHashMap<QueueFamilyProperties, Vec<Queue>>), ash::vk::Result> {
        //TODO VUID-VkDeviceCreateInfo-pNext-pNext
        //TODO VUID-VkDeviceCreateInfo-sType-unique
        //MUST VUID-VkDeviceCreateInfo-queueCreateInfoCount-arraylength
        if self.device_queue_create_infos.is_empty() {
            panic!("VUID-VkDeviceCreateInfo-queueCreateInfoCount-arraylength");
        }

        let mut device_create_info = ash::vk::DeviceCreateInfo::builder();
        let mut feature2_builder = ash::vk::PhysicalDeviceFeatures2::builder();
        unsafe {
            for (_, feature_union) in &mut self.enabled_features2 {
                // not safe, we assume the layout of p_next is the same.
                let next = feature_union.get_mut::<ash::vk::PhysicalDeviceVulkan11Features>();
                feature2_builder = feature2_builder.push_next(next);
            }
        }
        let feature2 = feature2_builder.features(self.vk_enabled_feature1).build();

        // SILENCE VUID-VkDeviceCreateInfo-pNext-00373
        device_create_info.p_next =
            &feature2 as *const ash::vk::PhysicalDeviceFeatures2 as *const std::ffi::c_void;

        let queue_create_infos: Vec<ash::vk::DeviceQueueCreateInfo> = self
            .device_queue_create_infos
            .iter()
            .map(|q| q.ash_builder().build())
            .collect();
        let extensions: Vec<*const i8> = self
            .vk_enabled_extensions
            .iter()
            .map(|extension| extension.as_ptr())
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

        // collect feature2
        let mut enabled_features: FxHashSet<TypeId> = self
            .enabled_features2
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        // collect feature1
        for id in self.enabled_features1 {
            enabled_features.insert(id);
        }

        let device = Arc::new(Device {
            physical_device: self.physical_device,
            ash_device: vk_device,
            enabled_extensions: self.enabled_extensions,
            enabled_features,
            #[cfg(feature = "max_memory_allocation_count_check")]
            allocations: AtomicU32::default(),
        });

        let mut queues_map = FxHashMap::default();
        self.device_queue_create_infos.into_iter().for_each(|q| {
            let queue_family = q.queue_family;
            let queues = q
                .priorities
                .iter()
                .enumerate()
                .map(|(index, priority)| unsafe {
                    // Host Synchronization: none
                    let queue = device
                        .ash_device
                        .get_device_queue(queue_family.index, index as u32);
                    Queue {
                        device: device.clone(),
                        vk_queue: queue,
                        queue_family_property: queue_family.clone(),
                        priority: *priority,
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
    pub(crate) enabled_extensions: FxHashSet<TypeId>,
    pub(crate) enabled_features: FxHashSet<TypeId>,
    #[cfg(feature = "max_memory_allocation_count_check")]
    pub(crate) allocations: AtomicU32,
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.ash_device.handle() == other.ash_device.handle()
    }
}

impl Device {
    pub fn builder(physical_device: &Arc<PhysicalDevice>) -> DeviceBuilder {
        DeviceBuilder {
            physical_device: physical_device.clone(),
            device_queue_create_infos: vec![],
            vk_enabled_feature1: Default::default(),
            enabled_features1: Default::default(),
            enabled_features2: Default::default(),
            vk_enabled_extensions: Default::default(),
            enabled_extensions: Default::default(),
        }
    }
    pub fn get_extension<EXT: DeviceExtension + 'static>(self: &Arc<Self>) -> Option<EXT> {
        if self.enabled_extensions.contains(&TypeId::of::<EXT>()) {
            Some(EXT::new(self))
        } else {
            None
        }
    }
    pub fn get_feature<FT: Feature + 'static>(self: &Arc<Self>) -> Option<FT> {
        if self.enabled_features.contains(&TypeId::of::<FT>()) {
            Some(FT::new(self))
        } else {
            None
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
