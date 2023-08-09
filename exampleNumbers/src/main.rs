use std::time::{Instant};
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..((num as f64).sqrt() as u64 + 1) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // Obtén el tiempo de inicio
    let start_time = Instant::now();
    // Algo que queremos medir
    let limit = 100000; // Número límite hasta el cual se generarán los primos
    let mut primes = Vec::new();
    
    for num in 2..limit {
        if is_prime(num) {
            primes.push(num);
        }
    }
    println!("Tiempo de ejecución: {:.2?}", start_time.elapsed());
    println!("Primos encontrados: {:?}", primes);
   /*  for _ in 0..1000000 {
    }
    // Obtén el tiempo de finalización
    let end_time = Instant::now();

    // Calcula la duración transcurrida
    let elapsed_time = end_time.duration_since(start_time);

    // Convierte la duración a microsegundos
    let microseconds = elapsed_time.as_micros();

    
    // Imprime el tiempo de ejecución en microsegundos
    println!("Tiempo de ejecución: {} microsegundos", microseconds); */
}