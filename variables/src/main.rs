fn main() {
    let mut x = 5;
    println!("The value of X is : {}", x);
    
    x = 6;
    println!("The value of X is : {}", x);

    let y = 5;
    let y = y + 5;
    let y = y * 2;
    println!("The value of Y is : {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Length of spaces : {}", spaces);

    let integer: i32 = 32;
    println!("The value of int Integer is : {}", integer);

    let float64 = 32.33;
    println!("The value of int Float 64b is : {}", float64);

    let float32: f32 = 32.33;
    println!("The value of int Float 32b is : {}", float32);

    // Operations
    let sum = 5 + 10;
    let subtraction = 10 - 7;
    let multiplication = 4 * 9;
    let division: f64 = 10.0 / 3.0;
    let remainder = 43 % 5;

    println!("Sum: {}, Subtraction: {}, Multiplication: {}, Division: {}, Remainde: {}", sum, subtraction, multiplication, division, remainder);

    // Booleans
    let t = true;
    let f: bool = false;
    println!("t: {}, f: {}", t, f);

    // Chars
    let c = 'z';
    let cat = '🐈';
    println!("Char: {}, Cat: {}", c, cat);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    let fivehundred = tup.0;

    println!("Tuple Y: {}", y);
    println!("Result: {}", fivehundred);

    // Arrays
    let arr = [1, 2, 3, 4, 5];
    let months = ["Enero", "Febrero", "Marzo", "Abril", "Mayo"];

    let first = arr[0];
    let first_month = months[0];
    println!("First Arr val: {}, First Month: {}", first, first_month);

    // arr err
    // let a = [1,2,3,4,5];
    // let index = 10;

    // let new_element = a[index];

    example_function();
}

fn example_function () {
    println!("Example function call!");
}
