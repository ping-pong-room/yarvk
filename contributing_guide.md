# How to contribute

## Code Style:
1. If your code implements validation rules without any cost, add a comment:
`// DONE VUID-vkDestroyPipelineLayout-pipelineLayout-02004`
2. If your code implements validation rules with any cost, add a comment:
`// Must VUID-VkDescriptorSetLayoutCreateInfo-binding-00279`, and panic if the check fails.
3. Use `Must VUID-XXX` only if when it is not checked, the vulkan will crash anyway. 
Otherwise do not check, for performance purpose, and it's validation layers' job.
4. Add Host Synchronization comments on vulkan funciton calling, for example:
```
// Host Synchronization: fence
            self.device.ash_device.destroy_fence(self.vk_fence, None);
```
5. If there's no Host Synchronization, add this: `Host Synchronization: none`
6. If a validation rule can be checked at compile time, it is encouraged to try, only if 
the implementation will change the whole codes too much or makes the API hard to use.
7. Follow Rust code styles, most of the can be checked by cargo or clippy.

## Commands:
First, please check the "Command Properties" in the sp, for example:

| Command Buffer Levels | Render Pass Scope |
|-----------------------|-------------------|
| Primary Secondary     | Both              |

Second, check the "Host Synchronization":


    Host access to commandBuffer must be externally synchronized

    Host access to the VkCommandPool that commandBuffer was allocated from must be externally synchronized



Then the command implementation should be:
```
impl<const LEVEL: Level, const SCOPE: RenderPassScope> CommandBuffer<LEVEL, { RECORDING }, SCOPE> {
    // DONE VUID-vkCmdBindPipeline-commandBuffer-recording
    pub fn cmd_bind_pipeline(
        &mut self, // host synchornization: command buffer
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        pipeline: &Pipeline,
    ) {
      let _pool = self.command_pool.write(); // host synchornization: command buffer pool
      ...
    }
```


## Vulkan Functions:
Vulkan functions is simple, just be careful the host synchronization.

## Vulkan Struct Types:
1. Do not create a `FooCrreateInfo` type, create a `FooBuilder` instead.
2. FooBuilder takes `mut self` as input and return `Self`
3. If the type has `p_next`, and can be a chain of `Bar`, `Baz`, it is suggested 
to add `Option<Bar>` and `Option<Baz>` as fields.

## Vulkan Flags
Most flags in yarvk are re-exposed from ash types. Most time you want to create a 
flag type for feature safety:
```
pub enum BufferCreateFlags {
    // DONE VUID-VkBufferCreateInfo-flags-00915
    SparseBinding(Feature<{ SparseBinding.into() }>),
    // DONE VUID-VkBufferCreateInfo-flags-00916
    SparseResidency(Feature<{ SparseResidencyBuffer.into() }>),
    // DONE VUID-VkBufferCreateInfo-flags-00917
    SparseAliased(Feature<{ SparseResidencyAliased.into() }>),
    // DONE VUID-VkBufferCreateInfo-flags-01887
    PROTECTED(Feature<{ ProtectedMemory.into() }>),
    // DONE VUID-VkBufferCreateInfo-flags-03338
    DeviceAddressCaptureReplay(Feature<{ BufferDeviceAddressCaptureReplay.into() }>),
}
```