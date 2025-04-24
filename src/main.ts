import { createApp } from "vue";
import 'vuetify/styles'
import App from './App.vue'
import '@mdi/font/css/materialdesignicons.min.css'
// @ts-ignore
import router from '../router/index.js'
import {vuetify} from './plugins/vuetify.ts'
const app = createApp(App)
app.use(router)
app.use(vuetify)
app.mount('#app')
