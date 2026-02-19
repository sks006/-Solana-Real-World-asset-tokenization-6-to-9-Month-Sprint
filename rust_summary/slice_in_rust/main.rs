// fn main() {
// //slice of array of char
//       let arr:[char;5]=['a','c','s','d','f'];
//       let slice:&[char]=&arr[1..4];
//       println!("{:?}",slice);
// //slice of vec

// let vec:Vec<i32>=vec![10,20,30,40,50];
// let slice:&[i32]=&vec[3..4];
// println!("{:?}",slice);

// //slice for String
// let s:String=String::from("hello world");
// let hello:&str=&s[0..5];
// let world:&str=&s[6..11];

// println!("{:?}",hello);
// println!("{:?}",world);

// // short cut for initial index
// let s=String::from("shihabkabir");
// let slice=&s[0..3];
// println!("{}",slice)

// }


//----------------------------------------------------------


fn main() {
// get the first word of a string (no slices)
 let s=String::from("hello world");
 let word=first_word(&s);
 println!("The s is ={}",s);
 println!("The first word is = {}",word)

//string literals are slices
let s2:&str="second world";
let word2=first_word(&s2);
println!("The second word is = {}",word2);
    
}

// //get the first word of a string (with slices)
// fn first_word(s:&String)->usize{
//     let bytes=s.as_bytes();
//     for(i,&item) in bytes.iter().enumerate(){
//         if item==b' '{
//             return i;
//         }
//     }
//     //if no space is found return the length of the string
//     s.len()
// }


// //get the first word of a string (with slices)
// fn first_word(s:&String)->&str{
//     let bytes=s.as_bytes();
//     for(i,&item) in bytes.iter().enumerate(){
//         if item==b' '{// if the byte is a space
//             // return i;
//             return &s[0..i];
//         }
//     }
//     //if no space is found return the length of the string
//     // s.len()
//     &s[..]
// }


//string literals are slices
fn first_word(s:&str)->&str{
    let bytes=s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item==b' '{// if the byte is a space
            // return i;
            return &s[0..i];
        }
    }
    //if no space is found return the length of the string
    // s.len()
    &s[..]
}