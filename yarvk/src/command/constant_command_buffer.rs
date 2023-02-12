use crate::command::command_buffer::Level::PRIMARY;
use crate::command::command_buffer::RenderPassScope::OUTSIDE;
use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, HoldingResources};
use crate::command::command_pool::CommandPool;
use crate::device::Device;
use crate::physical_device::queue_family_properties::QueueFamilyProperties;
use std::sync::Arc;

pub struct ConstantCommandBuffer {
    pub device: Arc<Device>,
    _command_pool: Arc<CommandPool>,
    pub(crate) vk_command_buffer: ash::vk::CommandBuffer,
    _holding_resources: HoldingResources,
}

impl ConstantCommandBuffer {
    pub fn new<
        It: IntoIterator<
            Item = Box<
                dyn FnOnce(
                    &mut CommandBuffer<{ PRIMARY }, { RECORDING }, { OUTSIDE }>,
                ) -> Result<(), ash::vk::Result>,
            >,
        >,
    >(
        device: &Arc<Device>,
        queue_family: QueueFamilyProperties,
        records: It,
    ) -> Result<Vec<ConstantCommandBuffer>, ash::vk::Result> {
        let mut records = records.into_iter();
        let pool = CommandPool::builder(device, queue_family).build()?;
        let size = records.size_hint().0;
        let mut buffers = Vec::with_capacity(size);
        let mut current_vk_buffers = Vec::new();
        let mut pool_holder = Some(pool);
        while let Some(record) = records.next() {
            if current_vk_buffers.is_empty() {
                let pool = pool_holder.take().unwrap();
                let size = records.size_hint().0 + 1;
                let create_info = ash::vk::CommandBufferAllocateInfo::builder()
                    .command_pool(pool.vk_command_pool)
                    .level(ash::vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(size as _)
                    .build();
                current_vk_buffers = unsafe {
                    // Host Synchronization: pAllocateInfo->commandPool
                    device.ash_device.allocate_command_buffers(&create_info)?
                };
                pool_holder = Some(pool);
            }
            let vk_command_buffer = current_vk_buffers.pop().unwrap();
            let pool = pool_holder.take().unwrap();
            let holding_resources = HoldingResources::default();
            // begin
            // DONE VUID-vkBeginCommandBuffer-commandBuffer-00049
            // DONE VUID-vkBeginCommandBuffer-commandBuffer-00050
            // DONE VUID-vkBeginCommandBuffer-commandBuffer-00051
            // Host Synchronization:commandBuffer, VkCommandPool
            let begin_info = ash::vk::CommandBufferBeginInfo::builder()
                .flags(ash::vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)
                .build();
            unsafe {
                device
                    .ash_device
                    .begin_command_buffer(vk_command_buffer, &begin_info)?;
            }
            let mut buffer: CommandBuffer<{ PRIMARY }, { RECORDING }, { OUTSIDE }> =
                CommandBuffer {
                    device: device.clone(),
                    command_pool: pool,
                    vk_command_buffer,
                    inheritance_info: Arc::new(Default::default()),
                    holding_resources,
                    secondary_buffers: vec![],
                };

            record(&mut buffer)?;
            // end
            // Host Synchronization:commandBuffer, VkCommandPool
            unsafe {
                device.ash_device.end_command_buffer(vk_command_buffer)?;
            }

            pool_holder = Some(buffer.command_pool);
            buffers.push((buffer.vk_command_buffer, buffer.holding_resources));
        }
        let pool = Arc::new(pool_holder.unwrap());
        Ok(buffers
            .into_iter()
            .map(
                |(vk_command_buffer, holding_resources)| ConstantCommandBuffer {
                    device: device.clone(),
                    _command_pool: pool.clone(),
                    vk_command_buffer,
                    _holding_resources: holding_resources,
                },
            )
            .collect())
    }
}
