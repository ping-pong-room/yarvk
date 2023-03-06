use crate::command::command_buffer::CommandBuffer;
use crate::command::command_buffer::Level::PRIMARY;
use crate::command::command_buffer::RenderPassScope::OUTSIDE;
use crate::command::command_buffer::State::{EXECUTABLE, INITIAL, INVALID};
use crate::command::constant_command_buffer::ConstantCommandBuffer;
use crate::fence::{SignalingFence, UnsignaledFence};
use crate::pipeline::pipeline_stage_flags::PipelineStageFlag;
use crate::queue::Queue;
use crate::semaphore::Semaphore;
use crate::Handle;
use rustc_hash::FxHashMap;
use std::cell::UnsafeCell;
use std::sync::Arc;

thread_local! {
    static SUBMIT_INFO_CACHE: UnsafeCell<Vec<SubmitInfo<'static>>> = UnsafeCell::new(Vec::new());
    static SUBMITTABLE_CACHE: UnsafeCell<Vec<Submittable<'static>>> = UnsafeCell::new(Vec::new());
    static SUBMIT_RESULT_CACHE: UnsafeCell<Vec<SubmitResultRaw>> = UnsafeCell::new(Vec::new());
}

pub struct SubmitInfo<'a> {
    wait_semaphores: Vec<(&'a Semaphore, PipelineStageFlag)>,
    signal_semaphores: Vec<&'a Semaphore>,
    reusable_command_buffers: Vec<Arc<ConstantCommandBuffer>>,
    onetime_submit_command_buffers: Vec<CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>>,
    ash_vk_wait_semaphores: Vec<ash::vk::Semaphore>,
    ash_vk_wait_dst_stage_masks: Vec<ash::vk::PipelineStageFlags>,
    ash_vk_signal_semaphores: Vec<ash::vk::Semaphore>,
    ash_vk_command_buffers: Vec<ash::vk::CommandBuffer>,
}

impl<'a> SubmitInfo<'a> {
    pub fn builder() -> SubmitInfoBuilder<'a> {
        let cache = SUBMIT_INFO_CACHE.with(|unsafe_cell| unsafe {
            let vec = &mut *unsafe_cell.get();
            vec.pop().unwrap_or(SubmitInfo {
                wait_semaphores: vec![],
                signal_semaphores: vec![],
                reusable_command_buffers: vec![],
                onetime_submit_command_buffers: vec![],
                ash_vk_wait_semaphores: vec![],
                ash_vk_wait_dst_stage_masks: vec![],
                ash_vk_signal_semaphores: vec![],
                ash_vk_command_buffers: vec![],
            })
        });
        SubmitInfoBuilder { submit_info: cache }
    }
    fn clear(&mut self) {
        self.wait_semaphores.clear();
        self.signal_semaphores.clear();
        self.reusable_command_buffers.clear();
        self.onetime_submit_command_buffers.clear();

        self.ash_vk_wait_semaphores.clear();
        self.ash_vk_wait_dst_stage_masks.clear();
        self.ash_vk_signal_semaphores.clear();
        self.ash_vk_command_buffers.clear();
    }

    fn ash_builder(&mut self) -> ash::vk::SubmitInfoBuilder {
        for (semaphore, masks) in &self.wait_semaphores {
            self.ash_vk_wait_semaphores.push(semaphore.ash_vk_semaphore);
            self.ash_vk_wait_dst_stage_masks.push(masks.to_ash());
        }
        for buffer in &self.onetime_submit_command_buffers {
            self.ash_vk_command_buffers.push(buffer.vk_command_buffer);
        }
        for buffer in &self.reusable_command_buffers {
            self.ash_vk_command_buffers.push(buffer.vk_command_buffer);
        }
        for semaphore in &self.signal_semaphores {
            self.ash_vk_signal_semaphores
                .push(semaphore.ash_vk_semaphore);
        }
        ash::vk::SubmitInfo::builder()
            .wait_semaphores(self.ash_vk_wait_semaphores.as_slice())
            .wait_dst_stage_mask(self.ash_vk_wait_dst_stage_masks.as_slice())
            .command_buffers(self.ash_vk_command_buffers.as_slice())
            .signal_semaphores(self.ash_vk_signal_semaphores.as_slice())
    }
}

pub struct SubmitInfoBuilder<'a> {
    submit_info: SubmitInfo<'a>,
}

impl<'a> SubmitInfoBuilder<'a> {
    pub fn add_wait_semaphore(
        mut self,
        wait_semaphore: &'a Semaphore,
        wait_mask: PipelineStageFlag,
    ) -> Self {
        self.submit_info
            .wait_semaphores
            .push((wait_semaphore, wait_mask));
        self
    }
    pub fn add_constant_command_buffer(
        mut self,
        command_buffer: &Arc<ConstantCommandBuffer>,
    ) -> Self {
        // DONE VUID-VkSubmitInfo-pCommandBuffers-00075
        self.submit_info
            .reusable_command_buffers
            .push(command_buffer.clone());
        self
    }
    pub fn add_one_time_submit_command_buffer(
        mut self,
        command_buffer: CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>,
    ) -> Self {
        // DONE VUID-VkSubmitInfo-pCommandBuffers-00075
        self.submit_info
            .onetime_submit_command_buffers
            .push(command_buffer);
        self
    }
    pub fn add_signal_semaphore(mut self, signal_semaphore: &'a Semaphore) -> Self {
        self.submit_info.signal_semaphores.push(signal_semaphore);
        self
    }

    pub fn build(self) -> SubmitInfo<'a> {
        self.submit_info
    }
}

pub struct Submittable<'a> {
    pub(crate) submit_infos: Vec<SubmitInfo<'a>>,
    pub(crate) vk_submit_infos: Vec<ash::vk::SubmitInfo>,
}

impl<'a> Submittable<'a> {
    pub fn new() -> Self {
        SUBMITTABLE_CACHE.with(|unsafe_cell| unsafe {
            let vec = &mut *unsafe_cell.get();
            vec.pop().unwrap_or(Submittable {
                submit_infos: vec![],
                vk_submit_infos: vec![],
            })
        })
    }
    pub fn add_submit_info(mut self, submit_info: SubmitInfo<'a>) -> Self {
        self.submit_infos.push(submit_info);
        self
    }
    pub fn submit(
        mut self,
        queue: &mut Queue,
        fence: UnsignaledFence,
    ) -> Result<SignalingFence<SubmitResult>, ash::vk::Result> {
        let mut submit_result = SUBMIT_RESULT_CACHE.with(|unsafe_cell| unsafe {
            let vec = &mut *unsafe_cell.get();
            match vec.pop() {
                Some(raw) => SubmitResult {
                    invalid_command_buffers: raw.invalid_command_buffers,
                },
                None => SubmitResult::default(),
            }
        });
        // Host Synchronization: queue fence
        // DONE VUID-vkQueueSubmit-fence-00063
        // DONE VUID-vkQueueSubmit-fence-00064
        // DONE VUID-vkQueueSubmit-pWaitSemaphores-00068
        // DONE VUID-vkQueueSubmit-pSignalSemaphores-00067
        for info in &mut self.submit_infos {
            self.vk_submit_infos.push(info.ash_builder().build());
        }
        unsafe {
            // Host Synchronization: queue fence
            queue.device.ash_device.queue_submit(
                queue.vk_queue,
                self.vk_submit_infos.as_slice(),
                fence.vk_fence,
            )?
        };

        // put all submit infos struct back to cache
        while !self.submit_infos.is_empty() {
            let mut info = self.submit_infos.pop().unwrap();
            while !info.onetime_submit_command_buffers.is_empty() {
                let onetime_submit_command_buffer =
                    info.onetime_submit_command_buffers.pop().unwrap();

                submit_result
                    .invalid_command_buffers
                    .insert(onetime_submit_command_buffer.handle(), unsafe {
                        std::mem::transmute(onetime_submit_command_buffer)
                    });
            }
            info.clear();
            SUBMIT_INFO_CACHE.with(|unsafe_cell| unsafe {
                let vec = &mut *unsafe_cell.get();
                vec.push(std::mem::transmute(info));
            });
        }

        // put submittable struct back to cache
        SUBMITTABLE_CACHE.with(|unsafe_cell| unsafe {
            let vec = &mut *unsafe_cell.get();
            self.vk_submit_infos.clear();
            vec.push(std::mem::transmute(self));
        });

        Ok(SignalingFence {
            inner: fence.0,
            t: submit_result,
        })
    }
}

#[derive(Default)]
pub struct SubmitResult {
    invalid_command_buffers: FxHashMap<u64, CommandBuffer<{ PRIMARY }, { INVALID }, { OUTSIDE }>>,
}
impl SubmitResult {
    pub fn add_primary_buffer(
        &mut self,
        buffer: CommandBuffer<{ PRIMARY }, { INITIAL }, { OUTSIDE }>,
    ) {
        self.invalid_command_buffers
            .insert(buffer.handle(), unsafe { std::mem::transmute(buffer) });
    }
    pub fn take_invalid_primary_buffer(
        &mut self,
        handler: &u64,
    ) -> Option<CommandBuffer<{ PRIMARY }, { INVALID }, { OUTSIDE }>> {
        let buffer = self.invalid_command_buffers.remove(handler);
        match buffer {
            None => None,
            Some(buffer) => unsafe { std::mem::transmute(buffer) },
        }
    }
}

impl Drop for SubmitResult {
    fn drop(&mut self) {
        SUBMIT_RESULT_CACHE.with(|unsafe_cell| unsafe {
            let vec = &mut *unsafe_cell.get();
            vec.push(SubmitResultRaw {
                invalid_command_buffers: std::mem::take(&mut self.invalid_command_buffers),
            });
        });
    }
}
struct SubmitResultRaw {
    invalid_command_buffers: FxHashMap<u64, CommandBuffer<{ PRIMARY }, { INVALID }, { OUTSIDE }>>,
}
