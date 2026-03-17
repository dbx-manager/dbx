import "vuetify/styles";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import { aliases, mdi } from "vuetify/iconsets/mdi";
import "@mdi/font/css/materialdesignicons.css"; 

export const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: "mdi",
    sets: { mdi },
  },
  theme: {
    defaultTheme: "dark",
    themes: {
      dark: {
        colors: {
          background: "#222226",
          surface: "#303030",
          primary: "#3584e4", 
        },
      },
    },
  },
});
