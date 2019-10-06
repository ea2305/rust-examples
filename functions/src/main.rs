fn main() {
    example_function(2, 3);

    // Statement
    let y = 6; // not return value after assignment
    let x = {
        let x = 2;
        x * 2
    };

    println!("Statement: {}, Expression: {}", y, x);

    // Return values
    let value = get_value();
    println!("Value: {}", value);

    let addition = add_one(10);
    println!("Addition: {}", addition);
}

fn example_function (x: i32, y: i32) {
    println!("X: {}", x);
    println!("Y: {}", y);
}

fn get_value () -> i32 {
    5
}

fn add_one (x: i32) -> i32 {
    x + 1
}