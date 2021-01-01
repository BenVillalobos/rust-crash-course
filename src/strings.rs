// Primitive str = Immutable, fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure. Use when you need to modify or own.

pub fn run() {
    // Immutable, fixed-length
    let immutable_string = "Hello";

    let mut heap_string = String::from("Hello ");

    println!("Immutable: {}\nHeap-Allocated: {}", immutable_string, heap_string);

    // Get length
    println!("Heap-Allocated Length: {}", heap_string.len());

    // push a char
    heap_string.push('W');
    // push_str a string
    heap_string.push_str("orld!");
    println!("New String: {}", heap_string);

    // Capacity in bytes
    println!("Capacity: {}", heap_string.capacity());

    // Check if empty
    println!("Is Empty: {}", heap_string.is_empty());

    // Contains
    println!("Contains 'World': {}", heap_string.contains("World"));
    
    // Replace
    println!("Replace: {}", heap_string.replace("World", "There"));

    // Loop through string by whitespace
    for word in heap_string.split_whitespace() {
        println!("Split: {}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}