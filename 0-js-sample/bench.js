const Benchmark = require("benchmark");
const fib = require("./fib");
const fibMulti = require("./fib-multi");
process.setMaxListeners(0);
const REPEAT = 4500;
let fib_seq = [2, 3, 4, 6, 8, 10, 11, 13, 15, 18, 20, 23, 25];
// repeat array
fib_seq = [...new Array(REPEAT)]
  .map(() => fib_seq)
  .reduce((acc, v) => acc.concat(v), []);

var suite = new Benchmark.Suite();

// add tests
suite
  .add("Fibonacci Iterative", function() {
    fib_seq.map(fib.fibonacci_iter);
  })
  .add("Fibonacci Recursive", function() {
    fib_seq.map(fib.fibonacci_rec);
  })
  .add("Fibonacci Memoized", function() {
    fib_seq.map(fib.fibonacci_memo);
  })
  .add("Fibonacci Recursive Tail optimized", function() {
    fib_seq.map(fib.fibonacci_rec_to);
  })
  .add(
    "Fibonacci Iterative - Workers",
    function(deferred) {
      fibMulti.fibonacci_iter_arr(fib_seq).then(() => deferred.resolve());
    },
    { defer: true }
  )
  .add(
    "Fibonacci Recursive - Workers",
    function(deferred) {
      fibMulti.fibonacci_rec_arr(fib_seq).then(() => deferred.resolve());
    },
    { defer: true }
  )
  .add(
    "Fibonacci Recursive Tail optimized - Workers",
    function(deferred) {
      fibMulti.fibonacci_rec_to_arr(fib_seq).then(() => deferred.resolve());
    },
    { defer: true }
  )
  // add listeners
  .on("cycle", function(event) {
    console.log(String(event.target));
  })
  .on("complete", function() {
    console.log("Fastest is " + this.filter("fastest").map("name"));
  })
  // run async
  .run({ async: true });
