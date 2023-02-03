use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn learn_thread_message() {
    multi_type_message();
    if false {
        buffer_channel_message();
        multi_sender_message();
        recv_loop_message();
        channel_owner_message();
        channel_noblock_message();
        channel_message();
    }
}

fn channel_message() {
    // tx(Transmit)发送，rx(Receive)接收。
    // 类型自动推导
    // 异步：sender发送消息不会阻塞，不必等接收者接收消息，可以直接发送第二条、第三条消息。
    // 相当于go的无限缓冲的chan。
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

fn channel_owner_message() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        // String的话，没有实现Copy trait， 所有权会被转移走。
        // 后续会无法print s。
        // let s = String::from("test move owner of String");
        let s = "test move(borrow) owner of &str";
        tx.send(s).unwrap();
        println!("str is {}", s);
    });

    let received = rx.recv().unwrap();
    println!("receive: {}", received);
}

fn recv_loop_message() {
    let (tx, rx) = mpsc::channel();
    let tx_handle = thread::spawn(move || {
        let vals = vec![
            String::from("hey"),
            String::from("如果我是"),
            String::from("DJ"),
            String::from("你会"),
            String::from("爱我吗"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(600));
        }
    });

    let rx_handle = thread::spawn(move || {
        for received in rx {
            println!("receive: {}", received);
        }
    });
    tx_handle.join().unwrap();
    rx_handle.join().unwrap();
}

fn multi_sender_message() {
    // 想使用多个sender发送给同一个receiver的话，使用clone()
    // 两个发送者线程谁先发送并不确定。
    // 当所有sender都drop以后，receiver才会结束for。
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(String::from("message from original tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("message from cloned tx")).unwrap();
    });

    for received in rx {
        println!("received: {}", received);
    }
}

fn buffer_channel_message() {
    // channel大小设置为0，相当于go的无缓冲chan。
    // channel大小设置大于0时，相当于go的等缓冲大小的chan。
    let (tx, rx) = mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("send one");
        tx.send(1).unwrap();
        println!("send two");
        tx.send(2).unwrap();
    });
    thread::sleep(Duration::from_millis(1000));
    println!("receive: {}", rx.recv().unwrap());
    println!("receive: {}", rx.recv().unwrap());
    handle.join().unwrap();
}

fn multi_type_message() {
    enum Fruit {
        Apple(u8),
        Banana(String),
        Orange(bool),
    }
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();
    tx.send(Fruit::Apple(10)).unwrap();
    tx.send(Fruit::Banana(String::from("msg"))).unwrap();
    tx.send(Fruit::Orange(false)).unwrap();
    for _ in 0..3 {
        match rx.recv().unwrap() {
            Fruit::Apple(v) => println!("receive Apple: {}", v),
            Fruit::Banana(v) => println!("receive Banana: {}", v),
            Fruit::Orange(v) => println!("receive Orange: {}", v),
        }
    }
}