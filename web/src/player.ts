import { container } from "./Seekbar.css";

type DurationChangeListener = (duration: number) => void;
type PositionChangeListener = (position: number) => void;
type IsPlayingChangeListener = (isPlaying: boolean) => void;

export interface SubtitleTrack {
  id: number;
  title: string | null;
  language: string | null;
  src: string;
}

export interface Player {
  init(
    container: HTMLDivElement,
    src: string,
    subtitles: SubtitleTrack[],
    start: number
  ): void;
  stop(): void;

  setPlaying(isPlaying: boolean): void;
  seekTo(position: number): void;
  setSubtitleTrack(id: number | null): void;

  addDurationChangeListener(listener: (duration: number) => void): void;
  removeDurationChangeListener(listener: (duration: number) => void): void;

  addPositionChangeListener(
    interval: number,
    listener: PositionChangeListener
  ): void;
  removePositionChangeListener(listener: PositionChangeListener): void;

  addIsPlayingChangeListener(listener: IsPlayingChangeListener): void;
  removeIsPlayingChangeListener(listener: IsPlayingChangeListener): void;
}

class Html5Player implements Player {
  private video: HTMLVideoElement | null = null;

  private durationChangeListeners: DurationChangeListener[] = [];
  private isPlayingChangeListeners: IsPlayingChangeListener[] = [];

  private positionChangeListeners: PositionChangeListener[] = [];
  private positionChangeIntervals: number[] = [];

  init(
    container: HTMLDivElement,
    src: string,
    subtitles: SubtitleTrack[],
    start: number
  ): void {
    const video = (this.video = document.createElement("video"));

    video.style.width = "100%";
    video.style.height = "100%";
    video.style.background = "black";
    video.src = src;
    video.autoplay = true;

    video.addEventListener("durationchange", () => {
      for (const listener of this.durationChangeListeners) {
        listener(video.duration);
      }
    });

    video.addEventListener("play", () => {
      for (const listener of this.isPlayingChangeListeners) {
        listener(true);
      }
    });

    video.addEventListener("pause", () => {
      for (const listener of this.isPlayingChangeListeners) {
        listener(false);
      }
    });

    for (const subtitle of subtitles) {
      const track = document.createElement("track");
      track.id = subtitle.id.toString();
      track.kind = "subtitles";
      track.src = subtitle.src;
      if (subtitle.title) {
        track.label = subtitle.title;
      }
      if (subtitle.language) {
        track.srclang = subtitle.language;
      }
      video.appendChild(track);
    }

    this.setSubtitleTrack(null);

    video.currentTime = start;

    container.appendChild(video);
  }

  stop(): void {
    if (this.video) {
      this.video.parentElement?.removeChild(this.video);
      this.video = null;
    }
  }

  setPlaying(isPlaying: boolean): void {
    if (isPlaying) {
      this.video?.play();
    } else {
      this.video?.pause();
    }
  }

  seekTo(position: number): void {
    if (this.video) {
      this.video.currentTime = position;
    }
  }

  setSubtitleTrack(id: number | null) {
    if (this.video) {
      for (const track of this.video.textTracks) {
        if (parseInt(track.id) === id) {
          track.mode = "showing";
        } else {
          track.mode = "hidden";
        }
      }
    }
  }

  addDurationChangeListener(listener: (duration: number) => void): void {
    this.durationChangeListeners.push(listener);
  }

  removeDurationChangeListener(listener: (duration: number) => void): void {
    this.durationChangeListeners = this.durationChangeListeners.filter(
      (it) => it !== listener
    );
  }

  addPositionChangeListener(
    interval: number,
    listener: PositionChangeListener
  ): void {
    const callback: TimerHandler = () => {
      if (this.video) {
        listener(this.video.currentTime);
      }
    };

    this.positionChangeListeners.push(listener);
    this.positionChangeIntervals.push(setInterval(callback, interval));
  }

  removePositionChangeListener(listener: PositionChangeListener): void {
    const index = this.positionChangeListeners.indexOf(listener);
    if (index > -1) {
      clearInterval(this.positionChangeIntervals[index]);
      this.positionChangeListeners.splice(index, 1);
      this.positionChangeIntervals.splice(index, 1);
    }
  }

  addIsPlayingChangeListener(listener: IsPlayingChangeListener): void {
    this.isPlayingChangeListeners.push(listener);
  }

  removeIsPlayingChangeListener(listener: IsPlayingChangeListener): void {
    this.isPlayingChangeListeners = this.isPlayingChangeListeners.filter(
      (it) => it !== listener
    );
  }
}

export default new Html5Player() as Player;
