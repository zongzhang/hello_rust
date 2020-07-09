// fn main() {
//     let s = String::from("hello");
//
//     change(&s);
// }
//
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }
// &s 不可变的

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//不过可变引用有一个很大的限制：在特定作用域中的特定数据只能有一个可变引用。这些代码会失败：
// {
// let mut s = String::from("hello");
//
// let r1 = &mut s;
// let r2 = &mut s;
//
// println ! ("{}, {}", r1, r2);
// }

//悬垂指针
// fn main() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String { // dangle 返回一个字符串的引用
//     let s = String::from("hello"); // s 是一个新字符串
//
//     &s // 返回字符串 s 的引用
// }  // 这里 s 离开作用域并被丢弃。其内存被释放。 // 危险！
// 当 dangle 的代码执行完毕后，s 将被释放。不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String