import { Component, createEffect, createSignal } from "solid-js";
import * as styles from "./Seekbar.css";

export const Seekbar: Component<{
  duration: number;
  position: number;
  onSeekStart: () => void;
  onSeekEnd: (position: number) => void;
}> = (p) => {
  let seekbar!: HTMLInputElement;
  let progress!: HTMLProgressElement;

  const [isSeeking, setSeeking] = createSignal(false);
  const [internalPosition, setInternalPosition] = createSignal(p.position);

  createEffect(() => {
    if (!isSeeking()) {
      setInternalPosition(p.position);
    }
  });

  createEffect(() => {
    seekbar.value = internalPosition().toString();
    progress.value = internalPosition();
  });

  const onPointerDown = () => {
    setSeeking(true);
    p.onSeekStart();
  };

  const onPointerMove = (e: PointerEvent) => {
    if (isSeeking()) {
      setInternalPosition(parseInt(seekbar.value));
    }
  };

  const onPointerUp = (e: PointerEvent) => {
    p.onSeekEnd(parseInt(seekbar.value));
    setSeeking(false);
  };

  return (
    <div class={styles.container}>
      <progress
        ref={progress}
        class={styles.progress}
        max={Math.floor(p.duration)}
        value={internalPosition()}
      />
      <input
        ref={seekbar}
        class={styles.range}
        type="range"
        min="0"
        max={Math.floor(p.duration)}
        value={internalPosition()}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
      />
    </div>
  );
};
