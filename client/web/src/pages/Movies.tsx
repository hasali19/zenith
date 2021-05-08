import { useEffect, useState } from "react";
import { useHistory } from "react-router";

import api, { Movie } from "../api";
import PosterMediaItem from "../components/PosterMediaItem";
import VirtualItemGrid from "../components/VirtualItemGrid";

export default function () {
  const history = useHistory();
  const [movies, setMovies] = useState<Movie[]>([]);

  useEffect(() => {
    api.movies.getMovies().then(setMovies);
  }, []);

  return (
    <div style={{ height: "100%" }}>
      <VirtualItemGrid count={movies.length}>
        {(i, style) => {
          const movie = movies[i];
          return (
            <PosterMediaItem
              key={movie.id}
              poster={movie.poster || undefined}
              primary={movie.title}
              secondary={movie.releaseYear()?.toString()}
              style={style}
              onClick={() => history.push(`/movies/${movie.id}`)}
            />
          );
        }}
      </VirtualItemGrid>
    </div>
  );
}
