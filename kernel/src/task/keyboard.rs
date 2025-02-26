use crate::cli::commands::handle_command;
use crate::interrupts::idt::KEYBOARD;
use crate::{print, println};
use conquer_once::spin::OnceCell;
use core::pin::Pin;
use core::task::{Context, Poll};
use crossbeam_queue::ArrayQueue;
use embedded_cli::cli::CliBuilder;
use futures_util::task::AtomicWaker;
use futures_util::{Stream, StreamExt};
use pc_keyboard::DecodedKey;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub struct ScancodeStream {
    _private: (),
}

impl Default for ScancodeStream {
    fn default() -> Self {
        Self::new()
    }
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        // fast path
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

pub fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        match queue.push(scancode) {
            Ok(_) => WAKER.wake(),
            Err(_) => println!("WARNING: scancode queue full; dropping keyboard input")
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub async fn handle_keyboard() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = KEYBOARD.write();

    let command_buffer: [u8; 100] = [0; 100];
    let history_buffer: [u8; 100] = [0; 100];

    let mut cli = CliBuilder::default()
        .command_buffer(command_buffer)
        .history_buffer(history_buffer)
        .build()
        .ok()
        .unwrap();

    print!("> ");

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        print!("{}", character);

                        handle_command(&mut cli, character as u8);
                        
                        // New line
                        if character == '\n' {
                            print!("> ");
                        }
                    },
                    DecodedKey::RawKey(_) => {},
                }
            }
        }
    }
}