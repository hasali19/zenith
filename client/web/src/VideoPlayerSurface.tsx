import { Component, createSignal, Show } from "solid-js";

import * as styles from "./VideoPlayerSurface.css";

const [src, setSrc] = createSignal<string | null>(null);

export function setPlayerSrc(src: string | null) {
  setSrc(src);
}

export const VideoPlayerSurface: Component = () => {
  return (
    <div class={styles.root}>
      <Show when={src()}>
        {(src) => <video class={styles.video} autoplay controls src={src} />}
      </Show>
    </div>
  );
};
