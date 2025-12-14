// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("40000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 14 ---------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("50000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 13 ---------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=6000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("70000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 12 ---------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("100000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 11 ---------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=4000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("60000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 10 ---------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=8000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("70000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 9 -----------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2000;
//     let z:u32=x*y;
//     return z
// }
// fn main(){
//     let l=String::from("100000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 8 -----------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1500;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("400000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 7 -----------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2000;
//     let z:u32=x*y;
//     return z
// }
// fn main(){
//     let l=String::from("5000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 6 -----------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=7000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("30000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 5 -----------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("40000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 4 -----------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=600;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("60000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 3 -----------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x*y;
//     return z
// }
// fn main(){
//     let l=String::from("300000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 2 -----------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=7000;
//     let z:u32=x/y;
//     return z
// }
// fn main(){
//     let l=String::from("1000000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 1 -----------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=500;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("100000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//----------------------------- 15 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x-y;
//     println!("value {}",z)
    
// }
// fn main(){
//     let l=String::from("100000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 14 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z:u32=x+y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("2000000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 13 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=4000;
//     let z:u32=x+y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("500000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 12 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=450;
//     let z:u32=x-y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("700000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 11 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=500;
//     let z:u32=x+y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("800000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 10 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=600;
//     let z:u32=x*y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("800000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 9 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=700;
//     let z:u32=x+y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("90000000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 8 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=505;
//     let z:u32=x+y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("69999999");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 7 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=4344;
//     let z:u32=x-y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("69999999");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 6 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z:u32=x/y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("80000000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 5 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2000;
//     let z:u32=x-y;
//     println!("value :{}",z)
// }
// fn main(){
//     let l=String::from("4000000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 4 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2000;
//     let z:u32=x+y;
//     println!("value :{}",z)
// }
// fn main(){
//     let l=String::from("4000000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 3 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z:u32=x*y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("3000000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 2 -----------------------------
// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=400;
//     let z:u32=x+y;
//     println!("value {}",z)
// }
// fn main(){
//     let l=String::from("3000000");
//     borrow(&l);
//     println!("value {}",l)
// }
//----------------------------- 1 -----------------------------
fn borrow(a:&String){
    let x:u32=a.parse().expect("this is the end");
    let y:u32=590;
    let z:u32=x+y;
    println!("value {}",z)
}
fn main(){
    let l=String::from("4000000");
    borrow(&l);
    println!("value {}",l)
}