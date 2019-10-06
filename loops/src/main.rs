fn main() {
    // infinite_loop();
    while_loop();
    while_array();
    for_loop_elements();
}

fn infinite_loop () {
    loop {
        println!("Hello, world!");
    }
}

fn while_loop () {
    let mut increment = 10;
    while increment >= 0 {
        println!("Hello: {}", increment);
        increment = increment - 1;
    }
}


fn while_array () {
    let arr = [1,2,3,4,5,6,7,8,9,10];
    let mut index = 0;

    while index < 5 {
        println!("Index ->: {}", arr[index]);
        index = index + 1;
    }
}

fn for_loop_elements () {
    let arr = [1,2,3,4,5,6,7,8,9,10];
    for element in arr.iter() {
        println!("For Index ->: {}", element);
    }
}

fn count_for () {
    for element in (1..10).rev() {
        println!("Index in for range: {}", element);
    }
}