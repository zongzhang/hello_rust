fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        _ => println!("ss"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("matched ,y={:?}", y),
        _ => println!("Default case,x = {:?}", x),
    };

    println!("at the end : x={:?},y={:?}", x, y);
}
