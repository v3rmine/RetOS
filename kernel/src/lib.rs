#![no_std]

#![feature(abi_x86_interrupt)]
#![feature(type_alias_impl_trait)]

use crate::interrupts::{gdt, idt};
use crate::interrupts::idt::IDT;

pub mod printer;
pub mod interrupts;


pub fn init() {
    print!("Initializing GDT... ");
    gdt::init();
    println!("initialized.");
    print!("Initializing IDT... ");
    idt::init_idt();
    println!("initialized.");

    print!("Initializing PICS... ");
    unsafe { interrupts::pics::PICS.write().initialize() };
    println!("initialized.");
    println!("IDT loaded at: {:?}", &IDT as *const _);

    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}