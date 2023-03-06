use crate::device::Device;
use crate::device_features::Feature;
use crate::device_features::PhysicalDeviceVulkan12Features::ImagelessFramebuffer;
use crate::image::image_view::ImageView;
use crate::render_pass::RenderPass;
use std::collections::btree_map::BTreeMap;
use std::sync::Arc;

pub enum FramebufferCreateFlags {
    // DONE VUID-VkFramebufferCreateInfo-flags-03189
    Imageless(Feature<{ ImagelessFramebuffer.into() }>),
}

impl FramebufferCreateFlags {
    pub(crate) fn to_ash(&self) -> ash::vk::FramebufferCreateFlags {
        match self {
            FramebufferCreateFlags::Imageless(_) => ash::vk::FramebufferCreateFlags::IMAGELESS,
        }
    }
}

pub struct Framebuffer {
    pub device: Arc<Device>,
    pub render_pass: Arc<RenderPass>,
    _attachments: BTreeMap<u32, Arc<ImageView>>,
    pub(crate) ash_vk_framebuffer: ash::vk::Framebuffer,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            // TODO execution VUID-vkDestroyFramebuffer-framebuffer-00892
            // Host Synchronization: framebuffer
            self.device
                .ash_device
                .destroy_framebuffer(self.ash_vk_framebuffer, None);
        }
    }
}

impl Framebuffer {
    pub fn builder(render_pass: Arc<RenderPass>) -> FramebufferBuilder {
        FramebufferBuilder {
            flags: Default::default(),
            render_pass,
            attachments: Default::default(),
            width: Default::default(),
            height: Default::default(),
            layers: Default::default(),
        }
    }
}

pub struct FramebufferBuilder {
    flags: ash::vk::FramebufferCreateFlags,
    render_pass: Arc<RenderPass>,
    attachments: BTreeMap<u32, Arc<ImageView>>,
    width: u32,
    height: u32,
    layers: u32,
}

impl FramebufferBuilder {
    pub fn add_flag(mut self, flag: FramebufferCreateFlags) -> Self {
        self.flags |= flag.to_ash();
        self
    }
    pub fn render_pass(mut self, render_pass: Arc<RenderPass>) -> Self {
        self.render_pass = render_pass;
        self
    }
    pub fn add_attachment(mut self, attachment_index: u32, attachment: Arc<ImageView>) -> Self {
        self.attachments.insert(attachment_index, attachment);
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    pub fn layers(mut self, layers: u32) -> Self {
        self.layers = layers;
        self
    }
    pub fn build(self, device: &Arc<Device>) -> Result<Arc<Framebuffer>, ash::vk::Result> {
        // DONE VUID-VkFramebufferCreateInfo-flags-02778
        let ash_vk_attachments = self
            .attachments
            .values()
            .map(|image_view| image_view.ash_vk_image_view)
            .collect::<Vec<_>>();
        let mut frame_buffer_create_info = ash::vk::FramebufferCreateInfo::builder()
            .flags(self.flags)
            .render_pass(self.render_pass.ash_vk_renderpass)
            .attachments(ash_vk_attachments.as_slice())
            .width(self.width)
            .height(self.height)
            .layers(self.layers)
            .build();

        // SILENCE VUID-VkFramebufferCreateInfo-attachmentCount-00876
        if ash_vk_attachments.is_empty() {
            frame_buffer_create_info.attachment_count = self.render_pass.vk_attachments.len() as _;
            frame_buffer_create_info.p_attachments = std::ptr::null();
        }
        let ash_vk_framebuffer = unsafe {
            // Host Synchronization: none
            device
                .ash_device
                .create_framebuffer(&frame_buffer_create_info, None)?
        };

        Ok(Arc::new(Framebuffer {
            device: device.clone(),
            render_pass: self.render_pass,
            _attachments: self.attachments,
            ash_vk_framebuffer,
        }))
    }
}
