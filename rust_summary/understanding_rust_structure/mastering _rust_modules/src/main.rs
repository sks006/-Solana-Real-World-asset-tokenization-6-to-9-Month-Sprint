use crate::gender::vegetables::Vegetable;
pub mod gender;

fn main() {
    let carrot = Vegetable {
        name: String::from("Carrot"),
        color: String::from("Orange"),
    };

    println!("Vegetable: {:?}", carrot);
}
