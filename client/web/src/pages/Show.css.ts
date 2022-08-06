import { style } from "@vanilla-extract/css";

export const seriesSection = style({
  marginTop: 48,
});

export const seriesGrid = style({
  display: "grid",
  gridTemplateColumns: "repeat(auto-fill, 160px)",
  gridGap: 32,
});

export const seriesItemWrapper = style({
  marginBottom: 16,
});
