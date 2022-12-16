// this configuration is adapted from https://github.com/gabrielnic/dfinity-react template

const TerserPlugin = require("terser-webpack-plugin");

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
        },
      };
    }
  },
};
