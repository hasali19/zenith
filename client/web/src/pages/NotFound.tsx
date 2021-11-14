import { Component } from "solid-js";
import * as styles from "./NotFound.css";

export const NotFoundScreen: Component = () => {
  return (
    <div class={styles.notFoundRoot}>
      <h1 class={styles.notFoundText}>Not Found</h1>
    </div>
  );
};
