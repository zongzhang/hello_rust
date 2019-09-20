use std::iter::Sum;
use std::fmt::{Display, Debug};

//fn largest<T>(list: &[T]) -> T {}
//pub 公有
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
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
        format!("{} : {}", self.username, self.content)
    }
}

//pub fn notify(item: impl Summary) {
//    println!("breaking new! {}", item.summarize())
//}

//语法糖
//pub fn notify<T: Summary>(item: T) {
//    println!("breaking new! {}", item.summarize())
//}

//指定多个
pub fn notify(item: impl Summary + Display) {
    println!("breaking new! {}", item.summarize())
}

//fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
//
//}

//使用 where 改进
//fn some_function<T, U>(t: T, u: U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug
//{
//
//}

//返回 trait，只适合返回单一类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}


#[allow(dead_code)]
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(dead_code)]
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
//    let number_list = vec![34, 50, 25, 100, 65];
//
//    let result = largest_i32(&number_list);
//    println!("The largest number is {}", result);
//
//    let char_list = vec!['y', 'm', 'a', 'q'];
//
//    let result = largest_char(&char_list);
//    println!("The largest char is {}", result);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already konwm ,people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize())
}