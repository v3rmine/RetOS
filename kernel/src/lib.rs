#![no_std]

#![feature(abi_x86_interrupt)]
#![feature(type_alias_impl_trait)]
#![feature(allocator_api)]

extern crate alloc;

use crate::allocator::ARENA;
use crate::interrupts::{gdt, idt};
use spin::Mutex;
use talc::{ClaimOnOom, Span, Talc, Talck};

pub mod printer;
pub mod interrupts;
pub mod memory;
pub mod allocator;
pub mod task;
pub mod cli;

pub fn init() {
    print!("\t> Initializing GDT... ");
    gdt::init();
    println!("initialized.");

    print!("\t> Initializing IDT... ");
    idt::init_idt();
    println!("initialized.");

    print!("\t> Initializing PICS... ");
    unsafe { interrupts::pics::PICS.write().initialize() };
    println!("initialized.");

    print!("\t> Initializing heap... ");
    #[global_allocator]
    static TALCK: Talck<Mutex<()>, ClaimOnOom> = Talc::new(unsafe {
        ClaimOnOom::new(Span::from_array(core::ptr::addr_of!(ARENA).cast_mut()))
    }).lock::<Mutex<()>>();
    unsafe {
        TALCK.lock().claim(Span::from(&raw mut ARENA)).expect("Could not claim heap");
    }
    println!("initialized");

    print!("\t> Enabling interrupts... ");
    x86_64::instructions::interrupts::enable();
    println!("enabled.");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}