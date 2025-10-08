import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import App from "./App.vue";
import { router } from "./router";
import "./index.css";
import sl from "./localization/sl.json";
import en from "./localization/en.json";

const i18n = createI18n({
  locale: "sl",
  fallbackLocale: "en",
  datetimeFormats: {
    en: {
      short: {
        year: "numeric",
        month: "short",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      },
      long: {
        year: "numeric",
        month: "long",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      },
    },
    sl: {
      short: {
        year: "numeric",
        month: "short",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      },
      long: {
        year: "numeric",
        month: "long",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      },
    },
  },
  messages: {
    sl,
    en,
  },
});

createApp(App).use(i18n).use(router).mount("#app");
