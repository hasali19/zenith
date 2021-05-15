import { css } from "@emotion/react";
import { MenuItem, Select, Typography } from "@material-ui/core";

export interface Props {
  subtitles: {
    index: number;
    title: string | null;
    language: string | null;
  }[];
  value: "none" | number;
  onChange: (value: "none" | number) => void;
}

export default function SubtitleSelect({ subtitles, value, onChange }: Props) {
  return (
    <div
      css={css`
        display: flex;
        align-items: center;
      `}
    >
      <Typography
        variant="subtitle2"
        css={(theme) =>
          css`
            margin-right: ${theme.spacing(2)};
            color: grey;
          `
        }
      >
        Subtitles
      </Typography>
      <Select
        variant="standard"
        style={{ width: "100%", maxWidth: 200 }}
        value={value}
        onChange={(e) => onChange(e.target.value)}
      >
        <MenuItem value="none">None</MenuItem>
        {subtitles.map((subtitle) => (
          <MenuItem key={subtitle.index} value={subtitle.index}>
            {subtitle.title || subtitle.language}
          </MenuItem>
        ))}
      </Select>
    </div>
  );
}
