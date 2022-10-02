import { Component, Show } from "solid-js";
import { Outlet, Route, Router, Routes } from "solid-app-router";
import preferences from "./preferences";

import { NavDrawer } from "./NavDrawer";

import { SelectServerScreen } from "./pages/SelectServer";
import { HomeScreen } from "./pages/Home";
import { MovieScreen } from "./pages/Movie";
import { MoviesScreen } from "./pages/Movies";
import { ShowScreen } from "./pages/Show";
import { ShowsScreen } from "./pages/Shows";
import { SeasonScreen } from "./pages/Season";
import { EpisodeScreen } from "./pages/Episode";
import { PlayerScreen } from "./pages/Player";
import { NotFoundScreen } from "./pages/NotFound";

export const App: Component = () => {
  return (
    <Router>
      <Show when={preferences.server != null} fallback={<SelectServerScreen />}>
        <Routes>
          <Route path="/player/:id" element={<PlayerScreen />} />
          <Route path="/*all" element={<MainScreen />}>
            <Route path="/" element={<HomeScreen />} />
            <Route path="/movies" element={<MoviesScreen />} />
            <Route path="/movies/:id" element={<MovieScreen />} />
            <Route path="/shows" element={<ShowsScreen />} />
            <Route path="/shows/:id" element={<ShowScreen />} />
            <Route path="/seasons/:id" element={<SeasonScreen />} />
            <Route path="/episodes/:id" element={<EpisodeScreen />} />
            <Route path="/*all" element={<NotFoundScreen />} />
          </Route>
        </Routes>
      </Show>
    </Router>
  );
};

const MainScreen: Component = () => {
  return (
    <div class="w-screen h-screen grid grid-cols-[256px_minmax(0,_1fr)]">
      <NavDrawer />
      <div class="100-screen overflow-auto">{<Outlet />}</div>
    </div>
  );
};
