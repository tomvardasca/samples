const path = require("path");

module.exports = {
  target: "node",
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "run"),
    filename: "index.js"
  },
  mode: "development"
};
