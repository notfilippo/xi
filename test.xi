fn makeCounter() {
  let i = 0;
  fn count() {
    i = i + 1;
    print i;
  }

  return count;
}

let counter = makeCounter();
counter();
counter();
counter();
counter();
counter();
counter();
counter();
counter();
counter();
counter();
counter();
counter();
counter();

return "True";
