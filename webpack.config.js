const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./index.js",
  output: {
    publicPath: "dist",
    path: path.resolve(__dirname, "dist"),
    filename: "index.js"
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, ".")
    })
  ],
  mode: "development"
};
