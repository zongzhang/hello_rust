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
    println!("z -> {}", z);

    let spaces = "   ";
    //隐藏了spaces,从新赋值，包括类型也改变了
    let spaces = spaces.len();

    println!("spaces -> {}", spaces);

    // 类型变了
    // let mut spaces = "   ";
    // spaces = spaces.len();

    //整形
    let _z1: i8 = 1;
    let _z2: u8 = 2;
    let _z3: i16 = 3;
    let _z4: u16 = 4;
    let _z5: i32 = 5;
    let _z6: u32 = 6;
    //在64位构架是64，在32位构架是32
    let _z7: isize = 7;
    let _z8: usize = 8;
    //浮点型
    let _f1: f32 = 1.0;
    let _f2: f64 = 2.0;

    let sum = 5 + 10;
    println!("sum->{}", sum);

    let bb: bool = true;
    println!("bb - > {}", bb);

    //char 是单引号
    let s = '😻';
    println!("z->>{}", s);

    //复合类型 元组 数组
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //模式匹配来解构
    let (_x3, _y3, _z3) = &tup;

    println!("_x3->{}", _x3);

    let big_num = tup.0;
    println!("big_num->{}", big_num);

    let _a = [1, 2, 3, 4, 5, 6];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //无效数组
    println!("ss->>{}", a[10]);
    println!("arr->{}", a[0]);
}
