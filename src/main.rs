/*
El programa generará un número entero aleatorio entre 1 y 100.
El programa le pedirá al jugador que ingrese número en ese rango a manera
de adivinanza.
El programa indicará si la adivinanza es demasiado baja o demasiado alta.
Si la adivinanza es correcta, el juego imprimirá un mensaje de felicitación y
terminará su ejecución.
*/

use rand::Rng;
use std::io;
use std::str::FromStr;

fn main() {    

    println!("Adivina el número!");
    
    // Generar un número entre 1 y 100
    let secret_num: i32 = rand::thread_rng().gen_range(0..100);

    loop {
        println!("Por favor, ingresa un número: ");

        // Capturar la entrada del usuario como una cadena
        let mut num: String = String::new();
        io::stdin().read_line(&mut num).expect("Error al leer la línea");

        // Intentar convertir la cadena a un número
        let num: i32 = match i32::from_str(num.trim()) {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Valor ingresado no es un número válido.");
                continue; // Vuelve al inicio del bucle para pedir un nuevo valor
            }
        };

        if num == 0 {
            println!("El número secreto era {secret_num}");
            break;
        } 
        if num > secret_num {
            println!("{num} > número secreto");
        } else if num < secret_num {
            println!("{num} < número secreto");
        } else {
            println!("{num} == número secreto");
            break;
        }
    }

    println!("El programa ha finalizado!");
}
