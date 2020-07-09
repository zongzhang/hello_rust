fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_param(5, 6);

    // 表达式
    let y = {
        let x = 3;
        x + 1
    };

    println!("five()->{}", five());
    println!("five_with_param()->{}", five_with_param(5));
}

fn another_function() {
    println!("another function")
}

fn another_function_with_param(x: i32, y: i32) {
    println!("another function x->{},y-{}", x, y);
}


fn five() -> i32 {
    5
}

fn five_with_param(x: i32) -> i32 {
    x + 5
}