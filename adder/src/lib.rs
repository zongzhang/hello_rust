// cargo test
#[cfg(test)] //cfg 属性代表 configuration
mod tests {
    #[test] //表明了测试函数
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}