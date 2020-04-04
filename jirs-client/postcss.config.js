module.exports = {
    parser: 'sugarss',
    plugins: [
        require('autoprefixer')({}),
        require('cssnano'),
    ],
};
