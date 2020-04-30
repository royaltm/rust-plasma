const prod = process.env.NODE_ENV === 'production'
const path = require('path');
const webpack = require('webpack');
// const AssetsPlugin = require('assets-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const dist = path.resolve(__dirname, "dist");
const favicon = path.resolve(__dirname, "..", "desktop", "plasma.ico");
module.exports = [{
  entry: "./src/ts/worker_loader.ts",
  output: {
    path: dist,
    filename: "worker.js",
  },
  devtool: !prod && 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    extensions: [ '.ts', '.js', '.wasm' ]
  },
  plugins: [
    // new AssetsPlugin({fullPath: false}),
  ],
  target: "webworker",
  mode: prod ? 'production' : 'development'
},
{
  entry: "./src/ts/plasma_loader.ts",
  output: {
    path: dist,
    filename: "plasma.js",
    libraryTarget: "umd",
    libraryExport: "default",
    library: "plasmaLoader"
  },
  devtool: !prod && 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    extensions: [ '.ts', '.js', '.wasm' ]
  },
  plugins: [
    new webpack.DefinePlugin({
      _WORKER_PATH_LOCATION_: JSON.stringify("worker.js"),
    }),
    prod && new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      favicon: favicon,
      template: path.resolve(__dirname, "./src/index.html")
    }),
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    // new webpack.ProvidePlugin({
    //   TextDecoder: ['text-encoding', 'TextDecoder'],
    //   TextEncoder: ['text-encoding', 'TextEncoder']
    // })
  ].filter(Boolean),
  target: "web",
  mode: prod ? 'production' : 'development'
}];
