import { style } from "@vanilla-extract/css";

export const item = style([
  "box",
  {
    userSelect: "none",
    borderRadius: 8,
    transition: "all 100ms",
    ":hover": {
      transform: "scale(0.98)",
    },
    ":active": {
      transform: "scale(0.95)",
    },
    selectors: {
      "&.box": {
        padding: 0,
        marginBottom: 0,
      },
    },
  },
]);

const imageWrapper = style({
  width: "100%",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  backgroundColor: "#1e2b3b",
  borderTopLeftRadius: 8,
  borderTopRightRadius: 8,
});

export const poster = style([
  imageWrapper,
  {
    aspectRatio: "2/3",
  },
]);

export const thumbnail = style([
  imageWrapper,
  {
    aspectRatio: "16/9",
  },
]);

export const image = style({
  width: "100%",
  height: "100%",
  objectFit: "cover",
  display: "block",
  borderTopLeftRadius: 8,
  borderTopRightRadius: 8,
});

export const details = style({
  padding: 16,
});

export const name = style({
  margin: 0,
  overflow: "hidden",
  textOverflow: "ellipsis",
  whiteSpace: "nowrap",
  fontSize: "0.9rem",
  fontWeight: "bold",
});

export const secondary = style({
  margin: 0,
  overflow: "hidden",
  textOverflow: "ellipsis",
  whiteSpace: "nowrap",
  fontSize: "0.8rem",
});
