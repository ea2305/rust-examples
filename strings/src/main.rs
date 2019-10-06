fn main() {
    example_one();
    example_two();
}

fn example_one () {
    let s1 = String::from("Hola");
    let s2 = s1; // Borrow the reference

    println!("Borrow reference | {}", s2); // Valid
    // println!("> {}", s1); // invalid, we borrwo the reference to s2, s1 is not longer valid.
}

fn example_two () {
    let s1 = String::from("Hola");
    let s2 = s1.clone(); // Borrow the reference

    println!("Clone | {}", s2); // Valid
    println!("Org ref | {}", s1); // Valid
}
