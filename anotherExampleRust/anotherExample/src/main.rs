use std::time::{Instant};
fn sum_natural_numbers(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    n + sum_natural_numbers(n - 1)
}

fn main() {
    let n = 10000;
    let result = sum_natural_numbers(n);
    // Obtén el tiempo de inicio

    let start_time = Instant::now();

    // Coloca aquí el código que deseas medir
    
    println!("La suma de los {} primeros números naturales es: {}", n, result);
    // Obtén el tiempo de finalización
    println!("Tiempo de ejecución: {:.2?}", start_time.elapsed());
}