use crate::device::Device;
use crate::render_pass::attachment::{AttachmentDescription, AttachmentIndex};
use crate::render_pass::subpass::{SubpassDependency, SubpassDescription, SubpassIndex};



use std::default::Default;
use std::sync::Arc;

pub mod attachment;
pub mod render_pass_begin_info;
pub mod subpass;

pub struct RenderPassBuilder {
    device: Arc<Device>,
    flags: ash::vk::RenderPassCreateFlags,
    attachments: Vec<AttachmentDescription>,
    subpasses: Vec<SubpassDescription>,
    subpass_dependencies: Vec<SubpassDependency>,
}

impl RenderPassBuilder {
    pub fn flag(&mut self, flags: ash::vk::RenderPassCreateFlags) {
        self.flags = flags;
    }
    pub fn add_attachment(&mut self, attachment: AttachmentDescription) -> AttachmentIndex {
        let index = AttachmentIndex(self.attachments.len() as _);
        self.attachments.push(attachment);
        index
    }
    pub fn add_subpass(&mut self, subpass_description: SubpassDescription) -> SubpassIndex {
        let index = self.subpasses.len();
        self.subpasses.push(subpass_description);
        SubpassIndex { 0: index as _ }
    }
    pub fn add_dependency(&mut self, subpass_dependency: SubpassDependency) -> &mut Self {
        self.subpass_dependencies.push(subpass_dependency);
        self
    }
    pub fn build(self) -> Result<Arc<RenderPass>, ash::vk::Result> {
        // DONE VUID-VkRenderPassCreateInfo-attachment-00834
        // DONE VUID-VkRenderPassCreateInfo-fragmentDensityMapAttachment-06471
        // DONE VUID-VkRenderPassCreateInfo-pNext-01926
        // DONE VUID-VkRenderPassCreateInfo-pNext-01927
        // DONE VUID-VkRenderPassCreateInfo-srcSubpass-02517
        // DONE VUID-VkRenderPassCreateInfo-dstSubpass-02518
        let vk_attachments = self
            .attachments
            .into_iter()
            .map(|attachment| attachment.0)
            .collect::<Vec<_>>();
        let vk_subpasses: Vec<_> = self
            .subpasses
            .iter()
            .map(|subpass_description| subpass_description.to_ash())
            .collect();
        let vk_subpass_dependencies = self
            .subpass_dependencies
            .iter()
            .map(|subpass_dependency| subpass_dependency.ash_builder().build())
            .collect::<Vec<_>>();
        let create_info = ash::vk::RenderPassCreateInfo::builder()
            .flags(self.flags)
            .attachments(vk_attachments.as_slice())
            .subpasses(vk_subpasses.as_slice())
            .dependencies(vk_subpass_dependencies.as_slice())
            .build();
        unsafe {
            // Host Synchronization: none
            let vk_renderpass = self
                .device
                .ash_device
                .create_render_pass(&create_info, None)?;
            Ok(Arc::new(RenderPass {
                device: self.device,
                vk_attachments,
                ash_vk_renderpass: vk_renderpass,
            }))
        }
    }
}

pub struct RenderPass {
    pub device: Arc<Device>,
    pub(crate) vk_attachments: Vec<ash::vk::AttachmentDescription>,
    pub(crate) ash_vk_renderpass: ash::vk::RenderPass,
}

impl RenderPass {
    pub fn builder(device: Arc<Device>) -> RenderPassBuilder {
        RenderPassBuilder {
            device,
            flags: Default::default(),
            attachments: vec![],
            subpasses: vec![],
            subpass_dependencies: vec![],
        }
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            // TODO execution VUID-vkDestroyRenderPass-renderPass-00873
            // Host Synchronization: renderpass
            self.device
                .ash_device
                .destroy_render_pass(self.ash_vk_renderpass, None);
        }
    }
}
