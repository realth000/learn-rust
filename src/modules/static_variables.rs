use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/*
 全局静态常量，在编译期初始化。
 1. 声明时使用const，且必须指明类型，名称大写+下划线。
 2. 全局声明周期，存在遮盖。
 3. 会内联，不是只有一个实例，所有调用的地方不是指向同一个内存地址。
 */
const MAX_ID: usize = usize::MAX / 2;

/*
 全局静态变量，在编译器初始化。
 1. 声明时使用static mut，且必须指明类型，名称大写+下划线。
 2. 全局变量周期，存在遮盖。
 3. 不会内联，只有一个实例，所有调用的地方指向同一个内存地址。
 4. 使用时必须使用unsafe包裹。
 5. 需要实现Sync特征。
 6. 默认不线程安全，只建议在单线程中使用。
 */
static mut REQUEST_RECV: usize = 0;

/*
 全局的原子类型，不需要mut（内部可变）。
 1. 线程安全。
 */
static ATOMIC_REQUST_RECV: AtomicUsize = AtomicUsize::new(0);

struct Factory {
    factory_id: usize,
}

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate_id() -> usize {
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("overflow!~");
    }
    let next_id = GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("overflow!~");
    }
    next_id
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id()
        }
    }
}

pub fn learn_static_variables() {
    for _ in 0..100 {
        ATOMIC_REQUST_RECV.fetch_add(1, Ordering::Relaxed);
    }

    unsafe {
        REQUEST_RECV += 1;
    }

    loop {
        let f = Factory::new();
        if f.factory_id > 100 {
            break;
        }
    }

    learn_box_leak();
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG: Option<&mut Config> = None;

trait Expose {
    fn exp(&self);
}

impl Expose for Option<&mut Config> {
    fn exp(&self) {
        match self {
            Some(v) => println!("a={:?}, b={:?}", v.a, v.b),
            None => println!("empty"),
        }
    }
}

fn learn_box_leak() {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        // 运行期制造全局变量
        CONFIG = Some(Box::leak(c));
        CONFIG.exp();
        match &CONFIG {
            Some(v) => println!("a={:?} b={:?}", v.a, v.b),
            None => println!("NONE"),
        }
    }
}
