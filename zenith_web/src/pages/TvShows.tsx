import { useLayoutEffect, useRef, useState } from "react";
import { Grid } from "../components/Grid";
import { useOnce } from "../hooks";

interface TvShow {
  id: number;
  name: string;
  poster_url: string | null;
}

export function TvShows() {
  const [shows, setShows] = useState<TvShow[]>([]);

  useOnce(() => {
    fetch("/api/tv_shows")
      .then((res) => res.json())
      .then((data) => setShows(data));
  });

  const items = shows.map((show) => ({
    id: show.id,
    poster: show.poster_url
      ? `https://image.tmdb.org/t/p/w185${show.poster_url}`
      : null,
    primary: show.name,
    secondary: null,
  }));

  return (
    <div style={{ padding: 16 }}>
      <h1>TV Shows</h1>
      <br />
      <Grid items={items} />
    </div>
  );
}
