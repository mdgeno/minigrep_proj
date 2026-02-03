
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
	let mut query_list = Vec::new();

	for line in contents.lines(){
		if line.contains(query){
			query_list.push(line);
		}
	}

	query_list  
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn one_result(){
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}
}
