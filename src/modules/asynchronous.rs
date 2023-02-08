use futures::executor::block_on;

pub fn learn_asynchronous() {
    learn_use_async();
    if false {
        //
    }
}

fn learn_use_async() {
    // .await
    async fn do_b() {
        println!("do b");
    }
    async fn do_a() {
        do_b().await;
        println!("async ABC");
    }

    block_on(do_a());
}
