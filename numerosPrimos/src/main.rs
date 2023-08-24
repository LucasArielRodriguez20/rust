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
    // Número límite hasta el cual se generarán los primos
    let limit = 100000; 
    let mut primes = Vec::new();
    
    for num in 2..limit {
        if is_prime(num) {
            primes.push(num);
        }
    }
    println!("Tiempo de ejecución: {:.2?}", start_time.elapsed());
    println!("Primos encontrados: {:?}", primes);

}