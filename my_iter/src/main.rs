fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFT OFF");


    let v1 = vec![1, 2, 3];

    //迭代器是惰性的 v1_iter是不可变的 iter_mut() 返回可变
    let v1_iter = v1.iter();


    for val in v1_iter {
        println!("Got: {}", val);
    }
    iterator_sum();
}

//type Item 和 Self::Item，他们定义了 trait 的 关联类型（associated type）
//trait Iterator {
//    type Item;
//
//    fn next(&mut self) -> Option<Self::Item>;
//
// 此处省略了方法的默认实现
//}

fn iterator_sum() {
    let v1 = vec![1, 2, 3, ];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();//sum方法消费适配器

    println!("{}", total);
}