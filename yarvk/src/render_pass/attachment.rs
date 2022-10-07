#[derive(Default)]
pub struct AttachmentDescriptionBuilder {
    attachment_description: ash::vk::AttachmentDescription,
}

impl AttachmentDescriptionBuilder {
    pub fn flags(mut self, flags: ash::vk::AttachmentDescriptionFlags) -> Self {
        self.attachment_description.flags = flags;
        self
    }
    pub fn format(mut self, format: ash::vk::Format) -> Self {
        self.attachment_description.format = format;
        self
    }
    pub fn samples(mut self, samples: ash::vk::SampleCountFlags) -> Self {
        self.attachment_description.samples = samples;
        self
    }
    pub fn load_op(mut self, load_op: ash::vk::AttachmentLoadOp) -> Self {
        self.attachment_description.load_op = load_op;
        self
    }
    pub fn store_op(mut self, store_op: ash::vk::AttachmentStoreOp) -> Self {
        self.attachment_description.store_op = store_op;
        self
    }
    pub fn stencil_load_op(mut self, stencil_load_op: ash::vk::AttachmentLoadOp) -> Self {
        self.attachment_description.stencil_load_op = stencil_load_op;
        self
    }
    pub fn stencil_store_op(mut self, stencil_store_op: ash::vk::AttachmentStoreOp) -> Self {
        self.attachment_description.stencil_store_op = stencil_store_op;
        self
    }
    pub fn initial_layout(mut self, initial_layout: ash::vk::ImageLayout) -> Self {
        self.attachment_description.initial_layout = initial_layout;
        self
    }
    pub fn final_layout(mut self, final_layout: ash::vk::ImageLayout) -> Self {
        self.attachment_description.final_layout = final_layout;
        self
    }
    pub fn build(self) -> AttachmentDescription {
        AttachmentDescription(self.attachment_description)
    }
}

pub struct AttachmentDescription(pub(crate) ash::vk::AttachmentDescription);

impl AttachmentDescription {
    pub fn builder() -> AttachmentDescriptionBuilder {
        AttachmentDescriptionBuilder::default()
    }
}

pub struct AttachmentReferenceBuilder {
    attachment: u32,
    layout: ash::vk::ImageLayout,
}

impl Default for AttachmentReference {
    fn default() -> Self {
        Self {
            attachment: ash::vk::ATTACHMENT_UNUSED,
            layout: ash::vk::ImageLayout::UNDEFINED,
        }
    }
}

impl AttachmentReferenceBuilder {
    pub fn attachment_index(mut self, index: u32) -> Self {
        self.attachment = index;
        self
    }
    pub fn layout(mut self, layout: ash::vk::ImageLayout) -> Self {
        self.layout = layout;
        self
    }
    pub fn build(self) -> AttachmentReference {
        AttachmentReference {
            attachment: self.attachment,
            layout: self.layout,
        }
    }
}

pub struct AttachmentReference {
    attachment: u32,
    layout: ash::vk::ImageLayout,
}

impl AttachmentReference {
    pub fn builder() -> AttachmentReferenceBuilder {
        AttachmentReferenceBuilder {
            attachment: ash::vk::ATTACHMENT_UNUSED,
            layout: ash::vk::ImageLayout::UNDEFINED,
        }
    }
    pub(crate) fn to_ash(&self) -> ash::vk::AttachmentReference {
        ash::vk::AttachmentReference {
            attachment: self.attachment,
            layout: self.layout,
        }
    }
}
