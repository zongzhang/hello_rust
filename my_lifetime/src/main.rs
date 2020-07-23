// Rust 中的每一个引用都有其 生命周期 也就是引用保持有效的作用域

// 生命周期避免了悬垂引用

// fn main() {
//     println!("Hello, world!");
// }
//
// //
// fn test1() {
//     Rust 编译器有一个 借用检查器
//     r 和 x 的生命周期注解，分别叫做 'a 和 'b
//     {
//         let r;               // ---------+-- 'a
//         {                           //          |
//             let x = 5;        // -+-- 'b  |
//             r = &x;                 //  |       |
//         }                           // -+       |
//         println!("r: {}", r);       //          |
//     }                               // ---------+
// }
//
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// 借用检查器自身同样也无法确定，因为它不知道 x 和 y 的生命周期是如何与返回值的生命周期相关联的
// 生命周期 注解 语法
//&i32        // 引用
//&'a i32     // 带有显式生命周期的引用
//&'a mut i32 // 带有显式生命周期的可变引用
//
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 就像泛型类型参数，泛型生命周期参数需要声明在函数名和参数列表间的尖括号中
//泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期。
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// 结构体定义中的生命周期注解
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.')
//         .next()
//         .expect("Could not find a '.'");
//     let i = ImportantExcerpt { part: first_sentence };
// }

//生命周期省略规则
//函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes）
//返回值的生命周期被称为 输出生命周期（output lifetimes）。
fn main() {

}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// 编译器采用三条规则来判断引用何时不需要明确的注解。
// 第一条规则适用于输入生命周期，
// 后两条规则适用于输出生命周期。
// 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。
// 这些规则适用于 fn 定义，以及 impl 块。

// 第一条规则是每一个是引用的参数都有它自己的生命周期参数
// 有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数有两个不同的生命周期参数

// 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。

//第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)
// 那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。

// 第三条深入 方法定义中的生命周期注解
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }
//其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// 静态生命周期
// 所有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样标注出来：
// let s: &'static str = "I have a static lifetime.";


// 结合泛型类型参数、trait bounds 和生命周期
// use std::fmt::Display;
//
// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
//     where T: Display
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }