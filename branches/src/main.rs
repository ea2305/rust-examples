fn main() {
    // IF Statement
    let number = 3;
    let boolean_value = true;
    
    if number < 5 {
        println!("True");
    } else {
        println!("False");
    }

    if boolean_value {
        println!("Content of Boolean");
    }

    let value = 10;

    if value > 11 {
        println!("value > 11");
    } else if value < 100 {
        println!("value < 100");
    }

    // If condition is an expression
    let validation = true;
    let result_if = if validation {
        3 // Should have the same type
    } else {
        5
    };

    println!("The value is: {}", result_if);
}