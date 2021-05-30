import { css } from "@emotion/react";
import { Card, Typography } from "@material-ui/core";

import { VideoInfo } from "../api";

export interface Props {
  info: VideoInfo;
}

export default function MediaInfo({ info }: Props) {
  return (
    <div>
      <Typography variant="h6">Media Info</Typography>
      <div
        css={(theme) =>
          css`
            margin: ${theme.spacing(1)} 0;
          `
        }
      >
        <Typography>File</Typography>
        <Typography variant="body2">
          <strong>Path:</strong> {info.path}
        </Typography>
        <Typography variant="body2">
          <strong>Format:</strong> {info.format}
        </Typography>
      </div>
      <div
        css={(theme) =>
          css`
            margin: ${theme.spacing(1)} 0;
          `
        }
      >
        <Typography>Video</Typography>
        <Typography variant="body2">
          <strong>Codec:</strong> {info.video.codec}
        </Typography>
        <Typography variant="body2">
          <strong>Profile:</strong> {info.video.profile}
        </Typography>
        <Typography variant="body2">
          <strong>Resolution:</strong> {info.video.width}x{info.video.height}
        </Typography>
      </div>
      <div
        css={(theme) =>
          css`
            margin: ${theme.spacing(1)} 0;
          `
        }
      >
        <Typography>Audio</Typography>
        <Typography variant="body2">
          <strong>Codec:</strong> {info.audio.codec}
        </Typography>
      </div>
    </div>
  );
}
