function test() {
  var a, b;
  a = 1;
  b = 2;
  function inner() {
    let c = 12;
    return a + b + c;
  }
  return inner() + 10;
}

test()
