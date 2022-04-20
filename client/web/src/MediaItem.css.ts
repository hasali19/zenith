import { style } from "@vanilla-extract/css";

export const item = style({
  cursor: "pointer",
  userSelect: "none",
  transition: "transform 100ms",

  ":hover": {
    transform: "scale(0.98)",
  },

  ":active": {
    transform: "scale(0.95)",
  },
});

const imageWrapper = style({
  width: "100%",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  borderRadius: 8,
});

export const poster = style([
  imageWrapper,
  {
    position: "relative",
    aspectRatio: "2/3",
  },
]);

export const thumbnail = style([
  imageWrapper,
  {
    position: "relative",
    aspectRatio: "16/9",
  },
]);

export const image = style({
  width: "100%",
  height: "100%",
  objectFit: "cover",
  display: "block",
  borderRadius: 8,
});

export const details = style({
  paddingTop: 16,
  paddingBottom: 16,
});

export const overlay = style({
  position: "absolute",
  inset: 0,
  background: "rgba(0,0,0,0.3)",
  borderRadius: 8,
});

export const posterCheck = style({
  position: "absolute",
  top: 0,
  right: 0,
  margin: 16,
});

export const name = style({
  margin: 0,
  overflow: "hidden",
  textOverflow: "ellipsis",
  whiteSpace: "nowrap",
  fontSize: "1.1rem",
  fontWeight: "bold",
});

export const secondary = style({
  margin: 0,
  overflow: "hidden",
  textOverflow: "ellipsis",
  whiteSpace: "nowrap",
  fontSize: "0.9rem",
});
