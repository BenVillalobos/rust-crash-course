// Enums - Types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // Perform action depending on movement
    // match = switch
    match m {
        Movement::Up => println!("Move Up"),
        Movement::Down => println!("Move Down"),
        Movement::Left => println!("Move Left"),
        Movement::Right => println!("Move Right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}