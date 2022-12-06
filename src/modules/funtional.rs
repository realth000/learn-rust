use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn learn_functional() {
    learn_iterator();
    if false {
        learn_closure();
    }
}

// 闭包
fn learn_closure() {
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(3, sum(2));

    fn workout(process: u32, random_number: u32) {
        // 类似C++ lambda，Rust每个闭包都有独立的类型，即使参数和返回值相同
        // 也是不同类型
        let action = || {
            println!("exercising!");
            thread::sleep(Duration::from_millis(500));
            process
        };
        if process < 25 {
            println!("Heavy exercise! {}", action());
            println!("Suitable exercise! {}", action());
        } else if random_number == 3 {
            println!("Have a rest");
        } else {
            println!("Light exercise! {}", action());
        }
    }
    let process = 10;
    let random_number = 7;
    workout(process, random_number);
    println!("---------------");
    // Fn -> FnOnce -> FnMut
    // move
    #[derive(Debug)]
    struct Cacher<T>
        where T: Fn(u32) -> u32, {
        query: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
        where T: Fn(u32) -> u32 {
        fn new(query: T) -> Cacher<T> {
            Cacher {
                query,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let s: u32 = 1;
    let mut c = Cacher::new(|x| x + s);
    println!("10 in cache={:?}", c.value(10));
    println!("---------------");
    fn exec<F: Fn()>(f: F) {
        f()
    }
    let s = String::from("ABC");
    let update_string = move || println!("{}", s);
    exec(update_string);
    println!("---------------");
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        if x > 1 {
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x + num + 1)
        }
    }
    let result = factory(0)(1);
    println!("factory 0={:?}", <i32 as Into<i32>>::into(result));
}

// 迭代器
fn learn_iterator() {
    // 惰性初始化
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
    // for_each
    v1.iter().for_each(|x| println!("value is {}", x));
    // any 暂时没搞明白怎么用，后面没法调用或者collect收起来
    v1.iter().any(|x| x > &2);
    let origin = 0;
    // collect!
    let v2: Vec<String> = v1.iter().map(|v| (v + 1).to_string()).collect();
    println!("result v2:{:?}", v2);
    // fold!
    // 可以直接iter().fold()，不一定必须有map
    let v4 = v1.iter().map(|v| v).fold(origin, |u, v| u + v);
    println!("result v4:{:?}", v4);
    // zip and collect!
    let v3: HashMap<i32, String> = v1.into_iter().zip(v2.into_iter()).collect();
    println!("result v3:{:?}", v3);
    println!("---------------");
    // 为自定义类型实现Iterator特征
    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    let counter = Counter::new();
    println!("counter fold: {}", counter.into_iter().fold(0, |u, v| u + v));
    println!("---------------");
    let v = vec![1u64, 2, 3, 4, 5, 6];
    for (i, v) in v.iter().enumerate() {
        println!("第{}个值是{}", i, v);
    }
}
