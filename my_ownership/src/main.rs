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
} // 自动调用一个drop函数

fn ownership_2() {
    let x = 5;
    let y = x;
    println!("y->{}", y);

    let s1 = String::from("hello");

    let s2 = s1; // 移动 mvoe 不是浅拷贝，这步执行完，s1就无效了
    println!("s2->{}", s2);

    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3->{},s4->{}", s3, s4);

    // 栈 上的数据拷贝,整形分配在栈上，可以直接拷贝
    let a1 = 5;
    let a2 = a1;
    println!("a1->{},a2->{}", a1, a2);

    //可copy的
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
}
