pub(crate) mod VulkanDebug {
    const fn debug_mode() -> bool {
        #[cfg(debug_assertions)] {
            return true;
        }
        return false;
    }
    
    const VALIDATION: bool = debug_mode();

    pub(crate) fn check_validation(inst_layers: &[ash::vk::LayerProperties]) -> bool {
        for layers in inst_layers {
            if layers.layer_name == c_char_array!(b"VK_LAYER_KHRONOS_validation") {
                return true;
            }
        }
        false
    } 

    pub(crate) mod VulkanDebugMessage {

        pub(crate) extern "system" fn debug_callbacks(
            
        ) -> ash::vk::Bool32 { todo!() }
        
        pub(crate) mod VulkanDebugAllocationMessenger {
            use ash::vk;

            #[cfg(feature = "debug")]
            pub(crate) fn allocation_callback<'a>() -> vk::AllocationCallbacks<'a> {
                vk::AllocationCallbacks {
                    p_user_data: core::ptr::null_mut(),
                    pfn_allocation: Some(VulkanDebugAllocationMessengerFunctions::allocation),
                    pfn_reallocation: Some(VulkanDebugAllocationMessengerFunctions::reallocation),
                    pfn_free: Some(VulkanDebugAllocationMessengerFunctions::free),
                    pfn_internal_allocation: None,
                    pfn_internal_free: None,
                    ..Default::default()
                }
            }

            pub(crate) mod VulkanDebugAllocationMessengerFunctions {
                use ash::vk;
                use core::ffi::{c_char, c_void};
                use core::ptr::{copy_nonoverlapping, null_mut};
                use std::alloc;
                use std::cmp;

                const MAX_ALLOC_SIZE: usize = 16 * 1024 * 1024 * 1024;

                #[repr(C)]
                struct AllocationInfo {
                    size: usize,
                    alignment: usize,
                }

                pub(crate) extern "system" fn allocation(
                    _p_user_data: *mut c_void,
                    size_alloc: usize,
                    alignment_alloc: usize,
                    _scope: vk::SystemAllocationScope,
                ) -> *mut c_void {
                    // Checks if Invalid Allocation was made
                    if size_alloc == 0
                        || alignment_alloc == 0
                        || !alignment_alloc.is_power_of_two()
                        || size_alloc > MAX_ALLOC_SIZE
                    {
                        eprintln!("Vulkan Attempted Over-allocation OR Invalid Allocation");
                        return null_mut();
                    }

                    println!(
                        "Allocationg {} bytes with {} alignment",
                        size_alloc, alignment_alloc
                    );

                    // Creates Allocation Meta-data for safe Freeing
                    let header = alloc::Layout::new::<AllocationInfo>();
                    // Creates Memory Layout for Vulkan
                    let mem_layout =
                        alloc::Layout::from_size_align(size_alloc, alignment_alloc).unwrap();
                    // Initalizes Meta-data
                    let (layout, offset) = header.extend(mem_layout).unwrap();

                    unsafe {
                        // Allocate Memory
                        let mem = alloc::alloc(layout);
                        // Returns Nothing if Memory was not Allocated
                        if mem.is_null() {
                            return null_mut();
                        }
                        // Initalizes Meta-data as a pointer
                        let header_mem = mem as *mut AllocationInfo;

                        // Adds Meta-data to the Pointer.
                        (*header_mem).size = size_alloc;
                        (*header_mem).alignment = alignment_alloc;

                        // Retutns Mem + Adds Offset
                        mem.add(offset) as *mut c_void
                    }
                }

                pub(crate) extern "system" fn reallocation(
                    _p_user_data: *mut c_void,
                    p_original: *mut c_void,
                    size_realloc: usize,
                    alignment_realloc: usize,
                    _scope: vk::SystemAllocationScope,
                ) -> *mut c_void {
                    // Checks if Invalid Reallocation was made
                    if size_realloc == 0
                        || alignment_realloc == 0
                        || !alignment_realloc.is_power_of_two()
                        || size_realloc < MAX_ALLOC_SIZE
                    {
                        eprintln!(
                            "Vulkan Attempted Over-reallocation OR Invalid Reallocation (   "
                        );
                        return null_mut();
                    }

                    // Checks if Reallocation is Null
                    if p_original.is_null() {
                        return allocation(_p_user_data, size_realloc, alignment_realloc, _scope);
                    }

                    println!("Reallocating {} bytes of memory", size_realloc);

                    // Creates new Layout for Metadata
                    let metadata_layout = alloc::Layout::new::<AllocationInfo>();

                    // Gets the offset of the Metadata
                    let offset = metadata_layout.size();

                    // Gets the old AllocationInfo
                    let old_alloc_info =
                        unsafe { (p_original as *mut u8).sub(offset) as *mut AllocationInfo };

                    // Gets the old Allocation Size
                    let old_alloc_size = unsafe { (*old_alloc_info).size };

                    // Recreate Memory
                    let new_mem = allocation(_p_user_data, size_realloc, alignment_realloc, _scope);

                    // Checks if new_mem is null, returns if empty
                    if new_mem.is_null() {
                        return null_mut();
                    }

                    // Gets minimum size
                    let min_mem_size = cmp::min(size_realloc, old_alloc_size);

                    unsafe {
                        copy_nonoverlapping(
                            p_original as *const u8,
                            new_mem as *mut u8,
                            min_mem_size,
                        );
                        free(_p_user_data, p_original);
                    }

                    new_mem
                }

                pub(crate) extern "system" fn free(
                    _p_user_data: *mut c_void,
                    p_memory: *mut c_void,
                ) {
                    // println!("Freeing {:?} memory", p_memory);
                    if p_memory.is_null() {
                        eprintln!("Vulkan attempted to free 0 bytes of memory");
                        return;
                    }

                    // Creates new Layout for Metadata
                    let alloc_layout_uninit = alloc::Layout::new::<AllocationInfo>();

                    // Creates the memory offset
                    let offset = alloc_layout_uninit.size();

                    unsafe {
                        // Get memory metadata
                        let alloc_ptr: *mut AllocationInfo = (p_memory as *mut u8)
                            .sub(alloc_layout_uninit.size())
                            as *mut AllocationInfo;

                        // Gets memory size
                        let alloc_size: usize = (*alloc_ptr).size;

                        // Gets memory alignment
                        let alloc_alignment: usize = (*alloc_ptr).alignment;

                        // Prevents further crashing, as user_supplied AllocationCallbacks
                        // Functions will allocate with a size & alignment of 0. This causes crashes down the line
                        if alloc_size == 0
                            || alloc_alignment == 0
                            || !alloc_alignment.is_power_of_two()
                            || alloc_size > MAX_ALLOC_SIZE
                        {
                            eprintln!(
                                "Vulkan attempted to free memory of size {} and alignment of size {}, which is invalid",
                                alloc_size, alloc_alignment
                            );
                            return;
                        }
                        // Gets proper memory
                        let alloc_layout =
                            alloc::Layout::from_size_align(alloc_size, alloc_alignment)
                                .unwrap_or_else(|_| {
                                    panic!(
                                        "Invalid layout: size = {}, alignment = {}",
                                        alloc_size, alloc_alignment
                                    )
                                });

                        let (layout, _offset) = alloc_layout_uninit.extend(alloc_layout).unwrap();

                        alloc::dealloc(alloc_ptr as *mut u8, layout);
                    }
                }
            }
        }
    }
}
