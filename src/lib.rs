use std::fs;
use std::env;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &content)
  } else {
    search(&config.query, &content)
  };

  for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  return results;
}

pub struct Config {
	pub query: String,
	pub file_path: String,
  pub ignore_case: bool
}

impl Config {
	pub fn build(args: &[String]) -> Result<Config, &'static str> {

		if args.len() < 3 {
			return Err("expected <query_string> <filepath> as arguments");
		}

		let query = args[1].clone();
		let file_path = args[2].clone();
    let ignore_case = env::var("IGNORE_CASE").is_ok();

		return Ok(Config{query, file_path, ignore_case});
	}		
}

#[cfg(test)]
mod tests {
	use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.
";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "dUcT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.
";
    assert_eq!(vec!["safe, fast, productive.", "Duct Tape."], search_case_insensitive(query, contents));
  }

}
