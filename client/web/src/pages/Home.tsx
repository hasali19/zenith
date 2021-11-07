import React, { useEffect, useState } from "react";
import { useHistory } from "react-router";
import { css, Theme } from "@emotion/react";
import { Typography } from "@material-ui/core";

import api, { Movie, Show } from "../api";
import PosterMediaItem from "../components/PosterMediaItem";
import { displayYear } from "../utils";

const styles = {
  root: (theme: Theme) => css`
    height: 100%;
    overflow-y: auto;
    padding: ${theme.spacing(4)};
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

    ${theme.breakpoints.up("md")} {
      min-width: 172px;
    }
  `,
};

interface FeaturedSectionProps<T> {
  title: string;
  items: T[];
  render: (item: T) => React.ReactNode;
}

function FeaturedSection<T>({ title, items, render }: FeaturedSectionProps<T>) {
  if (items.length == 0) {
    return null;
  }

  return (
    <React.Fragment>
      <Typography variant="h5" sx={{ mx: 2 }}>
        {title}
      </Typography>
      <div css={styles.slider}>{items.map(render)}</div>
    </React.Fragment>
  );
}

export default function () {
  const history = useHistory();
  const [movies, setMovies] = useState<Movie[]>([]);
  const [shows, setShows] = useState<Show[]>([]);

  useEffect(() => {
    api.movies.getRecent().then(setMovies);
    api.tv.getRecentShows().then(setShows);
  }, []);

  return (
    <div css={styles.root}>
      <FeaturedSection
        title="New Movies"
        items={movies}
        render={(movie) => (
          <div key={movie.id} css={styles.item}>
            <PosterMediaItem
              poster={movie.poster || undefined}
              primary={movie.title}
              secondary={displayYear(movie.release_date)}
              onClick={() => history.push(`/movies/${movie.id}`)}
            />
          </div>
        )}
      />
      <FeaturedSection
        title="Updated Shows"
        items={shows}
        render={(show) => (
          <div key={show.id} css={styles.item}>
            <PosterMediaItem
              poster={show.poster || undefined}
              primary={show.name}
              secondary={displayYear(show.start_date)}
              onClick={() => history.push(`/shows/${show.id}`)}
            />
          </div>
        )}
      />
    </div>
  );
}
