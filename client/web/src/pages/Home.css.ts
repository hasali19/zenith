import { style } from "@vanilla-extract/css";

export const heroSection = style({
  height: "40vh",
  backgroundSize: "cover",
  backgroundPosition: "center",
  borderRadius: 8,
  margin: 8,
});

export const featuredSection = style({
  padding: 4,
});

export const featuredSectionTitle = style([
  "title",
  "is-5",
  {
    margin: 4,
    marginTop: 20,
    selectors: {
      "&.title.is-5": {
        marginBottom: 12,
      },
    },
  },
]);
