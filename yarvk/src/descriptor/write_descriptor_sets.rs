use crate::descriptor::descriptor_pool::DescriptorPool;
use crate::device::Device;
use crate::device_memory::MemoryRequirement;
use crate::image_view::ImageView;
use crate::sampler::Sampler;
use crate::{Buffer, BufferView};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rustc_hash::FxHashMap;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct DescriptorImageInfo {
    sampler: Option<Arc<Sampler>>,
    image_view: Option<Arc<ImageView>>,
    image_layout: ash::vk::ImageLayout,
}

impl DescriptorImageInfo {
    pub fn builder() -> DescriptorImageInfoBuilder {
        DescriptorImageInfoBuilder::default()
    }
    pub(crate) fn ash_builder(&self) -> ash::vk::DescriptorImageInfoBuilder {
        ash::vk::DescriptorImageInfo::builder()
            .sampler(
                self.sampler
                    .as_ref()
                    .map(|x| x.ash_vk_sampler)
                    .unwrap_or_default(),
            )
            .image_view(
                self.image_view
                    .as_ref()
                    .map(|x| x.ash_vk_image_view)
                    .unwrap_or_default(),
            )
            .image_layout(self.image_layout)
    }
}

#[derive(Default)]
pub struct DescriptorImageInfoBuilder {
    inner: DescriptorImageInfo,
}

impl DescriptorImageInfoBuilder {
    pub fn sampler(mut self, sampler: Arc<Sampler>) -> Self {
        self.inner.sampler = Some(sampler);
        self
    }
    pub fn image_view(mut self, image_view: Arc<ImageView>) -> Self {
        self.inner.image_view = Some(image_view);
        self
    }
    pub fn image_layout(mut self, image_layout: ash::vk::ImageLayout) -> Self {
        self.inner.image_layout = image_layout;
        self
    }
    pub fn build(self) -> DescriptorImageInfo {
        self.inner
    }
}

#[derive(Clone)]
pub struct DescriptorBufferInfo {
    buffer: Option<Arc<dyn Buffer>>,
    offset: ash::vk::DeviceSize,
    range: ash::vk::DeviceSize,
}

impl Default for DescriptorBufferInfo {
    fn default() -> Self {
        Self {
            buffer: None,
            offset: 0,
            range: ash::vk::WHOLE_SIZE,
        }
    }
}

impl DescriptorBufferInfo {
    pub fn builder() -> DescriptorBufferInfoBuilder {
        DescriptorBufferInfoBuilder {
            inner: Default::default(),
        }
    }
    pub(crate) fn ash_builder(&self) -> ash::vk::DescriptorBufferInfoBuilder {
        ash::vk::DescriptorBufferInfo::builder()
            .buffer(
                self.buffer
                    .as_ref()
                    .map_or(Default::default(), |x| x.ash_vk_buffer),
            )
            .offset(self.offset)
            .range(self.range)
    }
}

pub struct DescriptorBufferInfoBuilder {
    inner: DescriptorBufferInfo,
}

impl DescriptorBufferInfoBuilder {
    pub fn buffer(mut self, buffer: Arc<dyn Buffer>) -> Self {
        self.inner.range = buffer.get_memory_requirements().size;
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: ash::vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn range(mut self, range: ash::vk::DeviceSize) -> Self {
        self.inner.range = range;
        self
    }
    pub fn build(self) -> DescriptorBufferInfo {
        self.inner
    }
}

struct WriteImage<'a> {
    dst_array_element: u32,
    image_info: &'a [DescriptorImageInfo],
}

struct WriteBuffer<'a> {
    dst_array_element: u32,
    buffer_info: &'a [DescriptorBufferInfo],
}

struct WriteTexelBufferView<'a> {
    dst_array_element: u32,
    texel_buffer_view: &'a [BufferView],
}

enum DescriptorInfoType<'a> {
    Image(WriteImage<'a>),
    Buffer(WriteBuffer<'a>),
    TexelBufferView(WriteTexelBufferView<'a>),
}

pub struct WriteDescriptorSets<'a> {
    descriptor_pool: &'a mut DescriptorPool,
    write_descriptor_sets:
        FxHashMap<usize, FxHashMap<u32, DescriptorInfoType<'a>> /*bindings*/>,
    tmp_vk_image_infos: Vec<Vec<ash::vk::DescriptorImageInfo>>,
    tmp_vk_buffer_infos: Vec<Vec<ash::vk::DescriptorBufferInfo>>,
    tmp_vk_texel_buffer_views: Vec<Vec<ash::vk::BufferView>>,
}

impl<'a> WriteDescriptorSets<'a> {
    pub fn new(descriptor_pool: &'a mut DescriptorPool) -> Self {
        Self {
            descriptor_pool,
            write_descriptor_sets: Default::default(),
            tmp_vk_image_infos: vec![],
            tmp_vk_buffer_infos: vec![],
            tmp_vk_texel_buffer_views: vec![],
        }
    }
    pub fn update_image(
        &mut self,
        descriptor_index: usize,
        binding: u32,
        dst_array_element: u32,
        image_info: &'a [DescriptorImageInfo],
    ) {
        let bindings = self
            .write_descriptor_sets
            .entry(descriptor_index)
            .or_default();
        bindings.insert(
            binding,
            DescriptorInfoType::Image(WriteImage {
                dst_array_element,
                image_info,
            }),
        );
    }

    pub fn update_buffer(
        &mut self,
        descriptor_index: usize,
        binding: u32,
        dst_array_element: u32,
        buffer_info: &'a [DescriptorBufferInfo],
    ) {
        let bindings = self
            .write_descriptor_sets
            .entry(descriptor_index)
            .or_default();
        bindings.insert(
            binding,
            DescriptorInfoType::Buffer(WriteBuffer {
                dst_array_element,
                buffer_info,
            }),
        );
    }

    pub fn update_texel_buffer_view(
        &mut self,
        descriptor_index: usize,
        binding: u32,
        dst_array_element: u32,
        texel_buffer_view: &'a [BufferView],
    ) {
        let bindings = self
            .write_descriptor_sets
            .entry(descriptor_index)
            .or_default();
        bindings.insert(
            binding,
            DescriptorInfoType::TexelBufferView(WriteTexelBufferView {
                dst_array_element,
                texel_buffer_view,
            }),
        );
    }

    fn push_ash(&mut self, vec: &mut ParallelSplitWriteDescriptorSets) {
        for (descriptor_set_index, bindings) in &self.write_descriptor_sets {
            for (binding, descriptor_info_type) in bindings {
                if let Some(descriptor_set) = self
                    .descriptor_pool
                    .allocated_descriptor_sets
                    .get(descriptor_set_index)
                {
                    if let Some(descriptor_set_layout_binding) =
                        descriptor_set.descriptor_set_layout.bindings.get(binding)
                    {
                        let mut builder = ash::vk::WriteDescriptorSet::builder()
                            .dst_set(descriptor_set.ash_vk_descriptor_set)
                            .dst_binding(*binding)
                            .descriptor_type(
                                descriptor_set_layout_binding.descriptor_type().to_ash(),
                            );
                        match descriptor_info_type {
                            DescriptorInfoType::Image(image_info) => {
                                builder = builder.dst_array_element(image_info.dst_array_element);
                                self.tmp_vk_image_infos.push(
                                    image_info
                                        .image_info
                                        .iter()
                                        .map(|info| info.ash_builder().build())
                                        .collect(),
                                );
                                let slice = self.tmp_vk_image_infos.last().unwrap().as_slice();
                                builder = builder.image_info(slice);
                            }
                            DescriptorInfoType::Buffer(buffer_info) => {
                                builder = builder.dst_array_element(buffer_info.dst_array_element);
                                self.tmp_vk_buffer_infos.push(
                                    buffer_info
                                        .buffer_info
                                        .iter()
                                        .map(|info| info.ash_builder().build())
                                        .collect(),
                                );
                                let slice = self.tmp_vk_buffer_infos.last().unwrap();
                                builder = builder.buffer_info(slice);
                            }
                            DescriptorInfoType::TexelBufferView(ash_vk_buffer_views) => {
                                builder = builder
                                    .dst_array_element(ash_vk_buffer_views.dst_array_element);
                                self.tmp_vk_texel_buffer_views.push(
                                    ash_vk_buffer_views
                                        .texel_buffer_view
                                        .iter()
                                        .map(|info| info.ash_vk_buffer_view)
                                        .collect(),
                                );
                                let slice = self.tmp_vk_texel_buffer_views.last().unwrap();
                                builder = builder.texel_buffer_view(slice);
                            }
                        }
                        vec.fix_push(builder);
                    }
                }
            }
            vec.focus_next();
        }
    }

    /// update descriptors in parallel
    pub fn par_update(&mut self) {
        let mut vec = ParallelSplitWriteDescriptorSets::new();
        self.push_ash(&mut vec);
        vec.update(&self.descriptor_pool.device);
    }
}

struct SyncWriteDescriptorSet(ash::vk::WriteDescriptorSet);
unsafe impl Sync for SyncWriteDescriptorSet {}

struct ParallelSplitWriteDescriptorSets<'a> {
    focused: usize,
    current_num_threads: usize,
    vectors: Vec<Vec<SyncWriteDescriptorSet>>,
    _phantom_data: PhantomData<&'a usize>,
}

impl<'a> ParallelSplitWriteDescriptorSets<'a> {
    fn new() -> Self {
        let current_num_threads = rayon::current_num_threads();
        let mut vectors = Vec::new();
        vectors.resize_with(rayon::current_num_threads(), || Vec::new());
        Self {
            focused: 0,
            current_num_threads,
            vectors,
            _phantom_data: Default::default(),
        }
    }

    // push the same vector in the last time.
    fn fix_push(&mut self, write_descriptor_set: ash::vk::WriteDescriptorSetBuilder) {
        let vector = &mut self.vectors[self.focused];
        vector.push(SyncWriteDescriptorSet(write_descriptor_set.build()));
    }

    fn focus_next(&mut self) {
        self.focused = (self.focused + 1) % self.current_num_threads;
    }

    fn update(self, device: &Device) {
        self.vectors.par_iter().for_each(|vec| {
            unsafe {
                // Host Synchronization: VUID-vkUpdateDescriptorSets-pDescriptorWrites-06993
                //  pDescriptorWrites[i].dstSet pDescriptorCopies[i].dstSet
                device
                    .ash_device
                    .update_descriptor_sets(std::mem::transmute(vec.as_slice()), &[]);
            }
        })
    }
}

impl Device {
    pub fn par_update_descriptor_sets<
        'a,
        It: IntoIterator<Item = &'a mut WriteDescriptorSets<'a>>,
    >(
        &self,
        descriptor_writes: It,
    ) {
        let mut vec = ParallelSplitWriteDescriptorSets::new();
        for write_descriptor_set in descriptor_writes.into_iter() {
            write_descriptor_set.push_ash(&mut vec);
        }
        vec.update(&self);
    }
}
