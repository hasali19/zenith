import React from "react";
import { css } from "@emotion/react";
import { Card, CardActionArea, CardMedia, Typography } from "@material-ui/core";

interface Props {
  poster?: string;
  primary?: string;
  secondary?: string;
  style?: React.CSSProperties;
  onClick?: () => void;
}

export default function PosterMediaItem({
  poster,
  primary,
  secondary,
  style,
  onClick,
}: Props) {
  return (
    <div style={style}>
      <Card>
        <CardActionArea onClick={onClick}>
          <CardMedia
            image={poster}
            css={css`
              aspect-ratio: 2/3;
            `}
          />
        </CardActionArea>
      </Card>
      <div
        css={(theme) => css`
          padding-top: ${theme.spacing(1)};
          padding-bottom: ${theme.spacing(2)};
        `}
      >
        <Typography
          variant="subtitle2"
          css={css`
            text-overflow: ellipsis;
            overflow: hidden;
            white-space: nowrap;
          `}
        >
          {primary}
        </Typography>
        <Typography variant="caption">{secondary}</Typography>
      </div>
    </div>
  );
}
