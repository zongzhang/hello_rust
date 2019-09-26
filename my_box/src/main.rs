//box
//当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
//当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
//当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
//fn main() {
//    let b = Box::new(5);
//    println!("b={}", b);
//}


//box打破了递归
//use crate::List::{Cons, Nil};
//
//enum List {
//    Cons(i32, Box<List>),
//    Nil,
//}
//
//fn main() {
//    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//}


use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//Deref trait，由标准库提供，要求实现名为 deref 的方法
impl<T> Deref for MyBox<T> {
    //type Target = T; 语法定义了用于此 trait 的关联类型
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    //*y = *(y.deref())
    assert_eq!(5, *y);

    let c = CustomSmartPointer { data: String::from("my stuff") };

//    如果我们需要强制提早清理值，可以使用 std::mem::drop 函数。
//    drop(c);

    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}

//Rc<T> 的类型。其名称为 引用计数（reference counting）的缩写。
//注意 Rc<T> 只能用于单线程场景；第十六章并发会涉及到如何在多线程程序中进行引用计数。
//enum List {
//    Cons(i32, Rc<List>),
//    Nil,
//}
//
//use crate::List::{Cons, Nil};
//use std::rc::Rc;
//
//fn main() {
//    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//    let b = Cons(3, Rc::clone(&a));
//    let c = Cons(4, Rc::clone(&a));
//}

//通过 RefCell<T> 在运行时检查借用规则