#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let mut user1 = User {
        //整体可变
        email: String::from("some@some.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    println!("email->{}", user1.email);
    user1.email = String::from("another@123.com");
    println!("email->{}", user1.email);

    //使用 结构体更新语法 从其他实例创建实例
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("user3@123.com"),
        username: String::from("username"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

//结构体
//String 有意选择的
//使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期
#[allow(dead_code)]
struct User {
    //字段
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        email,//简写
        username,//简写
        active: true,
        sign_in_count: 1,
    }
}

//元组结构体
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);
