// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 元组重构 元组帮助我们增加了一些结构性
// fn main() {
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

//使用结构体
#[derive(Debug)] // 调试使用 增加注解来派生 Debug trait，并使用调试格式打印 Rectangle 实例
struct Rectangle {
    width: u32,
    height: u32,
}

// 结合不够紧密
// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
//
//     println!("rect1 is {:#?}", rect1);
// }
//
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// 改写成一个定义于 Rectangle 结构体上的 area 方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 多个impl块
impl Rectangle {
    // 关联函数，没有self。是函数，不是方法。与结构体实例无关
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    //关联函数
    let rect2 = Rectangle::square(3);
}