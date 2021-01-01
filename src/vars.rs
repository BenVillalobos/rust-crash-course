// Variables hold primitive date or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Ben";
    let mut age = 27;

    // Rust will warn at compile time that the when age
    // had the value of 27, it was never used
    println!("My name is {} and I am {}", name, age);
    age = 28;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    // Constants require a type
    // Standard practice is const vars are all upper case.
    const ID: i32 = 001;
    println!("ID: {},", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Ben", 37);
    println!("{} is {}", my_name, my_age);
}