// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     let four = IpAddr2::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr2::V6(String::from("::1"));
//
// }
// //fn route(ip_type: IpAddrKind) { }
//
// enum IpAddrKind {
//     //成员（variants）
//     V4,
//     V6,
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// enum IpAddr2 {
//     V4(String),
//     V6(String),
// }
//
// enum IpAddr3 {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//
// }
//
// impl Message {
//     fn call(&self) {}
// }

//枚举Option<T>，而且它定义于标准库中
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let absent_number: Option<i32> = None;

    //Option<i8> 与 i8 相加 错误
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
}