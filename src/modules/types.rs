use std::convert::TryFrom;
use std::fmt;

pub fn learn_types() {
    learn_enums();
    if false {
        learn_new_type();
        learn_type_alias();
        learn_not_return();
        learn_dynamic_sized_type();
    }
}


// new type wrapper.
// 如果要为内置类型实现trait，无法直接impl，可以包括一层wrapper
fn learn_new_type() {
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("a"), String::from("b"), String::from("c"), String::from("d")]);
    println!("w = {}", w);
}

// Type alias
fn learn_type_alias() {
    // Meters和u32实际相同，可以混用
    #[allow(unused)]
    type Meters = u32;
    // 减少模板代码使用
    #[allow(unused)]
        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    #[allow(unused)]
    type Thunk = Box<dyn Fn() + Send + 'static>;
}

fn learn_not_return() {
    let i = 2;
    #[allow(unused)]
        let v = match i {
        0..=3 => i,
        _ => {
            panic!("Invalid value: {}", i);
        }
    };
}

// 不定长类型
fn learn_dynamic_sized_type() {
    // &str和String都是“定长类型”，因为使用时为指针，存贮在栈上，
    // 同时另外存放实际数据的长度、地址等信息
    // let s1: str = "aaa";
    #[allow(unused)]
        let s2: &str = "bbb";
    // 泛型
    // 除了&str外的几乎所有类型都实现了Sized，也就是T: Sized，这
    // 保证了即使类型T不加任何额外约束也能在编译时知道相应变量的大小
    #[allow(unused)]
    fn generic<T>(t: T) {
        println!("generic type={}", std::any::type_name::<T>());
    }
    generic(&123);
    generic(&"123");
}


// 枚举和整型的转换
pub fn learn_enums() {
    enum MyEnum {
        Alice = 1,
        Bob,
        Coral,
    }
    // type Error = ();
    // 定义从i32到MyEnum转换
    impl TryFrom<i32> for MyEnum {
        type Error = ();
        fn try_from(v: i32) -> Result<Self, Self::Error> {
            match v {
                x if x == MyEnum::Alice as i32 => Ok(MyEnum::Alice),
                x if x == MyEnum::Bob as i32 => Ok(MyEnum::Bob),
                x if x == MyEnum::Coral as i32 => Ok(MyEnum::Coral),
                _ => Err(()),
            }
        }
    }
    let x = MyEnum::Coral as i32;
    // 麻烦！
    match x.try_into() {
        Ok(MyEnum::Alice) => println!("Alice"),
        Ok(MyEnum::Bob) => println!("Bob"),
        Ok(MyEnum::Coral) => println!("Coral"),
        Err(ev) => eprintln!("unknown number {:?}", ev),
    }
}
