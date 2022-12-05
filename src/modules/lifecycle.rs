use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

pub fn learn_lifecycle() {
    lifecycle_troubleshooting();
    if false {
        lifecycle_struct();
        recognize();
        lifecycle_function();
    }
}

fn recognize() {
    // let r;
    // {
    //     let x = 5;
    //     // r悬垂指针
    //     r = &x; // 在花括号结束后，x生命周期到期，而r还在，因此报错
    // }
    // println!("r={}", r);
}

fn lifecycle_function() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }

        // 如果只用到了x，没有用到y，可以不给y标注生命周期
        // println!("y={}", y);
        // x

        // 返回局部变量的引用，会导致悬垂引用
        // let result = String::from("really long string");
        // result.as_str()
    }
    let string1 = String::from("abc");
    let string2 = "uvw_xyz";
    let result = longest(&string1, string2);
    println!("longest string is {}", result);
}

fn lifecycle_struct() {
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    // 如果有生命周期标注，在实现方法时需要标注
    impl<'a> ImportantExcerpt<'a> {
        // 方法有一个&self参数，所有输出生命周期都和&self的生命周期相同，
        // 因此不需要标注
        fn level(&self) -> i32 {
            println!("self.part={}", self.part);
            3
        }
        fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
            where
                'a : 'b {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Once upon a time, a little girl, tried to make a living");
    let first_sentence = novel.split('.').next().expect("Could not find a '.' to split");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    i.level();
    i.announce_and_return_part("abc");
    println!("i={:?}", i);
}

fn lifecycle_troubleshooting() {
    #[derive(Debug)]
    struct Foo;
    impl Foo {
        fn mutate_and_share(&mut self) -> &Self {
            &*self
        }
        fn share(&self) {}
    }

    let mut foo = Foo;
    foo.share();
    let loan = foo.mutate_and_share();
    // 虽然在mutate_and_share的可变借用参数&mut self的可变仅仅在方法内部有效，返回
    // 的&Self是不可变借用，但是由于生命周期消除规则的第三条，返回值的生命周期和参数里
    // 的self相同，并且返回的loan生命周期持续到main函数结束，所以可变借用参数&mut self
    // 的生命周期也持续到了main函数结束，导致下方再进行不可变借用share()时无法通过生命
    // 周期检查。
    // foo.share();
    println!("{:?}", loan);
    println!("--------------------------");
    fn get_default<K, V>(map: &mut HashMap<K, V>, key: K) -> &mut V
        where
            K: Clone + Eq + Hash,
            V: Default {
        // 如果下一行的map.get改成map.get_mut，编译器会认为下一行的可变借用
        // 持续到match结束，导致出现二次可变借用而无法通过编译。
        // 虽然代码上分析map.get_mut的可变借用在进入match之后就结束了。
        // 这也说明一个问题，在生命周期比较复杂或者编译器较为严格时，生命周期
        // 应该控制得尽量小，借用能小就小，能避免可变借用就避免可变借用。
        // 子本例子中体现为，match时使用map.get而非map.get_mut，
        // 转而把get_mut放到Some()情况中去。
        match map.get(&key) {
            Some(_) => map.get_mut(&key).unwrap(),
            None => {
                map.insert(key.clone(), V::default());
                map.get_mut(&key).unwrap()
            }
        }
    }
    let mut hm = HashMap::new();
    hm.insert("a", 1);
    hm.insert("b", 2);
    hm.insert("c", 3);
    let s = get_default(&mut hm, "a");
    println!("get_default result={}", s);
    println!("--------------------------");
    // 下方的f函数中的'a是凭空产生的生命周期，没有任何限制。
    // 实际编码过程中要尽量避免在操作裸指针或unsafe等操作时产生无界生命周期，
    // 比如运用生命周期消除规则
    fn f<'a, T>(x: *const T) -> &'a T {
        unsafe {
            &*x
        }
    }
    let a = 5;
    #[allow(unused)]
        let b = f(&a as *const i32);
    println!("--------------------------");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Point {
        fn move_to(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
    }
    let mut p = Point { x: 0, y: 1 };
    let r = &mut p;
    // rr是对r的再借用
    let rr: &Point = &*r;
    // 不能在再借用的持续时间内再次使用原来的借用
    // r.move_to(100, 100);
    println!("rr={:?}", rr);
    r.move_to(10, 10);
    println!("r={:?}", r);
    println!("--------------------------");
    // 复杂例子
    struct Manager<'a> {
        text: &'a str,
    }
    // 此处原本为Interface<'a>，通过编译的方法是，增加定义一个比'a更短的
    // 生命周期'b，用于标注成员manager的生命周期。
    struct Interface<'b, 'a: 'b> {
        manager: &'b mut Manager<'a>,
    }
    // 对应的，impl里也需要更改生命周期声明为<'b, 'a:'b>
    impl<'b, 'a: 'b> Interface<'b, 'a> {
        pub fn noop(self) {
            println!("Interface consumed {}", self.manager.text);
        }
    }
    struct List<'a> {
        manager: Manager<'a>,
    }
    // List同样更改生命周期标注
    impl<'b, 'a: 'b> List<'a> {
        // 此处的生命周期为'a，且List标注的生命周期也为'a，那么
        // get_interface的生命周期至少和List一样久，在下面的调用中就一直
        // 到main结束
        // 修改后，返回对的Interface生命周期为比'a更短的'b，而List生命周期
        // 依然为'a，因此List活的比其方法get_interface长，get_interface
        // 中的可变借用就不会持续到main结束
        pub fn get_interface(&'b mut self) -> Interface<'b, 'a> {
            Interface {
                manager: &mut self.manager,
            }
        }
    }
    fn use_list(list: &List) {
        println!("call use_list:{}", list.manager.text);
    }
    let mut list = List {
        manager: Manager {
            text: "hello",
        }
    };
    // let s: <'a> Interface = list.get_interface();
    list.get_interface().noop();
    println!("Interface should be dropped here and the borrow released");
    use_list(&list);
    println!("--------------------------");
    // &'static和T:'static
    fn get_memory_location() -> (usize, usize) {
        let string = "Hello World!";
        let pointer = string.as_ptr() as usize;
        let length = string.len();
        (pointer, length)
    }
    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
        unsafe {
            from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))
        }
    }
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!("The {} byts at 0x{:X} stored: {}", length, pointer
             , message);
    fn print_it<T: Debug + 'static>(input: &T) {
        println!("'static value passwd in is: {:?}", input);
    }
    fn print_it1(input: impl Debug + 'static) {
        println!("'static value passwd in is: {:?}", input);
    }
    let i = 5;
    print_it(&i);
    print_it1(i);
}
