#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    println!("{}", s);


    let mut s = "initial contents".to_string();

    println!("{}", s);

    s.push_str("ss");
    println!("{}", s);
}
