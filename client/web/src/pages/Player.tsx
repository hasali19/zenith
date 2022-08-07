import { css } from "@emotion/css";
import { useLocation, useParams } from "solid-app-router";
import {
  Component,
  createMemo,
  createResource,
  createSignal,
  onCleanup,
  Show,
} from "solid-js";
import preferences from "../preferences";
import { Controls, useControlsVisibility } from "./Player/Controls";
import { useVideoPlayer } from "./Player/player";

export const PlayerScreen: Component = () => {
  const params = useParams();
  const location = useLocation();

  const startPosition = () =>
    location.query.start ? parseFloat(location.query.start) ?? 0 : 0;

  const id = createMemo(() => parseInt(params.id));
  const [item] = createResource(() =>
    fetch(`${preferences.server}/api/items/${id()}`).then((res) => res.json())
  );

  return (
    <Show when={item()}>
      <Player item={item()} startPosition={startPosition()} />
    </Show>
  );
};

interface PlayerProps {
  item: any;
  startPosition: number;
}

const Player: Component<PlayerProps> = (p) => {
  const [videoContainer, setVideoContainer] = createSignal<HTMLDivElement>();
  const [autoHideControls, setAutoHideControls] = createSignal(true);

  const video = useVideoPlayer(
    videoContainer,
    () => p.item,
    () => p.startPosition
  );

  const controls = useControlsVisibility(
    () => video.isPlaying && autoHideControls()
  );

  const fullscreen = useFullscreen();

  const root = css`
    position: relative;
    width: 100vw;
    height: 100vh;
  `;

  const videoContainerCls = css`
    width: 100%;
    height: 100%;
    position: absolute;
    display: flex;
  `;

  return (
    <div class={root} onMouseMove={controls.reset}>
      <div ref={setVideoContainer} class={videoContainerCls} />
      <Controls
        isVisible={controls.isVisible}
        video={video}
        isFullscreen={fullscreen.isFullscreen}
        subtitles={p.item.video_info.subtitles}
        onSetAutoHide={setAutoHideControls}
        onToggleFullscreen={fullscreen.toggle}
        onSetSubtitleTrack={video.setSubtitleTrack}
      />
    </div>
  );
};

function useFullscreen() {
  const [isFullscreen, setFullscreen] = createSignal(
    !!document.fullscreenElement
  );

  function toggleFullscreen() {
    if (isFullscreen()) {
      document.exitFullscreen();
    } else {
      document.body.requestFullscreen();
    }
  }

  function onFullscreenChange() {
    setFullscreen(!!document.fullscreenElement);
  }

  document.addEventListener("fullscreenchange", onFullscreenChange);

  onCleanup(() => {
    document.removeEventListener("fullscreenchange", onFullscreenChange);
  });

  return {
    get isFullscreen() {
      return isFullscreen();
    },

    toggle() {
      toggleFullscreen();
    },
  };
}
