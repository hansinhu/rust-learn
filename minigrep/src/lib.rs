
use std::fs;
use std::error::Error;


pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {

    if args.len() < 3 {
      return Err("Not Enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // let contents = fs::read_to_string(config.filename)
  //     .expect("Something went wrong reading the file");
  let contents = fs::read_to_string(config.filename)?;
    
  println!("With text:\n{}", contents);

  Ok(())
}

// impl Config {
//   fn new(args: &[String]) -> Config {

//     if args.len() < 3 {
//       panic!("Not Enough arguments");
//     }

//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
//   }
// }

// fn parse_config(args: &[String]) -> Config {
//   let query = args[1].clone();
//   let filename = args[2].clone();

//   Config { query, filename }
// }
