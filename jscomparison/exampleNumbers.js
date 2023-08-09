function isPrime(num) {
    if (num <= 1) {
        return false;
    }
    for (let i = 2; i <= Math.sqrt(num); i++) {
        if (num % i === 0) {
            return false;
        }
    }
    return true;
}

function findPrimes(limit) {
    const primes = [];
    for (let num = 2; num < limit; num++) {
        if (isPrime(num)) {
            primes.push(num);
        }
    }
    return primes;
}
console.time();
const limit = 100000; // Número límite hasta el cual se generarán los primos
const primes = findPrimes(limit);
console.log("Primos encontrados: ", primes);
console.log(console.timeEnd()); 