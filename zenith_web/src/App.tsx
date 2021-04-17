import { useState } from "react";
import { BrowserRouter, Route, Switch } from "react-router-dom";
import { css, Theme } from "@emotion/react";
import { Toolbar, Typography } from "@material-ui/core";

import AppBar from "./AppBar";
import Drawer from "./Drawer";

const styles = {
  root: css`
    height: 100vh;
  `,

  main: (theme: Theme) => css`
    flex: 1;
    overflow: auto;

    ${theme.breakpoints.up("sm")} {
      padding-left: 256px;
    }
  `,

  content: (theme: Theme) => css`
    padding: ${theme.spacing(2)};
  `,
};

export default function App() {
  const [open, setOpen] = useState(false);
  return (
    <BrowserRouter>
      <div css={styles.root}>
        <AppBar onToggleDrawer={() => setOpen((v) => !v)} />
        <Drawer open={open} onClose={() => setOpen(false)} />
        <div css={styles.main}>
          <Toolbar />
          <main css={styles.content}>
            <Switch>
              <Route path="/movies">
                <Typography variant="h4">Movies</Typography>
              </Route>
              <Route path="/shows">
                <Typography variant="h4">Shows</Typography>
              </Route>
              <Route path="/">
                <Typography variant="h4">Home</Typography>
              </Route>
            </Switch>
          </main>
        </div>
      </div>
    </BrowserRouter>
  );
}
