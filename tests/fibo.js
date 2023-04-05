let start = Date.now();

let a = 0;
let b = 1;
let tmp;

let N = 1000;

for (let i = 0; i < N; i = i + 1) {
    tmp = b;
    b = a + b;
    a = tmp;
}

console.log(a);

let end = Date.now();
console.log("Execution took", end - start, "ms");
