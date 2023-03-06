use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct VertexInputBindingDescription {
    // DONE VUID-VkPipelineVertexInputStateCreateInfo-pVertexBindingDescriptions-00616
    // remove field binding, other types must contain this type, instead of using a binding number
    pub(crate) stride: Option<u32>,
    pub(crate) input_rate: ash::vk::VertexInputRate,
}

impl VertexInputBindingDescription {
    pub fn builder() -> VertexInputBindingDescriptionBuilder {
        VertexInputBindingDescriptionBuilder {
            stride: None,
            input_rate: Default::default(),
        }
    }
}

pub struct VertexInputBindingDescriptionBuilder {
    stride: Option<u32>,
    input_rate: ash::vk::VertexInputRate,
}

impl VertexInputBindingDescriptionBuilder {
    pub fn stride(mut self, stride: u32) -> Self {
        self.stride = Some(stride);
        self
    }

    pub fn input_rate(mut self, input_rate: ash::vk::VertexInputRate) -> Self {
        self.input_rate = input_rate;
        self
    }

    pub fn build(self) -> VertexInputBindingDescription {
        VertexInputBindingDescription {
            stride: self.stride,
            input_rate: self.input_rate,
        }
    }
}

pub struct VertexInputAttributeDescription {
    pub location: u32,
    // DONE VUID-VkPipelineVertexInputStateCreateInfo-binding-00615
    pub binding: VertexInputBindingDescription,
    pub format: ash::vk::Format,
    pub offset: u32,
}

#[derive(Default)]
pub struct PipelineVertexInputStateCreateInfo {
    pub flags: ash::vk::PipelineVertexInputStateCreateFlags,
    pub(crate) vertex_input_attribute_descriptions:
        FxHashMap<u32 /*location*/, VertexInputAttributeDescription>,
    ash_vk_vertex_attribute_descriptions: Vec<ash::vk::VertexInputAttributeDescription>,
    ash_vk_vertex_binding_descriptions: Vec<ash::vk::VertexInputBindingDescription>,
}

impl PipelineVertexInputStateCreateInfo {
    pub fn builder() -> PipelineVertexInputStateCreateInfoBuilder {
        PipelineVertexInputStateCreateInfoBuilder {
            inner: Self {
                flags: Default::default(),
                vertex_input_attribute_descriptions: FxHashMap::default(),
                ash_vk_vertex_attribute_descriptions: Default::default(),
                ash_vk_vertex_binding_descriptions: Default::default(),
            },
        }
    }
    pub(crate) fn ash_builder(&mut self) -> ash::vk::PipelineVertexInputStateCreateInfoBuilder {
        let vertex_input_binding_descriptions: FxHashSet<VertexInputBindingDescription> = self
            .vertex_input_attribute_descriptions
            .values()
            .map(|attr| attr.binding)
            .collect();
        self.ash_vk_vertex_attribute_descriptions = self
            .vertex_input_attribute_descriptions
            .values()
            .map(|attr| {
                ash::vk::VertexInputAttributeDescription::builder()
                    .binding(
                        vertex_input_binding_descriptions
                            .iter()
                            .position(|x| *x == attr.binding)
                            .unwrap() as _,
                    )
                    .location(attr.location)
                    .offset(attr.offset)
                    .format(attr.format)
                    .build()
            })
            .collect::<Vec<_>>();
        self.ash_vk_vertex_binding_descriptions = vertex_input_binding_descriptions
            .iter()
            .enumerate()
            .map(|(i, description)| {
                let mut builder = ash::vk::VertexInputBindingDescription::builder()
                    .binding(i as _)
                    .input_rate(description.input_rate);
                if let Some(stride) = description.stride {
                    builder = builder.stride(stride);
                }
                builder.build()
            })
            .collect::<Vec<_>>();
        let ash_vk_vertex_input_state = ash::vk::PipelineVertexInputStateCreateInfo::builder()
            .flags(self.flags)
            .vertex_attribute_descriptions(self.ash_vk_vertex_attribute_descriptions.as_slice())
            .vertex_binding_descriptions(self.ash_vk_vertex_binding_descriptions.as_slice());
        ash_vk_vertex_input_state
    }
}

pub struct PipelineVertexInputStateCreateInfoBuilder {
    inner: PipelineVertexInputStateCreateInfo,
}

impl PipelineVertexInputStateCreateInfoBuilder {
    pub fn add_vertex_input_attribute_description(
        mut self,
        vertex_input_attribute_description: VertexInputAttributeDescription,
    ) -> Self {
        // MUST VUID-VkPipelineVertexInputStateCreateInfo-pVertexAttributeDescriptions-00617
        if self
            .inner
            .vertex_input_attribute_descriptions
            .insert(
                vertex_input_attribute_description.location,
                vertex_input_attribute_description,
            )
            .is_some()
        {
            panic!("VUID-VkPipelineVertexInputStateCreateInfo-pVertexAttributeDescriptions-00617")
        }
        self
    }
    pub fn build(self) -> PipelineVertexInputStateCreateInfo {
        self.inner
    }
}
