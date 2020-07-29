use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 递给 minigrep 的命令行参数并将其收集到一个 vector 中
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // let query = &args[1];
    // let filename = &args[2];

    // let (query, filename) = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    // println!("Searching for {}", query);
    // println!("In file {}", filename);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}



// 废弃
// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//
//     (query, filename)
// }

