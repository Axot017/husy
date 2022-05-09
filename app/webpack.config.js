const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const webpack = require('webpack');

const distPath = path.resolve(__dirname, "dist");
require('dotenv').config({ path: './.env' }); 

module.exports = (env, argv) => {
  return {
    devServer: {
      historyApiFallback: true,
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8080
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "app.js",
      webassemblyModuleFilename: "app.wasm"
    },
    plugins: [
      new webpack.DefinePlugin({
        "FOO": JSON.stringify("bar"),
        "process.env": JSON.stringify(process.env),
      }),
      new CopyWebpackPlugin([
        { from: './static', to: distPath }
      ]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
    ],
    watch: argv.mode !== 'production'
  };
};
