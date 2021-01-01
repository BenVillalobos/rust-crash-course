use std::env;
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let name = "Ben";
    let command = args[1].clone();
    let status = "100%";

    println!("Args: {:?}", args);

    println!("Command: {}", command);

    if command == "Hello" {
        println!("Hello {}, how are you?", name);
    }
    else if command == status {
        println!("Status is {}", status);
    }
    else {
        println!("Please enter a valid command");
    }
}