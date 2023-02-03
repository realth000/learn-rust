use std::cell::RefCell;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;

pub fn learn_weak_reference() {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                List::Cons(_, item) => Some(item),
                _nil => None,
            }
        }
    }
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    println!("a的初始化rc={}", Rc::strong_count(&a));
    println!("a指向的节点={:?}", a.tail());

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("在b创建后，a的rc={}", Rc::strong_count(&a));
    println!("b的初始化rc={}", Rc::strong_count(&b));
    println!("b指向的节点={:?}", b.tail());

    // 利用RefCell可变，创建从a到b的引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("在更改a后，b的rc={}", Rc::strong_count(&b));
    println!("在更改a后，a的rc={}", Rc::strong_count(&a));

    // println!("a->nex = {:?}", a.tail());

    println!("---------------------");

    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);
    drop(five);
    let strong_five = weak_five.upgrade();
    assert_eq!(strong_five, None);

    println!("---------------------");
    // 自引用
    struct Unmovable {
        data: String,
        slice: NonNull<String>,
        _pin: PhantomPinned,
    }

    impl Unmovable {
        fn new(data: String) -> Pin<Box<Self>> {
            let res = Unmovable {
                data,
                slice: NonNull::dangling(),
                _pin: PhantomPinned,
            };
            let mut boxed = Box::pin(res);
            let slice = NonNull::from(&boxed.data);
            unsafe {
                let mut_ref = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).slice = slice;
            }
            boxed
        }
    }
    let _u = Unmovable::new("asd".to_string());
    println!("_u: {:?}", _u.data);
    

    println!("---------------------");
}
