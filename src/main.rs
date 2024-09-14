/*
El programa generará un número entero aleatorio entre 1 y 100.
El programa le pedirá al jugador que ingrese número en ese rango a manera
de adivinanza.
El programa indicará si la adivinanza es demasiado baja o demasiado alta.
Si la adivinanza es correcta, el juego imprimirá un mensaje de felicitación y
terminará su ejecución.
*/

use rand::Rng;
use text_io::read;

fn main() {    

    let random_num = rand::thread_rng().gen_range(0..100);

    loop{
        println!("Ingresa un entero de 1 a 99 (0 para terminar): ");
        let i: i32 = read!();

        if i == 0 {
            println!("Finalizó el programa.\nEl número a adivinar era {random_num}");
            break;
        } else if i > random_num {
            println!("{i} > número a adivinar");
        } else if i < random_num{
            println!("{i} < número a adivinar");
        } else {
            println!("{i} == número a adivinar\nEl programa ha finalizado");

            break;
        }
    }
}