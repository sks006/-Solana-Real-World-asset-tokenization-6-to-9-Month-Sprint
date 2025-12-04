

// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let result=divide(21.0,3.0);
//     match result{
//         Ok(value)=>println!("value {}",value),
//         Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 2 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let s = divide(22, 4);
//     match s {
//         Ok(value)=>println!("value {}",value),
//         Err(error)=>println!("erroe {}",error)
//     }
// }
//----------------------- 3 ----------------------------------
// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is Error".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let s = divide(33.3, 4.2);
//     match s {
//         Ok(value)=>println!("Value {}",value),
//         Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 4 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let x=divide(33, 4);
//     match x {
//          Ok(value)=>println!("Value {}",value),
//          Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 5 ----------------------------------

// use core::error;

// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is end".to_string())
//     }else{
//         Ok(a/b)
//     }
// }

// use core::error;
// use std::{collections::btree_map::Values, result};

// fn main(){
//     let l=divide(33,0);
//     match l {
//      Ok(value)=>println!("value : {}",value),
//      Err(error)=>println!("error : {}", error)   
//     }
// }
//----------------------- 6 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let y=divide(32, 4);
//     match y {
//         Ok(value)=>println!("value {}",value),
//         Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 7 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is  the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let l=divide(23, 23);
//      match l {
//          Ok(value)=>println!("value {}",value),
//          Err(error)=>println!("error {}",error)
//      }
// }
//----------------------- 8 ----------------------------------
// fn divide(s:f32,d:f32)->Result<f64,String>{
//     if d==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok((s/d).into())
//     }
// }
// fn main(){
//     let l=divide(34.4, 2.0);
//     match l {
//          Ok(value)=>println!("value {}",value),
//          Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 9 ----------------------------------
// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
// let result=divide(333.3, 4.3);
// match result {
//      Ok(value)=>println!("value {}",value),
//      Err(error)=>println!("error {}",error)
// }
// }

//----------------------- 10 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
// if b==0{
//     Err("this is the end".to_string())
// }else {
//     Ok(a/b)
// }
// }

// fn main(){
//     let result=divide(30, 41);
//     match result {
//          Ok(value)=>println!("value {}",value),
//          Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 11 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("error fuck up".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
// let result= divide(32, 2);
// match result {
//     Ok(value)=>println!("value {}",value),
//     Err(error)=>println!("error {}",error)
// }
// }
//----------------------- 12 ----------------------------------
// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let k=divide(23.0,3.0);
//     match k {
//         Ok(value)=>println!("value {}",value),
//         Err(error)=>println!("error {}",error)
//     }
// }

//----------------------- 13 ----------------------------------

// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let result=divide(3.0, 4.3);
//     match result {
//         Ok(values)=>println!("values {}",values),
//         Err(error)=>println!("error {}",error)
//     }
// }
//----------------------- 14 ----------------------------------
// fn divide(a:u32,b:u32)->Result<u32,String>{
//     if b==0{
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }

// fn main(){
//     let result=divide(34, 4);
//     match result {
//         Ok(values)=>println!("values {}",values),
//         Err(error)=>println!("error {}",error) 
//     }
// }
//----------------------- 15 ----------------------------------


// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0 {
//         Err("this is the end".to_string())
//     }else {
//         Ok(a/b)
//     }
// }
// fn main(){
//     let x=divide(34.4, 2.3);
//     match x {
//              Ok(values)=>println!("values {}",values),
//         Err(error)=>println!("error {}",error)   
//     }
// }

//---------------------- Options -----------------------------------------


// The core data structure
// #[derive(Debug, Clone)]
// struct PerpPosition {
//     pub size: u64, // Position quantity
//      // Price scaled (e.g., $30,000 * 100)
// }

// fn main() {
//     // 1. Position with value (Some)
//     let position_a: Option<PerpPosition> = Some(PerpPosition {
//         size: 2_500,
        
//     });

//     // 2. Position without value (None)
//     let position_b: Option<PerpPosition> = None;

//     println!("--- 1. Rule: Transformation (map) ---");
//     // Practice Point #6: map()
//     // Rule: Apply a function to the inner value IF Some, otherwise remain None.
    
//     // Convert Option<PerpPosition> into Option<u64> containing only the size.
//     let size_a: Option<u64> = position_a.map(|pos| {
//         println!("  Mapping: Extracting size from Some(T)...");
//         pos.size // The function returns the size
//     });
    
//     let size_b: Option<u64> = position_b.map(|pos| pos.size); // The function is NOT run here

//     println!("Size A result (Some): {:?}", size_a); // Output: Some(2500)
//     println!("Size B result (None): {:?}\n", size_b); // Output: None

// }

//---------------------- 2 ----------------------------------

// #[derive(Debug,Clone)]

// struct PerpPosition{
//      pub size: u64, 
// }
// fn main(){
//     let position_a:Option<PerpPosition>=Some(
//         PerpPosition { size: 2_400 }
//     );
//     let position_b:Option<PerpPosition>=None;
//     println!("--- 1. Rule: Transformation (map) ---");
//     let size_a: Option<u64>= position_a.map(|pos|{
//         println!("  Mapping: Extracting size from Some(T)...");
//         pos.size
//     });
//     let size_b: Option<u64> = position_b.map(|pos| pos.size);
//     println!("Size A {:?}",size_a);
//     println!("Size B {:?}",size_b)
// }

//---------------------- 3 ----------------------------------


// // The data structure (e.g., a margin/collateral record)
// #[derive(Debug, Clone)]
// struct AccountCollateral {
//      pub usd_value: u64, // Value in $1 units
// }

// fn main() {
//     // SCENARIO A: User HAS collateral (Some(T))
//     let collateral_record_a: Option<AccountCollateral> = Some(
//         AccountCollateral { usd_value: 1_000_000 } // $1 million value
//     );
    
//     // SCENARIO B: User HAS NO collateral (None)
//     let collateral_record_b: Option<AccountCollateral> = None;

//     println!("--- Rule: Transformation with map() ---");
//     // map() Rule: Safely applies a function to the value *IF* it exists.
//     // Purpose: Extracting a simpler piece of data (the value) from the complex Option struct.
    
//     // 1. Process SCENARIO A (Some)
//     // The inner function runs because the value is present.
//     let value_a: Option<u64> = collateral_record_a.map(|record| {
//         println!("  Map Rule A: Function RUNS, extracting value...");
//         record.usd_value // Function returns the u64 value
//     });

//     // 2. Process SCENARIO B (None)
//     // The inner function does NOT run because the value is absent.
//     let value_b: Option<u64> = collateral_record_b.map(|record| {
//         // This println! will not execute.
//         println!("  Map Rule B: Function SKIPPED. Input was None."); 
//         record.usd_value
//     });

//     println!("\nResult A (User has collateral): {:?}", value_a); 
//     // Output: Some(1000000). The function succeeded and returned the value.
    
//     println!("Result B (User has NO collateral): {:?}", value_b);
//     // Output: None. The function was skipped, and the result remains None.
// }

//---------------------- 4 ----------------------------------

// #[derive(Debug, Clone)]
// struct AccountCollateral {
//     pub usd_value: u64,
// }

// fn main() {
//     let collateral_record_a: Option<AccountCollateral> = Some(
//         AccountCollateral {
//             usd_value: 1_000_000,
//         }
//     );
    
//     let collateral_record_b: Option<AccountCollateral> = None;
    
//     println!("____ Rule of Transformation with map() ____");
    
//     let value_a: Option<u64> = collateral_record_a.map(|record| {
//         println!("  Map Rule A: Function RUNS, extracting value...");
//         record.usd_value
//     });
    
//     let value_b: Option<u64> = collateral_record_b.map(|record| {
//         println!("  Map Rule B: Function SKIPPED. Input was None.");
//         record.usd_value
//     });
    
//     println!("\nResult A (User has collateral): {:?}", value_a);
//     println!("Result B (User has NO collateral): {:?}", value_b);
// }

//----------------------------- 5 ----------------------------------


// #[derive(Debug,Clone)]

// struct AccountCollateral{
//     pub usd_value:u64,
// }

// fn main(){
//     let collateral_record_a:Option <AccountCollateral> = Some(
//         AccountCollateral{
//             usd_value:1_000_000,
//         }
//     );
//     let collateral_record_b:Option <AccountCollateral> =None;



//     let value_a:Option<u64>=collateral_record_a.map(|record|{
//         println!("map A");
//         record.usd_value
//     });
//     let value_b:Option<u64>=collateral_record_b.map(|record|{
//         println!("map B");
//         record.usd_value
//     });


//     println!("A result {:?}",value_a);
//     println!("B result {:?}",value_b);
// }

//----------------------------- 6 ----------------------------------

// #[derive(Debug,Clone)]

// struct AccountCollateral{
//    pub usd_value:u64,
// }

// fn main(){
//     let collateral_record_a:Option <AccountCollateral> =Some(
//         AccountCollateral{
//             usd_value:1_000_000,
//         }
//     );
//     let collateral_record_b:Option <AccountCollateral> = None;

//     let value_a:Option<u64>=collateral_record_a.map(|record|{
      
//         record.usd_value
//     });
//     let value_b:Option<u64>=collateral_record_b.map(|record|{
      
//         record.usd_value
//     });
//     println!("A result : {:?}",value_a);
//     println!("B result : {:?}",value_b );

// }


//----------------------------- 7 ----------------------------------

// #[derive(Debug,Clone)]
// struct AccountCollateral{
//     pub usd_value:u64,
// }

// fn main(){
//     let collateral_record_a:Option <AccountCollateral> = Some(
//         AccountCollateral{
//             usd_value:1_000_000,
//         }
//     );
//     let collateral_record_b:Option <AccountCollateral> =None;

//     let value_a:Option<u64> = collateral_record_a.map(|record|{
//         record.usd_value
//     });
//     let value_b:Option<u64> = collateral_record_b.map(|record|{
//         record.usd_value
//     });
//     println!("A result : {:?}",value_a);
//     println!("B result : {:?}",value_b);
// }
//----------------------------- 8 ----------------------------------

// #[derive(Clone,Debug)]

// struct AccountCollateral{
//     pub usd_value:u64,
// }

// fn main(){
//     let collateral_record_a:Option<AccountCollateral>=Some(
//         AccountCollateral{
//             usd_value:1_000_000,
//         }
//     );
//     let collateral_record_b:Option<AccountCollateral>= None;

//     let value_a:Option<u64>=collateral_record_a.map(|record|{
//         record.usd_value
//     });
//     let value_b:Option<u64>=collateral_record_b.map(|record|{
//         record.usd_value
//     });
//     println!("A result {:?}",value_a);
//     println!("B result {:?}",value_b);
// }

//----------------------------- 9 ----------------------------------

// #[derive(Debug,Clone)]
// struct AccountCollateral{
//     pub usd_value:u64,
// }
// fn main(){
//     let collateral_record_a:Option<AccountCollateral>=Some(
//         AccountCollateral{
//             usd_value:1000008888
//         }
//     );
//     let collateral_record_b:Option<AccountCollateral>=None;

//     let value_a:Option<u64>=collateral_record_a.map(|record|{
//         record.usd_value
//     });
//     let value_b:Option<u64>=collateral_record_b.map(|record|{
//         record.usd_value
//     });
//     println!("A result {:?}",value_a);
//     println!("B result {:?}",value_b);

// }

//----------------------------- 10 ----------------------------------

// #[derive(Clone,Debug)]
// struct AccountCollateral{
//     pub usd_value:u64,
// }

// fn main(){
//     let collateral_record_a:Option<AccountCollateral>=Some(
//         AccountCollateral{
//             usd_value:1000000003123
//         }
//     );
//     let collateral_record_b:Option<AccountCollateral>=None;
//     let value_a:Option <u64> = collateral_record_a.map(|record|{
//         record.usd_value
//     });
//     let value_b:Option <u64> =collateral_record_b.map(|record|{
//         record.usd_value
//     });
//     println!("A result {:?}",value_a);
//     println!("B result {:?}",value_b)
// }

//----------------------------- 11 ----------------------------------

// #[derive(Debug,Clone)]
// struct AccountCollateral{
//     pub usd_value:u64,
// }
// fn main(){
//     let collateral_record_a:Option<AccountCollateral>=Some(
//         AccountCollateral{
//             usd_value:3482983438
//         }
//     );
//     let collateral_record_b:Option<AccountCollateral>=None;
//     let value_a:Option<u64>=collateral_record_a.map(|record|{
//         record.usd_value
//     });
//     let value_b:Option<u64>= collateral_record_b.map(|record|{
//         record.usd_value
//     });
//     println!("A result {:?}",value_a);
//     println!("B result {:?}",value_b);
// }
//----------------------------- 12 ----------------------------------

// #[derive(Debug,Clone)]

// struct AccountCollateral{
//     pub usd_value:u64,
// }
// fn main(){
//     let collateral_record_a:Option<AccountCollateral>=Some(
//         AccountCollateral{
//             usd_value:400000000
//         }
//     );
//     let collateral_record_b:Option<AccountCollateral> =None;

//     let value_a:Option<u64>=collateral_record_a.map(|record|{
//         record.usd_value
//     });
//       let value_b:Option<u64>=collateral_record_b.map(|record|{
//         record.usd_value
//     });
//         println!("A result {:?}",value_a);
//     println!("B result {:?}",value_b);
// }
//----------------------------- 13 ----------------------------------
//  #[derive(Debug,Clone)]
//  struct AccountCollateral{
//     pub usd_value:u64,
//  }

//  fn main(){
//     let collateral_record_a:Option<AccountCollateral>=Some(
//         AccountCollateral{
//             usd_value:400003030
//         }
//     );
//     let collateral_record_b:Option<AccountCollateral>=None;
//     let value_a:Option<u64>=collateral_record_a.map(|record|{
//         record.usd_value
//     });
//      let value_b:Option<u64>=collateral_record_b.map(|record|{
//         record.usd_value
//     });
//       println!("A result {:?}",value_a);
//       println!("B result {:?}",value_b);
//  }
//----------------------------- 14 ----------------------------------

// #[derive(Debug,Clone)]

// struct AccountCollateral{
//     usd_value:u64,
// }

// fn main(){
//     let collateral_record_a:Option<AccountCollateral>=Some(
//         AccountCollateral{
//             usd_value:56789000
//         }
//     );
//     let collateral_record_b:Option<AccountCollateral>=None;
//     let value_a:Option<u64>=collateral_record_a.map(|record|{
//         record.usd_value
//     });
//      let value_b:Option<u64>=collateral_record_b.map(|record|{
//         record.usd_value
//     });
//       println!("A result {:?}",value_a);
//       println!("B result {:?}",value_b);
// }

//----------------------------- 15 ----------------------------------

#[derive(Debug,Clone)]

struct AccountCollateral{
    usd_value:String,
}
fn main(){
    let collateral_record_a:Option<AccountCollateral>=Some(
        AccountCollateral{
            usd_value:"56789000".to_string()
        }
    );
    let collateral_record_b:Option<AccountCollateral>=None;
    let value_a:Option<String>=collateral_record_a.map(|record|{
        record.usd_value
    });
     let value_b:Option<String>=collateral_record_b.map(|record|{
        record.usd_value
    });
      println!("A result {:?}",value_a);
      println!("B result {:?}",value_b);
}