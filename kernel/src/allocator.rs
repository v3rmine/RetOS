pub const HEAP_START: usize = 0x_4444_4444_0000;
/// 100 KiB
pub const HEAP_SIZE: usize = 100 * 1024;

pub static mut ARENA: [u8; HEAP_SIZE] = [0; HEAP_SIZE];