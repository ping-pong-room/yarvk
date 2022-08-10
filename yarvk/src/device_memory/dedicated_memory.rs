use crate::buffer::Buffer;
use crate::image::Image;
use crate::{buffer, image};

// DONE VUID-VkMemoryDedicatedAllocateInfo-image-01432
// TODO can image and buffer both be null?
// TODO VUID-VkMemoryDedicatedAllocateInfo-image-01434
// TODO VUID-VkMemoryDedicatedAllocateInfo-buffer-01436
pub enum DedicatedResource<'a> {
    Image(&'a Image<{ image::State::Unbound }>),
    Buffer(&'a Buffer<{ buffer::State::Unbound }>),
}

pub struct MemoryDedicatedAllocateInfo<'a> {
    pub resource: DedicatedResource<'a>,
}

impl MemoryDedicatedAllocateInfo<'_> {
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
