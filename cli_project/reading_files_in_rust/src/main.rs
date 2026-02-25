use std::fs::File;
use std::io::{self, Read};

//version = "0.1.0"
// fn main() -> io::Result<()> {
//     // open the file 
//     let mut file= File::open("string.txt")?;
//     // create a string to hold the contents of the file
//     let mut contents = String::new();
//     // read the contents of the file into the string
//     file.read_to_string(&mut contents)?;
//     // print the contents of the file
//     println!("{}", contents);
//     Ok(())
// }
//----------------------------------------------------
// version="0.2.0"

// fn main(){
//     match read_file_to_string("string.txt") {
//         Ok(contents) => println!("{}", contents),
//         Err(e) => eprintln!("Error reading file: {}", e),
//     }
// }

// fn read_file_to_string(file_name: &str) -> Result<String, io::Error> {
//     let mut file = File::open(file_name)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

//----------------------------------------------------
// version="0.3.0" reading the file line by line

use std::io::{ BufRead, BufReader};
fn main()->io::Result<()>{
    let file = File::open("string.txt")?;
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| {
        match line {
            Ok(line) => println!("{} ", line),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    });  
    Ok(())
}