import { createRouter, createMemoryHistory } from "vue-router";

import Home from "./pages/Main.vue";
import Overview from "./pages/Overview.vue";
import Admin from "./pages/Admin.vue";

const routes = [
  { path: "/", component: Home },
  { path: "/overview", component: Overview },
  { path: "/admin", component: Admin },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
