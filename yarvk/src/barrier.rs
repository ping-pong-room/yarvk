use crate::buffer::Buffer;

use crate::command::command_buffer::State::RECORDING;
use crate::command::command_buffer::{CommandBuffer, Level, RenderPassScope};
use crate::image::image_subresource_range::ImageSubresourceRange;
use crate::image::Image;
use crate::physical_device::queue_family_properties::QueueFamilyProperties;
use crate::pipeline::pipeline_stage_flags::PipelineStageFlags;
use ash::vk::Handle;

use std::cell::Cell;
use std::pin::Pin;
use std::sync::Arc;

pub struct MemoryBarrier {
    pub(crate) ash_vk_memory_barrier: ash::vk::MemoryBarrier,
}

impl MemoryBarrier {
    pub fn builder() -> MemoryBarrierBuilder {
        MemoryBarrierBuilder::default()
    }
}

#[derive(Default)]
pub struct MemoryBarrierBuilder {
    ash_vk_memory_barrier: ash::vk::MemoryBarrier,
}

impl MemoryBarrierBuilder {
    pub fn src_access_mask(mut self, src_access_mask: ash::vk::AccessFlags) -> Self {
        self.ash_vk_memory_barrier.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: ash::vk::AccessFlags) -> Self {
        self.ash_vk_memory_barrier.dst_access_mask = dst_access_mask;
        self
    }
    pub fn build(self) -> Arc<MemoryBarrier> {
        Arc::new(MemoryBarrier {
            ash_vk_memory_barrier: self.ash_vk_memory_barrier,
        })
    }
}

pub struct ImageMemoryBarrier {
    pub(crate) ash_vk_image_memory_barrier: ash::vk::ImageMemoryBarrier,
    pub image: Arc<dyn Image>,
    // src_queue_family: Option<QueueFamilyProperties>,
    // dst_queue_family: Option<QueueFamilyProperties>,
}

// impl VulkanWrapper for ImageMemoryBarrier {
//     type VulkanTy = ash::vk::ImageMemoryBarrier;
//
//     fn raw_value(&self) -> Self::VulkanTy {
//         self.borrow().ash_vk_image_memory_barrier
//     }
// }

impl ImageMemoryBarrier {
    // DONE VUID-VkImageMemoryBarrier-image-01932
    pub fn builder(image: Arc<dyn Image>) -> ImageMemoryBarrierBuilder {
        ImageMemoryBarrierBuilder {
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            old_layout: Default::default(),
            new_layout: Default::default(),
            src_queue_family: Default::default(),
            dst_queue_family: Default::default(),
            image,
            subresource_range: Default::default(),
        }
    }
}

pub struct ImageMemoryBarrierBuilder {
    src_access_mask: ash::vk::AccessFlags,
    dst_access_mask: ash::vk::AccessFlags,
    old_layout: ash::vk::ImageLayout,
    new_layout: ash::vk::ImageLayout,
    src_queue_family: Option<QueueFamilyProperties>,
    dst_queue_family: Option<QueueFamilyProperties>,
    image: Arc<dyn Image>,
    subresource_range: ImageSubresourceRange,
}

impl ImageMemoryBarrierBuilder {
    pub fn src_access_mask(mut self, src_access_mask: ash::vk::AccessFlags) -> Self {
        self.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: ash::vk::AccessFlags) -> Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
    pub fn old_layout(mut self, old_layout: ash::vk::ImageLayout) -> Self {
        self.old_layout = old_layout;
        self
    }
    pub fn new_layout(mut self, new_layout: ash::vk::ImageLayout) -> Self {
        self.new_layout = new_layout;
        self
    }
    pub fn src_queue_family(mut self, src_queue_family: QueueFamilyProperties) -> Self {
        self.src_queue_family = Some(src_queue_family);
        self
    }
    pub fn dst_queue_family(mut self, dst_queue_family: QueueFamilyProperties) -> Self {
        self.dst_queue_family = Some(dst_queue_family);
        self
    }
    pub fn subresource_range(mut self, subresource_range: ImageSubresourceRange) -> Self {
        self.subresource_range = subresource_range;
        self
    }
    pub fn build(self) -> Pin<Arc<ImageMemoryBarrier>> {
        let mut ash_vk_image_memory_barrier = ash::vk::ImageMemoryBarrier::builder()
            .src_access_mask(self.src_access_mask)
            .dst_access_mask(self.dst_access_mask)
            .old_layout(self.old_layout)
            .new_layout(self.new_layout)
            .image(self.image.vk_image)
            .subresource_range(self.subresource_range.0);
        if let Some(src_queue_family) = &self.src_queue_family {
            ash_vk_image_memory_barrier =
                ash_vk_image_memory_barrier.src_queue_family_index(src_queue_family.index);
        }
        if let Some(dst_queue_family) = &self.dst_queue_family {
            ash_vk_image_memory_barrier =
                ash_vk_image_memory_barrier.dst_queue_family_index(dst_queue_family.index);
        }
        // TODO handle p_next by using pin
        Arc::pin(ImageMemoryBarrier {
            ash_vk_image_memory_barrier: ash_vk_image_memory_barrier.build(),
            image: self.image,
            // src_queue_family: self.src_queue_family,
            // dst_queue_family: self.dst_queue_family,
        })
    }
}

pub struct BufferMemoryBarrier {
    pub(crate) buffer: Arc<dyn Buffer>,
    pub(crate) ash_vk_buffer_memory_barrier: ash::vk::BufferMemoryBarrier,
    // src_queue_family: Option<QueueFamilyProperties>,
    // dst_queue_family: Option<QueueFamilyProperties>,
}

// impl VulkanWrapper for BufferMemoryBarrier {
//     type VulkanTy = ash::vk::BufferMemoryBarrier;
//
//     fn raw_value(&self) -> Self::VulkanTy {
//         self.borrow().ash_vk_buffer_memory_barrier
//     }
// }

impl BufferMemoryBarrier {
    pub fn builder(buffer: Arc<dyn Buffer>) -> BufferMemoryBarrierBuilder {
        BufferMemoryBarrierBuilder {
            ash_vk_buffer_memory_barrier: ash::vk::BufferMemoryBarrier::builder()
                .buffer(buffer.ash_vk_buffer)
                .build(),
            buffer,
            src_queue_family: None,
            dst_queue_family: None,
        }
    }
}

pub struct BufferMemoryBarrierBuilder {
    ash_vk_buffer_memory_barrier: ash::vk::BufferMemoryBarrier,
    buffer: Arc<dyn Buffer>,
    src_queue_family: Option<QueueFamilyProperties>,
    dst_queue_family: Option<QueueFamilyProperties>,
}

impl BufferMemoryBarrierBuilder {
    pub fn src_access_mask(mut self, src_access_mask: ash::vk::AccessFlags) -> Self {
        self.ash_vk_buffer_memory_barrier.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: ash::vk::AccessFlags) -> Self {
        self.ash_vk_buffer_memory_barrier.dst_access_mask = dst_access_mask;
        self
    }
    pub fn src_queue_family(mut self, src_queue_family: QueueFamilyProperties) -> Self {
        self.ash_vk_buffer_memory_barrier.src_queue_family_index = src_queue_family.index;
        self.src_queue_family = Some(src_queue_family);
        self
    }
    pub fn dst_queue_family(mut self, dst_queue_family: QueueFamilyProperties) -> Self {
        self.ash_vk_buffer_memory_barrier.dst_queue_family_index = dst_queue_family.index;
        self.dst_queue_family = Some(dst_queue_family);
        self
    }
    pub fn offset(mut self, offset: ash::vk::DeviceSize) -> Self {
        self.ash_vk_buffer_memory_barrier.offset = offset;
        self
    }
    pub fn size(mut self, size: ash::vk::DeviceSize) -> Self {
        self.ash_vk_buffer_memory_barrier.size = size;
        self
    }
    pub fn builder(self) -> Pin<Arc<BufferMemoryBarrier>> {
        Arc::pin(BufferMemoryBarrier {
            buffer: self.buffer,
            ash_vk_buffer_memory_barrier: self.ash_vk_buffer_memory_barrier,
            // src_queue_family: self.src_queue_family,
            // dst_queue_family: self.dst_queue_family,
        })
    }
}

impl<const LEVEL: Level, const SCOPE: RenderPassScope, const ONE_TIME_SUBMIT: bool>
    CommandBuffer<LEVEL, { RECORDING }, SCOPE, ONE_TIME_SUBMIT>
{
    thread_local! {
        static BARRIERS_CACHES: Cell<(Vec<ash::vk::MemoryBarrier>,
                                        Vec<ash::vk::BufferMemoryBarrier>,
                                        Vec<ash::vk::ImageMemoryBarrier>
                                        )> = Cell::new((Vec::new(), Vec::new(), Vec::new()));
    }
    // DONE VUID-vkCmdPipelineBarrier-commandBuffer-recording
    pub fn cmd_pipeline_barrier(
        &mut self,
        src_stage_mask: &[PipelineStageFlags],
        dst_stage_mask: &[PipelineStageFlags],
        dependency_flags: ash::vk::DependencyFlags,
        memory_barriers: &[Pin<Arc<MemoryBarrier>>],
        buffer_memory_barriers: &[Pin<Arc<BufferMemoryBarrier>>],
        image_memory_barriers: &[Pin<Arc<ImageMemoryBarrier>>],
    ) {
        Self::BARRIERS_CACHES.with(|local| {
            let mut vk_src_stage_mask = ash::vk::PipelineStageFlags::default();
            src_stage_mask
                .iter()
                .for_each(|flag| vk_src_stage_mask |= flag.to_ash());
            let mut vk_dst_stage_mask = ash::vk::PipelineStageFlags::default();
            dst_stage_mask
                .iter()
                .for_each(|flag| vk_dst_stage_mask |= flag.to_ash());

            let (mut memory_cache, mut buffer_cache, mut image_cache) = local.take();
            for barrier in memory_barriers {
                memory_cache.push(barrier.ash_vk_memory_barrier);
            }
            for barrier in buffer_memory_barriers {
                let buffer = barrier.buffer.clone();
                self.holding_resources
                    .write_buffers
                    .insert(buffer.ash_vk_buffer.as_raw(), buffer);
                buffer_cache.push(barrier.ash_vk_buffer_memory_barrier);
            }
            for barrier in image_memory_barriers {
                let image = barrier.image.clone();
                self.holding_resources
                    .write_images
                    .insert(barrier.image.vk_image.as_raw(), image);
                image_cache.push(barrier.ash_vk_image_memory_barrier);
            }
            unsafe {
                // Host Synchronization: command_buffer command_pool
                self.device.ash_device.cmd_pipeline_barrier(
                    self.vk_command_buffer,
                    vk_src_stage_mask,
                    vk_dst_stage_mask,
                    dependency_flags,
                    memory_cache.as_slice(),
                    buffer_cache.as_slice(),
                    image_cache.as_slice(),
                );
            }
            memory_cache.clear();
            buffer_cache.clear();
            image_cache.clear();
            local.set((memory_cache, buffer_cache, image_cache))
        });
    }
}
