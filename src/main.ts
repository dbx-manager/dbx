import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from 'vue-router';
import { routes } from 'vue-router/auto-routes';
import { vuetify } from "./plugins/vuetify";
import { createPinia } from "pinia";
const router = createRouter({
  history: createWebHistory(),
  routes,
})
createApp(App).use(createPinia()).use(router).use(vuetify).mount("#app");
