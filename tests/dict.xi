let a = {"ciao": 1, 3: "banana"};

println(a);
println(a[3]);

for (let i = 0; i < 100; i = i + 1) {
    a[i] = i * 2;
}

println(a);
