import { FC, useEffect, useState } from "react";
import { useHistory, useParams } from "react-router";
import { css, Theme } from "@emotion/react";
import {
  Button,
  Card,
  CardMedia,
  Icon,
  LinearProgress,
  Typography,
  useMediaQuery,
} from "@material-ui/core";

import api, { Movie, VideoInfo } from "../api";

function displayDuration(duration: number) {
  if (duration <= 90 * 60) {
    return `${Math.floor(duration / 60)}m`;
  } else {
    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor((duration % 3600) / 60);
    return `${hours}h ${minutes}m`;
  }
}

export default function () {
  const params = useParams<any>();
  const history = useHistory();
  const mobile = useMediaQuery((theme: Theme) => theme.breakpoints.down("md"));

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
    <div
      css={css`
        height: 100%;
        overflow-y: auto;
        background-size: cover;
        background-position: center;
      `}
      style={{
        backgroundImage: mobile ? undefined : `url(${movie.backdrop})`,
      }}
    >
      <div
        css={(theme) => css`
          height: 100%;
          overflow-y: auto;

          ${theme.breakpoints.up("md")} {
            padding: 10%;
            background-color: #000a;
          }
        `}
      >
        {mobile && movie.backdrop && <MobileBackdrop src={movie.backdrop} />}
        <HeaderSection
          movie={movie}
          mobile={mobile}
          onPlay={(player) => history.push(`/${player}/${movie.id}`)}
        />
        {mobile && (
          <Typography
            variant="body2"
            css={(theme) => css`
              margin: 0 ${theme.spacing(2)};
            `}
          >
            {movie.overview}
          </Typography>
        )}
      </div>
    </div>
  );
}

const MobileBackdrop: FC<{ src: string }> = ({ src }) => (
  <img
    src={src}
    css={css`
      width: 100%;
      aspect-ratio: 16 / 9;
    `}
  />
);

const HeaderSection: FC<{
  movie: Movie;
  mobile: boolean;
  onPlay: (player: "player" | "cast") => void;
}> = ({ movie, mobile, onPlay }) => (
  <div
    css={(theme) => css`
      display: flex;
      align-items: center;

      ${theme.breakpoints.down("md")} {
        align-items: flex-end;
        padding: ${theme.spacing(2)};
        margin-top: -80px;
      }
    `}
  >
    <Card
      css={(theme) => css`
        min-width: 150px;

        ${theme.breakpoints.up("md")} {
          min-width: 280px;
        }
      `}
    >
      <CardMedia
        image={movie.poster!!}
        css={css`
          aspect-ratio: 2/3;
        `}
      />
    </Card>
    <div
      css={(theme) => css`
        margin-left: ${theme.spacing(2)};

        ${theme.breakpoints.up("md")} {
          margin-left: ${theme.spacing(4)};
        }
      `}
    >
      <Typography
        variant={mobile ? "h5" : "h3"}
        css={(theme) => css`
          ${theme.breakpoints.down("md")} {
            font-size: 1.2rem;
          }
        `}
      >
        {movie.title}
      </Typography>
      <Typography variant={mobile ? "caption" : "h6"} component="div">
        {displayDuration(movie.duration)}
      </Typography>
      <ActionsRow onPlay={onPlay} />
      {!mobile && (
        <Typography
          variant="body2"
          css={(theme) => css`
            margin-top: ${theme.spacing(2)};
          `}
        >
          {movie.overview}
        </Typography>
      )}
    </div>
  </div>
);

const ActionsRow: FC<{ onPlay: (player: "player" | "cast") => void }> = ({
  onPlay,
}) => {
  return (
    <div
      css={(theme) =>
        css`
          margin: ${theme.spacing(2)} 0;
        `
      }
    >
      <Button
        variant="contained"
        startIcon={<Icon>play_arrow</Icon>}
        onClick={() => onPlay("player")}
        css={(theme) => css`
          margin-right: ${theme.spacing(1)};
        `}
      >
        Play
      </Button>
      <Button variant="outlined" onClick={() => onPlay("cast")}>
        <Icon color="action">cast</Icon>
      </Button>
    </div>
  );
};
