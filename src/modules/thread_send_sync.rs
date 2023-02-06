use std::sync::Arc;
use std::thread;

/*
    Send: 实现此特征的类型可以在线程间传递所有权。
    Sync: 实现此特征的类型可以在线程间共享引用。
 */

pub fn learn_thread_send_sync() {
    learn_impl_trait();
    if false {
        learn_simple();
    }
}

fn learn_simple() {
    let v = Arc::new(5);
    let t = thread::spawn(move || {
        println!("v={v}");
    });
    t.join().unwrap();
}

// 为裸指针实现Send特征
fn learn_impl_trait() {
    #[derive(Debug)]
    struct MyBox(*mut u8);
    unsafe impl Send for MyBox {

    }

    let p = MyBox(5 as *mut u8);
    let t = thread::spawn(move || {
        println!("{p:?}");
    });
    t.join().unwrap();
}
