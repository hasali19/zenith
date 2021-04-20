import { useEffect, useState } from "react";
import { useHistory } from "react-router";
import { css } from "@emotion/react";
import { Card, CardActionArea, CardMedia, Typography } from "@material-ui/core";

import api, { Movie } from "../api";
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
            <div key={movie.id} style={style}>
              <Card>
                <CardActionArea
                  onClick={() => history.push(`/movies/${movie.id}`)}
                >
                  <CardMedia
                    image={movie.poster || undefined}
                    css={css`
                      aspect-ratio: 2/3;
                    `}
                  />
                </CardActionArea>
              </Card>
              <div
                css={(theme) => css`
                  padding-top: ${theme.spacing(1)};
                  padding-bottom: ${theme.spacing(2)};
                `}
              >
                <Typography
                  variant="subtitle2"
                  css={css`
                    text-overflow: ellipsis;
                    overflow: hidden;
                    white-space: nowrap;
                  `}
                >
                  {movie.title}
                </Typography>
                <Typography variant="caption">{movie.releaseYear()}</Typography>
              </div>
            </div>
          );
        }}
      </VirtualItemGrid>
    </div>
  );
}
