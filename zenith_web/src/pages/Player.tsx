import { useEffect, useRef } from "react";
import { useParams } from "react-router";
import { css } from "@material-ui/styled-engine";

import api, { ItemId } from "../api";

function reportVideoPosition(id: ItemId, video: HTMLVideoElement) {
  api.progress.update(id, Math.floor(video.currentTime));
}

export default function Player() {
  const params = useParams<any>();
  const video = useRef<HTMLVideoElement>(null);

  useEffect(() => {
    const interval = setInterval(() => {
      if (video.current) {
        reportVideoPosition(params.id, video.current);
      }
    }, 2000);

    return () => {
      clearInterval(interval);
    };
  }, [params]);

  return (
    <div
      css={css`
        width: 100%;
        height: 100%;
      `}
    >
      <video
        ref={video}
        src={api.videos.getVideoUrl(params.id)}
        controls
        autoPlay
        css={css`
          width: 100%;
          height: 100%;
        `}
      />
    </div>
  );
}
