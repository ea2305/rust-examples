fn main() {
    let mut word = String::from("hola mundo");
    let index_word = first_word(&word);

    println!("index: {}", index_word);

    word.clear(); // delete word and turn the index_word variable useless

    // Slices
    test_slices();
    test_limits_slices();

    // first word with slices
    let mut new_word = String::from("hello world");
    let slice_word = slice_first_word(&new_word);
    println!("slice_word of index: {}", slice_word);
    new_word.clear();

    // Slices references with literal strings
    let mut ref_word = String::from("hello world");
    let ref_slice_word = advance_first_word(&ref_word[..]); // slice of String

    let literal_string = "hello world";
    let red_literal_word = advance_first_word(&literal_string[..]);

    let red_literal_word_wo_slices = advance_first_word(&literal_string);
    // literal string are slices by nature

    println!("ref_slice_word of index: {}", ref_slice_word);
    println!("red_literal_word of index: {}", red_literal_word);
    println!("red_literal_word_wo_slices of index: {}", red_literal_word_wo_slices);
    ref_word.clear();
}

fn first_word (word: &String) -> usize {
    // Get bytes
    let bytes = word.as_bytes();

    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return i;
        }
    }
    word.len()
}

fn test_slices () {
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("w1: {}, w2: {}", hello, world);
}

fn test_limits_slices () {
    let s = String::from("Hello World");

    let index_hello = &s[..5];
    let hello = &s[..5]; // we can ignore the zero value using dots
    let index_world = &s[6..s.len()]; //
    let world = &s[6..]; //we can ignore the zero value using dots too
    let full = &s[..];

    println!("h1: {}, h2: {}, w1: {}, w2: {}", hello, index_hello, world, index_world);
    println!("full: {}", full);
}

fn slice_first_word (s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn advance_first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}