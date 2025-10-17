const path = require('path');

module.exports = {
  entry: './src/main.rs',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  mode: 'development',
  experiments: {
    asyncWebAssembly: true,
  },
};
