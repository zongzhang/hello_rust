use std::collections::HashMap;


fn main() {
    // 所有的键必须是相同类型，值也必须都是相同类型。
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("yellow"), 50);
    //
    //
    //
    //
    // let teams = vec![String::from("blue"),String::from("yellow")];
    // let initial_scores = vec![10,50];
    //
    // // HashMap<_, _> 类型注解是必要的
    // let _scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // //当 insert 调用将 field_name 和 field_value 移动到哈希 map 中后，将不能使用这两个绑定。
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // let team_name = String::from("Blue");
    // //score 类型是 Some(10)
    // let _score = scores.get(&team_name);
    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // 覆盖一个值
    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);
    //
    // println!("{:?}", scores);

    //只在键没有对应值时插入
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    //
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    //
    // println!("{:?}", scores);

    //根据旧的值更新一个新值
    // let text = "hello world wonderful world";
    //
    // let mut map = HashMap::new();
    //
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    //
    // println!("{:?}", map);

    //指定一个不同的 hasher 来切换为其它函数。hasher 是一个实现了 BuildHasher trait 的类型
}
