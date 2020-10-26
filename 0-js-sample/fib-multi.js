const { Worker } = require("worker_threads");
process.setMaxListeners(0);

const NUM_THREADS = 4;
const THREADS = [];
for (let i = 0; i < NUM_THREADS; i++) {
  THREADS.push(new Worker("./fib-multi-worker.js"));
}

function calc_on_workers(array, funcName) {
  let step = array.length / NUM_THREADS;
  let result = [];
  const uIntArray = new Uint8Array(
    new SharedArrayBuffer(Uint8Array.BYTES_PER_ELEMENT * array.length)
  );
  uIntArray.set(array);
  for (let i = 0; i < NUM_THREADS; i++) {
    result.push(
      new Promise((resolve, reject) => {
        const wid = Math.random().toString();
        const worker = THREADS[i];
        const uIntarrayToSend = uIntArray.slice(i * step, i * step + step);
        worker.postMessage({
          id: wid,
          array: uIntarrayToSend.buffer,
          func: funcName
        });
        worker.on("message", ({ id, result }) => {
          if (id === wid) {
            const r = new Uint8Array(result);
            resolve(r);
          }
        });
      })
    );
  }
  return Promise.all(result).then(r => {
    const allUintArray = new Uint8Array(r[0].length * NUM_THREADS);
    for (let i = 0; i < NUM_THREADS; i++) {
      allUintArray.set(r[i], i * r[i].length);
    }
    return allUintArray;
  });
}

module.exports.fibonacci_iter_arr = function(array) {
  return calc_on_workers(array, "fibonacci_iter");
};

module.exports.fibonacci_rec_arr = function(array) {
  return calc_on_workers(array, "fibonacci_rec");
};

module.exports.fibonacci_rec_to_arr = function(array) {
  return calc_on_workers(array, "fibonacci_rec_to");
};
