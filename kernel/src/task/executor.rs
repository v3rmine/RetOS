use crate::task::task::{Task, TaskId, TaskWaker};
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, sync::Arc};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;
use spin::{Lazy, Mutex};
use x86_64::instructions::interrupts;
use x86_64::instructions::interrupts::enable_and_hlt;

pub static WAKER_CACHE: Lazy<Mutex<BTreeMap<TaskId, Waker>>> = Lazy::new(|| Mutex::new(BTreeMap::new()));
pub static TASKS: Mutex<BTreeMap<TaskId, Task>> = Mutex::new(BTreeMap::new());
pub static NEW_TASKS: Mutex<Vec<Task>> = Mutex::new(Vec::new());
pub static TASK_QUEUE: Lazy<Mutex<Arc<ArrayQueue<TaskId>>>> = Lazy::new(|| Mutex::new(Arc::new(ArrayQueue::new(100))));

pub fn spawn_task(task: Task) {
    NEW_TASKS.lock().push(task);
}

fn run_ready_tasks() {
    let mut tasks = TASKS.lock();
    let mut new_tasks = NEW_TASKS.lock();

    for task in new_tasks.drain(..) {
        let task_id = task.id;
        if tasks.insert(task_id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        TASK_QUEUE.lock().push(task_id).expect("queue full");
    }

    drop(new_tasks);

    let task_queue = TASK_QUEUE.lock();
    while let Some(task_id) = task_queue.pop() {
        let mut waker_cache = WAKER_CACHE.lock();

        let task = match tasks.get_mut(&task_id) {
            Some(task) => task,
            // task no longer existe
            None => continue,
        };

        let waker = waker_cache
            .entry(task_id)
            .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));

        let mut context = Context::from_waker(waker);
        match task.poll(&mut context) {
            Poll::Ready(()) => {
                tasks.remove(&task_id);
                waker_cache.remove(&task_id);
            }
            Poll::Pending => {}
        }
    }
}

pub fn run_tasks() -> ! {
    loop {
        run_ready_tasks();
        sleep_if_idle();
    }
}

fn sleep_if_idle() {
    interrupts::disable();

    if TASK_QUEUE.lock().is_empty() {
        enable_and_hlt();
    } else {
        interrupts::enable();
    }
}