import { useEffect, useRef } from "react";
import { useParams } from "react-router";
import styled from "styled-components";

interface StreamInfo {
  path: string;
  duration: number;
}

async function getStreamInfo(id: any): Promise<StreamInfo> {
  const res = await fetch("/api/stream/" + id + "/info");
  const data = await res.json();
  return data;
}

export class VideoPlayer {
  private interval: any;

  private video: HTMLVideoElement;
  private source: HTMLSourceElement;
  private seekbar: HTMLInputElement;

  private seeking = false;
  private disabled = false;
  private startTime = 0;
  private streamId: number | string | null = null;

  constructor(private readonly container: HTMLDivElement) {
    this.container.style.position = "relative";

    this.video = document.createElement("video");
    this.source = document.createElement("source");

    this.video.style.width = "100%";
    this.video.style.height = "100%";
    this.video.style.objectFit = "contain";
    this.video.style.background = "black";

    const overlay = document.createElement("div");

    overlay.style.position = "absolute";
    overlay.style.width = "100%";
    overlay.style.height = "100%";
    overlay.style.top = "0";

    const controls = document.createElement("div");
    const seekbar = document.createElement("input");

    seekbar.type = "range";
    seekbar.min = "0";
    seekbar.value = "0";
    seekbar.style.width = "100%";
    seekbar.style.margin = "0";

    this.seekbar = seekbar;

    overlay.appendChild(controls);
    controls.appendChild(seekbar);

    this.container.appendChild(this.video);
    this.container.appendChild(overlay);
    this.video.appendChild(this.source);

    this.interval = window.setInterval(() => {
      if (!this.video.paused && !this.seeking) {
        const currentTime = this.startTime + this.video.currentTime;
        this.seekbar.value = currentTime.toString();
      }
    }, 100);

    this.seekbar.addEventListener("mousedown", () => {
      if (!this.disabled) {
        this.video.pause();
      }
    });

    this.seekbar.addEventListener("mouseup", () => {
      if (this.streamId != null && !this.disabled) {
        this.seeking = true;
        this.disabled = true;
        this.startTime = parseFloat(this.seekbar.value);
        this.source.src =
          "/api/stream/" + this.streamId + "?start=" + this.startTime;
        this.video.load();
        this.video.play().then(() => {
          this.disabled = false;
          this.seeking = false;
        });
      }
    });
  }

  public async setStreamId(id: string | number) {
    const info = await getStreamInfo(id);

    this.startTime = 0;
    this.streamId = id;

    this.seekbar.max = info.duration.toString();
    this.source.src = "/api/stream/" + id + "?start=" + this.startTime;

    this.video.load();
    this.video.play();
  }

  public destroy() {
    window.clearInterval(this.interval);
    this.container.innerHTML = "";
  }
}

export function Player() {
  const params = useParams<{ id: string }>();
  const video = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (video.current) {
      const player = new VideoPlayer(video.current);
      player.setStreamId(params.id);
      return () => player.destroy();
    }
  }, [params, video]);

  return <VideoContainer ref={video}></VideoContainer>;
}

const VideoContainer = styled.figure`
  margin: 0;
  width: 100vw;
  height: 100vh;
  position: relative;
`;
