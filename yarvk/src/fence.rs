use crate::device::Device;
use derive_more::Deref;
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::sync::Arc;

pub struct Fence {
    pub device: Arc<Device>,
    pub(crate) vk_fence: ash::vk::Fence,
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            // TODO VUID-vkDestroyFence-fence-01121
            // TODO VUID-vkDestroyFence-fence-01122
            // Host Synchronization: fence
            self.device.ash_device.destroy_fence(self.vk_fence, None);
        }
    }
}

impl Fence {
    pub fn new(device: &Arc<Device>) -> Result<UnsignaledFence, ash::vk::Result> {
        let create_info = ash::vk::FenceCreateInfo::builder()
            .flags(ash::vk::FenceCreateFlags::default())
            .build();
        // Host Synchronization: none
        let vk_fence = unsafe { device.ash_device.create_fence(&create_info, None)? };
        Ok(UnsignaledFence(Fence { device: device.clone(), vk_fence }))
    }

    pub fn new_signaling<T>(
        device: &Arc<Device>,
        t: T,
    ) -> Result<SignalingFence<T>, ash::vk::Result> {
        let create_info = ash::vk::FenceCreateInfo::builder()
            .flags(ash::vk::FenceCreateFlags::SIGNALED)
            .build();
        // Host Synchronization: none
        let vk_fence = unsafe { device.ash_device.create_fence(&create_info, None)? };
        Ok(SignalingFence {
            inner: Fence { device: device.clone(), vk_fence },
            t,
        })
    }
}

#[derive(Deref)]
pub struct UnsignaledFence(pub(crate) Fence);

impl UnsignaledFence {
    pub(crate) fn to_executing_fence<T>(self, t: T) -> SignalingFence<T> {
        SignalingFence { inner: self.0, t }
    }
}

#[derive(Deref)]
pub struct SignalingFence<T> {
    #[deref]
    pub(crate) inner: Fence,
    pub(crate) t: T,
}

pub struct TimeoutError<T> {
    pub error: ash::vk::Result,
    pub fence: SignalingFence<T>,
}

impl<T> Debug for TimeoutError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl<T> SignalingFence<T> {
    pub fn wait(self) -> Result<(SignaledFence, T), TimeoutError<T>> {
        self.wait_timeout(u64::MAX)
    }
    pub fn wait_timeout(self, timeout: u64) -> Result<(SignaledFence, T), TimeoutError<T>> {
        unsafe {
            // Host Synchronization: none
            match self
                .device
                .ash_device
                .wait_for_fences(&[self.vk_fence], true, timeout)
            {
                Ok(_) => {
                    let fence = ManuallyDrop::new(self);
                    let fence_inner = std::ptr::read(&fence.inner);
                    let t = std::ptr::read(&fence.t);
                    Ok((SignaledFence(fence_inner), t))
                }
                Err(error) => Err(TimeoutError { error, fence: self }),
            }
        }
    }
}

impl<T> Drop for SignalingFence<T> {
    fn drop(&mut self) {
        // MUST VUID-vkDestroyFence-fence-01120
        panic!("fence is dropped when using");
    }
}

#[derive(Deref)]
pub struct SignaledFence(pub(crate) Fence);

impl SignaledFence {
    pub fn reset(self) -> Result<UnsignaledFence, ash::vk::Result> {
        // DONE VUID-vkResetFences-pFences-01123
        // Host Synchronization: fence
        unsafe {
            self.device.ash_device.reset_fences(&[self.vk_fence])?;
            Ok(UnsignaledFence(self.0))
        }
    }
}

// TODO batching fence wait and reset
