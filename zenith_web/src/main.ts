import Vue from 'vue'
import App from './App.vue'
import router from './router'
import vuetify from './plugins/vuetify'
import gcast from './gcast'

import './main.css'

Vue.config.productionTip = false

gcast.init()

new Vue({
  router,
  vuetify,
  render: h => h(App),
}).$mount('#app')
