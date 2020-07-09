mod borrowing;

//所有权的存在就是为了管理堆数据
// Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
// 值在任一时刻有且只有一个所有者。
// 当所有者（变量）离开作用域，这个值将被丢弃。
fn main() {
    //----------------//s是无效的
    let _s = "hello"; //s是有效的，硬编码到文件中

    my_string();

    ownership_2();
} //作用域结束 s不在有效

fn my_string() {
    let _s = String::from(""); //在堆上分配了内存
    //必须在运行时向操作系统请求内存。
    //需要一个当我们处理完 String 时将内存返回给操作系统的方法。
    //::是运算符，
    //允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，而不需要使用类似 string_from 这样的名字。
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
} // 自动调用一个drop函数 释放s

fn ownership_2() {
    let x = 5;
    let y = x;
    println!("y->{}", y);

    let s1 = String::from("hello");

    let s2 = s1; // 移动 mvoe 不是浅拷贝，这步执行完，s1就无效了
    // println!("s1->{}", s1); s1 无效 已经move到s2了

    println!("s2->{}", s2);

    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3->{},s4->{}", s3, s4);

    // 栈 上的数据拷贝,整形分配在栈上，可以直接拷贝 无需使用clone
    let a1 = 5;
    let a2 = a1;
    println!("a1->{},a2->{}", a1, a2);

    //可copy的 Copy trait
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
}

//所有权与函数
fn takes_owership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
}// 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn make_copy(some_integer: i32)
{
    println!("{}", some_integer);
}// 这里，some_integer 移出作用域。不会有特殊操作


//我们想要函数使用一个值但不获取所有权该怎么办  rust提供引用
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

fn call_clbi() {
    let s1 = String::from("hello");

    let len = calculate_length_by_inf(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

//与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*
fn calculate_length_by_inf(s: &String) -> usize {
    s.len()
}