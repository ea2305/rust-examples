fn main() {
    let s = String::from("Hello");
    takes_ownership(s);

    // println!("{}", s)// if we use S again we will get an error

    let x = 5;
    makes_copy(x);

    // Return references
    let s1 = gives_ownership();
    let s2 = String::from("Hello2");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {}, s2: [s3], s3: {}", s1, s3);

    // Performing operations
    let s_1 = String::from("Hola!");
    let (s_2, len) = calculate_length(s_1); // pass the ownership

    println!("The length of {}, is: {}", s_2, len)
}

fn takes_ownership (some_string: String) {
    println!("{}", some_string);
}

fn makes_copy (some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership () -> String {
    let some_string = String::from("Hello");
    some_string
}
fn takes_and_gives_back (some_string: String) -> String {
    some_string
}

fn calculate_length (some_string: String) -> (String, usize) {
    let len = some_string.len();
    (some_string, len)
}