const path = require('path');

module.exports = function override(config) {
  // Add path aliases
  config.resolve.alias = {
    ...config.resolve.alias,
    '@': path.resolve(__dirname, 'src'),
    'react': path.resolve(__dirname, './node_modules/react'),
  };

  // Optimize build
  config.optimization = {
    ...config.optimization,
    splitChunks: {
      chunks: 'all',
      name: false,
    },
    runtimeChunk: {
      name: entrypoint => `runtime-${entrypoint.name}`,
    },
  };

  return config;
}; 