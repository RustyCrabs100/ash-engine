mod utilities;
pub(crate) mod vulkan;


fn main() {
    let vulkan_setup_items = vulkan::vulkan::VulkanInit::new();
    match vulkan_setup_items {
        Ok(vulkan_setup) => {
            println!("Vulkan Setup Successfully Completed");
            vulkan::vulkan::VulkanInit::clean_up(vulkan_setup);
        },
        Err(vulkan_failure) => panic!("Vulkan Setup failed, no other option currently avaliable"),
    }
    
}
