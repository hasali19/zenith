import { useEffect, useState } from "react";
import { useHistory, useParams } from "react-router";
import { css, Theme } from "@emotion/react";
import { Button, Icon, LinearProgress, Typography } from "@material-ui/core";

import api, { Movie, VideoInfo } from "../api";
import MediaInfo from "../components/MediaInfo";

function displayDuration(duration: number) {
  if (duration <= 90 * 60) {
    return `${Math.floor(duration / 60)}m`;
  } else {
    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor((duration % 3600) / 60);
    return `${hours}h ${minutes}m`;
  }
}

const styles = {
  root: css`
    height: 100%;
    overflow-y: auto;
  `,

  backdrop: css`
    width: 100%;
    aspect-ratio: 16 / 9;
  `,

  title: (theme: Theme) => css`
    margin: 0 ${theme.spacing(2)};
    margin-top: ${theme.spacing(3)};
  `,

  subtitle: (theme: Theme) => css`
    margin: 0 ${theme.spacing(2)};
  `,

  play: (theme: Theme) => css`
    margin: ${theme.spacing(2)};
  `,

  overview: (theme: Theme) =>
    css`
      margin: 0 ${theme.spacing(2)};
    `,
};

export default function () {
  const params = useParams<any>();
  const history = useHistory();
  const [movie, setMovie] = useState<Movie | null>(null);
  const [video, setVideo] = useState<VideoInfo | null>(null);

  useEffect(() => {
    api.movies.getMovie(params.id).then(setMovie);
    api.videos.getVideoInfo(params.id).then(setVideo);
  }, []);

  if (!movie || !video) {
    return <LinearProgress variant="indeterminate" />;
  }

  return (
    <div css={styles.root}>
      <img src={movie.backdrop!!} css={styles.backdrop} />
      <Typography variant="h4" css={styles.title}>
        {movie.title}
      </Typography>
      <Typography variant="caption" component="div" css={styles.subtitle}>
        {movie.releaseYear()} • {displayDuration(movie.duration)}
      </Typography>
      <Button
        variant="contained"
        startIcon={<Icon>play_arrow</Icon>}
        css={styles.play}
        onClick={() => history.push(`/player/${movie.id}`)}
      >
        Play
      </Button>
      <Typography variant="body2" css={styles.overview}>
        {movie.overview}
      </Typography>
      <MediaInfo info={video} />
    </div>
  );
}
