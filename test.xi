let a = 0;
let b = 1;
let tmp;

let N = 10000;

print("First", N, "numbers of the fibonacci sequence");

for (let i = 0; i < N; i = i + 1) {
    println(a);
    tmp = b;
    b = a + b;
    a = tmp;
}