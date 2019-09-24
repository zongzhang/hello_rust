use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );


    let x = 4;
    // x 在同一个作用域
    let equal_to_x = |z| z == x;

    let y = 4;

//    assert!(equal_to_x(y));
//    FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，
//   闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，
//   所以它只能被调用一次。
//    FnMut 获取可变的借用值所以可以改变其环境
//    Fn 从其环境获取不可变的借用值
}


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//尝试调用一个被推断为两个不同类型的闭包 会出错
//let example_closure = |x| x;
//
//let s = example_closure(String::from("hello"));
//let n = example_closure(5);


fn generate_workout(intensity: u32, random_number: u32) {
//    let expensive_result = simulated_expensive_calculation(intensity);

    // |param1, param2|
    let expensive_closure = |num| {
        println!("calulating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

//    let expensive_closure = |num: u32| -> u32 {
//        println!("calculating slowly...");
//        thread::sleep(Duration::from_secs(2));
//        num
//    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout_2(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}