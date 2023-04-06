let a = {"ciao": 1, 3: "banana"};

println(a);


# funny iterator...

let v = values(a);
let k = keys(a);

for (let i = 0; i < len(k); i = i + 1) {
    println(k[i], v[i]);
}
