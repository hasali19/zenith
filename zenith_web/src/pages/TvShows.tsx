import { useState } from "react";
import { useHistory } from "react-router";
import { Grid } from "../components/Grid";
import { useOnce } from "../hooks";

interface TvShow {
  id: number;
  name: string;
  poster_url: string | null;
}

export function TvShows() {
  const history = useHistory();
  const [shows, setShows] = useState<TvShow[]>([]);

  useOnce(() => {
    fetch("/api/tv_shows")
      .then((res) => res.json())
      .then((data) => setShows(data));
  });

  const items = shows.map((show) => ({
    id: show.id,
    poster: show.poster_url,
    primary: show.name,
    secondary: null,
  }));

  return (
    <div style={{ padding: 16 }}>
      <h1>TV Shows</h1>
      <br />
      <Grid
        items={items}
        onItemClick={(item) => history.push("/tv_shows/" + item.id)}
      />
    </div>
  );
}
