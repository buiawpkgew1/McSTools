import { createApp } from "vue";
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import App from './App.vue'
import '@mdi/font/css/materialdesignicons.min.css'
// @ts-ignore
import router from '../router/index.js'
const vuetify = createVuetify({
    icons: {
        defaultSet: 'mdi',
    }
})
const app = createApp(App)
app.use(router)
app.use(vuetify)
app.mount('#app')
