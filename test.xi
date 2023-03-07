let a = 0;
let temp = 1;

let start = time();

for (let b = 1; a < 1000000000000000000000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}

let end = time();

print "Execution took: " + ((end - start) / 1e6) + " ms";
