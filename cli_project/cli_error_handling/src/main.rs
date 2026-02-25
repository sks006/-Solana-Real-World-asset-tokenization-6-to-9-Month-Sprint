use std::env;
use std::process;
use cli_error_handling::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // //parse because we want to handle the error if there are not enough arguments
    // let config = Config::build(&args).expect("could not parse config");
    



    println!("searching for  ⭕ {}", config.query);
    println!("in file 🔰 {}", config.file_path);

    if let Err(e)=cli_error_handling::run(config){
        println!("application error: {}", e);
        process::exit(1);
    }
  
}

