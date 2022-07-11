use std::ffi::{c_void, CStr};
use std::os::raw::c_char;

unsafe fn char_pointer_to_str<'a>(p: *const c_char) -> &'a str {
    if p.is_null() {
        ""
    } else {
        CStr::from_ptr(p).to_str().unwrap()
    }
}

pub type CallbackType = dyn Fn(
    ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    ash::vk::DebugUtilsMessageTypeFlagsEXT,
    &DebugUtilsMessengerCallbackDataEXT,
);

// Done VUID-VkDebugUtilsMessengerCreateInfoEXT-pfnUserCallback-01914
unsafe extern "system" fn vulkan_debug_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    user_data: *mut std::os::raw::c_void,
) -> ash::vk::Bool32 {
    let call: &Box<CallbackType> = std::mem::transmute(user_data);
    call(
        message_severity,
        message_type,
        &DebugUtilsMessengerCallbackDataEXT::from_ash_ext(p_callback_data),
    );
    ash::vk::FALSE
}

pub struct DebugUtilsMessengerCreateInfoEXT {
    pub message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    cb: Box<
        Box<
            dyn Fn(
                    ash::vk::DebugUtilsMessageSeverityFlagsEXT,
                    ash::vk::DebugUtilsMessageTypeFlagsEXT,
                    &DebugUtilsMessengerCallbackDataEXT,
                ) + Sync
                + Send,
        >,
    >,
}

impl DebugUtilsMessengerCreateInfoEXT {
    pub fn builder() -> DebugUtilsMessengerCreateInfoEXTBuilder {
        DebugUtilsMessengerCreateInfoEXTBuilder::default()
    }
    pub(crate) fn ash_builder(&self) -> ash::vk::DebugUtilsMessengerCreateInfoEXTBuilder {
        ash::vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(self.message_severity)
            .message_type(self.message_type)
            .pfn_user_callback(Some(vulkan_debug_callback))
            .user_data(&*self.cb as *const _ as *mut c_void)
    }
}

pub struct DebugUtilsMessengerCreateInfoEXTBuilder {
    inner: DebugUtilsMessengerCreateInfoEXT,
}

impl Default for DebugUtilsMessengerCreateInfoEXTBuilder {
    fn default() -> Self {
        Self {
            inner: DebugUtilsMessengerCreateInfoEXT {
                message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
                message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
                cb: Box::new(Box::new(|_, _, _| {})),
            },
        }
    }
}

impl DebugUtilsMessengerCreateInfoEXTBuilder {
    pub fn set_callback<
        F: 'static
            + Sync
            + Send
            + Fn(
                ash::vk::DebugUtilsMessageSeverityFlagsEXT,
                ash::vk::DebugUtilsMessageTypeFlagsEXT,
                &DebugUtilsMessengerCallbackDataEXT,
            ),
    >(
        &mut self,
        callback: F,
    ) -> &mut Self {
        self.inner.cb = Box::new(Box::new(callback));
        self
    }
    pub fn callback<
        F: 'static
            + Sync
            + Send
            + Fn(
                ash::vk::DebugUtilsMessageSeverityFlagsEXT,
                ash::vk::DebugUtilsMessageTypeFlagsEXT,
                &DebugUtilsMessengerCallbackDataEXT,
            ),
    >(
        mut self,
        callback: F,
    ) -> Self {
        self.set_callback(callback);
        self
    }
    pub fn set_severity(
        &mut self,
        severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    ) -> &mut Self {
        self.inner.message_severity = severity;
        self
    }
    pub fn severity(mut self, severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT) -> Self {
        self.set_severity(severity);
        self
    }
    pub fn set_message_type(
        &mut self,
        message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    ) -> &mut Self {
        self.inner.message_type = message_type;
        self
    }
    pub fn message_type(mut self, message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT) -> Self {
        self.set_message_type(message_type);
        self
    }
    pub fn build(self) -> DebugUtilsMessengerCreateInfoEXT {
        self.inner
    }
}

pub struct DebugUtilsLabelEXT<'a> {
    pub p_label_name: &'a str,
    pub color: &'a [f32; 4],
}

impl<'a> DebugUtilsLabelEXT<'a> {
    unsafe fn from_ash_ext(ext: *const ash::vk::DebugUtilsLabelEXT) -> Self {
        Self {
            p_label_name: CStr::from_ptr((*ext).p_label_name).to_str().unwrap(),
            color: &(*ext).color,
        }
    }
}

pub struct DebugUtilsObjectNameInfoEXT<'a> {
    pub object_type: ash::vk::ObjectType,
    pub object_handle: u64,
    pub p_object_name: &'a str,
}

impl<'a> DebugUtilsObjectNameInfoEXT<'a> {
    unsafe fn from_ash_ext(ext: *const ash::vk::DebugUtilsObjectNameInfoEXT) -> Self {
        Self {
            object_type: (*ext).object_type,
            object_handle: (*ext).object_handle,
            p_object_name: char_pointer_to_str((*ext).p_object_name),
        }
    }
}

pub struct DebugUtilsMessengerCallbackDataEXT<'a> {
    pub p_message_id_name: &'a str,
    pub message_id_number: i32,
    pub p_message: &'a str,
    pub p_queue_labels: Vec<DebugUtilsLabelEXT<'a>>,
    pub p_cmd_buf_labels: Vec<DebugUtilsLabelEXT<'a>>,
    pub p_objects: Vec<DebugUtilsObjectNameInfoEXT<'a>>,
}

impl<'a> DebugUtilsMessengerCallbackDataEXT<'a> {
    unsafe fn from_ash_ext(ext: *const ash::vk::DebugUtilsMessengerCallbackDataEXT) -> Self {
        Self {
            p_message_id_name: char_pointer_to_str((*ext).p_message_id_name),
            message_id_number: (*ext).message_id_number,
            p_message: char_pointer_to_str((*ext).p_message),
            p_queue_labels: std::slice::from_raw_parts(
                (*ext).p_queue_labels,
                (*ext).queue_label_count as _,
            )
            .iter()
            .map(|ext| DebugUtilsLabelEXT::from_ash_ext(ext))
            .collect(),
            p_cmd_buf_labels: std::slice::from_raw_parts(
                (*ext).p_cmd_buf_labels,
                (*ext).cmd_buf_label_count as _,
            )
            .iter()
            .map(|ext| DebugUtilsLabelEXT::from_ash_ext(ext))
            .collect(),
            p_objects: std::slice::from_raw_parts((*ext).p_objects, (*ext).object_count as _)
                .iter()
                .map(|ext| DebugUtilsObjectNameInfoEXT::from_ash_ext(ext))
                .collect(),
        }
    }
}
