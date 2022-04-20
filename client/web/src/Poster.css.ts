import { style } from "@vanilla-extract/css";

export const root = style({
  position: "relative",
  userSelect: "none",
  transition: "transform 100ms",
});

export const clickable = style({
  cursor: "pointer",

  ":hover": {
    transform: "scale(0.98)",
  },

  ":active": {
    transform: "scale(0.95)",
  },
});

export const image = style({
  width: "100%",
  height: "100%",
  objectFit: "cover",
  display: "block",
  borderRadius: 8,
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
