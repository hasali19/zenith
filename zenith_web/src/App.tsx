import { useState } from "react";
import { BrowserRouter, Route, Switch } from "react-router-dom";
import { css, Theme } from "@emotion/react";
import { Toolbar, Typography } from "@material-ui/core";

import AppBar from "./AppBar";
import Drawer from "./Drawer";
import Movies from "./pages/Movies";
import Movie from "./pages/Movie";

const styles = {
  root: css`
    height: 100vh;
  `,

  content: (theme: Theme) => css`
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;

    ${theme.breakpoints.up("sm")} {
      padding-left: 256px;
    }
  `,

  main: css`
    flex: 1;
    overflow: hidden;
  `,
};

export default function App() {
  const [open, setOpen] = useState(false);
  return (
    <BrowserRouter>
      <div css={styles.root}>
        <AppBar onToggleDrawer={() => setOpen((v) => !v)} />
        <Drawer open={open} onClose={() => setOpen(false)} />
        <div css={styles.content}>
          <Toolbar />
          <main css={styles.main}>
            <Switch>
              <Route path="/movies/:id">
                <Movie />
              </Route>
              <Route path="/movies">
                <Movies />
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
