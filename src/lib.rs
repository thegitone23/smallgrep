use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = fs::read_to_string(config.file_path)?;

  for line in search(&config.query, &content) {
    println!("{line}");
  }

  return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  return results;
}

pub struct Config {
	pub query: String,
	pub file_path: String
}

impl Config {
	pub fn build(args: &[String]) -> Result<Config, &'static str> {

		if args.len() < 3 {
			return Err("expected <query_string> <filepath> as arguments");
		}

		let query = args[1].clone();
		let file_path = args[2].clone();
	
		return Ok(Config{query, file_path});
	}		
}

#[cfg(test)]
mod tests {
	use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}
