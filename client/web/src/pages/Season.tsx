import { useEffect, useState } from "react";
import { useHistory, useParams } from "react-router";
import { css, Theme } from "@emotion/react";
import {
  Card,
  CardActionArea,
  CardMedia,
  Icon,
  LinearProgress,
  Typography,
} from "@material-ui/core";

import api, { Episode, Season, Show } from "../api";

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
      margin: ${theme.spacing(2)};
    `,

  episodes: (theme: Theme) => css`
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    padding: ${theme.spacing(1.5)};

    @media (min-width: 768px) {
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    }
  `,

  episode: (theme: Theme) => css`
    padding: ${theme.spacing(0.5)};
  `,
};

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
  const [show, setShow] = useState<Show | null>(null);
  const [season, setSeason] = useState<Season | null>(null);
  const [episodes, setEpisodes] = useState<Episode[]>([]);

  useEffect(() => {
    api.tv.getSeason(params.id).then((season) => {
      setSeason(season);
      api.tv.getShow(season.show_id).then(setShow);
    });

    api.tv.getEpisodes(params.id).then(setEpisodes);
  }, []);

  if (!show || !season) {
    return <LinearProgress variant="indeterminate" />;
  }

  return (
    <div css={styles.root}>
      <Typography variant="h4" css={styles.title}>
        {season.name}
      </Typography>
      <Typography variant="body2" css={styles.overview}>
        {season.overview || show.overview}
      </Typography>
      <div css={styles.episodes}>
        {episodes.map((episode) => (
          <div key={episode.id} css={styles.episode}>
            <Card>
              <CardActionArea
                onClick={() => history.push(`/episodes/${episode.id}`)}
              >
                <CardMedia
                  image={episode.thumbnail || undefined}
                  css={css`
                    aspect-ratio: 16/9;
                  `}
                />
                {episode.user_data.is_watched && (
                  <div
                    css={css`
                      width: 100%;
                      height: 100%;
                      position: absolute;
                      top: 0;
                      display: flex;
                      align-items: center;
                      justify-content: center;
                      background-color: rgba(0, 0, 0, 0.5);
                    `}
                  >
                    <Icon>check</Icon>
                  </div>
                )}
              </CardActionArea>
            </Card>
            <div
              css={(theme) => css`
                padding: ${theme.spacing(2)} 0;
              `}
            >
              <Typography
                variant="subtitle2"
                css={css`
                  text-overflow: ellipsis;
                  overflow: hidden;
                  white-space: nowrap;
                  line-height: 1em;
                `}
              >
                {episode.episode_number} - {episode.name}
              </Typography>
              <Typography
                variant="caption"
                component="div"
                css={css`
                  color: darkgray;
                `}
              >
                {displayDuration(episode.video_info.duration)}
              </Typography>
              <Typography
                variant="caption"
                component="div"
                css={css`
                  line-height: 1.2em;
                  display: -webkit-box;
                  -webkit-line-clamp: 3;
                  -webkit-box-orient: vertical;
                  overflow: hidden;
                `}
              >
                {episode.overview}
              </Typography>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
