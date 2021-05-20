import React, { createContext, useContext, useRef, useState } from "react";
import { BrowserRouter, Route } from "react-router-dom";
import { Transition } from "react-transition-group";
import { css, Theme } from "@emotion/react";

import Drawer from "./Drawer";
import Home from "./pages/Home";
import Movies from "./pages/Movies";
import Shows from "./pages/Shows";
import Movie from "./pages/Movie";
import Show from "./pages/Show";
import Season from "./pages/Season";
import Episode from "./pages/Episode";
import Player from "./pages/Player";
import CastPlayer from "./pages/CastPlayer";
import ImportQueue from "./pages/ImportQueue";

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
            const offset = state === "exited" ? 76 : 0;
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

interface AppContext {
  openDrawer(): void;
}

const AppContext = createContext<AppContext>({} as AppContext);

export function useAppContext() {
  return useContext(AppContext);
}

export default function App() {
  const [open, setOpen] = useState(false);
  return (
    <AppContext.Provider
      value={{
        openDrawer() {
          setOpen(true);
        },
      }}
    >
      <BrowserRouter>
        <div css={styles.root}>
          <Drawer open={open} onClose={() => setOpen(false)} />
          <div css={styles.content}>
            <main css={styles.main}>
              <AnimatedScreen path="/player/:id">
                <Player />
              </AnimatedScreen>
              <AnimatedScreen path="/cast/:id">
                <CastPlayer />
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
              <AnimatedScreen path="/episodes/:id">
                <Episode />
              </AnimatedScreen>
              <AnimatedScreen path="/movies">
                <Movies />
              </AnimatedScreen>
              <AnimatedScreen path="/shows">
                <Shows />
              </AnimatedScreen>
              <AnimatedScreen path="/import">
                <ImportQueue />
              </AnimatedScreen>
              <AnimatedScreen path="/">
                <Home />
              </AnimatedScreen>
            </main>
          </div>
        </div>
      </BrowserRouter>
    </AppContext.Provider>
  );
}
