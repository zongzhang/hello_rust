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

    const Z: u32 = 5;
    println!("z->{}", Z);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces len -> {}", spaces);

    let _c = 'z';


    let xx: (i32, f64, u8) = (500, -1.1, 1);
    let first = xx.0;
    println!("first->{}", first);

    let a = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("a-->{}", a[1]);

    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let _c = [3, 5];

}
