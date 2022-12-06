use std::cell::{Cell, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

pub fn learn_smart_pointer() {
    learn_cell();
    if false {
        learn_rc();
        learn_box();
        learn_deref();
        learn_drop();
        learn_arc();
    }
}

// Box用法
fn learn_box() {
    // 将变量放到堆上：a和c在堆时，b在栈上
    let a = Box::new(3);
    let b = 3;
    let c = Box::new(4);
    println!("{:p}, {:p}, {:p}", a, &b, c);


    // 避免栈上数据拷贝
    let arr = [0; 1000];
    let arr1 = arr;
    // 各自拥有栈上的数组，栈没有所有权概念，“=”拷贝，不报错
    println!("arr: {:?}", arr.len());
    println!("arr1:{:?}", arr1.len());
    let arr = Box::new([0; 1000]);
    let arr1 = arr;
    println!("Box arr1:{:?}", arr1.len());
    // arr的所有权已经转移给arr1，下方print会报错
    // println!("Box arr:{:?}", arr.len());


    // 将动态代销转换为Sized固定大小
    #[allow(unused)]
    enum List {
        // Box包裹List放到堆上，由嵌套的位置大小换位为栈上固定大小
        Cons(i32, Box<List>),
        Nil,
    }


    // 包裹特征对象，实现使用和将不同实现放到同一个Box里
    trait Draw {
        fn draw(&self);
    }
    struct Button {
        id: u32,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("button index = {}", self.id);
        }
    }
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Button { id: 2 })];
    elems.iter().for_each(|e| e.draw());
}

// 自定义类型MyBox类似Box，实现Deref trait用以解引用
fn learn_deref() {
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            // 返回的是一个引用，可以被*解引用
            &self.0
        }
    }
    let b = MyBox::new(5);
    println!("value in MyBox b is {}", *b);
    // DerefMut，deref，但是是mut
    // 注意DerefMut是Deref的Super trait，必须分别写一个impl，不能简写到
    // DerefMut里
    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    let mut s = MyBox::new(String::from("hello"));
    fn display(s: &mut String) {
        s.push_str("world");
        println!("{}", s);
    }
    display(&mut s);
}

fn learn_drop() {
    struct HasDrop1;
    impl Drop for HasDrop1 {
        fn drop(&mut self) {
            println!("Drop HasDrop1");
        }
    }
    struct HasDrop2;
    impl Drop for HasDrop2 {
        fn drop(&mut self) {
            println!("Drop HasDrop2");
        }
    }
    struct HasDrop {
        #[allow(unused)]
        d1: HasDrop1,
        #[allow(unused)]
        d2: HasDrop2,
    }
    impl Drop for HasDrop {
        fn drop(&mut self) {
            println!("Drop HasDrop");
        }
    }
    struct Foo;
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Drop Foo");
        }
    }
    let _x = HasDrop {
        d1: HasDrop1,
        d2: HasDrop2,
    };
    let _f = Foo;
    println!("Start DROP!");
    /*
    输出：
    Start DROP!
    Drop Foo
    Drop HasDrop
    Drop HasDrop1
    Drop HasDrop2
    函数段内和defer一样，后声明先drop，先声明后drop
    变量内按声明顺序一样，先声明先drop，后声明后drop
     */
}

// RC：Reference Counting引用计数，用于一些必须“违反”所有权规则的场景，比如：
// * 图里可能多个多条边连接到同一个点
// * 多线程中可能多个线程持有同一个数据（其实用Arc）
fn learn_rc() {
    let a = Rc::new(String::from("hello, world!"));
    let b = Rc::clone(&a);
    println!("a strong count: {}", Rc::strong_count(&a));
    println!("b strong count: {}", Rc::strong_count(&b));
    {
        let c = Rc::clone(&a);
        println!("a strong count: {}", Rc::strong_count(&a));
        println!("c strong count: {}", Rc::strong_count(&c));
    }
    println!("a strong count: {}", Rc::strong_count(&a));

    println!("---------------------");
    // 多线程的Rc，会编译报错，因为Rc的引用计数并没有任何原语或锁，不支持多线程
    // let s = Rc::new(String::from("Multi Thread"));
    // for _ in 0..10 {
    //     let s = Rc::clone(&s);
    //     let handle = thread::spawn(move || {
    //         println!("Sub Thread {}", s)
    //     });
    // }
}

// Arc = Atomic Rc，原子化的Rc，故支持多线程（并发）
fn learn_arc() {
    let s = Arc::new(String::from("Multi Thread"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        thread::spawn(move || {
            println!("Sub Thread {}", s)
        });
    }
}

// Cell和RefCell用于在不可变引用中修改数据
// Cell和RefCell的区别仅在于Cell<T>适用于T实现了Copy的情况
fn learn_cell() {
    let c = Cell::new("Hello");
    let one = c.get();
    c.set("World");
    let two = c.get();
    println!("one={}, two={}", one, two);
    // RefCell将所有权报错推迟到运行期
    // 意义在于，如果有一个引用类型需要被四处借用和修改，导致借用关系难以管理时
    // 使用RefCell
    let s = RefCell::new(String::from("Hello World"));
    s.borrow();
    // s.borrow_mut();
    // println!("{} {}", s1, s2);
}
