const prod = process.env.NODE_ENV === 'production'
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
// const webpack = require('webpack');
module.exports = [{
  entry: "./src/js/loader.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "./src/index.html")
    }),
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    // new webpack.ProvidePlugin({
    //   TextDecoder: ['text-encoding', 'TextDecoder'],
    //   TextEncoder: ['text-encoding', 'TextEncoder']
    // })
  ],
  target: "web",
  mode: prod ? 'production' : 'development'
},
{
  entry: "./src/js/worker_loader.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "worker.js",
  },
  target: "webworker",
  plugins: [],
  mode: prod ? 'production' : 'development'
}];
