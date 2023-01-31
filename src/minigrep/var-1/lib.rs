//cargo run -- to poem.txt   // копировать в консольную строку для запуска
use std::fs;
use std::error::Error;
use std::env;
pub struct Config<'a> {
    pub query: &'a String,        // вариант решения со ссылками в элементах структуры
    pub file_path: &'a String,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
        pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {  

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config { query, file_path, ignore_case })

    }

    pub fn search<'b>(query: &str, contents: &'b str) -> Vec<&'b str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
            
/*  // старый код
        let mut results: Vec<&str> = Vec::new();
    
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        results
*/
    }
    
    pub fn search_case_insensitive <'b>(query: &str, contents: &'b str) -> Vec<&'b str> {
        let query = query.to_lowercase();
        contents
            .lines()
            .filter(|line| {
                line.to_lowercase().contains(&query)})
            .collect()

/*   // старый код
        let mut results: Vec<&str> = Vec::new();
        
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
        results
*/
    }

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {   //  Vec<&str>
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        Config::search_case_insensitive(&config.query, &contents)
    } else {
        Config::search(&config.query, &contents)
    };
    
    for line in result {
        println!("{line}");  
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one_result () {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], Config::search(query, contents));
    }
    #[test]
    fn case_sensitive () {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], Config::search_case_insensitive(query, contents));
    }
}
