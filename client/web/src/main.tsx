import React from "react";
import ReactDOM from "react-dom";
import { createMuiTheme, CssBaseline, ThemeProvider } from "@material-ui/core";

import App from "./App";
import { GCastProvider } from "./gcast";

const theme = createMuiTheme({
  palette: {
    mode: "dark",
  },
});

ReactDOM.render(
  <React.StrictMode>
    <GCastProvider>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <App />
      </ThemeProvider>
    </GCastProvider>
  </React.StrictMode>,
  document.getElementById("root")
);
