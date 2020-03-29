const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: 'development',
  context: __dirname,
  entry: path.join(__dirname, 'src/index.jsx'),
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, 'dev'),
    publicPath: '/',
  },
  module: {
    rules: [
      {
        test: /\.([tj])sx?$/,
        exclude: /node_modules/,
        use: [ 'swc-loader' ],
      },
      {
        test: /\.css$/,
        use: ['style-loader', { loader: 'css-loader' }],
      },
      {
        test: /\.(jpe?g|png|gif|woff2?|eot|ttf|otf|svg)$/,
        use: [
          {
            loader: 'url-loader',
            options: { limit: 15000 },
          },
        ],
      },
    ],
  },
  resolve: {
    modules: [ path.join(__dirname, 'src'), 'node_modules' ],
    extensions: [ '*', '.js', '.jsx', '.ts', '.tsx' ],
  },
  devtool: 'source-map',
  devServer: {
    contentBase: path.join(__dirname, 'dev'),
    historyApiFallback: true,
    hot: true,
    port: 8000,
    host: '0.0.0.0',
    allowedHosts: [
      'localhost:3000',
      'localhost:8000',
    ],
    headers: {
      'Access-Control-Allow-Origin': '*',
    }
  },
  plugins: [
    new webpack.HotModuleReplacementPlugin(),
    new HtmlWebpackPlugin({
      template: path.join(__dirname, 'src/index.html'),
      favicon: path.join(__dirname, 'src/favicon.png'),
    }),
  ],
};
