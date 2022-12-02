extern crate core;

use std::{fmt, io};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::net::IpAddr;
use std::num::TryFromIntError;

use crate::modules::lifecycle::learn_lifecycle;

mod modules;

fn main() {
    learn_lifecycle();
    if false {
        learn_format_output();
        learn_panic();
        learn_type_convert();
        learn_hash_map();
        learn_vector();
        learn_new_type();
        learn_trait_implement();
        learn_call_same_name_functions();
        learn_relate_type();
        learn_trait_object_dynamic_dispatch();
        learn_trait_object();
        learn_trait();
        learn_trait_2();
        learn_generics();
        learn_struct_method();
        learn_enum();
        learn_complex_type();
        learn_reference();
        learn_borrow();
        learn_ownership();
        learn_function();
    }
}

// 格式化输出
fn learn_format_output() {
    println!("Hello");
    println!("Hello, {}!", "world");
    println!("The number is {}", 1);
    println!("{:?}", (3, 4));
    println!("{value} {} {value2}", "111", value2 = 44, value = 4);
    println!("{} {}", 1, 2);
    println!("{:04} {:02.2}", 42, 1.2345);
    println!("{4} {3} {2} {1} {0}", 0, 1, 2, 3, 4);
    // 输出占五格，不够的话在后面用空格补齐，超出的话全输出
    println!("Hello {:5}123 {:5}", "x", "XXXXXXXxxxxx");
    // 用第[1]个参数来指定输出宽度
    println!("Hello {} {:2$}! {} {}", 4, "x", 5, 6);
    // 用第[0]个参数来指定输出宽度，输出的字符串是第[1]个参数
    println!("Hello {1:0$}!", 5, "x1");
    // 用指定名称的参数来指定输出宽度
    println!("Hello {:width$}! {width}", "x", width = 5);
    println!("-------------------");
    // 输出宽度5
    println!("Hello {:5}!", 5);
    // 用“+”输出正负号，用“-”输出负号
    println!("Hello {:+5}! {:-5}! {:+5}! {:-5}!", 5, 5, -5, -5);
    // 宽度为5，用0填充，负号也占一个宽度
    println!("Hello {:05}!", -5);
    println!("-------------------");
    // 左对齐
    println!("Hello {:<5}!", "x");
    // 右对齐
    println!("Hello {:>5}!", "x");
    // 居中对齐
    println!("Hello {:^5}!", "x");
    // 对齐并使用指定符号填充
    println!("Hello {:&^5}!", "x");
    println!("-------------------");
    let v = 3.1415926535897;
    let s = v.to_string();
    println!("{:.2}", v);
    println!("{:+.2}", v);
    println!("{:.0}", v);
    println!("{:^1$}", v, 20);
    println!("{:.3}", s);
    println!("Hello {:.*}!", 3, s);
    println!("-------------------");
    // 二进制
    println!("{:#b}!", 27);
    // 八进制
    println!("{:#o}!", 27);
    // 小写十六进制
    println!("{:#x}!", 27);
    // 大写十六进制
    println!("{:#X}!", 27);
    // 不带进制前缀
    println!("{:x}!", 27);
    // 二进制，用0填充，宽度为10
    println!("{:#010b}!", 27);
    println!("-------------------");
    // 指数 科学计数法？
    println!("{:e}", 100000000);
    println!("{:E}", 1000000);
    println!("{:e}", 0.005232);
    // 用空格填充10位宽度
    println!("{:10e}", 0.005232);
    println!("-------------------");
    let v = vec![1, 2, 3];
    println!("{:p}", v.as_ptr());
    println!("{:16p}", v.as_ptr());
}

// panic
fn learn_panic() {
    let _home: IpAddr = "127.0.0.1".parse().unwrap();
    // unwrap发生错误时直接panic
    // let _bad_home: IpAddr = "1127.0.0.1".parse().unwrap();
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("hello.txt not found and failed to create: {:?}", e),
            }
            other_error => panic!("error opening hello.txt: {:?}", other_error),
        },
    };
    let data = f.metadata().expect("failed to get metadata for file hello.txt");
    println!("file data: {:?}", data.is_file());
    println!("------------------- 传播Error");
    fn read_user_name_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    let e = match read_user_name_from_file() {
        Ok(un) => un,
        Err(e) => {
            println!("read user name failed: {}", e.to_string());
            String::new()
        }
    };
    println!("read user name success: {}", e);
    println!("------------------- 传播Error 简便写法 ex");
    fn read_user_name_from_file_ex() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        println!("read_user_name_from_file_ex(): {}", s);
        Ok(s)
    }
    let s = read_user_name_from_file_ex();
    println!("{}", s.unwrap());
    println!("------------------- 传播Error 简便写法 ex_ex");
    // Dart，Kotlin：我超
    // GO：麻了
    fn read_user_name_from_file_ex_ex() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    let s = read_user_name_from_file_ex_ex();
    println!("{}", s.unwrap());
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    last_char_of_first_line("123");
}

// 类型转换
/*
https://zhuanlan.zhihu.com/p/372082802
&str    -> String--| String::from(s) or s.to_string() or s.to_owned()
&str    -> &[u8]---| s.as_bytes()
&str    -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()
String  -> &str----| &s if possible* else s.as_str()
String  -> &[u8]---| s.as_bytes()
String  -> Vec<u8>-| s.into_bytes()
&[u8]   -> &str----| s.to_vec() or s.to_owned()
&[u8]   -> String--| std::str::from_utf8(s).unwrap(), but don't**
&[u8]   -> Vec<u8>-| String::from_utf8(s).unwrap(), but don't**
Vec<u8> -> &str----| &s if possible* else s.as_slice()
Vec<u8> -> String--| std::str::from_utf8(&s).unwrap(), but don't**
Vec<u8> -> &[u8]---| String::from_utf8(s).unwrap(), but don't**
 */
fn learn_type_convert() {
    // 用as进行转换
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("{} {} {}", a, b, c);

    // 转换&str和String
    let d: &str = "abc";
    let e: String = String::from(d);
    let f: String = d.to_string();
    let g: &str = f.as_str();
    let h: &str = &f;
    let i: &String = &f;
    // 由此可见 &str == &String?
    println!("{} {} {} {} {} {}", d, e, f, g, h, i);

    // 内存地址转换为指针
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    // 将p1的内存地址转换成一个整数
    let first_address = p1 as usize;
    // 往后跳4字节，到了values[1]的开头地址
    let second_address = first_address + 4;
    // 将values[2]的内存地址转化成一个指针
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);

    // 使用TryInto转换类型
    // let a: u8 = 10;
    let b: u16 = 100;
    // try_info然后match处理
    let b_: u8 = match b.try_into() {
        Ok(v) => v,
        Err(e) => {
            println!("ERROR! {}", e);
            0
        }
    };
    // try_info但是分两步处理
    let b__: Result<u8, TryFromIntError> = b.try_into();
    match b__ {
        Ok(v) => println!("OK! {}", v),
        Err(e) => println!("ERROR! {}", e),
    }
    println!("b_={}", b_);


    // 强制类型转换
}

// HashMap
fn learn_hash_map() {
    // 普通地创建一个，然后一个一个插入
    let mut my_gems = HashMap::new();
    my_gems.insert("RED", 1);
    my_gems.insert("BLUE", 2);
    let teams_list = vec![
        (String::from("Aaa"), 100),
        (String::from("Bbb"), 200),
        (String::from("Ccc"), 300),
    ];
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("通过collect定义和初始化HashMap:{:?}", teams_map);

    // 查询HashMap
    let mut scores = HashMap::new();
    scores.insert("Red", 10);
    scores.insert("Blue", 20);
    let score = scores.get("Red");
    println!("获取scores元素key=Red:{:?}, is_none={}", score, score.is_none());

    // 遍历查询HashMap
    for (key, value) in &scores {
        println!("遍历查询：key={}, value={}", key, value);
    }

    // 更新HashMap中的值
    // 覆盖已有的值
    let old = scores.insert("Red", 20);
    assert_eq!(old, Some(10));
    // 查询新的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));
    // 查询，并且若不存在则插入
    let v = scores.entry("Blue2").or_insert(5);
    assert_eq!(*v, 5);
    let vv = scores.entry("Blue2").or_insert(55);
    // Blue2已存在，不会将其值修改为55，下面的断言会失败
    assert_eq!(*vv, 55);
}

// Vector
fn learn_vector() {
    // let v: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(100);
    v.push(200);
    v.push(300);

    let mut u: Vec<i32> = Vec::with_capacity(1);
    u.push(1);
    println!("u={:?}, {}", u, u.capacity());
    u.push(2);
    println!("u={:?}, {}", u, u.capacity());

    // vec!
    let w = vec![1, 2, 3];
    println!("w={:?}", w);

    // 获取元素
    let a: &i32 = &v[2];
    println!("第三个元素:{}", a);
    match v.get(3) {
        Some(e) => println!("第三个元素In match:{}", e),
        None => println!("None in match"),
    }

    // Iterator
    for i in &v {
        println!("get i iterator: {}", i)
    }

    // Modify in iterating
    for i in &mut v {
        *i += 10
    }

    for i in &v {
        println!("get i iterator: {}", i)
    }

    // 存储不同类型元素
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let w = vec![
        IpAddr::V4(String::from("127.0.0.1")),
        IpAddr::V6(String::from("::1")),
    ];
    fn show_addr(ip: &IpAddr) {
        println!("元素：{:?}", ip);
    }
    println!("不同类型元素：");
    for i in &w {
        show_addr(i);
    }
    println!("----------------");

    // 存储不同的特征对象
    trait IpAddrTrait {
        fn display(&self);
    }
    struct V4(String);
    impl IpAddrTrait for V4 {
        fn display(&self) {
            println!("IpAddrTrait for V4: {:?}", self.0);
        }
    }
    struct V6(String);
    impl IpAddrTrait for V6 {
        fn display(&self) {
            println!("IpAddrTrait for V6: {:?}", self.0);
        }
    }
    let x: Vec<Box<dyn IpAddrTrait>> = vec![
        Box::new(V4(String::from("127.0.0.1"))),
        Box::new(V6(String::from("::1"))),
    ];
    for i in x {
        i.display();
    }
}

// new type
fn learn_new_type() {
    struct Wrapper(Vec<String>);
    impl Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world!")]);
    println!("w = {}", w);
}

// trait的“继承”
fn learn_trait_implement() {
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("outline_print result: {}({})", output, len);
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    impl Display for Point {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}
    let p = Point {
        x: 1,
        y: 2,
    };
    p.outline_print();
}

// 调用同名方法
fn learn_call_same_name_functions() {
    trait Pilot {
        fn fly(&self);
        fn jobs();
    }
    trait Wizard {
        fn fly(&self);
        fn jobs();
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("Captain's here!")
        }
        fn jobs() {
            println!("Drive planes");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Magic!")
        }
        fn jobs() {
            println!("Have power");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("Small and big step")
        }
        fn jobs() {
            println!("Launch on the moon.");
        }
    }
    let i = Human;
    i.fly();
    Pilot::fly(&i);
    Wizard::fly(&i);

    Human::jobs();
    // 完全限定语法
    <Human as Pilot>::jobs();
    <Human as Wizard>::jobs();
}

// 关联类型
fn learn_relate_type() {
    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    struct Counter {
        x: i32,
    }
    impl Iterator for Counter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            return Some(self.x + 100);
        }
    }
    println!("test Counter->Iterator={:?}", Counter { x: 1 }.next());
}

// 特征对象动态分发
fn learn_trait_object_dynamic_dispatch() {
    // self and Self.
    trait Draw {
        fn draw(&self) -> Self;
        // 返回Self类型，因此返回的类型在编译时不确定，因此Draw不具有对象安全。
    }
}

// 特征对象
fn learn_trait_object() {
    pub trait Draw {
        fn draw(&self);
    }
    pub struct Button {
        width: u32,
        height: u32,
        label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("Draw Button width={}, height={}, label={}", self.width, self.height, self.label);
        }
    }
    pub struct SelectBox {
        width: u32,
        height: u32,
        label: String,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Draw Button width={}, height={}, label={}", self.width, self.height, self.label);
        }
    }
    // pub struct Screen {
    //     pub components: Vec<Box<dyn Draw>>,
    // }
    // impl Screen {
    //     pub fn run(&self) {
    //         for component in self.components.iter() {
    //             component.draw();
    //         }
    //     }
    // }

    fn draw1(x: Box<dyn Draw>) {
        x.draw();
    }
    fn draw2(x: &dyn Draw) {
        x.draw();
    }
    let b1 = Box::new(SelectBox {
        width: 80,
        height: 30,
        label: String::from("Test SelectBox"),
    });
    draw1(b1);
    let b2 = Button {
        width: 60,
        height: 20,
        label: String::from("Test Button"),
    };
    draw2(&b2);
}

// Trait
fn learn_trait() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("Article:{}, Author:{}", self.title, self.author)
        }
    }
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }
    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published {}", self.username, self.content)
        }
    }
    println!("{}", Post {
        title: "t".to_string(),
        author: "a".to_string(),
        content: "c".to_string(),
    }.summarize());
    println!("{}", Weibo { username: "u".to_string(), content: "c".to_string() }.summarize());
    // Killer
    pub fn notify(item: &impl Summary) {
        println!("NOTIFY: {}", item.summarize());
    }
    // fn same_notify<T: Summary>(item1: &T, item2: &T) {}
    // fn multiple_notify(item: &(impl Summary + Display)) {}
    // fn same_multiple_notify<T: Summary + Display>(item: &T) {}
    // fn complex<T, U>(t: &T, u: &U) -> i32
    //     where T: Display + Clone,
    //           U: Clone + Debug
    // {
    //     1
    // }
    let w = Weibo { username: "".to_string(), content: "".to_string() };
    notify(&w);
}

fn learn_trait_2() {
    // 特征约束
    struct Pair<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Pair<T, U> {
        fn new(x: T, y: U) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    // 约束特征类型T和U
    impl<T: Display + PartialOrd + Debug, U: Display + Debug> Pair<T, U> {
        fn cmd_display(&self) {
            println!("{} {}", self.x, self.y);
        }
    }

    let p = Pair::<i32, i32> {
        x: 1,
        y: 2,
    };
    p.cmd_display();
    let q = Pair::new(1, 2);
    q.cmd_display();
}

// Generics
fn learn_generics() {
    fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
        a + b
    }
    println!("ADD i8: {}", add(2i8, 3i8));
    println!("ADD i32: {}", add(3i32, 4i32));
    println!("ADD f64: {}", add(5.67, 8.90));
    // Const Generics
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("display_array: {:?}, length={}", arr, N);
    }
    display_array([1, 2, 3]);
    display_array([1, 2, 3, 4]);
}

// Struct Method
fn learn_struct_method() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x,
                y,
                radius,
            }
        }
        fn info(&self) {
            println!("INFO: x={}, y={}, radius={}", self.x, self.y, self.radius);
        }
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }
    let c = Circle::new(2.0, 3.0, 1.0);
    println!("C: {}, {}", c.x, c.y);
    c.info();
    println!("AREA: {}", c.area());
}

// Enum
fn learn_enum() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    enum MyEnum {
        Foo,
        Bar,
    }
    impl MyEnum {
        fn call() {
            println!("called.");
        }
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let _ = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    MyEnum::call();
    println!("plus_one_result: {:?} {:?}", plus_one(Some(5)), plus_one(None));
}

// Complex Type
fn learn_complex_type() {
    #![allow(unused_variables)]
    type File = String;
    fn open(f: &mut File) -> bool {
        true
    }
    fn close(f: &mut File) -> bool {
        false
    }
    #[allow(dead_code)]
    fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
        unimplemented!()
    }

    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(&mut f1, &mut vec![]);
    close(&mut f1);


    // S1有两个匿名字段，分别用self.0和self.1访问
    #[derive(Debug)]
    struct S1(String, i32);
    impl S1 {
        fn display(&self) {
            println!("打印匿名字段(String,i32):({},{})", self.0, self.1);
        }
    }
    let f2 = S1(String::from("abc"), 1);
    f2.display();
}

// Reference
fn learn_reference() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(" hello");
    r1.push_str(" hello");
    println!("1 s={}", s);
    {
        let r2 = &mut s;
        r2.push_str(" r2");
    }
    println!("2 s={}", s);
}

// Borrow
fn learn_borrow() {
    let x = 5;
    let y = &x;
    let z = 5;
    let w = &z;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(x, z);
    assert_eq!(y, w);
    assert_eq!(*y, *w);
    assert_eq!(&x, y);
    assert_eq!(&x, w);
}

// Ownership

fn learn_ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy: {}", some_integer);
}

// Function

fn learn_function() {
    let x = 4;
    let y = if x % 2 == 1 { "AAA" } else { "BBB" };
    another_function(x, y);
}

fn another_function(x: i32, y: &str) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
