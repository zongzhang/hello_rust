fn main() {
    println!("Hello, world!");
    another_function();

    //这是一个语句，
    let _y = 6;
    //语句没有返回值
    // let x = (let y = 6); 错的

    // {.....} 是表达式
    let y = {
        let x = 3;
        x + 1 // 没有分号
    };
    println!("y->{}", y);

    let x = five();
    println!("x->{}", x);

    let number = 3;
    // 判断条件返回类型应该是bool
    if number < 5 {
        println!("number < 5");
    } else {
        println!("number > 5");
    }

    let condition = true;
    // if 是个表达式
    let number = if condition { 55 } else { 66 };

    println!("number -> {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result->{}", result);
    assert_eq!(result, 20);

    //for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value->{}", element);
    }
}

fn another_function() {
    println!("another function")
}

fn five() -> i32 {
    5
}