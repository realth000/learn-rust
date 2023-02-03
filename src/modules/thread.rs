use core::cell::RefCell;
use std::sync::Once;
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub fn learn_thread() {
    singleton_thread();
    if false {
        condition_thread();
        local_variable_thread();
        barrier_thread();
        move_var_thread();
        safe_thread();
        unsafe_thread();
    }
}

fn unsafe_thread() {
    // 启动子线程
    // 线程启动顺序不固定，也会因为主线程的结束而结束
    thread::spawn(|| {
        for i in 1..10 {
            println!(
                "number {} from spawned thread {:?}",
                i,
                thread::current().id()
            );
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 主线程等待一段时间
    thread::sleep(Duration::from_millis(20));
}

fn safe_thread() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!(
                "number {} from safely spawned thread {:?}",
                i,
                thread::current().id()
            );
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!(
            "number {} from safely spawned thread {:?}",
            i,
            thread::current().id()
        );
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn move_var_thread() {
    let v = vec![1, 2, 3];
    // 转移v的使用权到其他线程
    // 由于无法确定线程之间的开始顺序和结束顺序，
    // 不move的话无法确定在子线程的周期中v是否一直可用。
    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });
    handle.join().unwrap();
}

fn barrier_thread() {
    let mut handlers = Vec::with_capacity(6);
    // barrier提供线程在某个点wait，然后等到同一组barrier中所有线程均
    // 执行完毕后再继续执行的能力。
    // 类似于go的WaitGroup。
    let barrier = Arc::new(Barrier::new(6));
    for i in 0..6 {
        // 每个线程拿一个barrier
        let b = barrier.clone();
        handlers.push(thread::spawn(move || {
            println!("{} before wait", i);
            // 拿着barrier在这里等
            b.wait();
            // 等到所有barrier中的线程均wait以后，才会进行下面的代码。
            println!("{} after wait", i);
        }));
    }
    for handle in handlers {
        handle.join().unwrap();
    }
}

fn local_variable_thread() {
    // 创建线程局部变量FOO
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    // 通过FOO.with()获取FOO中的值。
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
        println!(
            "FOO in {:?}, addr={:?}, value addr={:?}",
            thread::current().id(),
            &FOO,
            f.as_ptr()
        );
    });
    let t = thread::spawn(move || {
        // 每个线程在开始时，都能拿到FOO的值。
        // 每个线程中的f会在定义的FOO被销毁时销毁，
        // 换言之，每个线程中的f可能存活到当前线程销毁以后。
        // 那么可能从中获取错误的thread::current().id()，因为此时原本的线程已经没了。
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            // 改变局部变量的值改为4。
            *f.borrow_mut() = 4;
            // 每个线程中FOO内的地址都不同
            println!(
                "FOO in {:?}, addr={:?}, value addr={:?}",
                thread::current().id(),
                &FOO,
                f.as_ptr()
            );
        });
    });
    t.join().unwrap();
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
        println!(
            "FOO in {:?}, addr={:?}, value addr={:?}",
            thread::current().id(),
            &FOO,
            f.as_ptr()
        );
    });
}

fn condition_thread() {
    // 条件变量Condition Variable
    // pair 和 pair2可以理解为同一把锁的两端，各自传给一个线程。
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(300));
        let &(ref lock, ref cvar) = &*pair2;
        // 获取到锁，然后change。
        let mut started = lock.lock().unwrap();
        println!("[sub] changing started");
        *started = true;
        // 如果注释notify_one()，cvar另一端的主线程会导致收不到通知
        // 虽然started 变成true，依然wait。
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("[main] wait begin");
    if !*started {
        println!("[main] wait once");
        // 开始wait，等待子线程把started改了。
        started = cvar.wait(started).unwrap();
    } else {
        println!("[main] started already changed");
    }
    println!("[main] wait end");
    println!("[main] started changed to {}", started);
}

fn singleton_thread() {
    static mut VAL: usize = 0;
    // INIT保证只被调用一次，类似单例初始化。
    static INIT: Once = Once::new();

    let handler1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });

    let handler2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });
    handler1.join().unwrap();
    handler2.join().unwrap();
    println!("{}", unsafe { VAL });
}
