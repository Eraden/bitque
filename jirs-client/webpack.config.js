const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const dotenv = require('dotenv');
const webpack = require('webpack');
const { execSync, exec } = require('child_process');

process.env.RUST_LOG = 'info';

execSync('cd .. && cargo build --bin jirs-css');

if (process.env.NODE_ENV === "production") {
    execSync("rm -Rf ./dist");
    execSync("mkdir -p ./dist");
    execSync('cd .. && ./target/debug/jirs-css -O ./jirs-client/dist/styles.css');
    console.log("CSS combined");
} else {
    exec('cd .. && ./target/debug/jirs-css --watch -O ./jirs-client/dev/styles.css');
    console.log("CSS combined, watching for changes...");
}

dotenv.config();

module.exports = {
    entry: path.resolve(__dirname, 'js', 'index.js'),
    output: {
        filename: '[name].js',
        path: path.resolve(__dirname, process.env.NODE_ENV === 'production' ? 'dist' : 'dev'),
        publicPath: '/',
    },
    ...(
        process.env.NODE_ENV === "production"
            ? {
                mode: "production",
                watch: false,
                devtool: false,
            }
            : {
                mode: "development",
                watch: true,
                devtool: 'source-map',
                devServer: {
                    contentBase: path.join(__dirname, 'dev'),
                    historyApiFallback: true,
                    hot: true,
                    port: process.env.JIRS_CLIENT_PORT || 6000,
                    host: process.env.JIRS_CLIENT_BIND || '0.0.0.0',
                    allowedHosts: [
                        'localhost:6000',
                        'localhost:8000',
                    ],
                    headers: {
                        'Access-Control-Allow-Origin': '*',
                    }
                },
            }
    ),
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname),
            extraArgs: '--no-typescript'
        }),
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, "js", "template.ejs"),
        }),
        new webpack.EnvironmentPlugin([
            'NODE_ENV',
            'DEBUG',
            'JIRS_CLIENT_PORT',
            'JIRS_CLIENT_BIND',
            'JIRS_SERVER_PORT',
            'JIRS_SERVER_BIND',
        ]),
    ],
};
