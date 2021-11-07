import React, { createContext, useContext } from "react";
import { BrowserRouter, Route } from "react-router-dom";
import { Box, Paper } from "@material-ui/core";

import { DrawerContent } from "./Drawer";
import Home from "./pages/Home";
import Movies from "./pages/Movies";
import Shows from "./pages/Shows";
import Movie from "./pages/Movie";
import Show from "./pages/Show";
import Season from "./pages/Season";
import Episode from "./pages/Episode";
import Player from "./pages/Player";

function AnimatedScreen({
  path,
  children,
}: {
  path: string;
  children: React.ReactNode;
}) {
  return (
    <Route path={path} exact>
      {children}
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
  return (
    <BrowserRouter>
      <Box height="100vh" display="flex">
        <Box>
          <DrawerContent onClose={() => {}} />
        </Box>
        <Box
          component={Paper}
          square
          elevation={4}
          flex={1}
          position="relative"
          overflow="hidden"
        >
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
          <AnimatedScreen path="/episodes/:id">
            <Episode />
          </AnimatedScreen>
          <AnimatedScreen path="/movies">
            <Movies />
          </AnimatedScreen>
          <AnimatedScreen path="/shows">
            <Shows />
          </AnimatedScreen>
          <AnimatedScreen path="/">
            <Home />
          </AnimatedScreen>
        </Box>
      </Box>
    </BrowserRouter>
  );
}
