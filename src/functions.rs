pub fn run() {
    greeting("Hello", "Joe");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("5 + 5 = {}", get_sum);

    // Closure
    let n3: i32 = 10;
    // Can use outside vars inside closures.
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // To return, don't use a semicolon
    n1 + n2
}