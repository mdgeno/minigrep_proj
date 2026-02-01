use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep_lib::search;  

fn main() {
    
	let argument: Vec<String> = env::args().collect();
/* 
//commented out to try match control flow construct equivalent
	let config = Config::build(&argument).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {err}");
		process::exit(1);
	});
*/

	let config = match Config::build(&argument){
		Ok(val) => val,
		Err(e) => {println!("Problem parisng arguments: {e}");
			  process::exit(1);}
	};


	println!("Searching for {}", config.query);
	println!("In file {}", config.file_path);
/* 
//commented out to try match control flow construct equivalent
	if let Err(e) = run(config){
		println!("Application error: {e}");
		process::exit(1);
	}; 
*/

	match run(config){
		Ok(_) => (),
		Err(e) => {println!("Application error: {e}");
			  process::exit(1);}
	}
}

struct Config{
	query: String,
	file_path: String 
}

impl Config{
	fn build(args: &Vec<String>) -> Result<Config, &'static str>{
		if args.len() < 3{
			return Err("not enough arguments");
		}
		Ok(Config{ query: args[1].clone(), file_path: args[2].clone() })
	}
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
	let lines = fs::read_to_string(config.file_path)?;  

	for slice in search(&config.query, &lines){  
		println!("{slice}");
	}

	Ok(())
}
