


pub(crate) mod debug;

#[cfg(feature = "vulkan")]
pub mod Vulkan {
    use ash::*;
    
    use std::sync::Arc;
    #[cfg(feature = "debug")]
    use crate::vulkan::debug::VulkanDebug::VulkanDebugMessage::VulkanDebugAllocationMessenger::allocation_callback;
    #[cfg_attr(feature = "debug", derive(Debug))]
    pub struct VulkanInit<'a> {
        entry: Arc<Entry>,
        instance: Arc<Instance>,
        allocation_callbacks: Arc<Option<vk::AllocationCallbacks<'a>>>,
    }

    impl VulkanInit<'_> {
        pub fn vulkan_init() {
            let entry = Arc::new(unsafe { Entry::load().expect("Failed to load Entry Points") });
            let instance = Self::create_vk_instance(&entry);
            #[cfg(feature = "debug")]
            let allocation_callbacks = Arc::new(Some(allocation_callback()));
            #[cfg(not(feature = "debug"))]
            let allocation_callbacks = Arc::new(None);
            Self { 
                entry, 
                instance, 
                allocation_callbacks
            };
        }

        fn create_vk_instance(entry: &Entry) -> Arc<Instance> {
            let app_info = vk::ApplicationInfo {
                api_version: vk::make_api_version(0, 1, 0, 0),
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
    }
}
