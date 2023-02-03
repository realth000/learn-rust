use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn learn_thread_message() {
    channel_owner_message();
    if false {
        channel_noblock_message();
        channel_message();
    }
}

fn channel_message() {
    // tx(Transmit)发送，rx(Receive)接收。
    // 类型自动推导
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });
    // rx.recv() 会阻塞当前线程，类似go的chan。
    println!("receive: {}", rx.recv().unwrap());
}

fn channel_noblock_message() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(300));
        tx.send(1).unwrap();
    });
    loop {
        match rx.try_recv() {
            Ok(v) => {
                println!("receive {}", v);
                break;
            }
            Err(e) => {
                println!("error: {}", e);
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

fn channel_owner_message() {}
