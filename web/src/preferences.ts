import { createStore } from "solid-js/store";

export interface Preferences {
  server: string | null;
}

const [preferences, setPreferences] = createStore<Preferences>({
  server: localStorage.getItem("server") ?? "",
});

export const server = () => preferences.server;
export const setServer = (value: string | null) => {
  if (value) {
    localStorage.setItem("server", value);
  } else {
    localStorage.removeItem("server");
  }
  setPreferences("server", value);
};

export default {
  get server(): string | null {
    return preferences.server;
  },

  set server(v: string | null) {
    setServer(v);
  },
};
