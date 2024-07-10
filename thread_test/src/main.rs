use std::{thread, time};

static mut L: bool = false;

static mut COUNT: i32 = 0;

fn lock() {
    unsafe {
        while L {}
        L = true;
    }
}

fn release() {
    unsafe {
        L = false;
    }
}

fn main() {
    thread::spawn(|| {
        for _i in 1..10000 {
            lock();
            unsafe {
                COUNT += 1;
            }
            release();
        }
    });
    thread::spawn(|| {
        for _i in 1..10000 {
            lock();
            unsafe {
                COUNT += 1;
            }
            release();
        }
    });
    thread::sleep(time::Duration::from_secs(2));
    unsafe {
        print!("{}", COUNT);
    }
}
