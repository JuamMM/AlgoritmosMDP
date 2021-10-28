use std::env;
use std::process;

pub mod conjunto;

fn main() {
    if env::args().len() < 2 {
        println!("Este programa requiere de al menos 1 argumento");
        process::exit(0x0100);
    }

    let argumentos: Vec<String> = env::args().collect();

    let mut a = conjunto::Conjunto::new(&argumentos[1]);

    a.solucion_greedy();

}
