// 功能重复
// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
// }


fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
// 结构体中的范型
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }
//
// struct Point<T, U> {
//     x: T,
//     y: U,
// }


// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// fn main() {
//     let p = Point { x: 5, y: 10 };
//
//     println!("p.x = {}", p.x());
// }

