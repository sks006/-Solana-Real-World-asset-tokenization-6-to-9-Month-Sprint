
use std::env;

fn main() {
   //read the command line arguments
   let args: Vec<String> = env::args().collect();
   //print the command line arguments
   dbg!(&args);
   //save the argument to a variable
    let first_arg = &args[1];
    //print the first argument
    println!("First argument: {}", first_arg);
}

