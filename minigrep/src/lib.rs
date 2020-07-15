use std::env;
use std::fs;
use std::error::Error;


pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {

    if args.len() < 3 {
      return Err("Not Enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    // CASE_INSENSITIVE=1 cargo run to poem.txt
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // let contents = fs::read_to_string(config.filename)
  //     .expect("Something went wrong reading the file");
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

// 不区分大小写
pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
