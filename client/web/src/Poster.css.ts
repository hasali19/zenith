import { style } from "@vanilla-extract/css";

export const root = style({
  position: "relative",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  userSelect: "none",
  transition: "transform 100ms",
  background: "rgba(100,100,100)",
  borderRadius: 8,
  aspectRatio: "2/3",
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
