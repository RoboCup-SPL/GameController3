const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = (_, { mode }) => ({
  devServer: {
    port: 3000,
    static: false,
  },
  entry: {
    main: path.resolve(__dirname, "src", "main.jsx"),
    launcher: path.resolve(__dirname, "src", "launcher.jsx"),
  },
  module: {
    rules: [
      {
        test: /\.jsx$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: [
              [
                "@babel/preset-react",
                {
                  runtime: "automatic",
                },
              ],
            ],
          },
        },
      },
      {
        test: /\.css$/,
        exclude: /node_modules/,
        use: [
          mode === "production" ? MiniCssExtractPlugin.loader : "style-loader",
          {
            loader: "css-loader",
            options: {
              importLoaders: 1,
            },
          },
          {
            loader: "postcss-loader",
            options: {
              postcssOptions: {
                plugins: [
                  [
                    "tailwindcss",
                    {
                      content: [path.resolve(__dirname, "src/**/*.jsx")],
                    },
                  ],
                  "autoprefixer",
                ].concat(mode === "production" ? ["cssnano"] : []),
              },
            },
          },
        ],
      },
    ],
  },
  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, "build"),
    clean: true,
  },
  plugins: [
    new HtmlWebpackPlugin({
      chunks: ["main"],
      filename: "main.html",
      title: "GameController",
    }),
    new HtmlWebpackPlugin({
      chunks: ["launcher"],
      filename: "launcher.html",
      title: "GameController (Launcher)",
    }),
  ].concat(mode === "production" ? [new MiniCssExtractPlugin()] : []),
  resolve: {
    extensions: ["...", ".jsx"],
  },
});
