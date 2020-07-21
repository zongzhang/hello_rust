// trait 类似于其他语言中的常被称为 接口（interfaces）的功能，但有一些不同。
// 不能为外部类型实现外部 trait
// 这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// 带默认实现
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 参数为实现了Summary的类型
// pub fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

//Trait Bound 语法
// item1 item2 可以不一样只要实现了Summary
// pub fn notify(item1: impl Summary, item2: impl Summary) {
// 强制item1 item2
//pub fn notify<T: Summary>(item1: T, item2: T) {


//pub fn notify(item: impl Summary + Display) {
// pub fn notify<T: Summary + Display>(item: T) {


// where
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// 简化后
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {


// 返回实现了trait
// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }

// error 这样不行
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best
//             hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// 使用 trait bound 有条件地实现方法
// 不过只有那些为 T 类型实现了 PartialOrd trait （来允许比较）
// 和 Display trait （来启用打印）的 Pair<T> 才会实现 cmp_display 方法：
// use std::fmt::Display;
//
// struct Pair<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }
//
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }