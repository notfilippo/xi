let a = 0;
let temp = 1;

let start = Date.now();

function fib(n) {
if (n <= 1) return n;
return fib(n - 2) + fib(n - 1);
}

for (let i = 0; i < 200; i = i + 1) {
    console.log(fib(i));
}

let end = Date.now();

console.log("Execution took: " + (end - start) + " ms");
