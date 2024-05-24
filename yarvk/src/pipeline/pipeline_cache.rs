use crate::device::Device;
use crate::device_features::physical_device_pipeline_creation_cache_control_features::FeaturePipelineCreationCacheControl;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct PipelineCache {}
impl PipelineCache {
    pub fn builder(device: &Arc<Device>) -> PipelineCacheBuilder {
        PipelineCacheBuilder {
            device: device.clone(),
            initial_data: None,
        }
    }
}

pub struct PipelineCacheMergeBuilder<'a> {
    device: Arc<Device>,
    dst_pipelines: ash::vk::PipelineCache,
    src_pipelines: Vec<ash::vk::PipelineCache>,
    _phantom_data: PhantomData<&'a usize>,
}

impl<'a> PipelineCacheMergeBuilder<'a> {
    pub fn add<const EXTERNALLY_SYNCHRONIZED: bool>(
        mut self,
        pipeline_cache: &'a PipelineCacheImpl<EXTERNALLY_SYNCHRONIZED>,
    ) -> Self {
        self.src_pipelines.push(pipeline_cache.vk_pipeline_cache);
        self
    }
    pub fn build(self) -> Result<(), ash::vk::Result> {
        unsafe {
            // Host Synchronization
            //Host access to dstCache must be externally synchronized
            self.device
                .ash_device
                .merge_pipeline_caches(self.dst_pipelines, self.src_pipelines.as_slice())
        }
    }
}

pub struct PipelineCacheImpl<const EXTERNALLY_SYNCHRONIZED: bool> {
    pub device: Arc<Device>,
    pub(crate) vk_pipeline_cache: ash::vk::PipelineCache,
}

impl<const EXTERNALLY_SYNCHRONIZED: bool> PipelineCacheImpl<EXTERNALLY_SYNCHRONIZED> {
    pub fn get_pipeline_cache_data(&self) -> Result<Vec<u8>, ash::vk::Result> {
        unsafe {
            self.device
                .ash_device
                .get_pipeline_cache_data(self.vk_pipeline_cache)
        }
    }
    pub fn merge_builder(&mut self) -> PipelineCacheMergeBuilder {
        // DONE VUID-vkMergePipelineCaches-dstCache-00770
        PipelineCacheMergeBuilder {
            device: self.device.clone(),
            dst_pipelines: self.vk_pipeline_cache,
            src_pipelines: vec![],
            _phantom_data: Default::default(),
        }
    }
}

impl<const EXTERNALLY_SYNCHRONIZED: bool> Drop for PipelineCacheImpl<EXTERNALLY_SYNCHRONIZED> {
    fn drop(&mut self) {
        unsafe {
            self.device
                .ash_device
                .destroy_pipeline_cache(self.vk_pipeline_cache, None)
        }
    }
}

pub struct PipelineCacheBuilder<'a> {
    device: Arc<Device>,
    initial_data: Option<&'a [u8]>,
}
impl<'a> PipelineCacheBuilder<'a> {
    pub fn initial_data(mut self, data: &'a [u8]) -> Self {
        self.initial_data = Some(data);
        self
    }

    pub fn build_externally_synchronized(
        self,
        _feature: &FeaturePipelineCreationCacheControl,
    ) -> Result<PipelineCacheImpl<true>, ash::vk::Result> {
        let device = self.device.clone();
        let vk_pipeline_cache = self.build(true)?;
        Ok(PipelineCacheImpl {
            device,
            vk_pipeline_cache,
        })
    }
    pub fn build_internally_synchronized(
        self,
    ) -> Result<PipelineCacheImpl<false>, ash::vk::Result> {
        let device = self.device.clone();
        let vk_pipeline_cache = self.build(false)?;
        Ok(PipelineCacheImpl {
            device,
            vk_pipeline_cache,
        })
    }
    fn build(
        self,
        externally_synchronized: bool,
    ) -> Result<ash::vk::PipelineCache, ash::vk::Result> {
        let mut flags = ash::vk::PipelineCacheCreateFlags::default();
        if externally_synchronized {
            flags |= ash::vk::PipelineCacheCreateFlags::EXTERNALLY_SYNCHRONIZED
        }
        let mut create_info_builder = ash::vk::PipelineCacheCreateInfo::builder().flags(flags);
        if let Some(initial_data) = self.initial_data {
            create_info_builder = create_info_builder.initial_data(initial_data);
        }
        let create_info = create_info_builder.build();
        let vk_pipeline_cache = unsafe {
            self.device
                .ash_device
                .create_pipeline_cache(&create_info, None)?
        };
        Ok(vk_pipeline_cache)
    }
}
