pub(crate) mod Vulkan;
pub(crate) mod vulkan;
pub(crate) mod utilities;

fn main() {
    Vulkan::Vulkan::VulkanInit::vulkan_init();
    println!("Hello, world!");
}
