fn main() {
    let some_u8_value = Some(3);

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("other");
    }
}


fn con() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

