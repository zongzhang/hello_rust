#[allow(unused_variables)]
fn main() {
    
    let s = String::from("hello world");

    let len = s.len();

    let slice = &s[..];

    println!("slice->{}",slice);


    //这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。
    let s = "hello world";

}

//“字符串 slice” 的类型声明写作 &str
#[allow(dead_code)]
fn first_word(s: &String)->&str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}