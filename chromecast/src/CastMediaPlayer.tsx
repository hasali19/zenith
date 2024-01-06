import { Component } from "solid-js";

declare module "solid-js" {
  namespace JSX {
    interface IntrinsicElements {
      "cast-media-player": any;
    }
  }
}

export const CastMediaPlayer: Component = () => (
  <cast-media-player></cast-media-player>
);
