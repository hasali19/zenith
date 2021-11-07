import { useEffect, useRef, useState } from "react";
import { useLocation, useParams } from "react-router";
import { css } from "@material-ui/styled-engine";

import api, { ItemId, MediaItem, Subtitle, VideoInfo } from "../api";

function reportVideoPosition(id: ItemId, video: HTMLVideoElement) {
  if (!video.paused && !video.ended) {
    api.progress.update(id, Math.floor(video.currentTime));
  }
}

function loadVideo(
  id: ItemId,
  position: number | null,
  video: HTMLVideoElement
) {
  video.src = api.videos.getUrl(id);
  video.currentTime = position || 0;
  video.play();
}

export default function Player() {
  const params = useParams<any>();
  const query = new URLSearchParams(useLocation().search);
  const video = useRef<HTMLVideoElement>(null);

  const [item, setItem] = useState<MediaItem | null>(null);

  const requestedSubtitle = parseInt(query.get("subtitle") || "-1");

  useEffect(() => {
    if (params.id) {
      api.items.getItem(params.id).then(setItem);

      const interval = setInterval(() => {
        if (video.current) {
          reportVideoPosition(params.id, video.current);
        }
      }, 5000);

      return () => {
        clearInterval(interval);
      };
    }
  }, [params]);

  useEffect(() => {
    if (
      params.id &&
      item &&
      (item.type == "movie" || item.type == "episode") &&
      video.current
    ) {
      loadVideo(params.id, item.user_data.position, video.current);
    }
  }, [params, item]);

  return (
    <div
      css={css`
        width: 100%;
        height: 100%;
      `}
    >
      <video
        ref={video}
        controls
        autoPlay
        css={css`
          width: 100%;
          height: 100%;
        `}
      >
        {params.id &&
          item &&
          (item.type == "movie" || item.type == "episode") &&
          item.video_info.subtitles.map((subtitle) => (
            <track
              key={subtitle.id}
              src={`/api/subtitles/${subtitle.id}`}
              kind="subtitles"
              label={buildSubtitleLabel(subtitle)}
              srcLang={subtitle.language ?? undefined}
              default={requestedSubtitle === subtitle.id}
            />
          ))}
      </video>
    </div>
  );
}

function buildSubtitleLabel(subtitle: Subtitle): string {
  const language = subtitle.language ?? "Unknown";

  if (subtitle.title) {
    return `${language} - ${subtitle.title}`;
  } else {
    return language;
  }
}
