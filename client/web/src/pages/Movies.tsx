import { useEffect, useState } from "react";
import { css } from "@emotion/react";
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
    <div
      css={css`
        height: 100%;
        display: flex;
        flex-direction: column;
      `}
    >
      <div
        css={css`
          min-height: 0;
          flex: 1;
        `}
      >
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
    </div>
  );
}
