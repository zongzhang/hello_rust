use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing argsuments: {}", err);
        process::exit(1);
    });


    //因为 run 在成功时返回 ()，而我们只关心检测错误，所以并不需要 unwrap_or_else 来返回未封装的值，因为它只会是 ()。
    if let Err(e) = minigrep::run(config) {
        eprintln!("Appliaction error: {}", e);
        process::exit(1);
    }
}
