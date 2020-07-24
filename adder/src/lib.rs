// cargo test
#[cfg(test)] //cfg 属性代表 configuration
mod tests {
    // #[test] //表明了测试函数
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// 使用 should_panic 检查 panic