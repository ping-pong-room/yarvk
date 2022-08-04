use crate::buffer::Buffer;
use crate::image::Image;
use std::sync::Arc;

// DONE VUID-VkMemoryDedicatedAllocateInfo-image-01432
// TODO can image and buffer both be null?
// TODO VUID-VkMemoryDedicatedAllocateInfo-image-01434
// TODO VUID-VkMemoryDedicatedAllocateInfo-buffer-01436
pub enum DedicatedResource {
    Image(Arc<Image>),
    Buffer(Arc<Buffer>),
}

pub struct MemoryDedicatedAllocateInfo {
    resource: DedicatedResource,
}

impl MemoryDedicatedAllocateInfo {
    pub(crate) fn ash_builder(&self) -> ash::vk::MemoryDedicatedAllocateInfoBuilder {
        let mut builder = ash::vk::MemoryDedicatedAllocateInfo::builder();
        match &self.resource {
            DedicatedResource::Image(image) => {
                builder = builder.image(image.vk_image);
            }
            DedicatedResource::Buffer(buffer) => {
                builder = builder.buffer(buffer.ash_vk_buffer);
            }
        }
        builder
    }
}
