use crate::pipeline::pipeline_stage_flags::PipelineStageFlag;
use crate::render_pass::attachment::AttachmentReference;

#[derive(Default)]
pub struct SubpassDependencyBuilder {
    subpass_dependency: SubpassDependency,
}

impl SubpassDependencyBuilder {
    pub fn src_subpass(mut self, src_subpass: u32) -> Self {
        self.subpass_dependency.src_subpass = src_subpass;
        self
    }
    pub fn dst_subpass(mut self, dst_subpass: u32) -> Self {
        self.subpass_dependency.dst_subpass = dst_subpass;
        self
    }
    pub fn add_src_stage_mask(mut self, src_stage_mask: PipelineStageFlag) -> Self {
        self.subpass_dependency.src_stage_mask |= src_stage_mask.to_ash();
        self
    }
    pub fn add_dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlag) -> Self {
        self.subpass_dependency.dst_stage_mask |= dst_stage_mask.to_ash();
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: ash::vk::AccessFlags) -> Self {
        self.subpass_dependency.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: ash::vk::AccessFlags) -> Self {
        self.subpass_dependency.dst_access_mask = dst_access_mask;
        self
    }
    pub fn dependency_flags(mut self, dependency_flags: ash::vk::DependencyFlags) -> Self {
        self.subpass_dependency.dependency_flags = dependency_flags;
        self
    }

    pub fn build(self) -> SubpassDependency {
        self.subpass_dependency
    }
}

#[derive(Default)]
pub struct SubpassDependency {
    src_subpass: u32,
    dst_subpass: u32,
    src_stage_mask: ash::vk::PipelineStageFlags,
    dst_stage_mask: ash::vk::PipelineStageFlags,
    src_access_mask: ash::vk::AccessFlags,
    dst_access_mask: ash::vk::AccessFlags,
    dependency_flags: ash::vk::DependencyFlags,
}

impl SubpassDependency {
    pub fn builder() -> SubpassDependencyBuilder {
        SubpassDependencyBuilder::default()
    }
    pub fn ash_builder(&self) -> ash::vk::SubpassDependencyBuilder {
        ash::vk::SubpassDependency::builder()
            .src_subpass(self.src_subpass)
            .dst_subpass(self.dst_subpass)
            .src_access_mask(self.src_access_mask)
            .dst_access_mask(self.dst_access_mask)
            .src_stage_mask(self.src_stage_mask)
            .dst_stage_mask(self.dst_stage_mask)
            .dependency_flags(self.dependency_flags)
    }
}

pub struct SubpassDescription {
    flags: ash::vk::SubpassDescriptionFlags,
    pipeline_bind_point: ash::vk::PipelineBindPoint,
    input_attachments: Vec<ash::vk::AttachmentReference>,
    color_attachments: Vec<ash::vk::AttachmentReference>,
    resolve_attachments: Vec<ash::vk::AttachmentReference>,
    depth_stencil_attachment: Option<ash::vk::AttachmentReference>,
    preserve_attachments: Vec<u32>,
}

impl SubpassDescription {
    pub fn builder() -> SubpassDescriptionBuilder {
        SubpassDescriptionBuilder::default()
    }
    pub(crate) fn to_ash(&self) -> ash::vk::SubpassDescription {
        let mut builder = ash::vk::SubpassDescription::builder()
            .flags(*&self.flags)
            .pipeline_bind_point(self.pipeline_bind_point)
            .input_attachments(self.input_attachments.as_slice())
            .color_attachments(self.color_attachments.as_slice())
            .resolve_attachments(self.resolve_attachments.as_slice())
            .preserve_attachments(self.preserve_attachments.as_slice());
        if let Some(depth_stencil_attachment) = &self.depth_stencil_attachment {
            builder = builder.depth_stencil_attachment(depth_stencil_attachment);
        }
        builder.build()
    }
}

#[derive(Default)]
pub struct SubpassDescriptionBuilder {
    flags: ash::vk::SubpassDescriptionFlags,
    pipeline_bind_point: ash::vk::PipelineBindPoint,
    input_attachments: Vec<AttachmentReference>,
    color_with_resolve_attachments: Vec<(AttachmentReference, AttachmentReference)>,
    depth_stencil_attachment: Option<AttachmentReference>,
    preserve_attachments: Vec<usize>,
}

impl SubpassDescriptionBuilder {
    pub fn flags(mut self, flags: ash::vk::SubpassDescriptionFlags) -> Self {
        self.flags = flags;
        self
    }
    // DONE VUID-VkSubpassDescription-pipelineBindPoint-04952
    pub fn pipeline_bind_point_graphics(mut self) -> Self {
        self.pipeline_bind_point = ash::vk::PipelineBindPoint::GRAPHICS;
        self
    }
    pub fn pipeline_bind_point_huawei(mut self) -> Self {
        self.pipeline_bind_point = ash::vk::PipelineBindPoint::SUBPASS_SHADING_HUAWEI;
        self
    }
    pub fn add_input_attachment(mut self, input_attachment: AttachmentReference) -> Self {
        self.input_attachments.push(input_attachment);
        self
    }
    pub fn add_color_attachment(mut self, color_attachment: AttachmentReference) -> Self {
        self.color_with_resolve_attachments
            .push((color_attachment, AttachmentReference::default()));
        self
    }
    pub fn add_color_attachment_with_resolve_attachment(
        mut self,
        color_attachment: AttachmentReference,
        resolve_attachment: AttachmentReference,
    ) -> Self {
        self.color_with_resolve_attachments
            .push((color_attachment, resolve_attachment));
        self
    }
    pub fn depth_stencil_attachment(
        mut self,
        depth_stencil_attachment: AttachmentReference,
    ) -> Self {
        self.depth_stencil_attachment = Some(depth_stencil_attachment);
        self
    }
    pub fn add_preserve_attachment(mut self, preserve_attachment_index: usize) -> Self {
        self.preserve_attachments.push(preserve_attachment_index);
        self
    }

    pub fn build(self) -> SubpassDescription {
        let input_attachments = self
            .input_attachments
            .into_iter()
            .map(|ar| ar.to_ash())
            .collect::<Vec<_>>();
        // let mut color_with_resolve_attachments =
        //     Vec::with_capacity(self.color_with_resolve_attachments.len());
        let mut color_attachments = Vec::with_capacity(self.color_with_resolve_attachments.len());
        let mut resolve_attachments = Vec::with_capacity(self.color_with_resolve_attachments.len());
        for (color_attachment, resolve_attachment) in self.color_with_resolve_attachments {
            // color_with_resolve_attachments.push((
            //     color_attachment.to_ash(),
            //     resolve_attachment.to_ash(),
            // ));
            color_attachments.push(color_attachment.to_ash());
            resolve_attachments.push(resolve_attachment.to_ash());
        }
        let preserve_attachments: Vec<u32> = self
            .preserve_attachments
            .into_iter()
            .map(|index| index as _)
            .collect();
        let depth_stencil_attachment =
            if let Some(depth_stencil_attachment) = self.depth_stencil_attachment {
                Some(depth_stencil_attachment.to_ash())
            } else {
                None
            };

        SubpassDescription {
            flags: self.flags,
            pipeline_bind_point: self.pipeline_bind_point,
            input_attachments,
            color_attachments,
            resolve_attachments,
            depth_stencil_attachment,
            preserve_attachments,
        }
    }
}
