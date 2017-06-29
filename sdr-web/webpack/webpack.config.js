const webpack = require('webpack');
var merge = require('webpack-merge');
var CopyWebpackPlugin = require('copy-webpack-plugin');
var HTMLWebpackPlugin = require('html-webpack-plugin');
var path = require('path');

module.exports = {
    entry: [
        "bootstrap-loader",
        "./web/static/js/app.js"
    ],
    output: {
        path: path.resolve(__dirname, "../priv/static/js"),
        filename: "app.js"
    },
    resolve: {
        modules: [
            path.join(__dirname, "..", "web/static/js"),
            path.join(__dirname, "..", "web/elm"),
            path.join(__dirname, "..", "node_modules")
        ],
        extensions: ['.js', '.jsx', '.elm', '.scss', '.png']
    },
    module: {
        rules: [
            {
                test: /\.css$/,
                loader: "style-loader!css-loader?importLoaders=1"
            },
            {
                test: /\.jsx?$/,
                exclude: /node_modules/,
                loader: "babel-loader",
                query: {
                    presets: ["es2015"]
                }
            },
            {
                test: /\.elm$/,
                exclude: [/elm-stuff/, /node_modules/],
                use: [
                    'elm-hot-loader',
                    'elm-webpack-loader?forceWatch=true',
                ]
            },
            {
                test: /\.scss$/,
                exclude: [
                    /elm-stuff/, /node_modules/
                ],
                loaders: ["style-loader", "css-loader", "sass-loader"]
            },
            {
                test: /\.css$/,
                exclude: [
                    /elm-stuff/, /node_modules/
                ],
                loaders: ["style-loader", "css-loader"]
            },
            {
                test: /\.(png|woff|woff2|eot|ttf|svg)$/,
                loader: 'url-loader?limit=100000'
            }
        ]
    },
    plugins: [
        new webpack.ProvidePlugin({
            $: "jquery",
            jQuery: "jquery"
        })
    ]
};