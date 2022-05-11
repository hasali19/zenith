import { useLocation, useParams } from "solid-app-router";
import { Component, createEffect, createSignal, on, onCleanup } from "solid-js";
import preferences from "../preferences";
import { Seekbar } from "../Seekbar";
import player from "../player";
import * as styles from "./Player.css";
import {
  BackwardIcon,
  BackwardStepIcon,
  CompressIcon,
  ExpandIcon,
  ForwardIcon,
  ForwardStepIcon,
  PauseIcon,
  PlayIcon,
} from "../icons";
import { formatPosition } from "../utils";

export const PlayerScreen: Component = () => {
  const params = useParams();
  const location = useLocation();

  const [isVisible, setVisible] = createSignal(true);
  const [isSeeking, setSeeking] = createSignal(false);
  const [isPlaying, setPlaying] = createSignal(false);
  const [duration, setDuration] = createSignal(0);
  const [position, setPosition] = createSignal(0);
  const [isFullscreen, setFullscreen] = createSignal(
    !!document.fullscreenElement
  );

  createEffect(() => {
    if (isVisible()) {
      document.body.style.removeProperty("cursor");
    } else {
      document.body.style.cursor = "none";
    }

    onCleanup(() => {
      document.body.style.removeProperty("cursor");
    });
  });

  let timeout: number | undefined;

  function resetTimeout() {
    setVisible(true);
    clearTimeout(timeout);
    if (!isSeeking()) {
      timeout = setTimeout((() => setVisible(false)) as TimerHandler, 5000);
    }
  }

  function onMouseMove() {
    setVisible(true);
    resetTimeout();
  }

  function onSeekStart() {
    setSeeking(true);
  }

  function seekTo(position: number) {
    player.seekTo(position);
    setPosition(position);
  }

  function onSeekEnd(position: number) {
    seekTo(position);
    setSeeking(false);
  }

  function toggleFullscreen() {
    if (isFullscreen()) {
      document.exitFullscreen();
    } else {
      document.body.requestFullscreen();
    }
  }

  createEffect(on(isSeeking, resetTimeout));

  function onDurationChange(duration: number) {
    setDuration(duration);
  }

  function onPositionChange(position: number) {
    setPosition(position);
  }

  async function updateServerPosition(position: number) {
    await fetch(
      `${preferences.server}/api/progress/${params.id}?position=${position}`,
      {
        method: "POST",
      }
    );
  }

  function onIsPlayingChange(isPlaying: boolean) {
    setPlaying(isPlaying);
  }

  function onFullscreenChange() {
    setFullscreen(!!document.fullscreenElement);
  }

  player.addDurationChangeListener(onDurationChange);
  player.addPositionChangeListener(500, onPositionChange);
  player.addPositionChangeListener(5000, updateServerPosition);
  player.addIsPlayingChangeListener(onIsPlayingChange);
  document.addEventListener("fullscreenchange", onFullscreenChange);

  onCleanup(() => {
    player.removeDurationChangeListener(onDurationChange);
    player.removePositionChangeListener(onPositionChange);
    player.removePositionChangeListener(updateServerPosition);
    player.removeIsPlayingChangeListener(onIsPlayingChange);
    document.removeEventListener("fullscreenchange", onFullscreenChange);
    player.stop();
  });

  return (
    <div class={styles.overlay} onMouseMove={onMouseMove}>
      <div
        ref={(elem) => {
          let start = parseFloat(location.query.start) ?? 0;
          return player.init(
            elem,
            `${preferences.server}/api/videos/${params.id}`,
            start
          );
        }}
        style={{
          width: "100%",
          height: "100%",
          position: "absolute",
          display: "flex",
        }}
      />
      <div
        class={styles.controlsContainer}
        style={{ opacity: isVisible() ? 1 : 0 }}
      >
        <div class={styles.controls}>
          <div class={styles.timeBar}>
            <span class={styles.timeText}>
              {formatPosition(position(), duration())}
            </span>
            <span
              class={styles.timeText}
              style={{ padding: 0, "font-size": "1em", color: "darkgrey" }}
            >
              /
            </span>
            <span class={styles.timeText} style={{ color: "darkgrey" }}>
              {formatPosition(duration() - position(), duration())}
            </span>
          </div>
          <div class={styles.timeBar}>
            <div class={styles.seekbarContainer}>
              <Seekbar
                duration={duration()}
                position={position()}
                onSeekStart={onSeekStart}
                onSeekEnd={onSeekEnd}
              />
            </div>
          </div>
          <div class={styles.actionsRow}>
            <div class={styles.mainActions}>
              <button class={styles.skipButton} disabled>
                <BackwardStepIcon />
              </button>
              <button
                class={styles.seekButton}
                onClick={() => seekTo(position() - 10)}
              >
                <BackwardIcon />
              </button>
              <button
                class={styles.playPauseButton}
                onClick={() =>
                  isPlaying()
                    ? player.setPlaying(false)
                    : player.setPlaying(true)
                }
              >
                {isPlaying() ? <PauseIcon size={32} /> : <PlayIcon size={32} />}
              </button>
              <button
                class={styles.seekButton}
                onClick={() => seekTo(position() + 30)}
              >
                <ForwardIcon />
              </button>
              <button class={styles.skipButton} disabled>
                <ForwardStepIcon />
              </button>
            </div>
            <div class={styles.secondaryActions}>
              <button
                disabled={!document.fullscreenEnabled}
                class={styles.seekButton}
                onClick={toggleFullscreen}
              >
                {isFullscreen() ? <CompressIcon /> : <ExpandIcon />}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
