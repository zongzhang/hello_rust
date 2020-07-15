use rand::Rng;


// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io}; 嵌套路径

// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// use std::collections::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("{}",secret_number);

}