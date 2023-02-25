use crate::device::Device;
use std::sync::Arc;

pub struct ShaderModule {
    pub device: Arc<Device>,
    pub(crate) ash_vk_shader_module: ash::vk::ShaderModule,
}

impl ShaderModule {
    pub fn builder<'a>(device: &Arc<Device>, code: &'a [u32]) -> ShaderModuleBuilder<'a> {
        ShaderModuleBuilder { device: device.clone(), code }
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            // Host Synchronization: shadermodule
            self.device
                .ash_device
                .destroy_shader_module(self.ash_vk_shader_module, None);
        }
    }
}

pub struct ShaderModuleBuilder<'a> {
    device: Arc<Device>,
    code: &'a [u32],
}

impl<'a> ShaderModuleBuilder<'a> {
    pub fn build(self) -> Result<Arc<ShaderModule>, ash::vk::Result> {
        let shader_info = ash::vk::ShaderModuleCreateInfo::builder()
            .code(&self.code)
            .build();
        let ash_vk_shader_module = unsafe {
            // Host Synchronization: none
            self.device
                .ash_device
                .create_shader_module(&shader_info, None)?
        };
        Ok(Arc::new(ShaderModule {
            device: self.device,
            ash_vk_shader_module,
        }))
    }
}
