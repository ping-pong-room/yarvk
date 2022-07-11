

use crate::buffer::Buffer;
use crate::command::command_buffer::Level::{PRIMARY, SECONDARY};
use crate::command::command_buffer::RenderPassScope::OUTSIDE;
use crate::command::command_buffer::State::{EXECUTABLE, INITIAL, INVALID, PENDING, RECORDING};
use crate::command::command_pool::CommandPool;
use crate::device::Device;
use crate::frame_buffer::Framebuffer;
use crate::image::Image;


use crate::render_pass::subpass::SubpassIndex;
use crate::render_pass::RenderPass;

use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::Arc;
lazy_static! {
    pub static ref DEFAULT_INHERITANCE_INFO: Pin<Arc<CommandBufferInheritanceInfo>> =
        CommandBufferInheritanceInfo::builder().build();
}

#[derive(PartialEq, Eq)]
pub enum State {
    INITIAL,
    RECORDING,
    EXECUTABLE,
    PENDING,
    INVALID,
}

#[derive(PartialEq, Eq)]
pub enum Level {
    PRIMARY,
    SECONDARY,
}

#[derive(PartialEq, Eq)]
pub enum RenderPassScope {
    INSIDE,
    OUTSIDE,
}

impl Level {
    pub(crate) fn to_ash(&self) -> ash::vk::CommandBufferLevel {
        match self {
            PRIMARY => ash::vk::CommandBufferLevel::PRIMARY,
            SECONDARY => ash::vk::CommandBufferLevel::SECONDARY,
        }
    }
}

// TODO Add p_next fields
#[derive(Default)]
pub struct CommandBufferInheritanceInfo {
    ash_vk_info: ash::vk::CommandBufferInheritanceInfo,
    render_pass: Option<Arc<RenderPass>>,
    subpass: Option<SubpassIndex>,
    framebuffer: Option<Arc<Framebuffer>>,
    _pin: PhantomPinned,
}

impl CommandBufferInheritanceInfo {
    pub fn builder() -> CommandBufferInheritanceInfoBuilder {
        CommandBufferInheritanceInfoBuilder::default()
    }
}

unsafe impl Sync for CommandBufferInheritanceInfo {}

unsafe impl Send for CommandBufferInheritanceInfo {}

#[derive(Default)]
pub struct CommandBufferInheritanceInfoBuilder {
    render_pass: Option<Arc<RenderPass>>,
    subpass: Option<SubpassIndex>,
    framebuffer: Option<Arc<Framebuffer>>,
    occlusion_query_enable: bool,
    query_flags: ash::vk::QueryControlFlags,
    pipeline_statistics: ash::vk::QueryPipelineStatisticFlags,
}

impl CommandBufferInheritanceInfoBuilder {
    pub fn build(self) -> Pin<Arc<CommandBufferInheritanceInfo>> {
        // TODO VUID-VkCommandBufferInheritanceInfo-occlusionQueryEnable-00056
        // TODO VUID-VkCommandBufferInheritanceInfo-queryFlags-00057
        // TODO VUID-VkCommandBufferInheritanceInfo-queryFlags-02788
        // TODO VUID-VkCommandBufferInheritanceInfo-pipelineStatistics-02789
        // TODO VUID-VkCommandBufferInheritanceInfo-pipelineStatistics-00058
        let mut arc_info = Arc::new(CommandBufferInheritanceInfo::default());
        unsafe {
            let mut info = Arc::get_mut(&mut arc_info).unwrap_unchecked();
            info.render_pass = self.render_pass;
            info.subpass = self.subpass;
            info.framebuffer = self.framebuffer;
            info.ash_vk_info.occlusion_query_enable = self.occlusion_query_enable as _;
            info.ash_vk_info.pipeline_statistics = self.pipeline_statistics;
            info.ash_vk_info.query_flags = self.query_flags;
            if let Some(render_pass) = &info.render_pass {
                info.ash_vk_info.render_pass = render_pass.ash_vk_renderpass;
            }
            if let Some(subpass_index) = &info.subpass {
                info.ash_vk_info.subpass = subpass_index.0;
            }
            if let Some(framebuffer) = &info.framebuffer {
                info.ash_vk_info.framebuffer = framebuffer.ash_vk_framebuffer;
            }
            Pin::new_unchecked(arc_info)
        }
    }
}

#[derive(Default)]
pub(crate) struct HoldingResources {
    pub write_images: FxHashMap<u64, Arc<Image>>,
    pub write_buffers: FxHashMap<u64, Arc<Buffer>>,
    pub read_images: FxHashMap<u64, Arc<Image>>,
    pub read_buffers: FxHashMap<u64, Arc<Buffer>>,
}

impl HoldingResources {
    fn clear(&mut self) {
        self.write_images.clear();
        self.read_images.clear();
        self.write_buffers.clear();
        self.read_buffers.clear();
    }
}

pub struct CommandBuffer<const LEVEL: Level, const STATE: State, const SCOPE: RenderPassScope> {
    pub device: Arc<Device>,
    pub(crate) command_pool: Arc<CommandPool>,
    pub(crate) vk_command_buffer: ash::vk::CommandBuffer,
    inheritance_info: Pin<Arc<CommandBufferInheritanceInfo>>,
    pub(crate) holding_resources: HoldingResources,
    pub(crate) one_time_submit: bool,
}

impl<const LEVEL: Level, const STATE: State, const SCOPE: RenderPassScope> Drop
    for CommandBuffer<LEVEL, STATE, SCOPE>
{
    fn drop(&mut self) {
        // DONE VUID-vkFreeCommandBuffers-pCommandBuffers-00048
        // MUST VUID-vkFreeCommandBuffers-pCommandBuffers-00047
        if STATE == PENDING {
            panic!("VUID-vkFreeCommandBuffers-pCommandBuffers-00047")
        }
        // Host Synchronization: commandPool, each member of pCommandBuffers
        unsafe {
            let vk_pool = self.command_pool.vk_command_pool.write();
            self.device
                .ash_device
                .free_command_buffers(*vk_pool, &[self.vk_command_buffer]);
        }
    }
}

macro_rules! reset_impls {
    ($($stage: expr),*) => {$(
        impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { $stage }, SCOPE> {
            pub fn reset(mut self) -> Result<CommandBuffer<LEVEL, { INITIAL }, SCOPE>, ash::vk::Result> {
                self.holding_resources.clear();
                // Host Synchronization: commandBuffer, VkCommandPool
                // DONE VUID-vkResetCommandBuffer-commandBuffer-00046
                // DONE VUID-vkResetCommandBuffer-commandBuffer-00045
                let pool = self.command_pool.vk_command_pool.write();
                unsafe {
                    self.device.ash_device.reset_command_buffer(self.vk_command_buffer, ash::vk::CommandBufferResetFlags::RELEASE_RESOURCES)?
                }
                drop(pool);
                Ok(unsafe { std::mem::transmute(self) })
            }
        }
    )*};
}

reset_impls!(INITIAL, RECORDING, EXECUTABLE, INVALID);

impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { RECORDING }, SCOPE> {
    fn end(self) -> Result<CommandBuffer<LEVEL, { EXECUTABLE }, SCOPE>, ash::vk::Result> {
        // Host Synchronization:commandBuffer, VkCommandPool
        let pool = self.command_pool.vk_command_pool.write();
        unsafe {
            self.device
                .ash_device
                .end_command_buffer(self.vk_command_buffer)?;
        }
        drop(pool);
        Ok(unsafe { std::mem::transmute(self) })
    }
}

impl<const STATE: State, const SCOPE: RenderPassScope> CommandBuffer<{ PRIMARY }, STATE, SCOPE> {
    fn begin(
        mut self,
        flags: ash::vk::CommandBufferUsageFlags,
    ) -> Result<CommandBuffer<{ PRIMARY }, { RECORDING }, SCOPE>, ash::vk::Result> {
        self.holding_resources.clear();
        self.one_time_submit = flags.contains(ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00049
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00050
        // VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT is forced to be set in yarvk
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00051
        // Host Synchronization:commandBuffer, VkCommandPool
        let begin_info = ash::vk::CommandBufferBeginInfo::builder()
            .flags(flags)
            .build();
        let pool = self.command_pool.vk_command_pool.write();
        unsafe {
            self.device
                .ash_device
                .begin_command_buffer(self.vk_command_buffer, &begin_info)?;
        }
        drop(pool);
        Ok(unsafe { std::mem::transmute(self) })
    }
}

macro_rules! primary_record_impls {
    ($($stage: expr),*) => {$(
        impl<const SCOPE: RenderPassScope> CommandBuffer<{ PRIMARY }, { $stage }, SCOPE> {
            pub fn record<F>(self, flags: ash::vk::CommandBufferUsageFlags, f: F)
                             -> Result<CommandBuffer<{ PRIMARY }, { EXECUTABLE }, SCOPE>, ash::vk::Result>
                where F: FnOnce(&mut CommandBuffer<{ PRIMARY }, { RECORDING }, SCOPE>) {
                let mut recording_buffer = self.begin(flags)?;
                f(&mut recording_buffer);
                recording_buffer.end()
            }
        }
    )*};
}

primary_record_impls!(INITIAL, EXECUTABLE, INVALID);

impl<const STATE: State, const SCOPE: RenderPassScope> CommandBuffer<{ SECONDARY }, STATE, SCOPE> {
    fn begin(
        mut self,
        flags: ash::vk::CommandBufferUsageFlags,
        inheritance_info: Pin<Arc<CommandBufferInheritanceInfo>>,
    ) -> Result<CommandBuffer<{ SECONDARY }, { RECORDING }, SCOPE>, ash::vk::Result> {
        self.holding_resources.clear();
        self.one_time_submit = flags.contains(ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        self.inheritance_info = inheritance_info;
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00049
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00050
        // VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT is forced to be set in yarvk
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00051
        // Host Synchronization:commandBuffer, VkCommandPool
        let begin_info = ash::vk::CommandBufferBeginInfo::builder()
            .flags(flags)
            .inheritance_info(&self.inheritance_info.ash_vk_info)
            .build();
        let pool = self.command_pool.vk_command_pool.write();
        unsafe {
            self.device
                .ash_device
                .begin_command_buffer(self.vk_command_buffer, &begin_info)?;
        }
        drop(pool);
        Ok(unsafe { std::mem::transmute(self) })
    }
}

macro_rules! secondary_record_impls {
    ($($stage: expr),*) => {$(
        impl<const SCOPE: RenderPassScope> CommandBuffer<{ SECONDARY }, { $stage }, SCOPE> {
            pub fn record<F>(self, flags: ash::vk::CommandBufferUsageFlags, inheritance_info: Pin<Arc<CommandBufferInheritanceInfo>>, f: F)
                             -> Result<CommandBuffer<{ SECONDARY }, { EXECUTABLE }, SCOPE>, ash::vk::Result>
                where F: FnOnce(&mut CommandBuffer<{ SECONDARY }, { RECORDING }, SCOPE>) {
                let mut recording_buffer = self.begin(flags, inheritance_info)?;
                f(&mut recording_buffer);
                recording_buffer.end()
            }
        }
    )*};
}

secondary_record_impls!(INITIAL, EXECUTABLE, INVALID);

impl CommandPool {
    pub fn allocate_command_buffers<const LEVEL: Level>(
        self: Arc<Self>,
        command_buffer_count: u32,
    ) -> Result<Vec<CommandBuffer<LEVEL, { INITIAL }, { OUTSIDE }>>, ash::vk::Result> {
        let vk_command_pool = self.vk_command_pool.write();
        let create_info = ash::vk::CommandBufferAllocateInfo::builder()
            .command_pool(*vk_command_pool)
            .level(LEVEL.to_ash())
            .command_buffer_count(command_buffer_count)
            .build();
        let vk_buffers = unsafe {
            // Host Synchronization: pAllocateInfo->commandPool
            self.device
                .ash_device
                .allocate_command_buffers(&create_info)?
        };

        let buffers = vk_buffers
            .into_iter()
            .map(|vk_command_buffer| CommandBuffer {
                device: self.device.clone(),
                command_pool: self.clone(),
                vk_command_buffer,
                inheritance_info: DEFAULT_INHERITANCE_INFO.clone(),
                holding_resources: Default::default(),
                one_time_submit: false
            })
            .collect();
        Ok(buffers)
    }
}
