use std::hint;
use std::ops::Sub;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::thread::{self, JoinHandle};

use tokio::time::Instant;

pub fn learn_thread_atomic() {
    learn_multi_thread_atomic();
    if false {
        learn_barrier();
        learn_simple_atomic();
    }
}

fn learn_simple_atomic() {
    const N_TIMES: u64 = 10000;
    const N_THREAD: usize = 10;

    static R: AtomicU64 = AtomicU64::new(0);

    fn add_n_times(n: u64) -> JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..n {
                R.fetch_add(1, Ordering::Relaxed);
            }
        })
    }

    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREAD);
    for _ in 0..N_THREAD {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }
    assert_eq!(N_TIMES * N_THREAD as u64, R.load(Ordering::Relaxed));
    println!("success: {:?}", Instant::now().sub(s));
}

fn learn_barrier() {
    static mut DATA: u64 = 0;
    static READY: AtomicBool = AtomicBool::new(false);

    fn reset() {
        unsafe {
            DATA = 0;
        }
        READY.store(false, Ordering::Relaxed);
    }

    fn producer() -> JoinHandle<()> {
        thread::spawn(move || {
            unsafe {
                DATA = 100;
            }
            READY.store(true, Ordering::Release);
        })
    }

    fn consumer() -> JoinHandle<()> {
        thread::spawn(move || {
            while !READY.load(Ordering::Acquire) {}
            assert_eq!(100, unsafe { DATA });
        })
    }

    reset(); // relaxed 不做限制
    let p = producer(); // released 前面的永远在前面
    let c = consumer(); // acquire 后面的永远在后面
    p.join().unwrap();
    c.join().unwrap();
    unsafe {
        println!("success: {DATA:?}");
    }
}

fn learn_multi_thread_atomic() {
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }
    if let Err(panic) = thread.join() {
        println!("error in thread: {panic:?}");
    } else {
        println!("success");
    }
}
