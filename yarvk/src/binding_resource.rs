use crate::device_memory::{DeviceMemory, IMemoryRequirements, UnboundResource};
use ash::vk::DeviceSize;

pub trait BindingResource: Send + Sync {
    type RawTy: IMemoryRequirements;
    fn raw(&self) -> &Self::RawTy;
    fn raw_mut(&mut self) -> &mut Self::RawTy;
    fn offset(&self) -> DeviceSize; // Do not use ash::vk::* in public interface
    fn size(&self) -> DeviceSize;
}

pub struct BindMemoryInfo<'a, T: UnboundResource> {
    pub resource: T,
    pub memory: &'a DeviceMemory,
    pub memory_offset: DeviceSize,
}
