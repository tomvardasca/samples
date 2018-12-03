function factorial(num) {
  if (num === 0 || num === 1) return 1;
  for (var i = num - 1; i >= 1; i--) {
    num *= i;
  }
  return num;
}

function factorial_recursion(num) {
  if (num < 0) return -1;
  else if (num == 0) return 1;
  else {
    return num * factorial_recursion(num - 1);
  }
}

var f = [];
function factorial_cache(n) {
  if (n == 0 || n == 1) return 1;
  if (f[n] > 0) return f[n];
  return (f[n] = factorial(n - 1) * n);
}

const BENCH = 50000;
try {
  BENCH = parseInt(process.argv[0], 10);
} catch {}
console.log("pure js:");
console.time("factorial");
factorial(BENCH).toString();
console.timeEnd("factorial");
console.time("factorial_cache");
factorial_cache(BENCH).toString();
console.timeEnd("factorial_cache");
console.time("factorial_recursion");
factorial_recursion(BENCH).toString();
console.timeEnd("factorial_recursion");
