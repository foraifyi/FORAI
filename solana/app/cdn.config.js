module.exports = {
  // CDN configuration
  cdn: {
    js: [
      'https://cdn.jsdelivr.net/npm/react@18.2.0/umd/react.production.min.js',
      'https://cdn.jsdelivr.net/npm/react-dom@18.2.0/umd/react-dom.production.min.js',
      'https://cdn.jsdelivr.net/npm/chart.js@4.4.1/dist/chart.umd.min.js',
    ],
    css: [
      'https://cdn.jsdelivr.net/npm/normalize.css@8.0.1/normalize.min.css',
    ],
  },
  
  // CDN domain configuration
  domains: [
    {
      name: 'default',
      domain: process.env.REACT_APP_CDN_DOMAIN || 'https://cdn.example.com',
      pattern: '**/*.{js,css,png,jpg,jpeg,gif,svg}',
    },
    {
      name: 'images',
      domain: process.env.REACT_APP_IMAGE_CDN_DOMAIN || 'https://img.example.com',
      pattern: '**/*.{png,jpg,jpeg,gif,svg}',
    },
  ],
}; 