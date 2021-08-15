use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str>{
        // args[0] is execution path, then come the command line args
        if args.len()<3 {
            return Err("not enough arguments");
        }

        // only checking if variable is set
        // if it is then case sensitive
        // if it isn't, then case insensitive
        // TODO: make UX clearer
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config {
            query:args[1].clone(), 
            filename:args[2].clone(),
            case_sensitive:case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!{"With text:\n{}", contents};

    let matched_lines = if config.case_sensitive{
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
        };

    println!{"\n Matching lines:"};
    for line in matched_lines {
        println!{"{}",line};
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
     // drawback of this approach: 
     // create a big vector first before filtering it out
    let lines: Vec<&str> = contents.split("\n").collect();
   
    let matching_lines = lines.into_iter().filter(
        |&line| line.contains(query)).collect();
    matching_lines
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query_lower = query.to_lowercase();
    
    // note: tried to call search, but 2 problems:
    // ownership a bit tricky, and
    // probably best to return the content with casing not lower
    // search(&query_lower, &content_lower)

    // here trying a different implementation as "search", for fun
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower) {
            results.push(line);
        }
    }
    results
}

# [cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_new_too_few_args(){ 
        let input_args: Vec<String> = vec!["one_arg_only".to_string()];
        let output = Config::new(&input_args);
        assert!(output.is_err());
    }

    #[test]
    fn config_correctly_parsed(){ 
        let input_args: Vec<String> = vec![
            "target/debug/minigrep".to_string(),
            "to_parse".to_string(),
            "file.txt".to_string()
            ];
        let output = Config::new(&input_args).unwrap();
        assert_eq!(output.filename, "file.txt".to_string());
    }

    #[test]
    fn successful_grep_example(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents));
        
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
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
