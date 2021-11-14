import { Component, Show } from "solid-js";
import { Route, Router, Routes } from "solid-app-router";

import * as styles from "./App.css";
import preferences from "./preferences";
import { SideBar } from "./SideBar";

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
      <div class={styles.app}>
        <Show
          when={preferences.server != null}
          fallback={<SelectServerScreen />}
        >
          <SideBar />
          <div class={styles.mainContent}>
            <Routes>
              <Route path="/" element={<HomeScreen />} />
              <Route path="/movies" element={<MoviesScreen />} />
              <Route path="/movies/:id" element={<MovieScreen />} />
              <Route path="/shows" element={<ShowsScreen />} />
              <Route path="/shows/:id" element={<ShowScreen />} />
              <Route path="/seasons/:id" element={<SeasonScreen />} />
              <Route path="/episodes/:id" element={<EpisodeScreen />} />
              <Route path="/player/:id" element={<PlayerScreen />} />
              <Route path="/*all" element={<NotFoundScreen />} />
            </Routes>
          </div>
        </Show>
      </div>
    </Router>
  );
};
