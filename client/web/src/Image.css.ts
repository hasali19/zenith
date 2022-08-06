import { style } from "@vanilla-extract/css";

export const img = style({
  opacity: 0,
  transition: "opacity 300ms",
});

export const loaded = style({
  opacity: 1,
});
