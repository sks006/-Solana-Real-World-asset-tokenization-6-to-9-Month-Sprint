// struct Loan{
//     acc:u64
// }
// impl Loan{
//     fn new(a:u64)-> Self{
//         Self{
//             acc:a,
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(783);
//     println!("acc {}",l.acc)
// }

//-------------------------- 2 ----------------------------------

// struct Loan{
//     acc:u64,
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a,
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(1200);
//     println!("acc : {}",l.acc)
// }
//-------------------------- 3 ----------------------------------

// enum Status{
//     pass,
//     reje
// }

// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(234,Status::pass);
//     match l.status{
//         Status::pass=>println!("pass {}",l.acc),
//         Status::reje=>println!("reje"),
//     }
// }

//-------------------------- 4 ----------------------------------

// struct Loan{
//     acc:u64
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(323);
//     println!("acc {}",l.acc)
// }

//-------------------------- 5 ----------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }

// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(324,Status::pass);
//     match l.status{
//         Status::pass=>println!("pass {}",l.acc),
//         Status::reje=>println!("reje {}",l.acc)
//     }
// }

//-------------------------- 6 ----------------------------------

// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(798,Status::Pass);
//     match l.status{
//         Status::Pass=>println!("pass {}",l.acc),
//         Status::Reje=>println!("reje {}",l.acc)  
//     }
// }

//-------------------------- 7 ----------------------------------

// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(2343,Status::Pass);
//     match l.status{
//         Status::Pass=>println!("pass {}", l.acc),
//          Status::Reje=>println!("reje {}", l.acc),
//     }
// }

//-------------------------- 8 ----------------------------------
// enum Status{
//     Pass,
//     Reje
// }

// struct Loan{
//     acc:u64,
//     status:Status,
// }

// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(324,Status::Pass);
//     match l.status{
//         Status::Pass=>println!("pass {}", l.acc),
//          Status::Reje=>println!("reje {}", l.acc),
//     }
// }
//-------------------------- 9 ----------------------------------

// struct Loan{
//     acc:u64,
    
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(32453);
//     println!("acc {}",l.acc)
// }
//-------------------------- 10 ----------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
// fn new(a:u64,s:Status)->Self{
//     Self{
//         acc:a,
//         status:s
//     }
// }
// }
// fn main(){
//     let l=Loan::new(234,Status::pass);
//     match l.status{
//              Status::pass=>println!("pass {}", l.acc),
//          Status::reje=>println!("reje {}", l.acc),   
//     }
// }

//-------------------------- 11 ----------------------------------
// struct Loan{
//     acc:u64,
    
// }
// impl Loan{
//     fn new(a:u64)->Self{
//         Self{
//             acc:a
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(234);
//     println!("acc {}",l.acc)
// }

//-------------------------- 12 ----------------------------------
#[derive(Debug)]  // Added for debugging
enum Status {
    Pass,
    Reje,
}

struct Loan {
    acc: u64,
    status: Status,
}

impl Loan {
    fn new(a: u64, s: Status) -> Self {
        Self {
            acc: a,
            status: s,  // Fixed: comma added
        }
    }
}

fn main() {
    let l = Loan::new(234, Status::Pass);
    println!("Account: {}", l.acc);
    
    match l.status {
        Status::Pass => println!("Loan Status: Approved"),
        Status::Reje => println!("Loan Status: Rejected"),
    }
}