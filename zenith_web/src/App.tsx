import { BrowserRouter, Route, Switch } from "react-router-dom";

import { AppContainer } from "./components/AppContainer";

import { Home } from "./pages/Home";
import { MovieDetails } from "./pages/MovieDetails";
import { Movies } from "./pages/Movies";
import { TvShows } from "./pages/TvShows";

export default function App() {
  return (
    <BrowserRouter>
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
        </Switch>
      </AppContainer>
    </BrowserRouter>
  );
}
