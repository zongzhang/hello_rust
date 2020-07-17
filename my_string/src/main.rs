// 核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str
//  String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    // let mut s = String::new();
    //
    // let data = "initial contents";
    //
    // let s = data.to_string();
    // println!("{}", s);
    //
    //
    // let mut s = "initial contents".to_string();
    // println!("{}", s);

    // let mut s = String::from("foo");
    // s.push_str("bar");
    // println!("{}", s);
    //
    // let mut s = String::from("lo");
    // s.push('l');//push 方法被定义为获取一个单独的字符作为参数
    // println!("{}", s);

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    // //s1 在相加后不再有效的原因，和使用 s2 的引用的原因，与使用 + 运算符时调用的函数签名有关。+ 运算符使用了 add 函数，这个函数签名看起来像这样：
    // //fn add(self, s: &str) -> String {
    // //之所以能够在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str
    // println!("{}", s3);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    //
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", s);

    // Rust 的字符串不支持索引
    // String 是一个 Vec<u8> 的封装。
    //字节、标量值和字形簇
    //最后一个 Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间 (O(1))。
    // 但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}",s); dddd
}
