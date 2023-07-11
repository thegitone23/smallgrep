use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = fs::read_to_string(config.file_path)?;
	println!("Contents in the file are : ");
	println!("{0}", content);
  println!("Done !");
	return Ok(());
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
