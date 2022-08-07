import { css } from "@emotion/css";
import { autoUpdate, offset, shift } from "@floating-ui/dom";
import { useFloating } from "solid-floating-ui";
import {
  Component,
  createEffect,
  createSignal,
  For,
  onCleanup,
  Show,
} from "solid-js";
import { Dynamic } from "solid-js/web";
import { Transition } from "solid-transition-group";
import {
  BackwardIcon,
  BackwardStepIcon,
  CheckIcon,
  ClosedCaptioningIcon,
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
  subtitles: any[];
  onSetAutoHide: (value: boolean) => void;
  onSetSubtitleTrack: (id: number | null) => void;
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
  const [showSubtitlesMenu, setShowSubtitlesMenu] = createSignal(false);

  function onSeekStart() {
    p.onSetAutoHide(false);
  }

  function onSeekEnd(position: number) {
    p.video.position = position;
    p.onSetAutoHide(true);
  }

  createEffect(() => {
    p.onSetAutoHide(!showSubtitlesMenu());
  });

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
            onSeekStart={onSeekStart}
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
            <SubtitlesButton
              subtitles={p.subtitles}
              visible={showSubtitlesMenu()}
              onVisibleChange={setShowSubtitlesMenu}
              onSetSubtitleTrack={p.onSetSubtitleTrack}
            />
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
  ref?: (e: HTMLButtonElement) => void;
  icon: Component;
  disabled?: boolean;
  onClick?: () => void;
}

const Button: Component<ButtonProps> = (p) => {
  return (
    <button
      ref={p.ref}
      class={button}
      disabled={p.disabled}
      onClick={p.onClick}
    >
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

interface SubtitlesButtonProps {
  subtitles: any[];
  visible: boolean;
  onVisibleChange: (visible: boolean) => void;
  onSetSubtitleTrack: (id: number | null) => void;
}

const SubtitlesButton: Component<SubtitlesButtonProps> = (p) => {
  const [selected, setSelected] = createSignal<number | null>(null);
  const [reference, setReference] = createSignal<HTMLElement>();
  const [floating, setFloating] = createSignal<HTMLElement>();

  const position = useFloating(reference, floating, {
    strategy: "fixed",
    placement: "top-start",
    middleware: [
      offset({ mainAxis: 8, crossAxis: -8 }),
      shift({ padding: 16 }),
    ],
    whileElementsMounted: autoUpdate,
  });

  createEffect(() => {
    const currentFloating = floating();
    const currentReference = reference();
    if (currentFloating && currentReference && p.visible) {
      const onClick = (e: MouseEvent) => {
        if (
          e.target &&
          !currentFloating.contains(e.target as HTMLElement) &&
          !currentReference.contains(e.target as HTMLElement)
        ) {
          e.preventDefault();
          p.onVisibleChange(false);
        }
      };
      document.body.addEventListener("click", onClick);
      onCleanup(() => document.body.removeEventListener("click", onClick));
    }
  });

  function setSelectedItem(id: number | null) {
    setSelected(id);
    p.onVisibleChange(false);
    p.onSetSubtitleTrack(id);
  }

  const root = css`
    width: 200px;
    padding: 16px 0px;
    border-radius: 4px;
    background-color: #1d1d1d;
    user-select: none;
    box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;

    &.enter {
      transform: scale(0.9);
      opacity: 0;
    }
  `;

  function onBeforeEnter(el: Element) {
    el.classList.add("enter");
  }

  function onEnter(el: Element, done: () => void) {
    el.classList.remove("enter");
    const a = el.animate(
      [
        { transform: "scale(0.9)", opacity: 0 },
        { transform: "scale(1)", opacity: 1 },
      ],
      {
        duration: 100,
        easing: "ease-in",
      }
    );
    a.finished.then(done);
  }

  function onExit(el: Element, done: () => void) {
    const a = el.animate(
      [
        { transform: "scale(1)", opacity: 1 },
        { transform: "scale(0.9)", opacity: 0 },
      ],
      {
        duration: 100,
        easing: "ease-in",
      }
    );
    a.finished.then(done);
  }

  return (
    <Show when={p.subtitles.length > 0}>
      <Button
        ref={setReference}
        icon={ClosedCaptioningIcon}
        onClick={() => p.onVisibleChange(!p.visible)}
      />
      <Transition
        onBeforeEnter={onBeforeEnter}
        onEnter={onEnter}
        onExit={onExit}
      >
        <Show when={p.visible}>
          <div
            ref={setFloating}
            class={root}
            style={{
              position: position.strategy,
              left: `${position.x ?? 0}px`,
              top: `${position.y ?? 0}px`,
            }}
          >
            <SubtitleMenuItem
              primary="None"
              active={selected() === null}
              onClick={() => setSelectedItem(null)}
            />
            <For each={p.subtitles}>
              {(subtitle) => (
                <SubtitleMenuItem
                  primary={subtitle.language}
                  secondary={subtitle.title}
                  active={selected() === subtitle.id}
                  onClick={() => setSelectedItem(subtitle.id)}
                />
              )}
            </For>
          </div>
        </Show>
      </Transition>
    </Show>
  );
};

interface SubtitlesMenuItemProps {
  active?: boolean;
  primary: string;
  secondary?: string;
  onClick?: () => void;
}

const SubtitleMenuItem: Component<SubtitlesMenuItemProps> = (p) => {
  const root = css`
    display: flex;
    align-items: center;
    padding: 8px 16px;
    cursor: pointer;
    transition: background-color 200ms, color 200ms;

    &.selected {
      color: orange;
    }

    &:hover {
      color: white;
      background-color: rgba(255, 255, 255, 0.2);
    }

    &:active {
      color: white;
      background-color: rgba(255, 255, 255, 0.3);
    }
  `;

  const main = css`
    flex: 1;
  `;

  const secondary = css`
    font-size: 0.9em;
    color: darkgrey;
  `;

  console.log(secondary);

  return (
    <div class={root} classList={{ selected: p.active }} onClick={p.onClick}>
      <div class={main}>
        <div>{p.primary}</div>
        <Show when={p.secondary}>
          <div class={secondary}>{p.secondary}</div>
        </Show>
      </div>
      <Show when={p.active}>
        <CheckIcon size={16} />
      </Show>
    </div>
  );
};
