#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(specialization)]
#![feature(min_specialization)]
#![feature(once_cell)]
pub mod buffer;
pub mod command;
pub mod debug_utils_messenger;
pub mod device;
pub mod device_features;
pub mod device_memory;
pub mod entry;
pub mod extensions;
pub mod fence;
pub mod frame_buffer;
pub mod image;
pub mod instance;
pub mod physical_device;
pub mod queue;
pub mod render_pass;
pub mod surface;
pub mod swapchain;
pub mod window;

pub mod barrier;
pub mod pipeline;
pub mod sampler;
pub mod semaphore;
pub mod shader_module;
pub mod descriptor_set;

pub use ash::util::read_spv;
pub use ash::vk::AccessFlags;
pub use ash::vk::AttachmentDescriptionFlags;
pub use ash::vk::AttachmentLoadOp;
pub use ash::vk::AttachmentStoreOp;
pub use ash::vk::BlendOp;
pub use ash::vk::BorderColor;
pub use ash::vk::BufferImageCopy;
pub use ash::vk::BufferUsageFlags;
pub use ash::vk::ClearColorValue;
pub use ash::vk::ClearDepthStencilValue;
pub use ash::vk::ClearValue;
pub use ash::vk::ColorComponentFlags;
pub use ash::vk::ColorSpaceKHR;
pub use ash::vk::CompareOp;
pub use ash::vk::ComponentMapping;
pub use ash::vk::ComponentSwizzle;
pub use ash::vk::CompositeAlphaFlagsKHR;
pub use ash::vk::DebugUtilsMessageSeverityFlagsEXT;
pub use ash::vk::DebugUtilsMessageTypeFlagsEXT;
pub use ash::vk::DependencyFlags;
pub use ash::vk::DescriptorPoolSize;
pub use ash::vk::DeviceSize;
pub use ash::vk::Extent2D;
pub use ash::vk::Extent3D;
pub use ash::vk::Filter;
pub use ash::vk::Format;
pub use ash::vk::FrontFace;
pub use ash::vk::ImageAspectFlags;
pub use ash::vk::ImageCreateFlags;
pub use ash::vk::ImageLayout;
pub use ash::vk::ImageSubresourceLayers;
pub use ash::vk::ImageTiling;
pub use ash::vk::ImageType;
pub use ash::vk::ImageUsageFlags;
pub use ash::vk::IndexType;
pub use ash::vk::LogicOp;
pub use ash::vk::MemoryPropertyFlags;
pub use ash::vk::MemoryRequirements;
pub use ash::vk::PhysicalDeviceAccelerationStructurePropertiesKHR;
pub use ash::vk::PhysicalDeviceBlendOperationAdvancedPropertiesEXT;
pub use ash::vk::PhysicalDeviceConservativeRasterizationPropertiesEXT;
pub use ash::vk::PhysicalDeviceCooperativeMatrixPropertiesNV;
pub use ash::vk::PhysicalDeviceCustomBorderColorPropertiesEXT;
pub use ash::vk::PhysicalDeviceDepthStencilResolveProperties;
pub use ash::vk::PhysicalDeviceDescriptorIndexingProperties;
pub use ash::vk::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV;
pub use ash::vk::PhysicalDeviceDiscardRectanglePropertiesEXT;
pub use ash::vk::PhysicalDeviceDriverProperties;
pub use ash::vk::PhysicalDeviceDrmPropertiesEXT;
pub use ash::vk::PhysicalDeviceExternalMemoryHostPropertiesEXT;
pub use ash::vk::PhysicalDeviceFloatControlsProperties;
pub use ash::vk::PhysicalDeviceFragmentDensityMap2PropertiesEXT;
pub use ash::vk::PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM;
pub use ash::vk::PhysicalDeviceFragmentDensityMapPropertiesEXT;
pub use ash::vk::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
pub use ash::vk::PhysicalDeviceFragmentShadingRatePropertiesKHR;
pub use ash::vk::PhysicalDeviceIDProperties;
pub use ash::vk::PhysicalDeviceInlineUniformBlockProperties;
pub use ash::vk::PhysicalDeviceLineRasterizationPropertiesEXT;
pub use ash::vk::PhysicalDeviceMaintenance3Properties;
pub use ash::vk::PhysicalDeviceMaintenance4Properties;
pub use ash::vk::PhysicalDeviceMemoryBudgetPropertiesEXT;
pub use ash::vk::PhysicalDeviceMeshShaderPropertiesNV;
pub use ash::vk::PhysicalDeviceMultiDrawPropertiesEXT;
pub use ash::vk::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX;
pub use ash::vk::PhysicalDeviceMultiviewProperties;
pub use ash::vk::PhysicalDevicePCIBusInfoPropertiesEXT;
pub use ash::vk::PhysicalDevicePerformanceQueryPropertiesKHR;
pub use ash::vk::PhysicalDevicePointClippingProperties;
pub use ash::vk::PhysicalDevicePortabilitySubsetPropertiesKHR;
pub use ash::vk::PhysicalDeviceProtectedMemoryProperties;
pub use ash::vk::PhysicalDeviceProvokingVertexPropertiesEXT;
pub use ash::vk::PhysicalDevicePushDescriptorPropertiesKHR;
pub use ash::vk::PhysicalDeviceRayTracingPipelinePropertiesKHR;
pub use ash::vk::PhysicalDeviceRayTracingPropertiesNV;
pub use ash::vk::PhysicalDeviceRobustness2PropertiesEXT;
pub use ash::vk::PhysicalDeviceSampleLocationsPropertiesEXT;
pub use ash::vk::PhysicalDeviceSamplerFilterMinmaxProperties;
pub use ash::vk::PhysicalDeviceShaderCoreProperties2AMD;
pub use ash::vk::PhysicalDeviceShaderCorePropertiesAMD;
pub use ash::vk::PhysicalDeviceShaderIntegerDotProductProperties;
pub use ash::vk::PhysicalDeviceShaderSMBuiltinsPropertiesNV;
pub use ash::vk::PhysicalDeviceShadingRateImagePropertiesNV;
pub use ash::vk::PhysicalDeviceSubgroupProperties;
pub use ash::vk::PhysicalDeviceSubgroupSizeControlProperties;
pub use ash::vk::PhysicalDeviceSubpassShadingPropertiesHUAWEI;
pub use ash::vk::PhysicalDeviceTexelBufferAlignmentProperties;
pub use ash::vk::PhysicalDeviceTimelineSemaphoreProperties;
pub use ash::vk::PhysicalDeviceTransformFeedbackPropertiesEXT;
pub use ash::vk::PhysicalDeviceType;
pub use ash::vk::PhysicalDeviceVertexAttributeDivisorPropertiesEXT;
pub use ash::vk::PhysicalDeviceVulkan11Properties;
pub use ash::vk::PhysicalDeviceVulkan12Properties;
pub use ash::vk::PhysicalDeviceVulkan13Properties;
pub use ash::vk::PipelineBindPoint;
pub use ash::vk::PresentModeKHR;
pub use ash::vk::QueryControlFlags;
pub use ash::vk::QueryPipelineStatisticFlags;
pub use ash::vk::QueueFlags;
pub use ash::vk::Rect2D;
pub use ash::vk::Result;
pub use ash::vk::SampleCountFlags;
pub use ash::vk::SamplerAddressMode;
pub use ash::vk::SamplerCreateFlags;
pub use ash::vk::SamplerMipmapMode;
pub use ash::vk::StencilOp;
pub use ash::vk::StencilOpState;
pub use ash::vk::SubpassContents;
pub use ash::vk::SurfaceTransformFlagsKHR;
pub use ash::vk::SwapchainCreateFlagsKHR;
pub use ash::vk::VertexInputRate;
pub use ash::vk::Viewport;
pub use ash::vk::SUBPASS_EXTERNAL;
pub use ash::vk::ExtendsMemoryRequirements2;
pub use ash::vk::SurfaceFormatKHR;
pub use ash::vk::CullModeFlags;

pub use buffer::*;
pub use image::*;

pub trait Handle {
    fn handle(&self) -> u64;
}
