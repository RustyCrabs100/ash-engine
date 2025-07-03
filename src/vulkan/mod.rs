

#[path ="debug.rs"]
pub(crate) mod debug;
#[path = "instance.rs"]
pub(crate) mod instance;

#[cfg(feature = "vulkan")]
pub mod vulkan {
    use ash::*;
    
    use std::sync::Arc;

    use crate::vulkan::instance::InstanceHandling;
    #[cfg(feature = "debug")]
    use crate::vulkan::debug::VulkanDebug::VulkanDebugMessage::VulkanDebugAllocationMessenger::allocation_callback;
    pub struct VulkanInit<'a> {
        entry: Arc<Entry>,
        instance: Arc<Instance>,
        allocation_callbacks: Arc<Option<vk::AllocationCallbacks<'a>>>,
    }

    impl VulkanInit<'_> {
        pub fn vulkan_init() {
            let entry = Arc::new(unsafe { Entry::load().expect("Failed to load Entry Points") });
            let instance = InstanceHandling::create_vk_instance(&entry);
            #[cfg(feature = "debug")]
            let allocation_callbacks = Arc::new(Some(allocation_callback()));
            #[cfg(not(feature = "debug"))]
            let allocation_callbacks = Arc::new(None);
            Self { 
                entry, 
                instance, 
                allocation_callbacks
            };
            println!("Working!")
        }
    }
}
