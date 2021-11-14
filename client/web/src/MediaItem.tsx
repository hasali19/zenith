import { Component, JSX, Show } from "solid-js";
import { Image } from "./Image";

import * as styles from "./MediaItem.css";

export const MediaItemWithPoster: Component<{
  poster: string;
  name: string;
  secondary: any;
  style?: JSX.CSSProperties;
  onClick: JSX.EventHandlerUnion<HTMLDivElement, MouseEvent>;
}> = (p) => (
  <div className={styles.item} style={p.style} onClick={p.onClick}>
    <div class={styles.poster}>
      <Show
        when={p.poster}
        fallback={<ImageFallback icon="play_circle_filled" />}
      >
        {(src) => <Image class={styles.image} src={src} />}
      </Show>
    </div>
    <div className={styles.details}>
      <p className={styles.name}>{p.name}</p>
      <p className={styles.secondary}>{p.secondary}</p>
    </div>
  </div>
);

export const MediaItemWithThumbnail: Component<{
  thumbnail: string;
  name: string;
  secondary: any;
  style?: JSX.CSSProperties;
  onClick: JSX.EventHandlerUnion<HTMLDivElement, MouseEvent>;
}> = (p) => (
  <div className={styles.item} style={p.style} onClick={p.onClick}>
    <div class={styles.thumbnail}>
      <Show
        when={p.thumbnail}
        fallback={<ImageFallback icon="play_circle_filled" />}
      >
        {(src) => <Image class={styles.image} src={src} />}
      </Show>
    </div>
    <div className={styles.details}>
      <p className={styles.name}>{p.name}</p>
      <p className={styles.secondary}>{p.secondary}</p>
    </div>
  </div>
);

const ImageFallback: Component<{ icon: string }> = (p) => (
  <span class="material-icons" style={{ "font-size": "5rem" }}>
    {p.icon}
  </span>
);
