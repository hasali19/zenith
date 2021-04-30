import { useEffect, useState } from "react";
import { useHistory, useParams } from "react-router";
import { css, Theme } from "@emotion/react";
import { LinearProgress, Typography } from "@material-ui/core";

import api, { TvSeason, TvShow } from "../api";
import PosterMediaItem from "../components/PosterMediaItem";

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

  overview: (theme: Theme) =>
    css`
      margin: 0 ${theme.spacing(2)};
    `,

  seasons: (theme: Theme) => css`
    display: flex;
    overflow-x: scroll;
    margin: ${theme.spacing(2)} 0px;
    padding: 0px ${theme.spacing(1.5)};

    &::-webkit-scrollbar {
      width: 0;
      height: 0;
    }
  `,

  season: (theme: Theme) => css`
    min-width: 130px;
    padding: ${theme.spacing(0.5)};
  `,
};

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
    <div css={styles.root}>
      <img src={show.backdrop!!} css={styles.backdrop} />
      <Typography variant="h4" css={styles.title}>
        {show.name}
      </Typography>
      <Typography variant="caption" component="div" css={styles.subtitle}>
        {show.startYear()}
      </Typography>
      <Typography variant="body2" css={styles.overview}>
        {show.overview}
      </Typography>
      <div css={styles.seasons}>
        {seasons.map((season) => (
          <div key={season.id} css={styles.season}>
            <PosterMediaItem
              poster={season.poster || undefined}
              primary={season.name || ""}
              secondary={show.name}
              onClick={() => history.push(`/seasons/${season.id}`)}
            />
          </div>
        ))}
      </div>
    </div>
  );
}
