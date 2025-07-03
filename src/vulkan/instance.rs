pub(crate) mod InstanceHandling { 
    use ash::*;
    use std::sync::Arc;

    use crate::c_char_array;
    pub fn create_vk_instance(
        entry: &Entry,
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

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
    };

        Arc::new(unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to Create Instance")
        })
    }

    pub fn filter_layers() {}
    pub fn filter_extensions() {}
}