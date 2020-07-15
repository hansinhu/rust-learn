use std::env;
use std::process;

use minigrep::Config;

// CASE_INSENSITIVE=1 cargo run to poem.txt > output.txt

fn main() {
    // 在迭代器上调用 collect 以将其转换为包含向量的集合
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
      eprintln!("Problem parsing arguments: {}", err);
      process::exit(1);
    });

    println!("Searching for {}, In file {}:", config.query, config.filename);

    // run(config);
    if let Err(e) = minigrep::run(config) {
      eprintln!("Application error: {}", e);
      process::exit(1);
    }
}
