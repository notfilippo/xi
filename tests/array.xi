let a = [1, 2, 3, "ciao"];

for (let i = 0; i < len(a); i = i + 1) {
    println(a[i]);
}

for (let i = 0; i < len(a); i = i + 1) {
    a[0] = 0;
}

println("And now with the change");

for (let i = 0; i < len(a); i = i + 1) {
    println(a[i]);
}
