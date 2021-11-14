import { style } from "@vanilla-extract/css";

export const seriesSection = style({
  marginTop: 48,
});

export const seriesHeading = style(["title", "is-5"]);

export const seriesGrid = style({
  display: "grid",
  gridTemplateColumns: "repeat(auto-fill, minmax(360px, 1fr))",
  gridGap: 32,
});
