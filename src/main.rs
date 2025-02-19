
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

//use blog_os::serial_println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use blog_os::println;
//use core::panic::PanicInfo; 

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};


//#[no_mangle]
//pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
    use blog_os::allocator;
    use blog_os::memory::{self, BootInfoFrameAllocator};

    //use blog_os::memory::translate_addr;
//    use blog_os::memory;
    //use blog_os::memory::active_level_4_table;
    //use x86_64::{structures::paging::Translate};
    //use x86_64::{structures::paging::Page};
    use x86_64::VirtAddr;
//    use blog_os::memory::BootInfoFrameAllocator;

    println!("Hello World{}", "!");
        
    blog_os::init();

    //let mut frame_allocator = unsafe {
    //    BootInfoFrameAllocator::init(&boot_info.memory_map)
    //};

    /*
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };


    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }

    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = reference_counted.clone();
    println!("courrent reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now ", Rc::strong_count(&cloned_reference));

    */ 

    /*
    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();
    */ 

    //let ptr = 0x204301 as *mut u8;
    //unsafe { let x = *ptr;}
    //println!("read worked");

    //unsafe { *ptr = 42;}
    //println!("write worked");
    

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset)};
    //let mut frame_allocator = memory::EmptyFrameAllocator;

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    
    /*
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);


    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e)
    };

    
    */ 

    //let l4_table = unsafe { active_level_4_table(phys_mem_offset)};

    /*
    for (i, entry) in l4_table.iter().enumerate(){
        if !entry.is_unused(){
            println!("L4 Entry {}: {:?}", i , entry);
    
            use x86_64::structures::paging::PageTable;

            if !entry.is_unused(){
                println!("L4 entry {}: {:?}", i, entry);    

                let phys = entry.frame().unwrap().start_address();
                let virt = phys.as_u64() + boot_info.physical_memory_offset;
                let ptr = VirtAddr::new(virt).as_mut_ptr();
                let l3_table: &PageTable = unsafe { &*ptr};


                // print non empty entries of the level 3 table 

                for (i, entry) in l3_table.iter().enumerate(){

                    if !entry.is_unused(){
                        println!(" l3 entry {}: {:?}", i , entry);
                    }
                }





            }

        }
    }
    */ 

    //use x86_64::registers::control::Cr3;

    //let (level_4_page_table, _) = Cr3::read();
   // println!("Level 4 page table at {:?}", level_4_page_table.start_address());
    /*
    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];


    for &address in &addresses {
        let virt = VirtAddr::new(address);
        //let phys = unsafe { translate_addr(virt, phys_mem_offset)};
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
    */ 
   
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {}", Rc::strong_count(&cloned_reference));

    

    #[cfg(test)]
    test_main();

    println!("it did not crash!");
    blog_os::hlt_loop();
    //loop{}
    //
    loop {
        use blog_os::print;
        print!("-");
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
    //loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

/*
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());

}
*/ 

#[test_case]
fn trivial_assertion() {
    serial_println!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}

