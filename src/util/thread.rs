use std::{
    thread
};

pub fn print_current_thread() {
    let thread_id = thread::current().id();
    println!("The thread: {:?}", thread_id);
}