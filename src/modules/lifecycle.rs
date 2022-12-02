pub fn learn_lifecycle() {
    lifecycle_erase();
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
            3
        }
    }

    let novel = String::from("Once upon a time, a little girl, tried to make a living");
    let first_sentence = novel.split('.').next().expect("Could not find a '.' to split");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i={:?}", i);
}

fn lifecycle_erase() {}
