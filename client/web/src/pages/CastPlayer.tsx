import { FC, useCallback, useEffect, useState } from "react";
import { useLocation, useParams } from "react-router";
import { css } from "@emotion/react";
import { Fab, Icon, IconButton } from "@material-ui/core";

import { useGCast } from "../gcast";
import api, { VideoInfo } from "../api";

export default function CastPlayer() {
  const params = useParams<any>();
  const query = new URLSearchParams(useLocation().search);
  const gcast = useGCast();
  const [info, setInfo] = useState<VideoInfo | null>(null);

  const subtitles = info?.subtitles || [];
  const requestedSubtitle = parseInt(query.get("subtitle") || "-1");

  useEffect(() => {
    if (params.id) {
      api.videos.getVideoInfo(params.id).then(setInfo);
    }
  }, [params]);

  const onStartCast = useCallback(() => {
    gcast.connect().then((session) => {
      const url = origin + api.videos.getVideoUrl(params.id);
      const info = new window.chrome.cast.media.MediaInfo(url, "video/mp4");

      if (requestedSubtitle !== -1) {
        const type = window.chrome.cast.media.TrackType.TEXT;
        const subtitle = new window.chrome.cast.media.Track(1, type);
        const item = subtitles.find((s) => s.index === requestedSubtitle);
        if (!item) {
          throw new Error("invalid subtitle track index");
        }
        subtitle.trackContentId =
          origin + `/api/videos/${params.id}/subtitles/${requestedSubtitle}`;
        subtitle.trackContentType = "text/vtt";
        subtitle.subtype = window.chrome.cast.media.TextTrackType.SUBTITLES;
        subtitle.name = item.title || item.language || "";
        subtitle.language = item.language || "";
        info.tracks = [subtitle];
      }

      const request = new window.chrome.cast.media.LoadRequest(info);

      session.loadMedia(request).then(() => {
        if (requestedSubtitle !== -1) {
          const media = session.getMediaSession();
          if (media) {
            media.editTracksInfo(
              new window.chrome.cast.media.EditTracksInfoRequest([1]),
              () => undefined,
              (e) => console.error(e)
            );
          }
        }
      });
    });
  }, [params, requestedSubtitle]);

  const onStopCast = useCallback(() => {
    const context = window.cast.framework.CastContext.getInstance();
    const session = context.getCurrentSession();
    if (session) {
      session.endSession(true);
    }
  }, []);

  const onPlayPause = useCallback(() => {
    const player = new cast.framework.RemotePlayer();
    const controller = new cast.framework.RemotePlayerController(player);
    controller.playOrPause();
  }, []);

  return (
    <div
      css={css`
        width: 100%;
        height: 100%;
      `}
    >
      <div
        css={css`
          width: 100%;
          height: 100%;
          display: flex;
          align-items: center;
          justify-content: center;
        `}
      >
        <CastButton
          connected={gcast.connected}
          onStart={onStartCast}
          onStop={onStopCast}
        />
      </div>
      {gcast.connected && (
        <Controls paused={gcast.paused} onPlayPause={onPlayPause} />
      )}
    </div>
  );
}

const CastButton: FC<{
  connected: boolean;
  onStart: () => void;
  onStop: () => void;
}> = ({ connected, onStart, onStop }) => (
  <IconButton
    onClick={() => {
      if (connected) {
        onStop();
      } else {
        onStart();
      }
    }}
    css={(theme) => css`
      padding: ${theme.spacing(4)};
    `}
  >
    <CastIcon connected={connected} />
  </IconButton>
);

const CastIcon: FC<{ connected: boolean }> = ({ connected }) =>
  connected ? (
    <Icon
      css={css`
        color: #2b5cff;
        font-size: 128px;
      `}
    >
      cast
    </Icon>
  ) : (
    <Icon
      css={css`
        font-size: 128px;
      `}
    >
      cast
    </Icon>
  );

const Controls: FC<{ paused: boolean; onPlayPause: () => void }> = ({
  paused,
  onPlayPause,
}) => (
  <div
    css={(theme) => css`
      position: absolute;
      width: 100%;
      bottom: 0;
      padding: ${theme.spacing(4)};
      display: flex;
      justify-content: center;
    `}
  >
    <Fab size="large" onClick={onPlayPause}>
      {paused ? <Icon>play_arrow</Icon> : <Icon>pause</Icon>}
    </Fab>
  </div>
);
