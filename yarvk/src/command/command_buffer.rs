use crate::buffer::Buffer;
use crate::command::command_buffer::Level::{PRIMARY, SECONDARY};
use crate::command::command_buffer::RenderPassScope::{INSIDE, OUTSIDE};
use crate::command::command_buffer::State::{EXECUTABLE, INITIAL, INVALID, RECORDING};
use crate::command::command_pool::CommandPool;
use crate::device::Device;
use crate::frame_buffer::Framebuffer;
use crate::image::Image;

use crate::render_pass::subpass::SubpassIndex;
use crate::render_pass::RenderPass;

use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

use crate::Handler;
use ash::vk::Handle;
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
    // pending buffers' are owned by yarvk structs, user cannot get a buffer which type is pending.
    // PENDING,
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
    pub fn render_pass(mut self, render_pass: Arc<RenderPass>) -> Self {
        self.render_pass = Some(render_pass);
        self
    }
    pub fn subpass(mut self, subpass: SubpassIndex) -> Self {
        self.subpass = Some(subpass);
        self
    }
    pub fn framebuffer(mut self, framebuffer: Arc<Framebuffer>) -> Self {
        self.framebuffer = Some(framebuffer);
        self
    }
    pub fn occlusion_query_enable(mut self, enable: bool) -> Self {
        self.occlusion_query_enable = enable;
        self
    }
    pub fn query_flags(mut self, query_flags: ash::vk::QueryControlFlags) -> Self {
        self.query_flags = query_flags;
        self
    }
    pub fn pipeline_statistics(mut self, pipeline_statistics: ash::vk::QueryPipelineStatisticFlags) -> Self {
        self.pipeline_statistics = pipeline_statistics;
        self
    }
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
    pub(crate) command_pool: CommandPool,
    pub(crate) vk_command_buffer: ash::vk::CommandBuffer,
    inheritance_info: Pin<Arc<CommandBufferInheritanceInfo>>,
    pub(crate) holding_resources: HoldingResources,
    pub(crate) one_time_submit: bool,
    pub(crate) secondary_buffers:
        FxHashMap<u64, CommandBuffer<{ SECONDARY }, { EXECUTABLE }, { OUTSIDE }>>,
}

impl<const LEVEL: Level, const STATE: State, const SCOPE: RenderPassScope> Handler
    for CommandBuffer<LEVEL, STATE, SCOPE>
{
    fn handler(&self) -> u64 {
        self.vk_command_buffer.as_raw()
    }
}

impl<const LEVEL: Level, const STATE: State, const SCOPE: RenderPassScope> Drop
    for CommandBuffer<LEVEL, STATE, SCOPE>
{
    fn drop(&mut self) {
        // DONE VUID-vkFreeCommandBuffers-pCommandBuffers-00048
        // TODO VUID-vkFreeCommandBuffers-pCommandBuffers-00047
        // Host Synchronization: commandPool, each member of pCommandBuffers
        unsafe {
            self.device
                .ash_device
                .free_command_buffers(self.command_pool.vk_command_pool, &[self.vk_command_buffer]);
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
                unsafe {
                    self.device.ash_device.reset_command_buffer(self.vk_command_buffer, ash::vk::CommandBufferResetFlags::RELEASE_RESOURCES)?
                }
                Ok(unsafe { std::mem::transmute(self) })
            }
        }
    )*};
}

reset_impls!(INITIAL, RECORDING, EXECUTABLE, INVALID);

impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { RECORDING }, SCOPE> {
    fn end(self) -> Result<CommandBuffer<LEVEL, { EXECUTABLE }, SCOPE>, ash::vk::Result> {
        // Host Synchronization:commandBuffer, VkCommandPool
        unsafe {
            self.device
                .ash_device
                .end_command_buffer(self.vk_command_buffer)?;
        }
        Ok(unsafe { std::mem::transmute(self) })
    }
}

impl<const STATE: State> CommandBuffer<{ PRIMARY }, STATE, { OUTSIDE }> {
    // DONE VUID-vkBeginCommandBuffer-commandBuffer-02840
    fn begin(
        mut self,
        one_time_submit: bool,
    ) -> Result<CommandBuffer<{ PRIMARY }, { RECORDING }, { OUTSIDE }>, ash::vk::Result> {
        self.holding_resources.clear();
        self.one_time_submit = one_time_submit;
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00049
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00050
        // VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT is forced to be set in yarvk
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00051
        // Host Synchronization:commandBuffer, VkCommandPool
        let flags = if one_time_submit {
            ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT
        } else {
            ash::vk::CommandBufferUsageFlags::default()
        };
        let begin_info = ash::vk::CommandBufferBeginInfo::builder()
            .flags(flags)
            .build();
        unsafe {
            self.device
                .ash_device
                .begin_command_buffer(self.vk_command_buffer, &begin_info)?;
        }
        Ok(unsafe { std::mem::transmute(self) })
    }
}

macro_rules! primary_record_impls {
    ($($stage: expr),*) => {$(
        impl CommandBuffer<{ PRIMARY }, { $stage }, { OUTSIDE }> {
            pub fn record<F>(self, one_time_submit: bool, f: F)
                             -> Result<CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>, ash::vk::Result>
                where F: FnOnce(&mut CommandBuffer<{ PRIMARY }, { RECORDING }, { OUTSIDE }>) {
                let mut recording_buffer = self.begin(one_time_submit)?;
                f(&mut recording_buffer);
                recording_buffer.end()
            }
        }
    )*};
}

primary_record_impls!(INITIAL, EXECUTABLE, INVALID);

impl<const STATE: State> CommandBuffer<{ SECONDARY }, STATE, { OUTSIDE }> {
    fn begin<const SCOPE: RenderPassScope>(
        mut self,
        one_time_submit: bool,
        inheritance_info: Pin<Arc<CommandBufferInheritanceInfo>>,
    ) -> Result<CommandBuffer<{ SECONDARY }, { RECORDING }, SCOPE>, ash::vk::Result> {
        self.holding_resources.clear();
        self.one_time_submit = one_time_submit;
        self.inheritance_info = inheritance_info;
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00049
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00050
        // VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT is forced to be set in yarvk
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00051
        // Host Synchronization:commandBuffer, VkCommandPool
        let mut flags = ash::vk::CommandBufferUsageFlags::default();
        if one_time_submit {
            flags |= ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT;
        }
        if SCOPE == INSIDE {
            flags |= ash::vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE;
        };
        let begin_info = ash::vk::CommandBufferBeginInfo::builder()
            .flags(flags)
            .inheritance_info(&self.inheritance_info.ash_vk_info)
            .build();
        unsafe {
            self.device
                .ash_device
                .begin_command_buffer(self.vk_command_buffer, &begin_info)?;
        }
        Ok(unsafe { std::mem::transmute(self) })
    }
}

macro_rules! secondary_record_impls {
    ($($stage: expr),*) => {$(
        impl CommandBuffer<{ SECONDARY }, { $stage },  { OUTSIDE }> {
            pub fn record<F>(self, one_time_submit: bool,
                            inheritance_info: Pin<Arc<CommandBufferInheritanceInfo>>, f: F)
                             -> Result<CommandBuffer<{ SECONDARY }, { EXECUTABLE },  { OUTSIDE }>, ash::vk::Result>
                where F: FnOnce(&mut CommandBuffer<{ SECONDARY }, { RECORDING },  { OUTSIDE }>) {
                let mut recording_buffer = self.begin::<{ OUTSIDE }>(one_time_submit, inheritance_info)?;
                f(&mut recording_buffer);
                recording_buffer.end()
            }
            pub fn record_render_pass_continue<F>(self, one_time_submit: bool,
                            inheritance_info: Pin<Arc<CommandBufferInheritanceInfo>>, f: F)
                             -> Result<CommandBuffer<{ SECONDARY }, { EXECUTABLE },  { INSIDE }>, ash::vk::Result>
                where F: FnOnce(&mut CommandBuffer<{ SECONDARY }, { RECORDING },  { INSIDE }>) {
                let mut recording_buffer = self.begin::<{ INSIDE }>(one_time_submit, inheritance_info)?;
                f(&mut recording_buffer);
                recording_buffer.end()
            }
        }
    )*};
}

secondary_record_impls!(INITIAL, EXECUTABLE, INVALID);

impl CommandPool {
    // yarvk use one pool per command buffer
    // Why: all vendors suggest that create Use L * T + N pools.
    // (L = the number of buffered frames, T = the number of threads that record command buffers,
    // N = extra pools for secondary command buffers),
    // this leads to a fact that it's really rare to see two command buffers shared the same pool.
    // Make the command buffer owns the pool will make the host synchronization easier.
    pub fn allocate_command_buffer<const LEVEL: Level>(
        self: Self,
    ) -> Result<CommandBuffer<LEVEL, { INITIAL }, { OUTSIDE }>, ash::vk::Result> {
        let create_info = ash::vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.vk_command_pool)
            .level(LEVEL.to_ash())
            .command_buffer_count(1)
            .build();
        let mut vk_buffers = unsafe {
            // Host Synchronization: pAllocateInfo->commandPool
            self.device
                .ash_device
                .allocate_command_buffers(&create_info)?
        };

        let vk_command_buffer = vk_buffers.pop().unwrap();
        Ok(CommandBuffer {
            device: self.device.clone(),
            command_pool: self,
            vk_command_buffer,
            inheritance_info: DEFAULT_INHERITANCE_INFO.clone(),
            holding_resources: Default::default(),
            one_time_submit: false,
            secondary_buffers: Default::default(),
        })
    }
}

impl<const SCOPE: RenderPassScope> CommandBuffer<{ PRIMARY }, { RECORDING }, SCOPE> {
    pub fn cmd_execute_commands(
        &mut self,
        secondary_command_buffers: &mut Vec<CommandBuffer<{ SECONDARY }, { EXECUTABLE }, SCOPE>>,
    ) {
        let mut vk_buffers = Vec::with_capacity(secondary_command_buffers.len());
        while !secondary_command_buffers.is_empty() {
            let buffer = secondary_command_buffers.pop().unwrap();
            vk_buffers.push(buffer.vk_command_buffer);
            let handler = buffer.vk_command_buffer.as_raw();
            let buffer = unsafe { std::mem::transmute(buffer) };
            self.secondary_buffers.insert(handler, buffer);
        }

        // Host Synchronization: commandBuffer, VkCommandPool
        unsafe {
            self.device
                .ash_device
                .cmd_execute_commands(self.vk_command_buffer, vk_buffers.as_slice());
        }
    }
}
