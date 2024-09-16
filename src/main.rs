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
use std::cmp::*; // Importa funciones de comparación
use colored::*; // Importa las funciones necesarias para aplicar colores a cadenas

fn main() {    

    println!("Adivina el número entre 1 y 100");
    
    // Generar un número entre 1 y 100 (incluyendo ambos límites)
    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Por favor, ingresa un número o pulsa 0 para terminar: ");

        // Capturar la entrada del usuario en una variable mutable de tipo string
        let mut my_num = String::new();
        io::stdin().read_line(&mut my_num).expect("Error al leer la línea");

        // Intentar convertir la cadena a un número
        let my_num: u32 = match my_num.trim().parse(){
            Ok(num) => num, // Si el número es válido, asignarlo a my_num
            Err(_) => { // Si no es válido, mostrar mensaje y continuar
                println! ("Tipo no válido"); 
                continue;
            }
        };

        // Si ingresamos 0, el programa finaliza.
        if my_num == 0 {
            println!("El número secreto era {secret_num}");
            break;
        } 

        // Comparamos el número introducido con el generado por el ordenador
        match my_num.cmp(&secret_num){
            Ordering::Less => println!("{}","Tú número es inferior al número secreto".red()),
            Ordering::Greater => println!("{}","Tú número es superior al número secreto".red()),
            Ordering::Equal => {
                println!("{} {}", "¡Adivinaste!, el número secreto es".green(), secret_num.to_string().green());
                break;
            },
        }
    }

    println!("El programa ha finalizado!");
}


/* 
// Versión simplificada, sin control de errores

use rand::Rng;
use text_io::read;

fn main() {    

    let secret_num: i32 = rand::thread_rng().gen_range(0..100);

    loop{
        println!("Ingresa un número: ");
        let i: i32 = read!()

        if i == 0 {
            println!("El programa ha finalizado!\nEl número secreto era {secret_num}");
            break;
        } 
        if i > secret_num {
            println!("{i} > número secreto");
        } else if i < secret_num{
            println!("{i} < número secreto");
        } else {
            println!("{i} == número secreto\nEl programa ha finalizado!");
            break;
        }
    }
}
*/