use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 在迭代器上调用 collect 以将其转换为包含向量的集合
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // run(config);
    if let Err(e) = minigrep::run(config) {
      println!("Application error: {}", e);
      process::exit(1);
    }
}
