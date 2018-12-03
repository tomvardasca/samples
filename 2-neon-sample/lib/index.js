var m = require("../native");

const BENCH = 50000;
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
