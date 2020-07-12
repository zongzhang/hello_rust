//match 控制流运算符
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// // 模式 => 代码或值
// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("luck penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }


// 绑定值的设计
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
//
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("state quarter from {:?}!", state);
//             25
//         }
//     }
// }

// 匹配Option<T>
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
//
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let _none = plus_one(None);
//     println!("six->{:?}",six);
// }

// _ 通配符
fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        _ => (),
    }
}