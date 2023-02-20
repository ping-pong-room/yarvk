use crate::buffer::Buffer;
use std::sync::Arc;

pub struct BufferView {
    pub buffer: Arc<Buffer>,
    pub(crate) ash_vk_buffer_view: ash::vk::BufferView,
}

impl PartialEq for BufferView {
    fn eq(&self, other: &Self) -> bool {
        self.buffer.device == other.buffer.device
            && self.ash_vk_buffer_view == other.ash_vk_buffer_view
    }
}

impl BufferView {
    pub fn builder(buffer: Arc<Buffer>) -> BufferViewBuilder {
        BufferViewBuilder {
            buffer,
            flags: Default::default(),
            format: Default::default(),
            offset: 0,
            range: 0,
        }
    }
}

impl Drop for BufferView {
    fn drop(&mut self) {
        unsafe {
            // Host Synchronization: bufferView
            self.buffer
                .device
                .ash_device
                .destroy_buffer_view(self.ash_vk_buffer_view, None);
        }
    }
}

pub struct BufferViewBuilder {
    buffer: Arc<Buffer>,
    pub flags: ash::vk::BufferViewCreateFlags,
    pub format: ash::vk::Format,
    pub offset: ash::vk::DeviceSize,
    pub range: ash::vk::DeviceSize,
}

impl BufferViewBuilder {
    pub fn flags(mut self, flags: ash::vk::BufferViewCreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn format(mut self, format: ash::vk::Format) -> Self {
        self.format = format;
        self
    }
    pub fn offset(mut self, offset: ash::vk::DeviceSize) -> Self {
        self.offset = offset;
        self
    }
    pub fn range(mut self, range: ash::vk::DeviceSize) -> Self {
        self.range = range;
        self
    }
    pub fn build(self) -> Result<Arc<BufferView>, ash::vk::Result> {
        let create_info = ash::vk::BufferViewCreateInfo::builder()
            .flags(self.flags)
            .format(self.format)
            .offset(self.offset)
            .range(self.range)
            .buffer(self.buffer.ash_vk_buffer);
        unsafe {
            // Host Synchronization: none
            let ash_vk_buffer_view = self
                .buffer
                .device
                .ash_device
                .create_buffer_view(&create_info, None)?;
            Ok(Arc::new(BufferView {
                buffer: self.buffer,
                ash_vk_buffer_view,
            }))
        }
    }
}
