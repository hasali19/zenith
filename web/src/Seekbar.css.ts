import { style } from "@vanilla-extract/css";

export const container = style({
  position: "relative",
  width: "100%",
  height: 32,
});

export const progress = style({
  appearance: "none",
  width: "100%",
  height: "var(--height)",
  position: "absolute",
  top: "calc((100% - var(--height)) / 2)",
  bottom: "calc((100% - var(--height)) / 2)",
  transition: "all 200ms",

  vars: {
    "--height": "6px",
  },

  "::-webkit-progress-bar": {
    background: "rgba(255,255,255,0.4)",
    borderRadius: "calc(var(--height) / 2)",
  },

  "::-webkit-progress-value": {
    background: "orange",
    borderRadius: "calc(var(--height) / 2)",
  },

  selectors: {
    [`${container}:hover &`]: {
      vars: {
        "--height": "8px",
      },
    },
  },
});

export const range = style({
  appearance: "none",
  background: "transparent",
  position: "absolute",
  inset: 0,
  width: "calc(100% + var(--thumb-size))",
  height: 32,
  touchAction: "none",
  margin: "0px calc(-1 * var(--thumb-size) / 2)",
  cursor: "pointer",

  vars: {
    "--thumb-size": "16px",
  },

  "::-webkit-slider-thumb": {
    appearance: "none",
    width: "var(--thumb-size)",
    height: "var(--thumb-size)",
    background: "orange",
    borderRadius: "50%",
    opacity: 0,
    transform: "scale(0.5)",
    transition: "opacity 200ms, transform 200ms",
  },

  selectors: {
    [`${container}:hover &::-webkit-slider-thumb`]: {
      opacity: 1,
      transform: "scale(1.0)",
    },
  },
});
