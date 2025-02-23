#![no_std]
#![no_main]

mod vga;

use bootloader_api::BootInfo;
use core::panic::PanicInfo;

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
    vga::buffer::set_framebuffer(buffer, info);

    println!("{HELLO_WORLD}");
    
    loop {
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}