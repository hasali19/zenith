import { Component } from "solid-js";
import { PlayIcon } from "./icons";
import * as styles from "./PlayButton.css";

export interface PlayButtonProps {
  resume?: boolean;
  onClick: () => void;
}

export const PlayButton: Component<PlayButtonProps> = (p) => {
  return (
    <button class={styles.button} onClick={p.onClick}>
      <PlayIcon size={16} class={styles.icon} />
      <span>{p.resume ? "Resume" : "Play"}</span>
    </button>
  );
};
