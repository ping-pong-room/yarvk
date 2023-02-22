use crate::buffer::IBuffer;
use crate::command::command_buffer::Level::{PRIMARY, SECONDARY};
use crate::command::command_buffer::RenderPassScope::{INSIDE, OUTSIDE};
use crate::command::command_buffer::State::{EXECUTABLE, INITIAL, INVALID, RECORDING};
use crate::command::command_pool::{CommandPool, CommandPoolCreateFlags};
use crate::descriptor_set::descriptor_set::IDescriptorSet;
use crate::device::Device;
use crate::frame_buffer::Framebuffer;
use crate::physical_device::queue_family_properties::QueueFamilyProperties;
use crate::pipeline::{Pipeline, PipelineLayout};
use crate::render_pass::RenderPass;
use ash::vk::Handle;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rustc_hash::FxHashMap;
use std::sync::Arc;
use crate::IImage;

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
    subpass: Option<u32>,
    framebuffer: Option<Arc<Framebuffer>>,
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
    subpass: Option<u32>,
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
    pub fn subpass(mut self, subpass: u32) -> Self {
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
    pub fn pipeline_statistics(
        mut self,
        pipeline_statistics: ash::vk::QueryPipelineStatisticFlags,
    ) -> Self {
        self.pipeline_statistics = pipeline_statistics;
        self
    }
    pub fn build(self) -> Arc<CommandBufferInheritanceInfo> {
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
                info.ash_vk_info.subpass = *subpass_index;
            }
            if let Some(framebuffer) = &info.framebuffer {
                info.ash_vk_info.framebuffer = framebuffer.ash_vk_framebuffer;
            }
            arc_info
        }
    }
}

#[derive(Default)]
pub(crate) struct HoldingResources {
    pub write_images: FxHashMap<u64, Arc<IImage>>,
    pub write_buffers: FxHashMap<u64, Arc<IBuffer>>,
    pub read_images: FxHashMap<u64, Arc<IImage>>,
    pub read_buffers: FxHashMap<u64, Arc<IBuffer>>,
    pub pipeline_layouts: FxHashMap<u64, Arc<PipelineLayout>>,
    pub pipelines: FxHashMap<u64, Arc<Pipeline>>,
    pub descriptor_sets: FxHashMap<u64, Arc<dyn IDescriptorSet>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub render_pass: Vec<Arc<RenderPass>>,
}

impl HoldingResources {
    fn clear(&mut self) {
        self.write_images.clear();
        self.read_images.clear();
        self.write_buffers.clear();
        self.read_buffers.clear();
        self.pipeline_layouts.clear();
        self.pipelines.clear();
        self.descriptor_sets.clear();
        self.render_pass.clear();
        self.framebuffers.clear();
    }
}

pub struct CommandBuffer<const LEVEL: Level, const STATE: State, const SCOPE: RenderPassScope> {
    pub device: Arc<Device>,
    pub(crate) command_pool: CommandPool,
    pub(crate) vk_command_buffer: ash::vk::CommandBuffer,
    pub(crate) inheritance_info: Arc<CommandBufferInheritanceInfo>,
    pub(crate) holding_resources: HoldingResources,
    pub(crate) secondary_buffers: Vec<CommandBuffer<{ SECONDARY }, STATE, { OUTSIDE }>>,
}

pub fn par_for_each<
    const S1: Level,
    const S2: State,
    const S3: RenderPassScope,
    const T1: Level,
    const T2: State,
    const T3: RenderPassScope,
>(
    source: Vec<CommandBuffer<S1, S2, S3>>,
    f: impl Fn(CommandBuffer<S1, S2, S3>) -> Result<CommandBuffer<T1, T2, T3>, ash::vk::Result> + Sync,
) -> Result<Vec<CommandBuffer<T1, T2, T3>>, ash::vk::Result> {
    unsafe {
        let mut buffers: Vec<CommandBuffer<T1, T2, T3>> = std::mem::transmute(source);
        let results = buffers
            .par_iter_mut()
            .map(|foo| {
                let temp = std::ptr::read(foo);
                let temp: CommandBuffer<S1, S2, S3> = std::mem::transmute(temp);
                match f(temp) {
                    Ok(temp) => {
                        std::ptr::write(foo, temp);
                        ash::vk::Result::SUCCESS
                    }
                    Err(err) => err,
                }
            })
            .collect::<Vec<ash::vk::Result>>();
        for result in &results {
            if *result != ash::vk::Result::SUCCESS {
                return Err(*result);
            }
        }
        Ok(buffers)
    }
}

impl<const LEVEL: Level, const STATE: State, const SCOPE: RenderPassScope> crate::Handle
    for CommandBuffer<LEVEL, STATE, SCOPE>
{
    fn handle(&self) -> u64 {
        self.vk_command_buffer.as_raw()
    }
}

macro_rules! reset_impls {
    ($($stage: expr),*) => {$(
        impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { $stage }, SCOPE> {
            pub fn reset(mut self) -> Result<CommandBuffer<LEVEL, { INITIAL }, SCOPE>, ash::vk::Result> {
                self.holding_resources.clear();
                self.command_pool.reset()?;
                self.secondary_buffers.clear();
                Ok(unsafe { std::mem::transmute(self) })
            }
        }
    )*};
}

reset_impls!(INITIAL, RECORDING, EXECUTABLE, INVALID);

impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { RECORDING }, SCOPE> {
    pub fn end(self) -> Result<CommandBuffer<LEVEL, { EXECUTABLE }, { OUTSIDE }>, ash::vk::Result> {
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
    pub fn begin(
        mut self,
    ) -> Result<CommandBuffer<{ PRIMARY }, { RECORDING }, { OUTSIDE }>, ash::vk::Result> {
        self.holding_resources.clear();
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00049
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00050
        // VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT is forced to be set in yarvk
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00051
        // Host Synchronization:commandBuffer, VkCommandPool
        let begin_info = ash::vk::CommandBufferBeginInfo::builder()
            .flags(ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
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
            pub fn record(self, f: impl FnOnce(&mut CommandBuffer<{ PRIMARY }, { RECORDING }, { OUTSIDE }>) -> Result<(), ash::vk::Result>)
                             -> Result<CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>, ash::vk::Result>
            {
                let mut recording_buffer = self.begin()?;
                f(&mut recording_buffer)?;
                recording_buffer.end()
            }
        }
    )*};
}

primary_record_impls!(INITIAL, EXECUTABLE, INVALID);

impl<const STATE: State> CommandBuffer<{ SECONDARY }, STATE, { OUTSIDE }> {
    pub fn begin<const SCOPE: RenderPassScope>(
        mut self,
        inheritance_info: Arc<CommandBufferInheritanceInfo>,
    ) -> Result<CommandBuffer<{ SECONDARY }, { RECORDING }, SCOPE>, ash::vk::Result> {
        self.holding_resources.clear();
        self.inheritance_info = inheritance_info;
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00049
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00050
        // VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT is forced to be set in yarvk
        // DONE VUID-vkBeginCommandBuffer-commandBuffer-00051
        // Host Synchronization:commandBuffer, VkCommandPool
        let mut flags = ash::vk::CommandBufferUsageFlags::default();
        flags |= ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT;
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
            pub fn record(self,
                            inheritance_info: Arc<CommandBufferInheritanceInfo>, f: impl FnOnce(&mut CommandBuffer<{ SECONDARY }, { RECORDING },  { OUTSIDE }>) -> Result<(), ash::vk::Result>)
                             -> Result<CommandBuffer<{ SECONDARY }, { EXECUTABLE },  { OUTSIDE }>, ash::vk::Result>
            {
                let mut recording_buffer = self.begin::<{ OUTSIDE }>(inheritance_info)?;
                f(&mut recording_buffer)?;
                recording_buffer.end()
            }
            pub fn record_render_pass_continue(self,
                            inheritance_info: Arc<CommandBufferInheritanceInfo>, f: impl FnOnce(&mut CommandBuffer<{ SECONDARY }, { RECORDING },  { INSIDE }>) -> Result<(), ash::vk::Result>)
                             -> Result<CommandBuffer<{ SECONDARY }, { EXECUTABLE },  { OUTSIDE }>, ash::vk::Result>
            {
                let mut recording_buffer = self.begin::<{ INSIDE }>(inheritance_info)?;
                f(&mut recording_buffer)?;
                recording_buffer.end()
            }
            pub fn record_buffers(
                buffers: Vec<Self>,
                inheritance_info: Arc<CommandBufferInheritanceInfo>,
                f: impl FnOnce(
                    &mut [CommandBuffer<{ SECONDARY }, { RECORDING }, { OUTSIDE }>],
                ) -> Result<(), ash::vk::Result>,
            ) -> Result<
                Vec<CommandBuffer<{ SECONDARY }, { EXECUTABLE }, { OUTSIDE }>>,
                ash::vk::Result,
            > {
                let mut buffers = par_for_each(buffers, |buffer| buffer.begin::<{ OUTSIDE }>(inheritance_info.clone()))?;
                f(buffers.as_mut_slice())?;
                let buffers = par_for_each(buffers, |buffer| buffer.end())?;
                Ok(buffers)
            }
            pub fn record_render_pass_continue_buffers(
                buffers: Vec<Self>,
                inheritance_info: Arc<CommandBufferInheritanceInfo>,
                f: impl FnOnce(
                    &mut [CommandBuffer<{ SECONDARY }, { RECORDING }, { INSIDE }>],
                ) -> Result<(), ash::vk::Result>,
            ) -> Result<
                Vec<CommandBuffer<{ SECONDARY }, { EXECUTABLE }, { OUTSIDE }>>,
                ash::vk::Result,
            > {
                let mut buffers = par_for_each(buffers, |buffer| buffer.begin::<{ INSIDE }>(inheritance_info.clone()))?;
                f(buffers.as_mut_slice())?;
                let buffers = par_for_each(buffers, |buffer| buffer.end())?;
                Ok(buffers)
            }
        }
    )*};
}

secondary_record_impls!(INITIAL, EXECUTABLE, INVALID);

impl<const SCOPE: RenderPassScope> CommandBuffer<{ PRIMARY }, { INVALID }, SCOPE> {
    pub fn secondary_buffers(
        &mut self,
    ) -> &mut Vec<CommandBuffer<{ SECONDARY }, { INVALID }, { OUTSIDE }>> {
        &mut self.secondary_buffers
    }
}
impl<const SCOPE: RenderPassScope> CommandBuffer<{ PRIMARY }, { INITIAL }, SCOPE> {
    // use this only in the first loop of rendering
    pub fn add_secondary_buffer(
        &mut self,
        buffer: CommandBuffer<{ SECONDARY }, { INITIAL }, { OUTSIDE }>,
    ) {
        self.secondary_buffers
            .push(unsafe { std::mem::transmute(buffer) });
    }
}

impl<const SCOPE: RenderPassScope> CommandBuffer<{ PRIMARY }, { RECORDING }, SCOPE> {
    pub fn cmd_execute_commands(
        &mut self,
        secondary_command_buffers: &mut Vec<
            CommandBuffer<{ SECONDARY }, { EXECUTABLE }, { OUTSIDE }>,
        >,
    ) {
        let mut vk_buffers = Vec::with_capacity(secondary_command_buffers.len());
        while !secondary_command_buffers.is_empty() {
            let buffer = secondary_command_buffers.pop().unwrap();
            vk_buffers.push(buffer.vk_command_buffer);
            let buffer = unsafe { std::mem::transmute(buffer) };
            self.secondary_buffers.push(buffer);
        }

        // Host Synchronization: commandBuffer, VkCommandPool
        unsafe {
            self.device
                .ash_device
                .cmd_execute_commands(self.vk_command_buffer, vk_buffers.as_slice());
        }
    }
}

pub struct TransientCommandBuffer<const LEVEL: Level> {}
impl<const LEVEL: Level> TransientCommandBuffer<LEVEL> {
    pub fn new(
        device: &Arc<Device>,
        queue_family: QueueFamilyProperties,
    ) -> Result<CommandBuffer<LEVEL, { INITIAL }, { OUTSIDE }>, ash::vk::Result> {
        let pool = CommandPool::builder(device, queue_family.clone())
            .add_flag(CommandPoolCreateFlags::TRANSIENT)
            .build()?;

        let create_info = ash::vk::CommandBufferAllocateInfo::builder()
            .command_pool(pool.vk_command_pool)
            .level(LEVEL.to_ash())
            .command_buffer_count(1)
            .build();
        let mut vk_buffers = unsafe {
            // Host Synchronization: pAllocateInfo->commandPool
            device.ash_device.allocate_command_buffers(&create_info)?
        };

        let vk_command_buffer = vk_buffers.pop().unwrap();
        Ok(CommandBuffer {
            device: device.clone(),
            command_pool: pool,
            vk_command_buffer,
            inheritance_info: Arc::new(CommandBufferInheritanceInfo::default()),
            holding_resources: Default::default(),
            secondary_buffers: Default::default(),
        })
    }
}
