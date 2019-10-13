fn main() {
    example_one();
    example_two();
    example_three();
    example_four();
}

/**
 - Mutable references -
 */

fn example_one () {
    let s1 = String::from("Hello");
    let len = calculate_len_withref(&s1);
    println!("Str: {}, length: {}", s1, len);
}

fn calculate_len_withref (s: &String) -> usize {
    s.len()
}

/**
 - Mutable references -
 */
fn example_two () {
    // let s1 = String::from("Hello"); // is not mutable
    let mut s1 = String::from("Hello"); // It works
    change_string(&mut s1);
    println!("Str: {}", s1);
}

// fn change_string (s: &String) {
fn change_string (s: &mut String) {
    s.push_str(", world");
}

/**
 - Doble reference -
 */

fn example_three () {
    let mut s = String::from("Hello");

    
    // let r1 = &mut s;
    // let r2 = &mut s; // Double reference not works

    {
        let _r1 = &mut s;
    }
    let r2 = &mut s;

    println!("{}", r2);
}

/**
 - Doble reference with no mutables and mutables-
 */

fn example_four () {
    let mut s = String::from("Hello");

    
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println!("{}, {} and {}", r1, r2, r3);
}