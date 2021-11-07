import React from "react";
import ReactDOM from "react-dom";
import { createMuiTheme, CssBaseline, ThemeProvider } from "@material-ui/core";

import App from "./App";

const theme = createMuiTheme({
  palette: {
    mode: "dark",
  },
});

ReactDOM.render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <App />
    </ThemeProvider>
  </React.StrictMode>,
  document.getElementById("root")
);
