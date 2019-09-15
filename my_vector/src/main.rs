#[allow(unused_variables)]
fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4, 5];

//    v.push(5);
//    v.push(6);
//    v.push(7);
//    v.push(8);

    let third: &i32 = &v[2];
    match v.get(2) {
        Some(third) => println!("{}", third),
        None => println!("none"),
    };
}// <- 这里 v 离开作用域并被丢弃,所有其内容也会被丢弃
