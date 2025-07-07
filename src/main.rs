mod utilities;
pub(crate) mod vulkan;


fn main() {
    let vulkan_setup_items = vulkan::vulkan::VulkanInit::new();
}
