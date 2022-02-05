import { style } from "@vanilla-extract/css";

export const container = style({
  position: "relative",
  width: "100%",
  height: 32,
});

export const progress = style({
  appearance: "none",
  width: "100%",
  height: 4,
  position: "absolute",
  inset: "14px 0px",

  "::-webkit-progress-bar": {
    background: "rgba(255,255,255,0.4)",
    borderRadius: 2,
  },

  "::-webkit-progress-value": {
    background: "orange",
    borderRadius: 2,
  },
});

export const range = style({
  appearance: "none",
  background: "transparent",
  position: "absolute",
  inset: 0,
  width: "calc(100% + 24px)",
  height: 32,
  touchAction: "none",
  margin: "0px -12px",
  cursor: "pointer",

  "::-webkit-slider-thumb": {
    appearance: "none",
    width: 24,
    height: 24,
    background: "orange",
    border: "6px solid rgb(40,40,40)",
    borderRadius: "50%",
  },

  selectors: {
    [`${container}:hover &::-webkit-slider-thumb`]: {
      borderWidth: 7,
    },
  },
});
