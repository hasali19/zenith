import { useEffect, useRef, useState } from "react";
import { useLocation, useParams } from "react-router";
import { css } from "@material-ui/styled-engine";

import api, { ItemId, VideoInfo } from "../api";

function reportVideoPosition(id: ItemId, video: HTMLVideoElement) {
  if (!video.paused && !video.ended) {
    api.progress.update(id, Math.floor(video.currentTime));
  }
}

function loadVideo(id: ItemId, info: VideoInfo, video: HTMLVideoElement) {
  video.src = api.videos.getVideoUrl(id);
  video.currentTime = info.position || 0;
  video.play();
}

export default function Player() {
  const params = useParams<any>();
  const query = new URLSearchParams(useLocation().search);
  const video = useRef<HTMLVideoElement>(null);
  const [info, setInfo] = useState<VideoInfo | null>(null);

  const requestedSubtitle = parseInt(query.get("subtitle") || "-1");

  useEffect(() => {
    if (params.id) {
      api.videos.getVideoInfo(params.id).then(setInfo);

      const interval = setInterval(() => {
        if (video.current) {
          reportVideoPosition(params.id, video.current);
        }
      }, 2000);

      return () => {
        clearInterval(interval);
      };
    }
  }, [params]);

  useEffect(() => {
    if (params.id && info && video.current) {
      loadVideo(params.id, info, video.current);
    }
  }, [params, info]);

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
          info &&
          info.subtitles.map((subtitle) => (
            <track
              key={subtitle.index}
              src={`/api/videos/${params.id}/subtitles/${subtitle.index}`}
              kind="subtitles"
              label={subtitle.title || subtitle.language || undefined}
              srcLang={subtitle.language || undefined}
              default={requestedSubtitle === subtitle.index}
            />
          ))}
      </video>
    </div>
  );
}
