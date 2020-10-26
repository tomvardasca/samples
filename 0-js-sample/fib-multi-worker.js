const fib = require("./fib");
const { parentPort } = require("worker_threads");
process.setMaxListeners(0);

parentPort.on("message", ({ id, array, func }) => {
  const result = new Uint8Array(array).map(fib[func]);
  parentPort.postMessage(
    {
      id,
      result: result.buffer
    },
    [result.buffer]
  );
});
