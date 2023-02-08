use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

pub fn learn_unsafe() {
    learn_use_unsafe();
    if false {
        learn_deref_pointer();
    }
}

fn learn_deref_pointer() {
    let num = 5;
    let r1 = &num as *const i32;
    unsafe {
        // 裸指针并不安全，可能为空，可能直接段错误，都是UB。
        // 需要用unsafe包裹。
        // 实际上安全，因为r1来自已有的引用。
        println!("r1={}", *r1);
    }

    fn get_memory_address() -> (usize, usize) {
        let s = "Hello World!";
        let pointer = s.as_ptr() as usize;
        let length = s.len();
        (pointer, length)
    }

    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    let (p, l) = get_memory_address();
    let message = get_str_at_location(p, l);
    println!("0x{p:X}: {message} (len={l})");

    let a = 1;
    // 尽量使用as，更明显。
    let b = &a as *const i32;
    let c: *const i32 = &a;
    unsafe {
        println!("b={}, c={}", *b, *c);
    }

    let a: Box<i32> = Box::new(1);
    let b: *const i32 = &*a;
    let c: *const i32 = Box::into_raw(a);
    unsafe {
        println!("b={}, c={}", *b, *c);
    }
}

fn learn_use_unsafe() {
    // 有unsafe标识的函数或方法说明它是不安全的，可能出现问题，可能有额外要求。
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
}
