module.exports.fibonacci_iter = function(num) {
  var a = 1,
    b = 0,
    temp;

  while (num >= 0) {
    temp = a;
    a = a + b;
    b = temp;
    num--;
  }

  return b;
};

module.exports.fibonacci_rec = function fibonacci_rec(num) {
  if (num <= 1) return 1;

  return fibonacci_rec(num - 1) + fibonacci_rec(num - 2);
};

var memo = {};
module.exports.fibonacci_memo = function fibonacci_memo(num) {
  if (memo[num]) return memo[num];
  if (num <= 1) return 1;

  return (memo[num] = fibonacci_memo(num - 1) + fibonacci_memo(num - 2));
};

module.exports.fibonacci_rec_to = function(n) {
  function recur(n, a, b) {
    if (n > 0) {
      return recur(n - 1, b, a + b);
    } else {
      return a;
    }
  }
  return recur(n, 0, 1);
};
