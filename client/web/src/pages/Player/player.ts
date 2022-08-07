import { createEffect, createSignal, onCleanup, untrack } from "solid-js";
import player from "../../player";
import preferences from "../../preferences";

export type VideoPlayer = ReturnType<typeof useVideoPlayer>;

export function useVideoPlayer(
  container: () => HTMLDivElement | undefined,
  id: () => number,
  start: () => number
) {
  const [duration, setDuration] = createSignal(0);
  const [position, setPosition] = createSignal(0);
  const [isPlaying, setPlaying] = createSignal(false);

  createEffect(() => {
    const currentContainer = container();
    if (currentContainer) {
      player.init(
        currentContainer,
        `${preferences.server}/api/videos/${id()}`,
        untrack(start)
      );
    }
    onCleanup(() => player.stop());
  });

  function onDurationChange(duration: number) {
    setDuration(duration);
  }

  function onPositionChange(position: number) {
    setPosition(position);
  }

  async function updateServerPosition(position: number) {
    await fetch(
      `${preferences.server}/api/progress/${id()}?position=${position}`,
      {
        method: "POST",
      }
    );
  }

  function onIsPlayingChange(isPlaying: boolean) {
    setPlaying(isPlaying);
  }

  player.addDurationChangeListener(onDurationChange);
  player.addPositionChangeListener(500, onPositionChange);
  player.addPositionChangeListener(5000, updateServerPosition);
  player.addIsPlayingChangeListener(onIsPlayingChange);

  onCleanup(() => {
    player.removeDurationChangeListener(onDurationChange);
    player.removePositionChangeListener(onPositionChange);
    player.removePositionChangeListener(updateServerPosition);
    player.removeIsPlayingChangeListener(onIsPlayingChange);
  });

  return {
    get duration() {
      return duration();
    },

    get position() {
      return position();
    },

    set position(value: number) {
      player.seekTo(value);
      setPosition(position);
    },

    get isPlaying() {
      return isPlaying();
    },

    set isPlaying(value: boolean) {
      player.setPlaying(value);
    },
  };
}
