// crate 隐式模块
// └── front_of_house
// ├── hosting
// │   ├── add_to_waitlist
// │   └── seat_at_table
// └── serving
// ├── take_order
// ├── serve_order
// └── take_payment
// 绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
// 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。

// Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。
// 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。
// mod front_of_house {
//     pub mod hosting {//模块可以被引用
//         pub fn add_to_waitlist() {}//函数可以被引用
//
//         fn seat_at_table() {}//私有
//     }
//
//     mod serving {
//         fn take_order() {}
//
//         fn server_order() {}
//
//         fn take_payment() {}
//     }
// }
//
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

//super 关键字
// fn serve_order() {}
//
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }
//
//     fn cook_order() {}
// }

//结构体
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,//私有
//     }
//
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }
//
// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//
//     // meal.seasonal_fruit = String::from("blueberries");
//
// }

//枚举
// mod back_of_house {
//     pub enum Appetizer {
//         Soup, //全部公有
//         Salad,//全部公有
//     }
// }
//
// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// use关键字
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
//
// use crate::front_of_house::hosting;
//
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

//使用 pub use 重导出名称
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
//
// pub use crate::front_of_house::hosting;
//
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}