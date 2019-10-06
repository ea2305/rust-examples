// crate
extern crate rand;

// libs
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el número!");
   
    // Regresa u32 (Entero sin signo de 32 bits)
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("[hide] Número secreto es: {}", secret_number);
    
    loop {
        println!("Por favor, ingresa un número mayor a 0 y menor igual a 100: \n");

        // Declaración de nueva referencia de variable
        let mut guess = String::new();

        // Lectura
        io::stdin().read_line(&mut guess)
            .expect("Error al leer.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("==> Muy pequeño"),
            Ordering::Greater => println!("==> Muy Grande"),
            Ordering::Equal => {
                println!("==> Es Correcto!");
                break;
            }
        }
    };

}
