#![feature(abi_x86_interrupt)]
#![no_std]

use crate::interrupts::{idt,gdt};

pub mod print;
pub mod interrupts;

#[allow(dead_code)]
pub fn init() {
    gdt::init();
    idt::init_idt();
}