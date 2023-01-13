use crate::device::Device;
// use crate::device_features::Feature;
// use crate::device_features::PhysicalDeviceVulkan11Features::ProtectedMemory;
use crate::physical_device::queue_family_properties::QueueFamilyProperties;
use std::sync::Arc;

pub enum CommandPoolCreateFlags {
    TRANSIENT,
    /* yarvk do not reset buffer, either reset pool, or a const command buffer */
    // ResetCommandBuffer,
    // DONE VUID-VkCommandPoolCreateInfo-flags-02860
    // PROTECTED(Feature<{ ProtectedMemory.into() }>),
}

impl CommandPoolCreateFlags {
    fn to_ash(&self) -> ash::vk::CommandPoolCreateFlags {
        match self {
            CommandPoolCreateFlags::TRANSIENT => ash::vk::CommandPoolCreateFlags::TRANSIENT,
            // CommandPoolCreateFlags::ResetCommandBuffer => {
            //     ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
            // }
            // CommandPoolCreateFlags::PROTECTED(_) => ash::vk::CommandPoolCreateFlags::PROTECTED,
        }
    }
}

pub struct CommandPool {
    pub device: Arc<Device>,
    pub(crate) vk_command_pool: ash::vk::CommandPool,
}

impl CommandPool {
    pub fn builder(
        device: &Arc<Device>,
        queue_family: QueueFamilyProperties,
    ) -> CommandPoolBuilder {
        CommandPoolBuilder {
            device: device.clone(),
            flags: Default::default(),
            queue_family_index: queue_family,
        }
    }
    pub fn reset(&mut self) -> Result<(), ash::vk::Result> {
        unsafe {
            // Host Synchronization: commandPool
            self.device.ash_device.reset_command_pool(
                self.vk_command_pool,
                ash::vk::CommandPoolResetFlags::RELEASE_RESOURCES,
            )?
        }
        Ok(())
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe {
            // DONE VUID-vkDestroyCommandPool-commandPool-00041
            // do not allow command pool destroy when command buffer exists
            // TODO VUID-vkDestroyCommandPool-commandPool-00042
            // TODO VUID-vkDestroyCommandPool-commandPool-00043
            // Host Synchronization: commandPool
            self.device
                .ash_device
                .destroy_command_pool(self.vk_command_pool, None);
        }
    }
}

pub struct CommandPoolBuilder {
    device: Arc<Device>,
    flags: ash::vk::CommandPoolCreateFlags,
    queue_family_index: QueueFamilyProperties,
}

impl CommandPoolBuilder {
    pub fn add_flag(mut self, flag: CommandPoolCreateFlags) -> Self {
        self.flags |= flag.to_ash();
        self
    }

    pub fn build(self) -> Result<CommandPool, ash::vk::Result> {
        let pool_create_info = ash::vk::CommandPoolCreateInfo::builder()
            .queue_family_index(self.queue_family_index.index)
            .flags(self.flags)
            .build();
        let vk_command_pool = unsafe {
            self.device
                .ash_device
                .create_command_pool(&pool_create_info, None)?
        };

        Ok(CommandPool {
            device: self.device,
            vk_command_pool,
        })
    }
}
