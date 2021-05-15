import { useEffect, useState } from "react";
import { useHistory, useParams } from "react-router";
import { css, Theme } from "@emotion/react";
import { Button, Icon, LinearProgress, Typography } from "@material-ui/core";

import api, { TvEpisode, VideoInfo } from "../api";
import MediaInfo from "../components/MediaInfo";
import SubtitleSelect from "../components/SubtitleSelect";

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
  const [episode, setEpisode] = useState<TvEpisode | null>(null);
  const [video, setVideo] = useState<VideoInfo | null>(null);
  const [subtitle, setSubtitle] = useState<"none" | number>("none");

  useEffect(() => {
    api.tv.getEpisode(params.id).then(setEpisode);
    api.videos.getVideoInfo(params.id).then(setVideo);
  }, []);

  if (!episode || !video) {
    return <LinearProgress variant="indeterminate" />;
  }

  function onPlay() {
    if (episode) {
      let url = `/player/${episode.id}`;

      if (subtitle !== "none") {
        url += `?subtitle=${subtitle}`;
      }

      history.push(url);
    }
  }

  return (
    <div css={styles.root}>
      <img src={episode.thumbnail!!} css={styles.backdrop} />
      <Typography variant="h4" css={styles.title}>
        {episode.name}
      </Typography>
      <Typography variant="caption" component="div" css={styles.subtitle}>
        E{episode.episode_number.toString().padStart(2, "0")} -{" "}
        {displayDuration(episode.duration)}
      </Typography>
      <Button
        variant="contained"
        startIcon={<Icon>play_arrow</Icon>}
        css={styles.play}
        onClick={onPlay}
      >
        Play
      </Button>
      <div
        css={(theme) => css`
          margin: 0 ${theme.spacing(2)};
          margin-bottom: ${theme.spacing(2)};
        `}
      >
        <SubtitleSelect
          subtitles={video.subtitles}
          value={subtitle}
          onChange={setSubtitle}
        />
      </div>
      <Typography variant="body2" css={styles.overview}>
        {episode.overview}
      </Typography>
      <MediaInfo info={video} />
    </div>
  );
}
