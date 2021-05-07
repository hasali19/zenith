import React, { useRef, useState } from "react";
import { BrowserRouter, Route } from "react-router-dom";
import { Transition } from "react-transition-group";
import { css, Theme } from "@emotion/react";
import { Toolbar, Typography } from "@material-ui/core";

import AppBar from "./AppBar";
import Drawer from "./Drawer";
import Movies from "./pages/Movies";
import Shows from "./pages/Shows";
import Movie from "./pages/Movie";
import Show from "./pages/Show";
import Season from "./pages/Season";
import Player from "./pages/Player";

const styles = {
  root: css`
    height: 100vh;
  `,

  content: (theme: Theme) => css`
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;

    ${theme.breakpoints.up("md")} {
      padding-left: 256px;
    }
  `,

  main: css`
    flex: 1;
    overflow: hidden;
    position: relative;
  `,
};

function AnimatedScreen({
  path,
  children,
}: {
  path: string;
  children: React.ReactNode;
}) {
  const ref = useRef(null);
  return (
    <Route path={path} exact>
      {({ match }) => (
        <Transition
          in={!!match}
          nodeRef={ref}
          addEndListener={() => {}}
          timeout={200}
        >
          {(state) => {
            const shown = state === "entering" || state === "entered";
            const offset = state === "exited" ? 50 : 0;
            return (
              <div
                ref={ref}
                style={{
                  opacity: shown ? 1 : 0,
                  transform: `translateY(${offset}px)`,
                  zIndex: shown ? 1 : 0,
                }}
                css={css`
                  position: absolute;
                  width: 100%;
                  height: 100%;
                  overflow: hidden;
                  transition: all 200ms ease-in;
                `}
              >
                {state !== "exited" && children}
              </div>
            );
          }}
        </Transition>
      )}
    </Route>
  );
}

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
            <AnimatedScreen path="/player/:id">
              <Player />
            </AnimatedScreen>
            <AnimatedScreen path="/movies/:id">
              <Movie />
            </AnimatedScreen>
            <AnimatedScreen path="/shows/:id">
              <Show />
            </AnimatedScreen>
            <AnimatedScreen path="/seasons/:id">
              <Season />
            </AnimatedScreen>
            <AnimatedScreen path="/movies">
              <Movies />
            </AnimatedScreen>
            <AnimatedScreen path="/shows">
              <Shows />
            </AnimatedScreen>
            <AnimatedScreen path="/">
              <Typography variant="h4">Home</Typography>
            </AnimatedScreen>
          </main>
        </div>
      </div>
    </BrowserRouter>
  );
}
