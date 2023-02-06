use std::sync::{Arc, Barrier, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

pub fn learn_thread_lock() {
    if false {
        learn_semaphore_thread();
        learn_condvar_sync_thread();
        learn_rwlock_thread();
        learn_mutex_thread();
        learn_mutex();
    }
}

fn learn_mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        match m.try_lock() {
            Ok(v) => println!("success: {v:?}"),
            Err(e) => println!("failed: {e:?}"),
        }
        *num = 6;
    }
    println!("m = {m:?}");
}

fn learn_mutex_thread() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Update num to {num} in thread {:?}", thread::current().id())
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("finish: num = {}", counter.lock().unwrap());
}

fn learn_rwlock_thread() {
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        match lock.try_read() {
            Ok(v) => println!("try_read success: {v}"),
            Err(e) => println!("try_read failed: {e}"),
        }
    }
    let guard = lock.try_read();
    println!("result: {guard:?}");
}

// 条件变量两个线程交替打印输出。
// FIXME: 似乎不用在两个线程里都notify_all，不可靠。
fn learn_condvar_sync_thread() {
    // cond内包含一个条件变量，用arc包起来以便用于多线程。
    // clone的cflag和ccond用来move到子线程。
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let sub_pair = Arc::clone(&pair);
    let barrier = Arc::new(Barrier::new(2));
    let sub_barrier = Arc::clone(&barrier);

    let handle = thread::spawn(move || {
        let mut counter = 0;
        let (lock, cvar) = &*sub_pair;
        sub_barrier.wait();
        let mut m = lock.lock().unwrap();
        while counter < 3 {
            while !*m {
                // 每次走完一个counter循环后，m都是false，必须等外面把它改成true。
                m = cvar.wait(m).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
            // 这块本来套在一个block里，不明所以，去掉了。
            // 改成false，让主线程等待。
            // 注意m是bool，不是 *bool， 通过m = false无法开始等待，
            // *cflag.lock().unwrap() = false;
            // println!("inside set m to {m}");
            counter += 1;
            *m = false;
            cvar.notify_one();
            println!("inside counter: {counter}, {}", *m);
        }
    });
    let (lock, cvar) = &*pair;
    let mut m = lock.lock().unwrap();
    let mut counter = 0;
    barrier.wait();
    while counter < 3 {
        while *m {
            // 每次走完一个counter循环后，m都是true，必须等里面把它改成false。
            // 默认m是true， 需要等外面把它改成false才跳出循环。
            m = cvar.wait(m).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
        counter += 1;
        *m = true;
        cvar.notify_one();
        println!("outside counter: {counter}, {}", *m);
    }
    handle.join().unwrap();
    println!("finish, counter={counter}");
}

// #[tokio::main]
fn learn_semaphore_thread() {
    // let semaphore = Arc::new(Semaphore::new(3));
    // let mut join_handlers = Vec::new();
    //
    // for _ in 0..5 {
    //     let permit = semaphore.clone().acquire_owned().await.unwrap();
    //     join_handlers.push(tokio::spawn(async move || {
    //         drop(permit);
    //     }));
    // }
}