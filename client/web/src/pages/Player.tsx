import { css } from "@emotion/css";
import { useLocation, useParams } from "solid-app-router";
import { Component, createMemo, createSignal, onCleanup } from "solid-js";
import { Controls, useControlsVisibility } from "./Player/Controls";
import { useVideoPlayer } from "./Player/player";

export const PlayerScreen: Component = () => {
  const params = useParams();
  const location = useLocation();

  const startPosition = () =>
    location.query.start ? parseFloat(location.query.start) ?? 0 : 0;

  const [videoContainer, setVideoContainer] = createSignal<HTMLDivElement>();
  const [isSeeking, setSeeking] = createSignal(false);

  const id = createMemo(() => parseInt(params.id));
  const video = useVideoPlayer(videoContainer, id, startPosition);
  const controls = useControlsVisibility(() => video.isPlaying && !isSeeking());
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
        onSeekStart={() => setSeeking(true)}
        onSeekEnd={() => setSeeking(false)}
        onToggleFullscreen={fullscreen.toggle}
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
