import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHistory } from 'vue-router';
import { routes } from 'vue-router/auto-routes';
import { vuetify } from "./plugins/vuetify";
const router = createRouter({
  history: createWebHistory(),
  routes,
})
createApp(App).use(router).use(vuetify).mount("#app");
