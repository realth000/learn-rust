use std::fmt;

pub fn learn_error_handle() {
    learn_custom_err_type();
    if false {
        learn_composite();
    }
}

fn learn_composite() {
    /*
        组合器：
        a.or(b): 表达式从左到右求值，若任何一个表达式的结果是 Some 或 Ok，则该值会立刻返回
        a.and(b)：若两个表达式的结果都是 Some 或 Ok，则第二个表达式中的值被返回。若任何一个的结果是 None 或 Err ，则立刻返回。
        a.filter(|x| -> bool)：过滤Option，剩下能得到true的。不能过滤Result。
        a.map(|x: | -> bool)：根据map的闭包参数，将a映射成另一个东西。
        a.ok_or()：将Option转为Result。
     */
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n1: Option<&str> = None;
    let n2: Option<&str> = None;

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");

    // a.or(b): 表达式从左到右求值，若任何一个表达式的结果是 Some 或 Ok，则该值会立刻返回。
    assert_eq!(s1.or(s2), s1);
    assert_eq!(s2.or(s1), s2);
    assert_eq!(s1.or(n1), s1);
    assert_eq!(n1.or(s1), s1);
    assert_eq!(n1.or(n2), n1);
    assert_eq!(n1.or(n2), n2);
    assert_eq!(o1.or(o2), o1);
    assert_eq!(o1.or(e1), o1);
    assert_eq!(e1.or(o1), o1);
    assert_eq!(e1.or(e2), e2);

    // a.and(b)：若两个表达式的结果都是 Some 或 Ok，则第二个表达式中的值被返回。若任何一个的结果是 None 或 Err ，则立刻返回。
    assert_eq!(s1.and(s2), s2);
    assert_eq!(s2.and(s1), s1);
    assert_eq!(s1.and(n1), n1);
    assert_eq!(n1.and(s1), n1);
    assert_eq!(n1.and(n2), n1);
    assert_eq!(n1.and(n2), n2);
    assert_eq!(o1.and(o2), o2);
    assert_eq!(o1.and(e1), e1);
    assert_eq!(e1.and(o1), e1);
    assert_eq!(e1.and(e2), e1);

    // a.filter(|x| -> bool)：过滤Option，剩下能得到true的。不能过滤Result。
    let fn_contains_1 = |x: &&str| x.contains("1");
    let test = Some(5);
    let test_fn = |x: &i8| x % 2 == 0;
    assert_eq!(test.filter(test_fn), None);
    assert_eq!(s1.filter(fn_contains_1), s1);
    assert_eq!(s2.filter(fn_contains_1), n1);

    // a.map(|x: | -> bool)：根据map的闭包参数，将a映射成另一个东西。
    let fn_char_count = |s: &str| s.chars().count();
    let fn_contains_x = |s: &str| s.contains('x');
    assert_eq!(s1.map(fn_char_count), Some(5));
    assert_eq!(s1.map(fn_contains_x), Some(false));

    // a.ok_or()：将Option转为Result。
    let o3: Result<&str, &str> = Ok("some1");
    assert_eq!(s1.ok_or("asd"), o3);
    assert_eq!(n1.ok_or("asd"), Err("asd"));
}

fn learn_custom_err_type() {
    #[derive(Debug)]
    struct AppError;

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "App Error")
        }
    }

    fn produce_app_error() -> Result<(), AppError> {
        Err(AppError)
    }

    match produce_app_error() {
        Err(e) => eprintln!("{e}"),
        _ => println!("no error"),
    }
}