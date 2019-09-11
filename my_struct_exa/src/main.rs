#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

//方法
#[allow(dead_code)]
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //关联函数 没有self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}
#[allow(unused_variables)]
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };


    println!("mian ji {:#?}", rect1.area());
//    println!("mian ji->{:?}", area(&rect1));

    let rect2 = Rectangle::square(3);
    println!("hehe {:#?}", rect2);

    let s = String::from("");
}

//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}
