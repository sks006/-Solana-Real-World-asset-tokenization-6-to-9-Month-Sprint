// fn borrow(a:String)->String{
//     println!("this is {}",a);
//     return a
// }
// fn main(){
//     let l=String::from("shihab");
//     let x =borrow(l);
//     println!("println! {}",x)
// }

//----------------------------------- 9 ---------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=23412;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("455667");
//         let x =borrow(l);
//     println!("print {}",x)
// }
//----------------------------------- 8 ---------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2342;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("345345");
//     let x =borrow(l);
//     println!("print {}",x)
// }
//----------------------------------- 7 ---------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3452;
//     let z=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("232222");
//         let x =borrow(l);
//     println!("print {}",x)
// }
//----------------------------------- 6 ---------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2222;
//     let z=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("3232222");
//             let x =borrow(l);
//     println!("print {}",x)
// }
//----------------------------------- 5 ---------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=20000;
//     let z=x-y;
//     return z;
// }
// fn main(){
//     let l=String::from("42343234");
//     let x=borrow(l);
//     println!("print {}",x);
// }
//----------------------------------- 4 ---------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("44440040");
//     let x=borrow(l);
//     println!("print {}",x)
// }
//----------------------------------- 3 ---------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=29999;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("40000044");
//     let x=borrow(l);
//     println!("print {}",x)
// }
//----------------------------------- 2 ---------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=20030;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("20000000");
//     let x=borrow(l);
//     println!("print {}",x)
// }
//----------------------------------- 1 ---------------------
fn borrow(a:&String){
    let x:u32=a.parse().expect("this is the end");
    let y:u32=20030;
    let z:u32=x-y;
}
fn main(){
    let l=String::from("20000ddd000");
    borrow(&l);
    println!("print {}",l)
}