import { useParams } from "solid-app-router";
import { Component, createEffect, createSignal, on, onCleanup } from "solid-js";
import {
  mdiFastForward30,
  mdiFullscreen,
  mdiFullscreenExit,
  mdiPause,
  mdiPlay,
  mdiRewind10,
  mdiSkipNext,
  mdiSkipPrevious,
} from "@mdi/js";
import preferences from "../preferences";
import { Seekbar } from "../Seekbar";
import player from "../player";
import * as styles from "./Player.css";
import { SvgIcon } from "../SvgIcon";

export const PlayerScreen: Component = () => {
  const params = useParams();

  const [isVisible, setVisible] = createSignal(true);
  const [isSeeking, setSeeking] = createSignal(false);
  const [isPlaying, setPlaying] = createSignal(true);
  const [duration, setDuration] = createSignal(0);
  const [position, setPosition] = createSignal(0);
  const [isFullscreen, setFullscreen] = createSignal(
    !!document.fullscreenElement
  );

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

  function onIsPlayingChange(isPlaying: boolean) {
    setPlaying(isPlaying);
  }

  function onFullscreenChange() {
    setFullscreen(!!document.fullscreenElement);
  }

  player.addDurationChangeListener(onDurationChange);
  player.addPositionChangeListener(500, onPositionChange);
  player.addIsPlayingChangeListener(onIsPlayingChange);
  document.addEventListener("fullscreenchange", onFullscreenChange);

  onCleanup(() => {
    player.removeDurationChangeListener(onDurationChange);
    player.removePositionChangeListener(onPositionChange);
    player.removeIsPlayingChangeListener(onIsPlayingChange);
    document.removeEventListener("fullscreenchange", onFullscreenChange);
    player.stop();
  });

  return (
    <div class={styles.overlay} onMouseMove={onMouseMove}>
      <div
        ref={(elem) =>
          player.init(elem, `${preferences.server}/api/videos/${params.id}`)
        }
        style={{
          position: "absolute",
          width: "100%",
          height: "100%",
        }}
      />
      <div class={styles.controls} style={{ opacity: isVisible() ? 1 : 0 }}>
        <div class={styles.timeBar}>
          <span class={styles.timeText}>
            {formatPosition(position(), duration())}
          </span>
          <div class={styles.seekbarContainer}>
            <Seekbar
              duration={duration()}
              position={position()}
              onSeekStart={onSeekStart}
              onSeekEnd={onSeekEnd}
            />
          </div>
          <span class={styles.timeText}>
            {formatPosition(duration() - position(), duration())}
          </span>
        </div>
        <div class={styles.actionsRow}>
          <div class={styles.mainActions}>
            <button class={styles.skipButton} disabled>
              <SvgIcon path={mdiSkipPrevious} />
            </button>
            <button
              class={styles.seekButton}
              onClick={() => seekTo(position() - 10)}
            >
              <SvgIcon path={mdiRewind10} />
            </button>
            <button
              class={styles.playPauseButton}
              onClick={() =>
                isPlaying() ? player.setPlaying(false) : player.setPlaying(true)
              }
            >
              <SvgIcon path={isPlaying() ? mdiPause : mdiPlay} size={40} />
            </button>
            <button
              class={styles.seekButton}
              onClick={() => seekTo(position() + 30)}
            >
              <SvgIcon path={mdiFastForward30} />
            </button>
            <button class={styles.skipButton} disabled>
              <SvgIcon path={mdiSkipNext} />
            </button>
          </div>
          <div class={styles.secondaryActions}>
            <button
              disabled={!document.fullscreenEnabled}
              class={styles.seekButton}
              onClick={toggleFullscreen}
            >
              <SvgIcon
                path={isFullscreen() ? mdiFullscreenExit : mdiFullscreen}
              />
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

function formatPosition(position: number, duration: number) {
  const hours = Math.floor(position / 3600);
  const minutes = Math.floor((position % 3600) / 60);
  const seconds = Math.floor(position % 60);
  const fmt = (v: number) => v.toString().padStart(2, "0");
  if (duration > 3600) {
    return `${fmt(hours)}:${fmt(minutes)}:${fmt(seconds)}`;
  } else {
    return `${fmt(minutes)}:${fmt(seconds)}`;
  }
}
