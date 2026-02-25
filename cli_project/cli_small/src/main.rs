// simple cli project 
// get user input and return back reversed string
//use env;
use std::env;



fn main() {
//collect the arguments form the vector and store in a variable
   let args:Vec<String>=env::args().collect();
    //check if the user has provided an argument or string to reverse
    if args.len() < 2 {
        println!("Please provide a string to reverse.");
        return;
    }

    //store the string to a variable
    let input=args[1].clone();

    //get the string to reverse from the arguments
    let reverse=input.chars().rev().collect::<String>();
    println!("Reversed string: {}", reverse);
}