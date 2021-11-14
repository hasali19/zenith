import { useParams } from "solid-app-router";
import { Component } from "solid-js";
import preferences from "../preferences";
import * as styles from "./Player.css";

export const PlayerScreen: Component = () => {
  const params = useParams();
  return (
    <div class={styles.root}>
      <video
        autoplay
        controls
        src={`${preferences.server}/api/videos/${params.id}`}
        class={styles.video}
      />
    </div>
  );
};
