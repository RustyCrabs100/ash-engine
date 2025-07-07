pub(crate) mod InstanceHandling {
    use ash::*;
    use std::sync::Arc;
    use core::ffi::c_char;

    use crate::c_char_array;
    pub fn create_vk_instance(
        entry: &Entry,
        inst_layer_items: (Vec<vk::LayerProperties>, usize),
        inst_extension_items: (Vec<vk::ExtensionProperties>, usize),
        allocation: Option<&vk::AllocationCallbacks>,
    ) -> Arc<Instance> {
        let engine_name = c_char_array!(b"Ash Engine\0");
        let app_name = c_char_array!(b"Rust Game Engine\0");

        let app_info = vk::ApplicationInfo {
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::make_api_version(0, 1, 3, 296),
            ..Default::default()
        };

        let inst_layer_names = inst_layer_items
            .0
            .iter()
            .map(|f| f.layer_name.as_ptr())
            .collect::<Vec<*const c_char>>();

        let inst_extension_names = inst_extension_items
            .0
            .iter()
            .map(|f| f.extension_name.as_ptr())
            .collect::<Vec<*const c_char>>();

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            enabled_layer_count: inst_layer_items.1.try_into().expect("Prototyping; Failure: Unable to convert Layer Count to u32"),
            pp_enabled_layer_names: inst_layer_names.as_ptr(),
            enabled_extension_count: inst_extension_items.1.try_into().expect("Prototyping; Failure: Unable to convert Extension Count to u32"),
            pp_enabled_extension_names: inst_extension_names.as_ptr(),
            ..Default::default()
        };

        

        Arc::new(unsafe {
            entry
                .create_instance(&create_info, allocation)
                .expect("Failed to Create Instance")
        })
    }

    pub fn get_layers(entry: &Entry) -> Option<(Vec<vk::LayerProperties>, usize)> {
        let layer_properties = unsafe {
            entry
                .enumerate_instance_layer_properties()
                .expect("Failed to Enumerate through Layer Properties")
        };
        let layer_count = layer_properties.len();
        if layer_count == 0 {
            return None;
        } else {
            return Some((layer_properties, layer_count));
        }
    }
    pub fn get_extensions(entry: &Entry) -> Option<(Vec<vk::ExtensionProperties>, usize)> {
        let extension_properties = unsafe {
            entry
                .enumerate_instance_extension_properties(None)
                .expect("Failed to enumerate instance extensions")
        };
        let extension_count = extension_properties.len();
        if extension_count == 0 {
            return None;
        } else {
            return Some((extension_properties, extension_count));
        }
    }
}
