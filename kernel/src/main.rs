#![no_std]
#![no_main]

pub mod print;

use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use retos_kernel::interrupts::{gdt, idt};

const HELLO_WORLD: &str = r#"
╭----------------------------------╮
|            Welcome to            |
|   _____      _    ____   _____   |
|  |  __ \    | |  / __ \ / ____|  |
|  | |__) |___| |_| |  | | (___    |
|  |  _  // _ \ __| |  | |\___ \   |
|  | | \ \  __/ |_| |__| |____) |  |
|  |_|  \_\___|\__|\____/|_____/   |
╰----------------------------------╯
"#;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().expect("No framebuffer");
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();
    print::buffer::set_framebuffer(buffer, info);

    println!("{HELLO_WORLD}");
    println!();

    print!("Initializing GDT... ");
    gdt::init();
    println!("initialized.");
    print!("Initializing IDT... ");
    idt::init_idt();
    println!("initialized.");

    loop {
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}