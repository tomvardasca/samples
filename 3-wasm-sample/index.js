const rust = import("./wasm_sample");
rust.then(m => {
  const BENCH = 50000;
  try {
    BENCH = parseInt(process.argv[0], 10);
  } catch {}
  console.log("node.js wasm:");
  console.time("factorial_while");
  m.factorial_while(BENCH);
  console.timeEnd("factorial_while");
  console.time("factorial");
  m.factorial(BENCH);
  console.timeEnd("factorial");
  console.time("factorial_recursion");
  m.factorial_recursion(BENCH);
  console.timeEnd("factorial_recursion");
  console.time("factorial_par_iter");
  m.factorial_par_iter(BENCH);
  console.timeEnd("factorial_par_iter");
});
