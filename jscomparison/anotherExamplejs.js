console.time();
function sumNaturalNumbers(n) {
    if (n === 0) {
        return 0;
    }
    return n + sumNaturalNumbers(n - 1);
}

const n = 10000;
const result = sumNaturalNumbers(n);
console.log(`La suma de los ${n} primeros números naturales es: ${result}`);
console.log(console.timeEnd());