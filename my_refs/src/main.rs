// 引用与借用
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效。
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("len->,{}", len);

    //在特定作用域中的特定数据有且只有一个可变引用
    let mut s = String::from("hello");

    change(&mut s);

    println!("s->{}", s);

    let mut s2 = String::from("hello");
    {
        let _r1 = &mut s2;
    }
    let _r2 = &mut s2;
}

// &-引用 *-解引用
// 借用，这里的&s不可变
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可变引用
fn change(some_string: &mut String) {
    some_string.push_str(",world");
}

#[allow(dead_code)]
fn change2() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}
