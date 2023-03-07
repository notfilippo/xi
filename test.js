let a = 0;
let temp = 1;

let start = Date.now();

for (let b = 1; a < 1000000000000000000000; b = temp + b) {
  console.log(a);
  temp = a;
  a = b;
}

let end = Date.now();

console.log("Execution took: " + (end - start) + " ms");
