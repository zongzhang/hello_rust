fn main() {
    let x = 5;
    println!("x->{}", x);
    // 不能编译，x赋值了两次
    // x = 6;
    // println!("x->{}",x)
    let mut y = 5;
    println!("y->{}", y);
    y = 6;
    println!("y->{}", y);

    //常量
    const MAX_POINTS: u32 = 100_000;

    println!("const - > {}", MAX_POINTS);

    //隐藏
    let z = 5;
    let z = z + 1;
    let z = z * 2;
}
