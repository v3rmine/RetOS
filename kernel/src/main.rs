#![no_std]
#![no_main]

use bootloader_api::config::Mapping;
use bootloader_api::{BootInfo, BootloaderConfig};
use core::panic::PanicInfo;
use retos_kernel::{printer, println};

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

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().expect("No framebuffer");
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();
    printer::buffer::set_framebuffer(buffer, info);

    println!("{HELLO_WORLD}");
    println!();

    retos_kernel::init();
    retos_kernel::hlt_loop();
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}