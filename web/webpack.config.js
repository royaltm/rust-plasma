const prod = process.env.NODE_ENV === 'production'
const path = require('path');
const webpack = require('webpack');
// const AssetsPlugin = require('assets-webpack-plugin');
// const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const dist = path.resolve(__dirname, "dist");
const favicon = path.resolve(__dirname, "..", "desktop", "plasma.ico");
module.exports = [{
  context: path.join(__dirname, "."),
  entry: "./src/ts/worker_loader.ts",
  output: {
    path: dist,
    filename: "worker.js",
  },
  experiments: {
      asyncWebAssembly: true
  },
  devtool: !prod && 'inline-source-map',
  module: {
    rules: [
      {
        // all files with a `.ts`, `.cts`, `.mts` or `.tsx` extension will be handled by `ts-loader`
        test: /\.([cm]?ts|tsx)$/,
        loader: "ts-loader",
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    // Add `.ts` and `.tsx` as a resolvable extension.
    extensions: [".ts", ".tsx", ".js", ".wasm"],
    // Add support for TypeScripts fully qualified ESM imports.
    extensionAlias: {
     ".js": [".js", ".ts"],
     ".cjs": [".cjs", ".cts"],
     ".mjs": [".mjs", ".mts"]
    }
  },
  plugins: [
    // prod && new CleanWebpackPlugin(),
    // new AssetsPlugin({fullPath: false}),
  ].filter(Boolean),
  target: "webworker",
  mode: prod ? 'production' : 'development'
},
{
  context: path.join(__dirname, "."),
  entry: "./src/ts/plasma_loader.ts",
  output: {
    path: dist,
    filename: "plasma.js",
    libraryTarget: "umd",
    libraryExport: "default",
    library: "plasmaLoader"
  },
  experiments: {
      asyncWebAssembly: true
  },
  devtool: !prod && 'inline-source-map',
  module: {
    rules: [
      {
        // all files with a `.ts`, `.cts`, `.mts` or `.tsx` extension will be handled by `ts-loader`
        test: /\.([cm]?ts|tsx)$/,
        loader: "ts-loader",
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    // Add `.ts` and `.tsx` as a resolvable extension.
    extensions: [".ts", ".tsx", ".js", ".wasm"],
    // Add support for TypeScripts fully qualified ESM imports.
    extensionAlias: {
     ".js": [".js", ".ts"],
     ".cjs": [".cjs", ".cts"],
     ".mjs": [".mjs", ".mts"]
    }
  },
  plugins: [
    new webpack.DefinePlugin({
      _WORKER_PATH_LOCATION_: JSON.stringify("worker.js"),
    }),
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
  ],
  target: "web",
  mode: prod ? 'production' : 'development'
}];
