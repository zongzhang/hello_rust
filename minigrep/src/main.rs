use std::env;
use std::fs;
use std::process;
use minigrep;
use minigrep::Config;

fn main() {
    // 递给 minigrep 的命令行参数并将其收集到一个 vector 中
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // let query = &args[1];
    // let filename = &args[2];

    let (query, filename) = parse_config(&args);


    println!("Searching for {}", query);
    println!("In file {}", filename);


    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);


}

// 废弃
// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//
//     (query, filename)
// }

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}