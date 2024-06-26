#![feature(const_trait_impl)]

use raw_window_handle::HasRawDisplayHandle;
use raw_window_handle::HasRawWindowHandle;
use std::collections::HashMap;
use std::ffi::CStr;
use std::io::Cursor;
use std::sync::Arc;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use yarvk::barrier::ImageMemoryBarrier;
use yarvk::command::command_buffer::Level::{PRIMARY, SECONDARY};
use yarvk::command::command_buffer::RenderPassScope::OUTSIDE;
use yarvk::command::command_buffer::State::{EXECUTABLE, INITIAL};
use yarvk::command::command_buffer::{
    CommandBuffer, CommandBufferInheritanceInfo, TransientCommandBuffer,
};
use yarvk::debug_utils_messenger::DebugUtilsMessengerCreateInfoEXT;
use yarvk::descriptor_set::desccriptor_pool::DescriptorPool;
use yarvk::descriptor_set::descriptor_set::{DescriptorSetValue, IDescriptorSet};
use yarvk::descriptor_set::descriptor_set_layout::DescriptorSetLayout;
use yarvk::descriptor_set::descriptor_type::DescriptorKind;
use yarvk::descriptor_set::descriptor_variadic_generics::{
    ConstDescriptorSetValue2, DescriptorSetValue2,
};
use yarvk::device::{Device, DeviceQueueCreateInfo};
use yarvk::device_memory::dedicated_memory::{DedicatedResource, MemoryDedicatedAllocateInfo};
use yarvk::device_memory::{DeviceMemory, IMemoryRequirements, UnboundResource};
use yarvk::entry::Entry;

use yarvk::device_features::physical_device_features::FeatureSamplerAnisotropy;
use yarvk::extensions::{ExtensionKhrSurface, ExtensionKhrSwapchain};
use yarvk::fence::{Fence, UnsignaledFence};
use yarvk::frame_buffer::Framebuffer;
use yarvk::image::image_subresource_range::ImageSubresourceRange;
use yarvk::image::image_view::{ImageView, ImageViewType};
use yarvk::instance::{ApplicationInfo, Instance};
use yarvk::physical_device::memory_properties::{MemoryType, PhysicalDeviceMemoryProperties};
use yarvk::physical_device::SharingMode;
use yarvk::pipeline::color_blend_state::{
    BlendFactor, PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo,
};
use yarvk::pipeline::depth_stencil_state::PipelineDepthStencilStateCreateInfo;
use yarvk::pipeline::input_assembly_state::{
    PipelineInputAssemblyStateCreateInfo, PrimitiveTopology,
};
use yarvk::pipeline::multisample_state::PipelineMultisampleStateCreateInfo;
use yarvk::pipeline::pipeline_stage_flags::PipelineStageFlag;
use yarvk::pipeline::rasterization_state::{PipelineRasterizationStateCreateInfo, PolygonMode};
use yarvk::pipeline::shader_stage::{PipelineShaderStageCreateInfo, ShaderStage};
use yarvk::pipeline::vertex_input_state::{
    PipelineVertexInputStateCreateInfo, VertexInputAttributeDescription,
    VertexInputBindingDescription,
};
use yarvk::pipeline::{Pipeline, PipelineCacheType, PipelineLayout};
use yarvk::queue::submit_info::{SubmitInfo, Submittable};
use yarvk::queue::Queue;
use yarvk::render_pass::attachment::{AttachmentDescription, AttachmentReference};
use yarvk::render_pass::render_pass_begin_info::RenderPassBeginInfo;
use yarvk::render_pass::subpass::{SubpassDependency, SubpassDescription};
use yarvk::render_pass::RenderPass;
use yarvk::sampler::Sampler;
use yarvk::semaphore::Semaphore;
use yarvk::shader_module::ShaderModule;
use yarvk::surface::Surface;
use yarvk::swapchain::{PresentInfo, Swapchain};
use yarvk::window::enable_required_wsi_extensions;
use yarvk::{read_spv, ContinuousBuffer, ContinuousImage, Handle, WHOLE_SIZE};
use yarvk::{
    AccessFlags, AttachmentLoadOp, AttachmentStoreOp, BlendOp, BorderColor, BufferImageCopy,
    BufferUsageFlags, ClearColorValue, ClearDepthStencilValue, ClearValue, ColorComponentFlags,
    CompareOp, ComponentMapping, ComponentSwizzle, CompositeAlphaFlagsKHR,
    DebugUtilsMessageSeverityFlagsEXT, DependencyFlags, Extent2D, Extent3D, Filter, Format,
    FrontFace, ImageAspectFlags, ImageLayout, ImageSubresourceLayers, ImageTiling, ImageType,
    ImageUsageFlags, IndexType, MemoryPropertyFlags, MemoryRequirements, PipelineBindPoint,
    PresentModeKHR, QueueFlags, Rect2D, SampleCountFlags, SamplerAddressMode, SamplerMipmapMode,
    StencilOp, StencilOpState, SubpassContents, SurfaceTransformFlagsKHR, VertexInputRate,
    Viewport, SUBPASS_EXTERNAL,
};

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}

#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    uv: [f32; 2],
}

#[derive(Clone, Debug, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub _pad: f32,
}

pub fn submit(
    queue: &mut Queue,
    buffer: CommandBuffer<{ PRIMARY }, { EXECUTABLE }, { OUTSIDE }>,
    fence: UnsignaledFence,
) -> (
    UnsignaledFence,
    CommandBuffer<{ PRIMARY }, { INITIAL }, { OUTSIDE }>,
) {
    let handler = buffer.handle();
    let submit_info = SubmitInfo::builder()
        .add_one_time_submit_command_buffer(buffer)
        .build();
    let fence = Submittable::new()
        .add_submit_info(submit_info)
        .submit(queue, fence)
        .expect("queue submit failed.");
    let (fence, mut invalid_buffers) = fence.wait().unwrap();
    let fence = fence.reset().unwrap();
    let buffer = invalid_buffers
        .take_invalid_primary_buffer(&handler)
        .unwrap();
    let buffer = buffer.reset().unwrap();
    (fence, buffer)
}

pub fn find_memory_type_index<'a>(
    memory_req: &MemoryRequirements,
    memory_prop: &'a PhysicalDeviceMemoryProperties,
    flags: MemoryPropertyFlags,
) -> Option<&'a MemoryType> {
    memory_prop.memory_types().iter().find(|&memory_type| {
        (1 << memory_type.index) & memory_req.memory_type_bits != 0
            && memory_type.property_flags & flags == flags
    })
}

type MyDescriptorLayout = DescriptorSetValue2<
    0,
    { DescriptorKind::UniformBuffer },
    1,
    1,
    { DescriptorKind::CombinedImageSamplerImmutable },
    1,
>;

fn main() {
    let window_width = 1920;
    let window_height = 1080;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("yarvk_example")
        .with_inner_size(winit::dpi::LogicalSize::new(
            f64::from(1920),
            f64::from(1080),
        ))
        .build(&event_loop)
        .unwrap();

    let entry = Entry::load().unwrap();

    let layer = unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") };
    // let debug_utils_messenger_callback = DebugUtilsMessengerCreateInfoEXT::builder()
    //     .callback(|message_severity, message_type, p_callback_data| {
    //         let message_id_number = p_callback_data.message_id_number;
    //         let message_id_name = p_callback_data.p_message_id_name;
    //         let message = p_callback_data.p_message;
    //         println!(
    //             "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    //         );
    //     })
    //     .severity(
    //         // DebugUtilsMessageSeverityFlagsEXT::VERBOSE | DebugUtilsMessageSeverityFlagsEXT::INFO |
    //         DebugUtilsMessageSeverityFlagsEXT::WARNING | DebugUtilsMessageSeverityFlagsEXT::ERROR,
    //     )
    //     .build();
    let application_info = ApplicationInfo::builder()
        .engine_name("yarvk_example")
        .build();
    let mut instance_builder = Instance::builder(entry)
        .application_info(application_info)
        .add_layer(layer)
        // .debug_utils_messenger_exts(vec![debug_utils_messenger_callback])
        ;
    instance_builder =
        enable_required_wsi_extensions(window.raw_display_handle(), instance_builder).unwrap();
    let instance = instance_builder.build().unwrap();
    let khr_surface_ext = instance.get_extension::<ExtensionKhrSurface>().unwrap();
    let pdevices = instance.enumerate_physical_devices().unwrap();
    let (pdevice, queue_family, surface) = pdevices
        .iter()
        .filter_map(|pdevice| {
            pdevice
                .get_physical_device_queue_family_properties()
                .into_iter()
                .filter_map(|queue_family_properties| {
                    if let Some(surface) = Surface::get_physical_device_surface_support(
                        khr_surface_ext.clone(),
                        window.raw_display_handle(),
                        window.raw_window_handle(),
                        &queue_family_properties,
                    )
                    .unwrap()
                    {
                        if queue_family_properties
                            .queue_flags
                            .contains(QueueFlags::GRAPHICS)
                        {
                            return Some((pdevice, queue_family_properties, surface));
                        }
                    }
                    None
                })
                .next()
        })
        .next()
        .expect("Couldn't find suitable device.");
    // let portable_property = pdevice.get_physical_device_properties2::<PhysicalDevicePortabilitySubsetPropertiesKHR>();
    // println!("min_vertex_input_binding_stride_alignment: {}", portable_property.min_vertex_input_binding_stride_alignment);
    // let prop2_ext = instance.get_extension::<{ PhysicalInstanceExtensionType::KhrGetPhysicalDeviceProperties2 }>().unwrap();
    let surface_ext = instance.get_extension::<ExtensionKhrSurface>().unwrap();
    let queue_create_info = DeviceQueueCreateInfo::builder(queue_family.clone())
        .add_priority(0.9)
        .build();
    let (device, mut queues) = Device::builder(pdevice)
        .add_queue_info(queue_create_info)
        .add_extension::<ExtensionKhrSwapchain>(surface_ext)
        .unwrap()
        // .add_feature(DeviceFeatures::LogicOp)
        .add_feature::<FeatureSamplerAnisotropy>(())
        .unwrap()
        .build()
        .unwrap();
    let feature_sampler_anisotropy = device.get_feature::<FeatureSamplerAnisotropy>().unwrap();
    let swapchian_extension = device.get_extension::<ExtensionKhrSwapchain>().unwrap();
    let mut present_queue = queues.get_mut(&queue_family).unwrap().pop().unwrap();
    let surface_format = surface.get_physical_device_surface_formats()[0];
    let surface_capabilities = surface.get_physical_device_surface_capabilities();
    let mut desired_image_count = surface_capabilities.min_image_count + 1;
    if surface_capabilities.max_image_count > 0
        && desired_image_count > surface_capabilities.max_image_count
    {
        desired_image_count = surface_capabilities.max_image_count;
    }
    let surface_resolution = match surface_capabilities.current_extent.width {
        u32::MAX => Extent2D {
            width: window_width,
            height: window_height,
        },
        _ => surface_capabilities.current_extent,
    };
    let pre_transform = if surface_capabilities
        .supported_transforms
        .contains(SurfaceTransformFlagsKHR::IDENTITY)
    {
        SurfaceTransformFlagsKHR::IDENTITY
    } else {
        surface_capabilities.current_transform
    };
    let present_modes = surface.get_physical_device_surface_present_modes();
    let present_mode = present_modes
        .iter()
        .cloned()
        .find(|&mode| mode == PresentModeKHR::MAILBOX)
        .unwrap_or(PresentModeKHR::FIFO);
    let mut swapchain = Swapchain::builder(surface.clone(), swapchian_extension)
        .min_image_count(desired_image_count)
        .image_color_space(surface_format.color_space)
        .image_format(surface_format.format)
        .image_extent(surface_resolution)
        .image_sharing_mode(SharingMode::EXCLUSIVE)
        .pre_transform(pre_transform)
        .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped()
        .image_array_layers(1)
        .build()
        .unwrap();
    let setup_command_buffer =
        TransientCommandBuffer::<{ PRIMARY }>::new(&device, queue_family.clone()).unwrap();
    let command_buffer_handler = setup_command_buffer.handle();
    let present_images = swapchain.get_swapchain_images();
    let present_image_views: Vec<Arc<ImageView>> = present_images
        .iter()
        .map(|image| {
            ImageView::builder(image.clone())
                .view_type(ImageViewType::Type2d)
                .format(surface_format.format)
                .components(ComponentMapping {
                    r: ComponentSwizzle::R,
                    g: ComponentSwizzle::G,
                    b: ComponentSwizzle::B,
                    a: ComponentSwizzle::A,
                })
                .subresource_range(
                    ImageSubresourceRange::builder()
                        .aspect_mask(ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build(),
                )
                .build()
                .unwrap()
        })
        .collect();
    let device_memory_properties = pdevice.memory_properties();
    let mut image_builder = ContinuousImage::builder(&device);
    image_builder.image_type(ImageType::TYPE_2D);
    image_builder.format(Format::D16_UNORM);
    image_builder.extent(surface_resolution.into());
    image_builder.mip_levels(1);
    image_builder.array_layers(1);
    image_builder.samples(SampleCountFlags::TYPE_1);
    image_builder.tiling(ImageTiling::OPTIMAL);
    image_builder
        .usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | ImageUsageFlags::TRANSIENT_ATTACHMENT);
    image_builder.sharing_mode(SharingMode::EXCLUSIVE);
    let depth_image = image_builder.build().unwrap();

    let depth_image_memory_req = depth_image.get_memory_requirements();
    let depth_image_memory = find_memory_type_index(
        depth_image_memory_req,
        device_memory_properties,
        MemoryPropertyFlags::DEVICE_LOCAL,
    )
    .expect("Unable to find suitable memory index for depth image.");
    let depth_image_memory = DeviceMemory::builder(depth_image_memory, &device)
        .allocation_size(depth_image_memory_req.size)
        // example of how to use dedicated memory
        .dedicated_info(MemoryDedicatedAllocateInfo {
            resource: DedicatedResource::Image(&depth_image),
        })
        .build()
        .unwrap();
    let depth_image = Arc::new(
        depth_image
            .bind_memory(&depth_image_memory, 0)
            .expect("Unable to bind depth image memory"),
    );

    let setup_commands_reuse_fence = Fence::new(&device).unwrap();

    let depth_image_view = ImageView::builder(depth_image)
        .subresource_range(
            ImageSubresourceRange::builder()
                .aspect_mask(ImageAspectFlags::DEPTH)
                .level_count(1)
                .layer_count(1)
                .build(),
        )
        .format(Format::D16_UNORM)
        .view_type(ImageViewType::Type2d)
        .build()
        .unwrap();

    let renderpass = RenderPass::builder(&device)
        .add_attachment(
            AttachmentDescription::builder()
                .format(surface_format.format)
                .samples(SampleCountFlags::TYPE_1)
                .load_op(AttachmentLoadOp::CLEAR)
                .store_op(AttachmentStoreOp::STORE)
                .final_layout(ImageLayout::PRESENT_SRC_KHR)
                .build(),
        )
        .add_attachment(
            AttachmentDescription::builder()
                .format(Format::D16_UNORM)
                .samples(SampleCountFlags::TYPE_1)
                .load_op(AttachmentLoadOp::CLEAR)
                .store_op(AttachmentStoreOp::DONT_CARE)
                // .initial_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                .final_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                .build(),
        )
        .add_subpass(
            SubpassDescription::builder()
                .add_color_attachment(
                    AttachmentReference::builder()
                        .attachment_index(0)
                        .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                        .build(),
                )
                .depth_stencil_attachment(
                    AttachmentReference::builder()
                        .attachment_index(1)
                        .layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                        .build(),
                )
                .build(),
        )
        .add_dependency(
            SubpassDependency::builder()
                .src_subpass(SUBPASS_EXTERNAL)
                .add_src_stage_mask(PipelineStageFlag::ColorAttachmentOutput)
                .add_dst_stage_mask(PipelineStageFlag::ColorAttachmentOutput)
                .dst_access_mask(
                    AccessFlags::COLOR_ATTACHMENT_READ | AccessFlags::COLOR_ATTACHMENT_WRITE,
                )
                .build(),
        )
        .build()
        .unwrap();
    let framebuffers: HashMap<u64, Arc<Framebuffer>> = present_image_views
        .iter()
        .map(|present_image_view| {
            let framebuffer = Framebuffer::builder(renderpass.clone())
                .add_attachment(0, present_image_view.clone())
                .add_attachment(1, depth_image_view.clone())
                .width(surface_resolution.width)
                .height(surface_resolution.height)
                .layers(1)
                .build(&device)
                .unwrap();
            (present_image_view.image.handle(), framebuffer)
        })
        .collect();
    let mut buffer_builder = ContinuousBuffer::builder(&device);
    buffer_builder.sharing_mode(SharingMode::EXCLUSIVE);

    let index_buffer_data = [0u32, 1, 2, 2, 3, 0];
    buffer_builder.size(std::mem::size_of_val(&index_buffer_data) as u64);
    buffer_builder.usage(BufferUsageFlags::INDEX_BUFFER);
    let index_buffer = buffer_builder.build().unwrap();
    let index_buffer_memory_req = index_buffer.get_memory_requirements();
    let index_buffer_memory_index = find_memory_type_index(
        index_buffer_memory_req,
        device_memory_properties,
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
    )
    .expect("Unable to find suitable memorytype for the index buffer.");
    let mut index_buffer_memory = DeviceMemory::builder(index_buffer_memory_index, &device)
        .allocation_size(index_buffer_memory_req.size)
        .build()
        .unwrap();
    index_buffer_memory
        .map_memory(0, index_buffer_memory_req.size)
        .unwrap();
    let mapped_memory = index_buffer_memory
        .get_memory(0, index_buffer_memory_req.size)
        .unwrap();
    mapped_memory[0..std::mem::size_of_val(&index_buffer_data)].copy_from_slice(unsafe {
        std::slice::from_raw_parts(
            index_buffer_data.as_ptr() as *const u8,
            std::mem::size_of_val(&index_buffer_data),
        )
    });
    let index_buffer = Arc::new(index_buffer.bind_memory(&index_buffer_memory, 0).unwrap());

    let vertices = [
        Vertex {
            pos: [-1.0, -1.0, 0.0, 1.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            pos: [-1.0, 1.0, 0.0, 1.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            pos: [1.0, 1.0, 0.0, 1.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            pos: [1.0, -1.0, 0.0, 1.0],
            uv: [1.0, 0.0],
        },
    ];

    buffer_builder.size(std::mem::size_of_val(&vertices) as _);
    buffer_builder.usage(BufferUsageFlags::VERTEX_BUFFER);
    let vertex_input_buffer = buffer_builder.build().unwrap();

    let vertex_input_buffer_memory_req = vertex_input_buffer.get_memory_requirements();

    let vertex_input_buffer_memory_index = find_memory_type_index(
        vertex_input_buffer_memory_req,
        device_memory_properties,
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
    )
    .expect("Unable to find suitable memorytype for the vertex buffer.");

    let mut vertex_input_buffer_memory =
        DeviceMemory::builder(vertex_input_buffer_memory_index, &device)
            .allocation_size(vertex_input_buffer_memory_req.size)
            .build()
            .unwrap();
    vertex_input_buffer_memory
        .map_memory(0, WHOLE_SIZE)
        .unwrap();
    let mapped_memory = vertex_input_buffer_memory
        .get_memory(0, vertex_input_buffer_memory_req.size)
        .unwrap();
    mapped_memory[0..std::mem::size_of_val(&vertices)].copy_from_slice(unsafe {
        std::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            std::mem::size_of_val(&vertices),
        )
    });
    let vertex_input_buffer = Arc::new(
        vertex_input_buffer
            .bind_memory(&vertex_input_buffer_memory, 0)
            .unwrap(),
    );

    let uniform_color_buffer_data = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        _pad: 0.0,
    };

    buffer_builder.size(std::mem::size_of_val(&uniform_color_buffer_data) as u64);
    buffer_builder.usage(BufferUsageFlags::UNIFORM_BUFFER);
    let uniform_color_buffer = buffer_builder.build().unwrap();
    let uniform_color_buffer_memory_req = uniform_color_buffer.get_memory_requirements();
    let uniform_color_buffer_memory_index = find_memory_type_index(
        uniform_color_buffer_memory_req,
        device_memory_properties,
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
    )
    .expect("Unable to find suitable memorytype for the vertex buffer.");
    let mut uniform_color_buffer_memory =
        DeviceMemory::builder(uniform_color_buffer_memory_index, &device)
            .allocation_size(uniform_color_buffer_memory_req.size)
            .build()
            .unwrap();
    uniform_color_buffer_memory
        .map_memory(0, WHOLE_SIZE)
        .unwrap();
    let mapped_memory = uniform_color_buffer_memory
        .get_memory(0, uniform_color_buffer_memory_req.size)
        .unwrap();
    mapped_memory[0..std::mem::size_of_val(&uniform_color_buffer_data)].copy_from_slice(unsafe {
        std::slice::from_raw_parts(
            &uniform_color_buffer_data as *const _ as *const u8,
            std::mem::size_of_val(&uniform_color_buffer_data),
        )
    });
    let uniform_color_buffer = Arc::new(
        uniform_color_buffer
            .bind_memory(&uniform_color_buffer_memory, 0)
            .unwrap(),
    );

    let image = image::load_from_memory(include_bytes!("rust.png"))
        .unwrap()
        .to_bgra8();
    let (width, height) = image.dimensions();
    let image_extent = Extent2D { width, height };
    let image_data = image.into_raw();

    buffer_builder.size(image_data.len() as _);
    buffer_builder.usage(BufferUsageFlags::TRANSFER_SRC);
    let image_buffer = buffer_builder.build().unwrap();
    let image_buffer_memory_req = image_buffer.get_memory_requirements();
    let image_buffer_memory_index = find_memory_type_index(
        image_buffer_memory_req,
        device_memory_properties,
        MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
    )
    .expect("Unable to find suitable memorytype for the vertex buffer.");

    let mut image_buffer_memory = DeviceMemory::builder(image_buffer_memory_index, &device)
        .allocation_size(image_buffer_memory_req.size)
        .build()
        .unwrap();
    image_buffer_memory
        .map_memory(0, image_buffer_memory_req.size)
        .unwrap();
    let mapped_memory = image_buffer_memory
        .get_memory(0, image_buffer_memory_req.size)
        .unwrap();
    mapped_memory[0..image_data.len()].copy_from_slice(image_data.as_slice());
    // flush example if you used a non-coherent device memory
    // let mut mapped_ranges = MappedRanges::new(&device);
    // mapped_ranges.add_range(&mapped_memory, 0, image_buffer_memory_req.size);
    // mapped_ranges.flush().unwrap();
    let image_buffer = Arc::new(image_buffer.bind_memory(&image_buffer_memory, 0).unwrap());
    image_builder.image_type(ImageType::TYPE_2D);
    image_builder.format(Format::from_raw(44));
    image_builder.extent(image_extent.into());
    image_builder.mip_levels(1);
    image_builder.array_layers(1);
    image_builder.samples(SampleCountFlags::TYPE_1);
    image_builder.tiling(ImageTiling::OPTIMAL);
    image_builder.usage(ImageUsageFlags::TRANSFER_DST | ImageUsageFlags::SAMPLED);
    image_builder.sharing_mode(SharingMode::EXCLUSIVE);
    let texture_image = image_builder.build().unwrap();
    let texture_memory_req = texture_image.get_memory_requirements();
    let texture_memory_index = find_memory_type_index(
        texture_memory_req,
        device_memory_properties,
        MemoryPropertyFlags::DEVICE_LOCAL,
    )
    .expect("Unable to find suitable memory index for depth image.");

    let texture_memory = DeviceMemory::builder(texture_memory_index, &device)
        .allocation_size(texture_memory_req.size)
        .build()
        .unwrap();
    let texture_image = Arc::new(
        texture_image
            .bind_memory(&texture_memory, 0)
            .expect("Unable to bind depth image memory"),
    );

    let sampler = Sampler::builder(&device)
        .mag_filter(Filter::LINEAR)
        .min_filter(Filter::LINEAR)
        .mipmap_mode(SamplerMipmapMode::LINEAR)
        .address_mode_u(SamplerAddressMode::MIRRORED_REPEAT)
        .address_mode_v(SamplerAddressMode::MIRRORED_REPEAT)
        .address_mode_w(SamplerAddressMode::MIRRORED_REPEAT)
        .max_anisotropy(1.0, feature_sampler_anisotropy)
        .border_color(BorderColor::FLOAT_OPAQUE_WHITE)
        .compare_op(CompareOp::NEVER)
        .build()
        .unwrap();

    let tex_image_view = ImageView::builder(texture_image.clone())
        .view_type(ImageViewType::Type2d)
        .format(Format::from_raw(44))
        .components(ComponentMapping {
            r: ComponentSwizzle::R,
            g: ComponentSwizzle::G,
            b: ComponentSwizzle::B,
            a: ComponentSwizzle::A,
        })
        .subresource_range(
            ImageSubresourceRange::builder()
                .aspect_mask(ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1)
                .build(),
        )
        .build()
        .unwrap();

    let layout_const: <MyDescriptorLayout as DescriptorSetValue>::ConstDescriptorSetValue =
        ConstDescriptorSetValue2 {
            t0: ([(); 1], ShaderStage::Fragment),
            t1: ([sampler; 1], ShaderStage::Fragment),
        };

    let desc_set_layout = DescriptorSetLayout::new(&device, layout_const).unwrap();

    let descriptor_pool = DescriptorPool::new(&desc_set_layout, 1).unwrap();

    let init_value = MyDescriptorLayout {
        t0: [(
            uniform_color_buffer,
            0,
            std::mem::size_of_val(&uniform_color_buffer_data) as u64,
        )],
        t1: [(tex_image_view, ImageLayout::SHADER_READ_ONLY_OPTIMAL)],
    };

    let mut descriptor_set = descriptor_pool.allocate().unwrap();

    let mut update = device.update_descriptor_sets();
    update.add(&mut descriptor_set, |_| init_value);
    update.update();

    let descriptor_set: Arc<dyn IDescriptorSet> = Arc::new(descriptor_set);
    let command_buffer = setup_command_buffer
        .record(|command_buffer| {
            let texture_barrier = ImageMemoryBarrier::builder(texture_image.clone())
                .dst_access_mask(AccessFlags::TRANSFER_WRITE)
                .new_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
                .subresource_range(
                    ImageSubresourceRange::builder()
                        .aspect_mask(ImageAspectFlags::COLOR)
                        .level_count(1)
                        .layer_count(1)
                        .build(),
                )
                .build();
            let image_barriers = vec![texture_barrier];
            command_buffer.cmd_pipeline_barrier(
                PipelineStageFlag::BottomOfPipe.into(),
                PipelineStageFlag::Transfer.into(),
                DependencyFlags::empty(),
                [],
                [],
                image_barriers,
            );
            let buffer_copy_regions = BufferImageCopy::builder()
                .image_subresource(
                    ImageSubresourceLayers::builder()
                        .aspect_mask(ImageAspectFlags::COLOR)
                        .layer_count(1)
                        .build(),
                )
                .image_extent(Extent3D {
                    width,
                    height,
                    depth: 1,
                });

            command_buffer.cmd_copy_buffer_to_image(
                image_buffer.clone(),
                texture_image.clone(),
                ImageLayout::TRANSFER_DST_OPTIMAL,
                &[buffer_copy_regions.build()],
            );
            let texture_barrier_end = ImageMemoryBarrier::builder(texture_image.clone())
                .src_access_mask(AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(AccessFlags::SHADER_READ)
                .old_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .subresource_range(
                    ImageSubresourceRange::builder()
                        .aspect_mask(ImageAspectFlags::COLOR)
                        .level_count(1)
                        .layer_count(1)
                        .build(),
                )
                .build();
            let image_barriers = vec![texture_barrier_end];
            command_buffer.cmd_pipeline_barrier(
                PipelineStageFlag::Transfer.into(),
                PipelineStageFlag::FragmentShader.into(),
                DependencyFlags::empty(),
                [],
                [],
                image_barriers,
            );
            Ok(())
        })
        .unwrap();
    let (fence, command_buffer) = submit(
        &mut present_queue,
        command_buffer,
        setup_commands_reuse_fence,
    );

    let mut vertex_spv_file = Cursor::new(&include_bytes!("vert.spv")[..]);
    let mut frag_spv_file = Cursor::new(&include_bytes!("frag.spv")[..]);

    let vertex_code =
        read_spv(&mut vertex_spv_file).expect("Failed to read vertex shader spv file");

    let frag_code = read_spv(&mut frag_spv_file).expect("Failed to read fragment shader spv file");

    let vertex_shader_module = ShaderModule::builder(&device, &vertex_code)
        .build()
        .unwrap();

    let fragment_shader_module = ShaderModule::builder(&device, &frag_code).build().unwrap();

    let pipeline_layout = PipelineLayout::builder(&device)
        .add_set_layout(desc_set_layout)
        .build()
        .unwrap();

    let vertex_input_binding_descriptions = VertexInputBindingDescription::builder()
        .stride(std::mem::size_of::<Vertex>() as u32)
        .input_rate(VertexInputRate::VERTEX)
        .build();
    let vertex_input_state_info = PipelineVertexInputStateCreateInfo::builder()
        .add_vertex_input_attribute_description(VertexInputAttributeDescription {
            location: 0,
            binding: vertex_input_binding_descriptions,
            format: Format::R32G32B32A32_SFLOAT,
            offset: offset_of!(Vertex, pos) as u32,
        })
        .add_vertex_input_attribute_description(VertexInputAttributeDescription {
            location: 1,
            binding: vertex_input_binding_descriptions,
            format: Format::R32G32_SFLOAT,
            offset: offset_of!(Vertex, uv) as u32,
        })
        .build();
    let noop_stencil_state = StencilOpState {
        fail_op: StencilOp::KEEP,
        pass_op: StencilOp::KEEP,
        depth_fail_op: StencilOp::KEEP,
        compare_op: CompareOp::ALWAYS,
        ..Default::default()
    };

    let entry_name = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"main\0") };
    // let op_feature = device.get_feature::<{ FeatureType::DeviceFeatures(PhysicalDeviceFeatures::LogicOp) }>().unwrap();
    let graphic_pipeline_builder = Pipeline::builder(pipeline_layout.clone())
        .add_stage(
            PipelineShaderStageCreateInfo::builder(vertex_shader_module, entry_name)
                .stage(ShaderStage::Vertex)
                .build(),
        )
        .add_stage(
            PipelineShaderStageCreateInfo::builder(fragment_shader_module, entry_name)
                .stage(ShaderStage::Fragment)
                .build(),
        )
        .vertex_input_state(vertex_input_state_info)
        // suggest to use dynamic viewport and scissor
        // .viewport_state(
        //     PipelineViewportStateCreateInfo::builder()
        //         .viewport(Viewport {
        //             x: 0.0,
        //             y: 0.0,
        //             width: surface_resolution.width as f32,
        //             height: surface_resolution.height as f32,
        //             min_depth: 0.0,
        //             max_depth: 1.0,
        //         })
        //         .scissor(Rect2D {
        //             extent: surface_resolution,
        //             ..Default::default()
        //         })
        //         .build(),
        // )
        .input_assembly_state(
            PipelineInputAssemblyStateCreateInfo::builder()
                .topology::<{ PrimitiveTopology::TriangleList }>()
                .build(),
        )
        .rasterization_state(
            PipelineRasterizationStateCreateInfo::builder()
                .front_face(FrontFace::COUNTER_CLOCKWISE)
                .line_width(1.0)
                .polygon_mode(PolygonMode::Fill)
                .build(),
        )
        .multisample_state(
            PipelineMultisampleStateCreateInfo::builder()
                .rasterization_samples(SampleCountFlags::TYPE_1)
                .build(),
        )
        .depth_stencil_state(
            PipelineDepthStencilStateCreateInfo::builder()
                .depth_test_enable()
                .depth_write_enable()
                .depth_compare_op(CompareOp::LESS_OR_EQUAL)
                .front(noop_stencil_state)
                .back(noop_stencil_state)
                .depth_bounds(0.0, 1.0)
                .build(),
        )
        .color_blend_state(
            PipelineColorBlendStateCreateInfo::builder()
                // .logic_op(LogicOp::CLEAR, op_feature)
                .add_attachment(
                    PipelineColorBlendAttachmentState::builder()
                        .src_color_blend_factor(BlendFactor::SrcColor)
                        .dst_color_blend_factor(BlendFactor::OneMinusDstColor)
                        .color_blend_op(BlendOp::ADD)
                        .src_alpha_blend_factor(BlendFactor::Zero)
                        .dst_alpha_blend_factor(BlendFactor::Zero)
                        .alpha_blend_op(BlendOp::ADD)
                        .color_write_mask(ColorComponentFlags::RGBA)
                        .build(),
                )
                .build(),
        )
        .render_pass(renderpass.clone(), 0);
    let graphic_pipeline = device
        .create_pipelines(vec![graphic_pipeline_builder], PipelineCacheType::None)
        .unwrap()
        .pop()
        .unwrap();

    let present_complete_semaphore = Semaphore::new(&device).unwrap();
    let mut rendering_complete_semaphore = Semaphore::new(&device).unwrap();
    let mut draw_commands_reuse_fence = Some(fence);
    let mut draw_command_buffer = Some(command_buffer);
    let secondary_command_buffer =
        TransientCommandBuffer::<{ SECONDARY }>::new(&device, queue_family).unwrap();
    let mut secondary_command_buffer = Some(secondary_command_buffer);
    let inheritance_info = CommandBufferInheritanceInfo::builder()
        .render_pass(renderpass.clone())
        .subpass(0)
        .build();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                present_queue.wait_idle().unwrap();
            }
            Event::MainEventsCleared => {
                let image = swapchain
                    .acquire_next_image_semaphore_only(u64::MAX, &present_complete_semaphore)
                    .unwrap();
                let framebuffer = framebuffers.get(&image.handle()).unwrap();
                let render_pass_begin_info = Arc::new(
                    RenderPassBeginInfo::builder(renderpass.clone(), framebuffer.clone())
                        .render_area(surface_resolution.into())
                        .add_clear_value(ClearValue {
                            color: ClearColorValue {
                                float32: [0.0, 0.0, 0.0, 0.0],
                            },
                        })
                        .add_clear_value(ClearValue {
                            depth_stencil: ClearDepthStencilValue {
                                depth: 1.0,
                                stencil: 0,
                            },
                        })
                        .build(),
                );
                let command_buffer = draw_command_buffer.take().unwrap();
                let recording_command_buffer = command_buffer.begin().unwrap();
                let mut render_pass_command_buffer = recording_command_buffer
                    .cmd_begin_render_pass(
                        render_pass_begin_info.clone(),
                        SubpassContents::SECONDARY_COMMAND_BUFFERS,
                    );
                let secondary_buffer = secondary_command_buffer.take().unwrap();
                let mut command_buffer = secondary_buffer.begin(inheritance_info.clone()).unwrap();
                // use thread pool in real cases
                std::thread::scope(|s| {
                    s.spawn(|| {
                        command_buffer.cmd_set_viewport(&Viewport {
                            x: 0.0,
                            y: 0.0,
                            width: surface_resolution.width as f32,
                            height: surface_resolution.height as f32,
                            min_depth: 0.0,
                            max_depth: 1.0,
                        });
                        command_buffer.cmd_set_scissor(&Rect2D {
                            extent: surface_resolution,
                            ..Default::default()
                        });
                        command_buffer.cmd_bind_pipeline(
                            PipelineBindPoint::GRAPHICS,
                            graphic_pipeline.clone(),
                        );
                        // command_buffer.cmd_bind_descriptor_sets(
                        //     PipelineBindPoint::GRAPHICS,
                        //     pipeline_layout.clone(),
                        //     0,
                        //     [descriptor_set.clone()],
                        //     &[],
                        // );
                        command_buffer.cmd_bind_descriptor_sets(
                            PipelineBindPoint::GRAPHICS,
                            pipeline_layout.clone(),
                            0,
                            [descriptor_set.clone()],
                            &[],
                        );
                        command_buffer.cmd_bind_vertex_buffers(
                            0,
                            [vertex_input_buffer.clone() as _],
                            &[0],
                        );
                        command_buffer.cmd_bind_index_buffer(
                            index_buffer.clone(),
                            0,
                            IndexType::UINT32,
                        );
                        command_buffer.cmd_draw_indexed(index_buffer_data.len() as u32, 1, 0, 0, 1);
                    })
                    .join()
                    .unwrap();
                });
                let vec = vec![command_buffer.end().unwrap()];
                render_pass_command_buffer.cmd_execute_commands(vec);
                let recording_command_buffer = render_pass_command_buffer.cmd_end_render_pass();
                let command_buffer = recording_command_buffer.end().unwrap();
                let submit_info = SubmitInfo::builder()
                    .add_wait_semaphore(
                        &present_complete_semaphore,
                        PipelineStageFlag::BottomOfPipe,
                    )
                    .add_one_time_submit_command_buffer(command_buffer)
                    .add_signal_semaphore(&rendering_complete_semaphore)
                    .build();
                let fence = draw_commands_reuse_fence.take().unwrap();
                let fence = Submittable::new()
                    .add_submit_info(submit_info)
                    .submit(&mut present_queue, fence)
                    .expect("queue submit failed.");

                let (fence, mut result) = fence.wait().unwrap();
                let fence = fence.reset().unwrap();
                let mut command_buffer = result
                    .take_invalid_primary_buffer(&command_buffer_handler)
                    .unwrap();
                let secondary_buffer = command_buffer.secondary_buffers().pop().unwrap();
                let command_buffer = command_buffer.reset().unwrap();
                let secondary_buffer = secondary_buffer.reset().unwrap();
                draw_command_buffer = Some(command_buffer);
                draw_commands_reuse_fence = Some(fence);
                secondary_command_buffer = Some(secondary_buffer);
                let mut present_info = PresentInfo::builder()
                    .add_swapchain_and_image(&mut swapchain, &image)
                    .add_wait_semaphore(&mut rendering_complete_semaphore)
                    .build();
                present_queue.queue_present(&mut present_info).unwrap();
            }
            _ => (),
        }
    });
}
