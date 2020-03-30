const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    entry: path.resolve(__dirname, 'js', 'index.js'),
    output: {
        filename: '[name].js',
        path: path.resolve(__dirname, 'dev'),
        publicPath: '/',
    },
    devtool: 'source-map',
    devServer: {
        contentBase: path.join(__dirname, 'dev'),
        historyApiFallback: true,
        hot: true,
        port: 4000,
        host: '0.0.0.0',
        allowedHosts: [
            'localhost:4000',
            'localhost:8000',
        ],
        headers: {
            'Access-Control-Allow-Origin': '*',
        }
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname),
            args: "--log-level warn",
            extraArgs: "--no-typescript",
        }),
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, "js", "template.ejs"),
        }),
    ]
};
