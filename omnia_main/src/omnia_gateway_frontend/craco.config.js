// this configuration is adapted from https://github.com/gabrielnic/dfinity-react template

const webpack = require('webpack');
const path = require('path');
const TerserPlugin = require("terser-webpack-plugin");

function initCanisterEnv() {
  let localCanisters, prodCanisters;
  try {
    localCanisters = require(path.resolve(
      "../..",
      ".dfx",
      "local",
      "canister_ids.json"
    ));
  } catch (error) {
    console.log("No local canister_ids.json found. Continuing production");
  }
  try {
    prodCanisters = require(path.resolve("..", "canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local");
  }

  const network =
    process.env.DFX_NETWORK ||
    (process.env.NODE_ENV === "production" ? "ic" : "local");

  const canisterConfig = network === "local" ? localCanisters : prodCanisters;

  return Object.entries(canisterConfig).reduce((prev, current) => {
    const [canisterName, canisterDetails] = current;
    prev[canisterName.toUpperCase() + "_CANISTER_ID"] =
      canisterDetails[network];
    return prev;
  }, {
    DFX_NETWORK: network,
  });
}

const canisterEnvVariables = initCanisterEnv();

const isDevelopment = process.env.NODE_ENV !== "production";

module.exports = {
  mode: "development",
  eslint: {
    enable: false,
  },
  css: {
    loaderOptions: (cssLoaderOptions, { env, paths }) => { return cssLoaderOptions; }
  },
  webpack: {
    alias: {},
    plugins: [
      new webpack.EnvironmentPlugin({
        NODE_ENV: "development",
        ...canisterEnvVariables,
      }),
    ],
    configure: (webpackConfig, { env, paths }) => {
      return {
        ...webpackConfig,
        mode: isDevelopment ? "development" : "production",
        devtool: isDevelopment ? "source-map" : false,
        optimization: {
          minimize: !isDevelopment,
          minimizer: [new TerserPlugin()],
        },
        resolve: {
          extensions: [".js", ".ts", ".jsx", ".tsx"],
          fallback: {
            assert: require.resolve("assert/"),
            buffer: require.resolve("buffer/"),
            events: require.resolve("events/"),
            stream: require.resolve("stream-browserify/"),
            util: require.resolve("util/"),
          },
        },
      };
    }
  },
  devServer: {
    proxy: {
      "/api": {
        target: "http://localhost:4943",
        changeOrigin: true,
        pathRewrite: {
          "^/api": "/api",
        },
      },
    },
    hot: true,
  },
};
