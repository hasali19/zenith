import { FC, useEffect, useState } from "react";
import { useHistory, useParams } from "react-router";
import { css, Theme } from "@emotion/react";
import {
  Card,
  CardMedia,
  LinearProgress,
  Typography,
  useMediaQuery,
} from "@material-ui/core";

import api, { TvSeason, TvShow } from "../api";
import AppBar from "../AppBar";
import PosterMediaItem from "../components/PosterMediaItem";

export default function () {
  const params = useParams<any>();
  const history = useHistory();
  const [show, setShow] = useState<TvShow | null>(null);
  const [seasons, setSeasons] = useState<TvSeason[]>([]);
  const mobile = useMediaQuery((theme: Theme) => theme.breakpoints.down("md"));

  useEffect(() => {
    api.tv.getShow(params.id).then(setShow);
    api.tv.getSeasons(params.id).then(setSeasons);
  }, []);

  if (!show) {
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
        backgroundImage: mobile ? undefined : `url(${show.backdrop})`,
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
        {mobile && show.backdrop && <MobileBackdrop src={show.backdrop} />}
        <HeaderSection show={show} mobile={mobile} />
        {mobile && (
          <Typography
            variant="body2"
            css={(theme) =>
              css`
                margin: 0 ${theme.spacing(2)};
              `
            }
          >
            {show.overview}
          </Typography>
        )}
        <SeasonsSection
          show={show}
          seasons={seasons}
          onItemClick={(season) => history.push(`/seasons/${season.id}`)}
        />
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

const HeaderSection: FC<{ show: TvShow; mobile: boolean }> = ({
  show,
  mobile,
}) => {
  return (
    <div
      css={(theme) => css`
        display: flex;
        align-items: center;

        ${theme.breakpoints.down("md")} {
          padding: ${theme.spacing(2)};
          margin-top: -80px;
        }
      `}
    >
      <Poster src={show.poster!!} />
      <HeaderContent show={show} mobile={mobile} />
    </div>
  );
};

const Poster: FC<{ src: string }> = ({ src }) => (
  <Card
    css={(theme) => css`
      min-width: 150px;

      ${theme.breakpoints.up("md")} {
        min-width: 240px;
      }
    `}
  >
    <CardMedia
      image={src}
      css={css`
        aspect-ratio: 2/3;
      `}
    />
  </Card>
);

const HeaderContent: FC<{ show: TvShow; mobile: boolean }> = ({
  show,
  mobile,
}) => (
  <div
    css={(theme) => css`
      margin-left: ${theme.spacing(2)};

      ${theme.breakpoints.up("md")} {
        margin-left: ${theme.spacing(4)};
      }
    `}
  >
    <Typography variant={mobile ? "h4" : "h2"}>{show.name}</Typography>
    <Typography variant={mobile ? "caption" : "h5"} component="div">
      {show.startYear()}
    </Typography>
    {!mobile && (
      <Typography
        variant="body2"
        css={(theme) => css`
          margin-top: ${theme.spacing(2)};
        `}
      >
        {show.overview}
      </Typography>
    )}
  </div>
);

const SeasonsSection: FC<{
  show: TvShow;
  seasons: TvSeason[];
  onItemClick: (item: TvSeason) => void;
}> = ({ show, seasons, onItemClick }) => (
  <div
    css={(theme) => css`
      margin: ${theme.spacing(2)} 0px;
      padding: 0px ${theme.spacing(1.5)};

      ${theme.breakpoints.up("md")} {
        margin-top: ${theme.spacing(8)};
        padding: 0;
      }
    `}
  >
    <Typography
      variant="h6"
      css={(theme) =>
        css`
          margin: 0 ${theme.spacing(0.5)};

          ${theme.breakpoints.up("md")} {
            font-size: 1.5em;
          }
        `
      }
    >
      Seasons
    </Typography>
    <div
      css={(theme) => css`
        display: flex;
        overflow-x: scroll;

        &::-webkit-scrollbar {
          width: 0;
          height: 0;
        }

        ${theme.breakpoints.up("md")} {
          margin-top: ${theme.spacing(2)};
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
          grid-gap: ${theme.spacing(2)};
        }
      `}
    >
      {seasons.map((season) => (
        <div
          key={season.id}
          css={(theme) => css`
            ${theme.breakpoints.down("md")} {
              min-width: 130px;
              padding: ${theme.spacing(0.5)};
            }
          `}
        >
          <PosterMediaItem
            poster={season.poster || undefined}
            primary={season.name || ""}
            secondary={show.name}
            onClick={() => onItemClick(season)}
          />
        </div>
      ))}
    </div>
  </div>
);
