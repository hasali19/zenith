module.exports = {
  pwa: {
    manifestOptions: {
      name: 'Zenith',
      short_name: 'Zenith', // eslint-disable-line
    },
  },
  transpileDependencies: ['vuetify'],
  devServer: {
    disableHostCheck: true,
    proxy: {
      '^/api': { target: 'http://localhost:8000' },
    },
  },
}
