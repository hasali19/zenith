import { BrowserRouter, Route, Switch } from "react-router-dom";

import { AppContainer } from "./components/AppContainer";

import { Home } from "./pages/Home";
import { MovieDetails } from "./pages/MovieDetails";
import { Movies } from "./pages/Movies";
import { TvShowDetails } from "./pages/TvShowDetails";
import { TvShows } from "./pages/TvShows";
import { Player } from "./pages/Player";

export default function App() {
  return (
    <BrowserRouter>
      <Switch>
        <Route path="/player/:id">
          <Player />
        </Route>
        <Route path="/">
          <AppContainer>
            <Switch>
              <Route path="/" exact>
                <Home />
              </Route>
              <Route path="/movies" exact>
                <Movies />
              </Route>
              <Route path="/movies/:id">
                <MovieDetails />
              </Route>
              <Route path="/tv_shows" exact>
                <TvShows />
              </Route>
              <Route path="/tv_shows/:id">
                <TvShowDetails />
              </Route>
            </Switch>
          </AppContainer>
        </Route>
      </Switch>
    </BrowserRouter>
  );
}
