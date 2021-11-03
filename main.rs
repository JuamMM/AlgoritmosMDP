use std::env;
use std::process;
use std::time::Instant;

mod conjunto;
mod algoritmos;

use algoritmos::solucion_greedy;

fn main() {
    if env::args().len() < 2 {
        println!("Este programa requiere de al menos 1 argumento");
        process::exit(0x0100);
    }

    let argumentos: Vec<String> = env::args().collect();

    let mut a = conjunto::Conjunto::new(&argumentos[1]);

    let now = Instant::now();

    let a = solucion_greedy(a);

    let elapsed_time = now.elapsed();

    println!("La solucion es: {:?}", a.get_solucion());
    println!("La diversidad es: {}", a.calcula_diversidad());
    println!("El tiempo ha sido {} segundos.", elapsed_time.as_secs());

}
