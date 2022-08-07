import { css } from "@emotion/css";
import {
  Component,
  createEffect,
  createSignal,
  onCleanup,
  Show,
} from "solid-js";
import { Dynamic } from "solid-js/web";
import {
  BackwardIcon,
  BackwardStepIcon,
  CompressIcon,
  ExpandIcon,
  ForwardIcon,
  ForwardStepIcon,
  PauseIcon,
  PlayIcon,
} from "../../icons";
import { Seekbar } from "../../Seekbar";
import { formatPosition } from "../../utils";
import { VideoPlayer } from "./player";

export function useControlsVisibility(autoHide: () => boolean) {
  const [isVisible, setVisible] = createSignal(true);

  createEffect(() => {
    if (isVisible()) {
      document.body.style.removeProperty("cursor");
    } else {
      document.body.style.cursor = "none";
    }
  });

  let timeout: number | undefined;

  function resetTimeout() {
    clearTimeout(timeout);
    setVisible(true);
    if (autoHide()) {
      timeout = setTimeout((() => setVisible(false)) as TimerHandler, 5000);
    }
  }

  createEffect(() => {
    if (!autoHide()) {
      clearTimeout(timeout);
      setVisible(true);
    }
  });

  onCleanup(() => {
    document.body.style.removeProperty("cursor");
    clearTimeout(timeout);
  });

  return {
    get isVisible() {
      return isVisible();
    },

    reset() {
      resetTimeout();
    },
  };
}

export interface ControlsProps {
  isVisible: boolean;
  video: VideoPlayer;
  isFullscreen: boolean;
  onSeekStart: () => void;
  onSeekEnd: () => void;
  onToggleFullscreen: () => void;
}

const button = css`
  padding: 0px;
  border: none;
  border-radius: 8px;
  transition: all 50ms;
  cursor: pointer;
  color: white;
  background: transparent;
  margin: 0px 16px;
  width: 48px;
  height: 48px;

  &:hover {
    fill: orange;
  }

  &:active {
    background: rgba(255, 255, 255, 0.15);
  }

  &:disabled {
    cursor: default;
    color: rgb(100, 100, 100);
  }

  &:active:disabled {
    background: transparent;
  }
`;

export const Controls: Component<ControlsProps> = (p) => {
  function onSeekEnd(position: number) {
    p.video.position = position;
    p.onSeekEnd();
  }

  const root = css`
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    display: flex;
    justify-content: center;
    padding: 32px;
    transition: all 200ms;
    background: linear-gradient(transparent, rgba(0, 0, 0, 0.7));
    opacity: 0;
  `;

  const visible = css`
    opacity: 1;
  `;

  const controls = css`
    flex: 1;
    max-width: 1200px;
    display: flex;
    flex-direction: column;
  `;

  const timeBar = css`
    display: flex;
    align-items: center;
  `;

  const seekbarContainer = css`
    flex: 1;
    padding: 0px 8px;
  `;

  const actionsRow = css`
    display: grid;
    grid-template-columns: [start] 1fr [mid] auto [end] 1fr;
    justify-content: center;
    padding-top: 16px;
    color: white;
  `;

  const mainActions = css`
    grid-column: mid;
    display: flex;
    align-items: center;
  `;

  const secondaryActions = css`
    grid-column: end;
    display: flex;
    align-items: center;
    justify-content: flex-end;
  `;

  return (
    <div class={root} classList={{ [visible]: p.isVisible }}>
      <div class={controls}>
        <div class={timeBar}>
          <TimeText duration={p.video.duration} position={p.video.position} />
        </div>
        <div class={seekbarContainer}>
          <Seekbar
            duration={p.video.duration}
            position={p.video.position}
            onSeekStart={p.onSeekStart}
            onSeekEnd={onSeekEnd}
          />
        </div>
        <div class={actionsRow}>
          <div class={mainActions}>
            <Button disabled icon={BackwardStepIcon} />
            <Button
              icon={BackwardIcon}
              onClick={() => (p.video.position -= 10)}
            />
            <PlayPauseButton
              isPlaying={p.video.isPlaying}
              onClick={() => (p.video.isPlaying = !p.video.isPlaying)}
            />
            <Button
              icon={ForwardIcon}
              onClick={() => (p.video.position += 30)}
            />
            <Button disabled icon={ForwardStepIcon} />
          </div>
          <div class={secondaryActions}>
            <Show when={document.fullscreenEnabled}>
              <Button
                icon={p.isFullscreen ? CompressIcon : ExpandIcon}
                onClick={p.onToggleFullscreen}
              />
            </Show>
          </div>
        </div>
      </div>
    </div>
  );
};

interface TimeTextProps {
  position: number;
  duration: number;
}

const TimeText: Component<TimeTextProps> = (p) => {
  const timeText = css`
    line-height: 32px;
    font-size: 0.78em;
    font-family: "Exo 2";
    color: rgb(210, 210, 210);
    padding: 0px 8px;
    user-select: none;
  `;

  const divider = css`
    padding: 0;
    font-size: 1em;
    color: darkgrey;
  `;

  const duration = css`
    color: darkgrey;
  `;

  return (
    <>
      <span class={timeText}>{formatPosition(p.position, p.duration)}</span>
      <span class={`${timeText} ${divider}`}>/</span>
      <span class={`${timeText} ${duration}`}>
        {formatPosition(p.duration - p.position, p.duration)}
      </span>
    </>
  );
};

interface ButtonProps {
  icon: Component;
  disabled?: boolean;
  onClick?: () => void;
}

const Button: Component<ButtonProps> = (p) => {
  return (
    <button class={button} disabled={p.disabled} onClick={p.onClick}>
      <Dynamic component={p.icon} />
    </button>
  );
};

interface PlayPauseButtonProps {
  isPlaying: boolean;
  onClick: () => void;
}

const PlayPauseButton: Component<PlayPauseButtonProps> = (p) => {
  const playPauseButton = css`
    ${button}
    width: 64px;
    height: 64px;
  `;

  return (
    <button class={playPauseButton} onClick={p.onClick}>
      {p.isPlaying ? <PauseIcon size={32} /> : <PlayIcon size={32} />}
    </button>
  );
};
