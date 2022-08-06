import { style } from "@vanilla-extract/css";

export const heroSection = style({
  width: "100%",
  height: "40vh",
  backgroundColor: "rgb(100,100,100)",
  borderRadius: 8,
  margin: 8,
});

export const heroSectionImg = style({
  width: "100%",
  height: "100%",
  backgroundColor: "rgb(100,100,100)",
  borderRadius: 8,
});

export const featuredSection = style({
  padding: 4,
});

export const featuredSectionTitle = style({
  margin: 4,
  marginTop: 20,
  selectors: {
    "&.title.is-5": {
      marginBottom: 12,
    },
  },
});
