#[allow(unused_variables)]
fn main() {
    //创建
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3, 4, 5];

    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);


    // 读取
    // let v = vec![1, 2, 3, 4, 5];
    //
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);
    //
    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // 越界情况
    // let does_not_exist = &v[100];  // 抛panic
    // let does_not_exist = v.get(100); // get返回Option<&T> 返回None

    // 以下代码编译不通过
    // 不能在相同作用域中同时存在可变和不可变引用
    // 以下代码：在拥有 vector 中项的引用的同时向其增加一个元素
    // vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存
    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &v[0];
    //
    // v.push(6);
    //
    // println!("The first element is: {}", first);

    // 遍历
    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }

    // 利用枚举保存多个值
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}// <- 这里 v 离开作用域并被丢弃   所有其内容也会被丢弃

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}