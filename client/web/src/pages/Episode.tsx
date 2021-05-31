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

import api, { TvEpisode, TvSeason, VideoInfo } from "../api";
import AppBar from "../AppBar";
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

export default function () {
  const params = useParams<any>();
  const history = useHistory();
  const mobile = useMediaQuery((theme: Theme) => theme.breakpoints.down("md"));

  const [season, setSeason] = useState<TvSeason | null>(null);
  const [episode, setEpisode] = useState<TvEpisode | null>(null);
  const [video, setVideo] = useState<VideoInfo | null>(null);

  useEffect(() => {
    api.tv.getEpisode(params.id).then((episode) => {
      setEpisode(episode);
      api.tv.getSeason(episode.season_id).then(setSeason);
    });

    api.videos.getVideoInfo(params.id).then(setVideo);
  }, []);

  if (!season || !episode || !video) {
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
        backgroundImage: mobile ? undefined : `url(${episode.thumbnail})`,
      }}
    >
      <div
        css={(theme) => css`
          height: 100%;
          overflow-y: auto;

          ${theme.breakpoints.up("md")} {
            padding: 5%;
            background-color: #000a;
          }
        `}
      >
        <AppBar translucent />
        {mobile && episode.thumbnail && (
          <MobileBackdrop src={episode.thumbnail} />
        )}
        <HeaderSection
          season={season}
          episode={episode}
          mobile={mobile}
          onPlay={(player) => history.push(`/${player}/${episode.id}`)}
        />
        {mobile && (
          <Typography
            variant="body2"
            css={(theme) => css`
              margin: 0 ${theme.spacing(2)};
            `}
          >
            {episode.overview}
          </Typography>
        )}
        <div
          css={(theme) =>
            css`
              margin-top: ${theme.spacing(2)};

              ${theme.breakpoints.up("md")} {
                margin-top: ${theme.spacing(4)};
              }

              ${theme.breakpoints.down("md")} {
                padding: 0 ${theme.spacing(2)};
              }
            `
          }
        >
          <MediaInfo info={video} />
        </div>
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
  season: TvSeason;
  episode: TvEpisode;
  mobile: boolean;
  onPlay: (player: "player" | "cast") => void;
}> = ({ season, episode, mobile, onPlay }) => (
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
          min-width: 240px;
        }
      `}
    >
      <CardMedia
        image={season.poster!!}
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
        {episode.name}
      </Typography>
      <Typography variant={mobile ? "caption" : "h6"} component="div">
        S{season.season_number.toString().padStart(2, "0")}E
        {episode.episode_number.toString().padStart(2, "0")} -{" "}
        {displayDuration(episode.duration)}
      </Typography>
      <ActionsRow onPlay={onPlay} />
      {!mobile && (
        <Typography
          variant="body2"
          css={(theme) => css`
            margin-top: ${theme.spacing(2)};
          `}
        >
          {episode.overview}
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
