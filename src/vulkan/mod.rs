#[path = "debug.rs"]
pub(crate) mod debug;
#[path = "instance.rs"]
pub(crate) mod instance;

pub mod vulkan {
    use ash::*;
    use std::sync::Arc;

    #[cfg(feature = "debug")]
    use crate::vulkan::debug::VulkanDebug::VulkanDebugMessage::VulkanDebugAllocationMessenger::allocation_callback;
    
    use crate::vulkan::instance::InstanceHandling;
    pub(crate) struct VulkanInit<'a> {
        entry: Arc<Entry>,
        instance: Arc<Instance>,
        allocation_callbacks: Arc<Option<&'a vk::AllocationCallbacks<'a>>>,
    }

    impl VulkanInit<'_> {
        pub(crate) fn new() -> Result<Self, vk::Result> {
            // Dynamically Load in Vulkan Entry Points
            let entry = Arc::new(unsafe { Entry::load()? });
            // Enable Allocation Callbacks IF Debugging is enabled
            let allocation_callbacks = {
                #[cfg(feature = "debug")]
                {
                    // This is done since my AllocationCallback Struct causes my computer to run out of memory...
                    Arc::new(None)
                    // Arc::new(Some(allocation_callback()))
                }
                #[cfg(not(feature = "debug"))]
                {
                    Arc::new(None)
                }
            };
            let inst_layer_items: (Vec<vk::LayerProperties>, usize) =
                match InstanceHandling::get_layers(&entry) {
                    Some(layers) => layers,
                    None => (Vec::new(), 0),
                };
            let inst_extension_items: (Vec<vk::ExtensionProperties>, usize) =
                match InstanceHandling::get_extensions(&entry) {
                    Some(extensions) => extensions,
                    None => (Vec::new(), 0),
                };
            let instance = InstanceHandling::create_vk_instance(
                &entry,
                inst_layer_items,
                inst_extension_items,
                *allocation_callbacks,
            );
            Ok(Self {
                entry,
                instance,
                allocation_callbacks,
            })
        }

        pub(crate) fn clean_up(self) {
            unsafe {
               self.instance.destroy_instance(*self.allocation_callbacks); 
            }
        }
    }

    pub struct Vulkan;

    impl Vulkan {
        fn main_loop(&mut VulkanInit) {}
    }
}
