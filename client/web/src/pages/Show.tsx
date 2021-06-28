import { FC, useEffect, useState } from "react";
import { useHistory, useParams } from "react-router";
import { css } from "@emotion/react";
import {
  Box,
  Card,
  CardMedia,
  LinearProgress,
  Typography,
} from "@material-ui/core";

import api, { TvSeason, TvShow } from "../api";
import PosterMediaItem from "../components/PosterMediaItem";

export default function () {
  const params = useParams<any>();
  const history = useHistory();
  const [show, setShow] = useState<TvShow | null>(null);
  const [seasons, setSeasons] = useState<TvSeason[]>([]);

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
        backgroundImage: `url(${show.backdrop})`,
      }}
    >
      <div
        css={css`
          height: 100%;
          overflow-y: auto;
          padding: 5%;
          background-color: #000a;
        `}
      >
        <HeaderSection show={show} />
        <SeasonsSection
          show={show}
          seasons={seasons}
          onItemClick={(season) => history.push(`/seasons/${season.id}`)}
        />
      </div>
    </div>
  );
}

const HeaderSection: FC<{ show: TvShow }> = ({ show }) => {
  return (
    <div
      css={css`
        display: flex;
        align-items: center;
      `}
    >
      <Poster src={show.poster!!} />
      <HeaderContent show={show} />
    </div>
  );
};

const Poster: FC<{ src: string }> = ({ src }) => (
  <Card sx={{ minWidth: 240 }}>
    <CardMedia image={src} sx={{ aspectRatio: "2/3" }} />
  </Card>
);

const HeaderContent: FC<{ show: TvShow }> = ({ show }) => (
  <Box ml={4}>
    <Typography variant="h2">{show.name}</Typography>
    <Typography variant="h5">{show.startYear()}</Typography>
    <Typography variant="body2" sx={{ mt: 2 }}>
      {show.overview}
    </Typography>
  </Box>
);

const SeasonsSection: FC<{
  show: TvShow;
  seasons: TvSeason[];
  onItemClick: (item: TvSeason) => void;
}> = ({ show, seasons, onItemClick }) => (
  <Box my={2} mt={8}>
    <Typography variant="h6" sx={{ mx: 0.5, fontSize: "1.5em" }}>
      Seasons
    </Typography>
    <Box
      sx={{
        mt: 2,
        display: "grid",
        gridTemplateColumns: "repeat(auto-fill, minmax(150px, 1fr))",
        gridGap: (theme) => theme.spacing(2),
      }}
    >
      {seasons.map((season) => (
        <div key={season.id}>
          <PosterMediaItem
            poster={season.poster || undefined}
            primary={season.name || ""}
            secondary={show.name}
            onClick={() => onItemClick(season)}
          />
        </div>
      ))}
    </Box>
  </Box>
);
