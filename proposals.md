1. https://github.com/KhronosGroup/Vulkan-Docs/issues/1828
2. https://github.com/KhronosGroup/Vulkan-Docs/issues/1829
3. Consider remove the host synchronization of pCreateInfo->surface when create_swapchain
4. Consider remove the host synchronization of image when destroy_image_view
5. Remove the descriptorType in VkWriteDescriptorSet since it must match dstBinding
