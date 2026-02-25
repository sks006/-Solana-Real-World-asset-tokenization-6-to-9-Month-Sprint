use std::fs;
use std::process;
 

pub fn run(config:Config)->Result<(), Box<dyn std::error::Error>>{
      let content=fs::read_to_string(config.file_path).unwrap_or_else(|err|{
        println!("could not read file: {}", err);
        process::exit(1);
    });
    println!("with content:\n{}", content);
    Ok(())
}

// fn parse_config(args: &[String])->(&str, &str){
//     let query=&args[1];
//     let file_path=&args[2];
//tuple as return type   
//  (query, file_path)    
// }

/*
This is the improved version that includes structs
*/
pub struct Config{
    pub  query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config{
    pub fn build(mut args:impl Iterator<Item=String>)-> Result<Config, &'static str>{
        

        args.next();
        let query=match args.next(){
            Some(arg)=>arg,
            None=>return Err("did not get a query string"),    
        }; 
        let file_path=match args.next(){
            Some(arg)=>arg,
            None=>return Err("did not get a file path"),    
        };
        let ignore_case=std::env::var("IGNORE_CASE").is_ok();
        
      
        
        Ok(Config{query, file_path, ignore_case})
    }
}

// fn parse_config(args:&[String])->Config{
//     let query=args[1].clone();
//     let file_path=args[2].clone();
//     Config{query, file_path}
// }

fn search<'a>(query: &str, content: &'a str)-> Vec<&'a str>{
    // let mut results=Vec::new();
    // for line in content.lines(){
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }
    // results
    content.lines().filter(|line| line.contains(query)).collect()


}

#[cfg(test)]
mod tests {
    use super::*;   
    #[test]
    fn one_result(){
        let query="duct";
        let content="/
Rust:
safe, fast, productive.
Pick three. 
Trust me"
        ;
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}