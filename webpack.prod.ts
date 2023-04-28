const CopyWebpackPlugin = require("copy-webpack-plugin");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
import * as path from "path";
import * as webpack from "webpack";
import { GenerateSW } from "workbox-webpack-plugin";
import WebpackPwaManifest from "webpack-pwa-manifest";

const config: webpack.Configuration = {
  mode: "production",
  experiments: {
    asyncWebAssembly: true
  },
  entry: path.resolve(__dirname, "src/main.ts"),
  output: {
    path: path.resolve(__dirname, "dist"),
    clean: true,
    asyncChunks: true
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        {
          from: "static",
          to: "static"
        }
      ]
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "src/index.html")
    }),
    new MiniCssExtractPlugin(),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "rust-backend"),
      outDir: path.resolve(__dirname, "rust-backend/pkg")
    }),
    new GenerateSW({
      clientsClaim: true,
      skipWaiting: true
    }),
    new WebpackPwaManifest({
      name: "KinareYuki's Extrame Blog",
      short_name: "Yuki's Blog",
      description: "Kinare Yuki's extrame blog, build with rust and typescript.",
      theme_color: "#fdfdfd",
      background_color: "#fdfdfd",
      crossorigin: "use-credentials",
      orientation: "portrait",
      display: "standalone",
      inject: true,
      fingerprints: true,
      ios: true,
      publicPath: "/",
      icons: [
        {
          src: path.resolve(__dirname, "assets/icons/icon_96.png"),
          size: "96x96"
        },
        {
          src: path.resolve(__dirname, "assets/icons/icon_128.png"),
          size: "128x128"
        },
        {
          src: path.resolve(__dirname, "assets/icons/icon_192.png"),
          size: "192x192"
        },
        {
          src: path.resolve(__dirname, "assets/icons/icon_256.png"),
          size: "256x256"
        },
        {
          src: path.resolve(__dirname, "assets/icons/icon_384.png"),
          size: "384x384"
        },
        {
          src: path.resolve(__dirname, "assets/icons/icon_512.png"),
          size: "512x512"
        },
        {
          src: path.resolve(__dirname, "assets/icons/icon_1024.png"),
          size: "1024x1024"
        },
        {
          src: path.resolve(__dirname, "assets/icons/icon_maskable.png"),
          size: "2048x2048",
          purpose: "maskable"
        }
      ]
    })
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: "ts-loader",
        options: {
          transpileOnly: true
        }
      },
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"]
      }
    ]
  },
  optimization: {
    minimizer: [
      new CssMinimizerPlugin(),
      "..."
    ]
  }
};

export default config;