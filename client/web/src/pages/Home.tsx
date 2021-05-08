import { useEffect, useState } from "react";
import { useHistory } from "react-router";
import { css, Theme } from "@emotion/react";
import { Typography } from "@material-ui/core";

import api, { Movie, TvShow } from "../api";
import PosterMediaItem from "../components/PosterMediaItem";

const styles = {
  root: (theme: Theme) => css`
    height: 100%;
    overflow-y: auto;
    padding: ${theme.spacing(2)} 0;
  `,

  slider: (theme: Theme) => css`
    display: flex;
    overflow-x: scroll;
    margin: ${theme.spacing(2)} 0px;
    padding: 0px ${theme.spacing(1.5)};

    &::-webkit-scrollbar {
      width: 0;
      height: 0;
    }
  `,

  item: (theme: Theme) => css`
    min-width: 130px;
    padding: ${theme.spacing(0.5)};
  `,
};

export default function () {
  const history = useHistory();
  const [movies, setMovies] = useState<Movie[]>([]);
  const [shows, setShows] = useState<TvShow[]>([]);

  useEffect(() => {
    api.movies.getRecent().then(setMovies);
    api.tv.getRecentShows().then(setShows);
  }, []);

  return (
    <div css={styles.root}>
      <Typography
        variant="h5"
        css={(theme) =>
          css`
            margin: 0 ${theme.spacing(2)};
          `
        }
      >
        New Movies
      </Typography>
      <div css={styles.slider}>
        {movies.map((movie) => (
          <div key={movie.id} css={styles.item}>
            <PosterMediaItem
              poster={movie.poster || undefined}
              primary={movie.title}
              secondary={movie.releaseYear()?.toString()}
              onClick={() => history.push(`/movies/${movie.id}`)}
            />
          </div>
        ))}
      </div>
      <Typography
        variant="h5"
        css={(theme) =>
          css`
            margin: 0 ${theme.spacing(2)};
          `
        }
      >
        Updated Shows
      </Typography>
      <div css={styles.slider}>
        {shows.map((show) => (
          <div key={show.id} css={styles.item}>
            <PosterMediaItem
              poster={show.poster || undefined}
              primary={show.name}
              secondary={show.startYear()?.toString()}
              onClick={() => history.push(`/shows/${show.id}`)}
            />
          </div>
        ))}
      </div>
    </div>
  );
}
