use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::descriptor_set::descriptor_set_layout::IDescriptorSetLayout;
use crate::device::Device;
use crate::pipeline::color_blend_state::PipelineColorBlendStateCreateInfo;
use crate::pipeline::depth_stencil_state::PipelineDepthStencilStateCreateInfo;
use crate::pipeline::input_assembly_state::PipelineInputAssemblyStateCreateInfo;
use crate::pipeline::multisample_state::PipelineMultisampleStateCreateInfo;
use crate::pipeline::pipeline_cache::PipelineCacheImpl;
use crate::pipeline::rasterization_state::PipelineRasterizationStateCreateInfo;
use crate::pipeline::shader_stage::{PipelineShaderStageCreateInfo, ShaderStage};
use crate::pipeline::vertex_input_state::PipelineVertexInputStateCreateInfo;
use crate::pipeline::viewport_state::PipelineViewportStateCreateInfo;
use crate::render_pass::RenderPass;
use crate::shader_module::ShaderModule;
use ash::vk::Handle;
use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::Arc;

pub mod color_blend_state;
pub mod depth_stencil_state;
pub mod input_assembly_state;
pub mod multisample_state;
pub mod pipeline_cache;
pub mod pipeline_stage_flags;
pub mod primitive_topology;
pub mod rasterization_state;
pub mod shader_stage;
pub mod vertex_input_state;
pub mod viewport_state;

pub struct PipelineLayout {
    pub device: Arc<Device>,
    pub set_layouts: Vec<Arc<dyn IDescriptorSetLayout + Send + Sync>>,
    pub(crate) ash_vk_pipeline_layout: ash::vk::PipelineLayout,
}

impl PipelineLayout {
    pub fn builder(device: &Arc<Device>) -> PipelineLayoutBuilder {
        PipelineLayoutBuilder {
            device: device.clone(),
            set_layouts: vec![],
            push_constant_ranges: vec![],
        }
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe {
            // DONE VUID-vkDestroyPipelineLayout-pipelineLayout-02004
            // Host Synchronization: pipelineLayout
            self.device
                .ash_device
                .destroy_pipeline_layout(self.ash_vk_pipeline_layout, None);
        }
    }
}

pub struct PushConstantRange {
    inner: ash::vk::PushConstantRange,
}
impl PushConstantRange {
    pub fn builder() -> PushConstantRangeBuilder {
        PushConstantRangeBuilder::default()
    }
    fn to_ash(&self) -> ash::vk::PushConstantRange {
        self.inner
    }
}
#[derive(Default)]
pub struct PushConstantRangeBuilder {
    inner: ash::vk::PushConstantRange,
}
impl PushConstantRangeBuilder {
    pub fn add_stage(mut self, stage: ShaderStage) -> Self {
        self.inner.stage_flags |= stage.to_ash();
        self
    }
    pub fn offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: u32) -> Self {
        self.inner.size = size;
        self
    }
    pub fn build(self) -> PushConstantRange {
        PushConstantRange { inner: self.inner }
    }
}

pub struct PipelineLayoutBuilder {
    device: Arc<Device>,
    set_layouts: Vec<Arc<dyn IDescriptorSetLayout + Send + Sync>>,
    push_constant_ranges: Vec<ash::vk::PushConstantRange>,
}

impl PipelineLayoutBuilder {
    pub fn add_set_layout(
        mut self,
        set_layout: Arc<dyn IDescriptorSetLayout + Send + Sync>,
    ) -> Self {
        self.set_layouts.push(set_layout);
        self
    }
    pub fn add_push_constant_range(mut self, push_constant_range: PushConstantRange) -> Self {
        self.push_constant_ranges.push(push_constant_range.to_ash());
        self
    }
    pub fn build(self) -> Result<Arc<PipelineLayout>, ash::vk::Result> {
        let vk_set_layouts = self
            .set_layouts
            .iter()
            .map(|layout| layout.raw())
            .collect::<Vec<_>>();
        let create_info = ash::vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(vk_set_layouts.as_slice())
            .push_constant_ranges(self.push_constant_ranges.as_slice())
            .build();
        unsafe {
            // Host Synchronization: none
            let ash_vk_pipeline_layout = self
                .device
                .ash_device
                .create_pipeline_layout(&create_info, None)?;
            Ok(Arc::new(PipelineLayout {
                device: self.device,
                set_layouts: self.set_layouts,
                ash_vk_pipeline_layout,
            }))
        }
    }
}

#[derive(Default)]
pub struct PipelineTessellationStateCreateInfo {
    patch_control_points: u32,
}

impl PipelineTessellationStateCreateInfo {
    pub fn builder() -> PipelineTessellationStateCreateInfoBuilder {
        PipelineTessellationStateCreateInfoBuilder::default()
    }
    fn ash_builder(&self) -> ash::vk::PipelineTessellationStateCreateInfoBuilder {
        ash::vk::PipelineTessellationStateCreateInfo::builder()
            .patch_control_points(self.patch_control_points)
    }
}

#[derive(Default)]
pub struct PipelineTessellationStateCreateInfoBuilder {
    inner: PipelineTessellationStateCreateInfo,
}

impl PipelineTessellationStateCreateInfoBuilder {
    pub fn patch_control_points(mut self, patch_control_points: u32) -> Self {
        self.inner.patch_control_points = patch_control_points;
        self
    }
    pub fn build(self) -> PipelineTessellationStateCreateInfo {
        self.inner
    }
}

pub struct Pipeline {
    pub device: Arc<Device>,
    pub pipeline_layout: Arc<PipelineLayout>,
    _render_pass_holder: Option<Arc<RenderPass>>,
    _shader_modules_holder: Vec<Arc<ShaderModule>>,
    ash_vk_pipeline: ash::vk::Pipeline,
}

impl Pipeline {
    pub fn builder<'a>(layout: Arc<PipelineLayout>) -> PipelineBuilder<'a> {
        PipelineBuilder {
            device: layout.device.clone(),
            pipeline_cache: PipelineCacheType::None,
            flags: Default::default(),
            pipeline_vertex_input_state_create_info: Default::default(),
            stages: Default::default(),
            input_assembly_state: PipelineInputAssemblyStateCreateInfo::default(),
            viewport_state: PipelineViewportStateCreateInfo::default(),
            tessellation_state: Default::default(),
            rasterization_state: Default::default(),
            multisample_state: Default::default(),
            depth_stencil_state: Default::default(),
            color_blend_state: Default::default(),
            layout,
            dynamic_states: Default::default(),
            render_pass: None,
        }
    }
}

pub enum PipelineCacheType<'a> {
    ExternallySynchronized(&'a mut PipelineCacheImpl<true>),
    InternallySynchronized(&'a PipelineCacheImpl<false>),
    None,
}

impl<'a> PipelineCacheType<'a> {
    unsafe fn to_vk_types(&self) -> ash::vk::PipelineCache {
        match self {
            PipelineCacheType::ExternallySynchronized(pipeline_cache) => {
                pipeline_cache.vk_pipeline_cache
            }
            PipelineCacheType::InternallySynchronized(pipeline_cache) => {
                pipeline_cache.vk_pipeline_cache
            }
            PipelineCacheType::None => ash::vk::PipelineCache::null(),
        }
    }
}

pub struct PipelineBuilder<'a> {
    device: Arc<Device>,
    pipeline_cache: PipelineCacheType<'a>,
    flags: ash::vk::PipelineCreateFlags,
    pipeline_vertex_input_state_create_info: PipelineVertexInputStateCreateInfo,
    stages: FxHashMap<ash::vk::ShaderStageFlags, PipelineShaderStageCreateInfo<'a>>,
    input_assembly_state: PipelineInputAssemblyStateCreateInfo,
    viewport_state: PipelineViewportStateCreateInfo,
    tessellation_state: PipelineTessellationStateCreateInfo,
    rasterization_state: PipelineRasterizationStateCreateInfo,
    multisample_state: PipelineMultisampleStateCreateInfo,
    depth_stencil_state: PipelineDepthStencilStateCreateInfo,
    color_blend_state: PipelineColorBlendStateCreateInfo,
    layout: Arc<PipelineLayout>,
    dynamic_states: FxHashSet<ash::vk::DynamicState>,
    render_pass: Option<(Arc<RenderPass>, u32)>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn cache(mut self, pipeline_cache: PipelineCacheType<'a>) -> Self {
        self.pipeline_cache = pipeline_cache;
        self
    }
    pub fn flags(mut self, flags: ash::vk::PipelineCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn add_stage(mut self, stage: PipelineShaderStageCreateInfo<'a>) -> Self {
        // MUST VUID-VkGraphicsPipelineCreateInfo-stage-00726
        if self.stages.insert(stage.stage, stage).is_some() {
            panic!("VUID-VkGraphicsPipelineCreateInfo-stage-00726")
        }
        self
    }
    pub fn vertex_input_state(
        mut self,
        vertex_input_state: PipelineVertexInputStateCreateInfo,
    ) -> Self {
        self.pipeline_vertex_input_state_create_info = vertex_input_state;
        self
    }
    pub fn input_assembly_state(
        mut self,
        input_assembly_state: PipelineInputAssemblyStateCreateInfo,
    ) -> Self {
        self.input_assembly_state = input_assembly_state;
        self
    }
    pub fn tessellation_state(
        mut self,
        tessellation_state: PipelineTessellationStateCreateInfo,
    ) -> Self {
        self.tessellation_state = tessellation_state;
        self
    }
    pub fn viewport_state(mut self, viewport_state: PipelineViewportStateCreateInfo) -> Self {
        self.viewport_state = viewport_state;
        self
    }
    pub fn rasterization_state(
        mut self,
        rasterization_state: PipelineRasterizationStateCreateInfo,
    ) -> Self {
        self.rasterization_state = rasterization_state;
        self
    }
    pub fn multisample_state(
        mut self,
        multisample_state: PipelineMultisampleStateCreateInfo,
    ) -> Self {
        self.multisample_state = multisample_state;
        self
    }
    pub fn depth_stencil_state(
        mut self,
        depth_stencil_state: PipelineDepthStencilStateCreateInfo,
    ) -> Self {
        self.depth_stencil_state = depth_stencil_state;
        self
    }
    pub fn color_blend_state(
        mut self,
        color_blend_state: PipelineColorBlendStateCreateInfo,
    ) -> Self {
        self.color_blend_state = color_blend_state;
        self
    }
    pub fn render_pass(mut self, render_pass: Arc<RenderPass>, subpass: u32) -> Self {
        // DONE VUID-VkGraphicsPipelineCreateInfo-renderPass-06046
        self.render_pass = Some((render_pass, subpass));
        self
    }
    // All vendors suggest to avoid using pipeline derivatives, and the API design is a little
    // tricky (need build a tree to avoid reference loop. So I just leave it unimplemented
    // pub fn base_pipeline_handle(mut self, base_pipeline_handle: Arc<Pipeline>) -> Self {
    //     self.base_pipeline_handle = Some(base_pipeline_handle);
    //     self.flags |= ash::vk::PipelineCreateFlags::ALLOW_DERIVATIVES;
    //     self
    // }
    // pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
    //     self.base_pipeline_index = base_pipeline_index;
    //     self.flags |= ash::vk::PipelineCreateFlags::ALLOW_DERIVATIVES;
    //     self
    // }
    pub fn build(mut self) -> Result<Arc<Pipeline>, ash::vk::Result> {
        // stages
        let mut shader_modules_holder = Vec::with_capacity(self.stages.len());
        let mut ash_vk_stages = Vec::with_capacity(self.stages.len());
        for (_, info) in self.stages {
            ash_vk_stages.push(info.ash_builder());
            shader_modules_holder.push(info.module);
        }
        // vertex input
        let ash_vk_vertex_input_state = self
            .pipeline_vertex_input_state_create_info
            .ash_builder()
            .build();
        // input assembly
        let ash_vk_input_assembly_state = self.input_assembly_state.ash_builder().build();
        // tessellation
        let ash_vk_tessellation_state = self.tessellation_state.ash_builder().build();
        // view port
        let ash_vk_viewport_state = self.viewport_state.ash_builder().build();
        if ash_vk_viewport_state.p_viewports.is_null() {
            if ash_vk_viewport_state.viewport_count > 1 {
                self.dynamic_states
                    .insert(ash::vk::DynamicState::VIEWPORT_WITH_COUNT);
            } else {
                self.dynamic_states.insert(ash::vk::DynamicState::VIEWPORT);
            }
        }
        if ash_vk_viewport_state.p_scissors.is_null() {
            if ash_vk_viewport_state.scissor_count > 1 {
                self.dynamic_states
                    .insert(ash::vk::DynamicState::SCISSOR_WITH_COUNT);
            } else {
                self.dynamic_states.insert(ash::vk::DynamicState::SCISSOR);
            }
        }
        // rasterization
        let ash_vk_rasterization_state = self.rasterization_state.ash_builder().build();
        // multisample
        let ash_vk_multisample_state = self.multisample_state.ash_builder().build();
        // depth stencil
        let ash_vk_depth_stencil_state = self.depth_stencil_state.ash_builder().build();
        // color blend
        let ash_vk_color_blend_state = self.color_blend_state.ash_builder().build();
        // dynamic states
        let ash_vk_dynamic_states = self.dynamic_states.into_iter().collect::<Vec<_>>();
        let ash_vk_pipeline_dynamic_state_create_info =
            ash::vk::PipelineDynamicStateCreateInfo::builder()
                .dynamic_states(ash_vk_dynamic_states.as_slice())
                .build();
        let mut create_info_builder = ash::vk::GraphicsPipelineCreateInfo::builder()
            .flags(self.flags)
            .stages(ash_vk_stages.as_slice())
            .vertex_input_state(&ash_vk_vertex_input_state)
            .input_assembly_state(&ash_vk_input_assembly_state)
            .tessellation_state(&ash_vk_tessellation_state)
            .viewport_state(&ash_vk_viewport_state)
            .rasterization_state(&ash_vk_rasterization_state)
            .multisample_state(&ash_vk_multisample_state)
            .depth_stencil_state(&ash_vk_depth_stencil_state)
            .color_blend_state(&ash_vk_color_blend_state)
            .layout(self.layout.ash_vk_pipeline_layout)
            .dynamic_state(&ash_vk_pipeline_dynamic_state_create_info);
        let mut render_pass_holder = None;
        if let Some((render_pass, subpass_index)) = self.render_pass {
            create_info_builder = create_info_builder
                .render_pass(render_pass.ash_vk_renderpass)
                .subpass(subpass_index);
            render_pass_holder = Some(render_pass);
        }
        let create_info = create_info_builder.build();
        // TODO deal with VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT
        let ash_vk_pipeline = unsafe {
            match self.device.ash_device.create_graphics_pipelines(
                self.pipeline_cache.to_vk_types(),
                &[create_info],
                None,
            ) {
                Ok(pipelines) => pipelines[0],
                Err((_, error)) => {
                    return Err(error);
                }
            }
        };
        Ok(Arc::new(Pipeline {
            device: self.device,
            pipeline_layout: self.layout,
            _render_pass_holder: render_pass_holder,
            _shader_modules_holder: shader_modules_holder,
            ash_vk_pipeline,
        }))
    }
}

struct PipelineCreateInfoTmp {
    flags: ash::vk::PipelineCreateFlags,
    layout: Arc<PipelineLayout>,
    shader_modules_holder: Vec<Arc<ShaderModule>>,
    ash_vk_stages: Vec<ash::vk::PipelineShaderStageCreateInfo>,
    ash_vk_vertex_input_state: ash::vk::PipelineVertexInputStateCreateInfo,
    ash_vk_input_assembly_state: ash::vk::PipelineInputAssemblyStateCreateInfo,
    ash_vk_tessellation_state: ash::vk::PipelineTessellationStateCreateInfo,
    ash_vk_viewport_state: ash::vk::PipelineViewportStateCreateInfo,
    ash_vk_rasterization_state: ash::vk::PipelineRasterizationStateCreateInfo,
    ash_vk_multisample_state: ash::vk::PipelineMultisampleStateCreateInfo,
    ash_vk_depth_stencil_state: ash::vk::PipelineDepthStencilStateCreateInfo,
    ash_vk_color_blend_state: ash::vk::PipelineColorBlendStateCreateInfo,
    _ash_vk_dynamic_states: Vec<ash::vk::DynamicState>,
    ash_vk_pipeline_dynamic_state_create_info: ash::vk::PipelineDynamicStateCreateInfo,
    render_pass_holder: Option<(Arc<RenderPass>, u32)>,
}

impl Device {
    pub fn create_pipelines(
        &self,
        mut builders: Vec<PipelineBuilder>,
        pipeline_cache: PipelineCacheType,
    ) -> Result<Vec<Arc<Pipeline>>, ash::vk::Result> {
        let mut vk_tmps = builders
            .iter_mut()
            .map(|builder| {
                // stages
                let mut shader_modules_holder = Vec::with_capacity(builder.stages.len());
                let mut ash_vk_stages = Vec::with_capacity(builder.stages.len());
                for (_, info) in &builder.stages {
                    ash_vk_stages.push(info.ash_builder());
                    shader_modules_holder.push(info.module.clone());
                }
                // vertex input
                let ash_vk_vertex_input_state = builder
                    .pipeline_vertex_input_state_create_info
                    .ash_builder()
                    .build();
                // input assembly
                let ash_vk_input_assembly_state =
                    builder.input_assembly_state.ash_builder().build();
                // tessellation
                let ash_vk_tessellation_state = builder.tessellation_state.ash_builder().build();
                // view port
                let ash_vk_viewport_state = builder.viewport_state.ash_builder().build();
                if ash_vk_viewport_state.p_viewports.is_null() {
                    if ash_vk_viewport_state.viewport_count > 1 {
                        builder
                            .dynamic_states
                            .insert(ash::vk::DynamicState::VIEWPORT_WITH_COUNT);
                    } else {
                        builder
                            .dynamic_states
                            .insert(ash::vk::DynamicState::VIEWPORT);
                    }
                }
                if ash_vk_viewport_state.p_scissors.is_null() {
                    if ash_vk_viewport_state.scissor_count > 1 {
                        builder
                            .dynamic_states
                            .insert(ash::vk::DynamicState::SCISSOR_WITH_COUNT);
                    } else {
                        builder
                            .dynamic_states
                            .insert(ash::vk::DynamicState::SCISSOR);
                    }
                }
                // rasterization
                let ash_vk_rasterization_state = builder.rasterization_state.ash_builder().build();
                // multisample
                let ash_vk_multisample_state = builder.multisample_state.ash_builder().build();
                // depth stencil
                let ash_vk_depth_stencil_state = builder.depth_stencil_state.ash_builder().build();
                // color blend
                let ash_vk_color_blend_state = builder.color_blend_state.ash_builder().build();
                // dynamic states
                let ash_vk_dynamic_states =
                    builder.dynamic_states.iter().cloned().collect::<Vec<_>>();
                let ash_vk_pipeline_dynamic_state_create_info =
                    ash::vk::PipelineDynamicStateCreateInfo::builder()
                        .dynamic_states(ash_vk_dynamic_states.as_slice())
                        .build();
                PipelineCreateInfoTmp {
                    flags: builder.flags,
                    layout: builder.layout.clone(),
                    shader_modules_holder,
                    ash_vk_stages,
                    ash_vk_vertex_input_state,
                    ash_vk_input_assembly_state,
                    ash_vk_tessellation_state,
                    ash_vk_viewport_state,
                    ash_vk_rasterization_state,
                    ash_vk_multisample_state,
                    ash_vk_depth_stencil_state,
                    ash_vk_color_blend_state,
                    _ash_vk_dynamic_states: ash_vk_dynamic_states,
                    ash_vk_pipeline_dynamic_state_create_info,
                    render_pass_holder: builder.render_pass.clone(),
                }
            })
            .collect::<Vec<_>>();
        let create_infos = vk_tmps
            .iter()
            .map(|vk_tmp| {
                let mut create_info_builder = ash::vk::GraphicsPipelineCreateInfo::builder()
                    .flags(vk_tmp.flags)
                    .stages(vk_tmp.ash_vk_stages.as_slice())
                    .vertex_input_state(&vk_tmp.ash_vk_vertex_input_state)
                    .input_assembly_state(&vk_tmp.ash_vk_input_assembly_state)
                    .tessellation_state(&vk_tmp.ash_vk_tessellation_state)
                    .viewport_state(&vk_tmp.ash_vk_viewport_state)
                    .rasterization_state(&vk_tmp.ash_vk_rasterization_state)
                    .multisample_state(&vk_tmp.ash_vk_multisample_state)
                    .depth_stencil_state(&vk_tmp.ash_vk_depth_stencil_state)
                    .color_blend_state(&vk_tmp.ash_vk_color_blend_state)
                    .layout(vk_tmp.layout.ash_vk_pipeline_layout)
                    .dynamic_state(&vk_tmp.ash_vk_pipeline_dynamic_state_create_info);
                if let Some((render_pass, subpass_index)) = &vk_tmp.render_pass_holder {
                    create_info_builder = create_info_builder
                        .render_pass(render_pass.ash_vk_renderpass)
                        .subpass(*subpass_index);
                }
                create_info_builder.build()
            })
            .collect::<Vec<_>>();
        // TODO deal with VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT
        unsafe {
            let vk_pipelines = match self.ash_device.create_graphics_pipelines(
                pipeline_cache.to_vk_types(),
                create_infos.as_slice(),
                None,
            ) {
                Ok(vk_pipelines) => vk_pipelines,
                Err((_, error)) => {
                    return Err(error);
                }
            };
            let pipelines = vk_pipelines
                .into_iter()
                .enumerate()
                .map(|(index, ash_vk_pipeline)| {
                    Arc::new(Pipeline {
                        device: builders[index].device.clone(),
                        pipeline_layout: builders[index].layout.clone(),
                        _render_pass_holder: if let Some((render_pass, _)) =
                            &vk_tmps[index].render_pass_holder
                        {
                            Some(render_pass.clone())
                        } else {
                            None
                        },
                        _shader_modules_holder: std::mem::take(
                            &mut vk_tmps[index].shader_modules_holder,
                        ),
                        ash_vk_pipeline,
                    })
                })
                .collect();
            Ok(pipelines)
        }
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            // TODO VUID-vkDestroyPipeline-pipeline-00765
            // Host Synchronization pipeline
            self.device
                .ash_device
                .destroy_pipeline(self.ash_vk_pipeline, None);
        }
    }
}

impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { RECORDING }, SCOPE> {
    // DONE VUID-vkCmdBindPipeline-commandBuffer-recording
    pub fn cmd_bind_pipeline(
        &mut self,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        pipeline: Arc<Pipeline>,
    ) {
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_bind_pipeline(
                self.vk_command_buffer,
                pipeline_bind_point,
                pipeline.ash_vk_pipeline,
            );
        }
        self.holding_resources
            .pipelines
            .insert(pipeline.ash_vk_pipeline.as_raw(), pipeline);
    }

    pub fn cmd_push_constants(
        &mut self,
        layout: &Arc<PipelineLayout>,
        stage: &ShaderStage,
        offset: u32,
        constants: &[u8],
    ) {
        self.holding_resources
            .pipeline_layouts
            .insert(layout.ash_vk_pipeline_layout.as_raw(), layout.clone());
        unsafe {
            // Host Synchronization: commandBuffer, VkCommandPool
            self.device.ash_device.cmd_push_constants(
                self.vk_command_buffer,
                layout.ash_vk_pipeline_layout,
                stage.to_ash(),
                offset,
                constants,
            )
        }
    }
}
