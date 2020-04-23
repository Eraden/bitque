const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const dotenv = require('dotenv');
const webpack = require('webpack');

const MiniCssExtractPlugin = require('mini-css-extract-plugin');

process.env.RUST_LOG = 'info';

dotenv.config();

module.exports = {
    entry: path.resolve(__dirname, 'js', 'index.js'),
    output: {
        filename: '[name].js',
        path: path.resolve(__dirname, process.env.NODE_ENV === 'production' ? 'dist' : 'dev'),
        publicPath: '/',
    },
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
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: [
                    MiniCssExtractPlugin.loader,
                    {
                        loader: 'css-loader',
                    },
                ],
            },
            {
                test: /\.svg$/,
                use: [
                    { loader: 'file-loader' },
                    {
                        loader: 'svgo-loader',
                        options: {
                            externalConfig: "svgo-config.yml"
                        }
                    }
                ]
            }
        ],
    },
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
        new MiniCssExtractPlugin({
            filename: '[name].css',
            chunkFilename: '[id].css',
            ignoreOrder: true,
        }),
    ]
};
