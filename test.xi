let a = 0;
let temp = 1;

for (let b = 1; a < 10000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
