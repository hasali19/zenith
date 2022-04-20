import { Component, JSX, Show } from "solid-js";
import { CircleCheckIcon, FileVideoIcon } from "./icons";
import { Image } from "./Image";

import * as styles from "./MediaItem.css";
import { Poster } from "./Poster";

export interface MediaItemWithPosterProps {
  poster: string;
  name: string;
  secondary: any;
  watched: boolean;
  style?: JSX.CSSProperties;
  onClick: JSX.EventHandlerUnion<HTMLDivElement, MouseEvent>;
}

export const MediaItemWithPoster: Component<MediaItemWithPosterProps> = (p) => (
  <div className={styles.item} style={p.style} onClick={p.onClick}>
    <Poster src={p.poster} watched={p.watched} clickable />
    <div className={styles.details}>
      <p className={styles.name}>{p.name}</p>
      <p className={styles.secondary}>{p.secondary}</p>
    </div>
  </div>
);

export interface MediaItemWithThumbnailProps {
  thumbnail: string;
  name: string;
  secondary: any;
  watched: boolean;
  style?: JSX.CSSProperties;
  onClick: JSX.EventHandlerUnion<HTMLDivElement, MouseEvent>;
}

export const MediaItemWithThumbnail: Component<MediaItemWithThumbnailProps> = (
  p
) => (
  <div className={styles.item} style={p.style} onClick={p.onClick}>
    <div class={styles.thumbnail}>
      <Show when={p.thumbnail} fallback={<ImageFallback />}>
        {(src) => <Image class={styles.image} src={src} />}
      </Show>
      <Show when={p.watched}>
        <div class={styles.overlay}>
          <CircleCheckIcon class={styles.posterCheck} />
        </div>
      </Show>
    </div>
    <div className={styles.details}>
      <p className={styles.name}>{p.name}</p>
      <p className={styles.secondary}>{p.secondary}</p>
    </div>
  </div>
);

const ImageFallback: Component = (p) => (
  <FileVideoIcon size={56} style={{ color: "white" }} />
);
