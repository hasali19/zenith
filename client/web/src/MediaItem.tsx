import { Component, JSX, Show } from "solid-js";
import { CircleCheckIcon, FileVideoIcon } from "./icons";
import { Image } from "./Image";
import { Poster } from "./Poster";

const mediaItemClasses =
  "cursor-pointer select-none transition-transform duration-100 hover:scale-[0.98] active:scale-[0.95]";

export interface MediaItemWithPosterProps {
  poster: string;
  name: string;
  secondary: any;
  watched: boolean;
  style?: JSX.CSSProperties;
  onClick: JSX.EventHandlerUnion<HTMLDivElement, MouseEvent>;
}

export const MediaItemWithPoster: Component<MediaItemWithPosterProps> = (p) => (
  <div class={mediaItemClasses} style={p.style} onClick={p.onClick}>
    <Poster src={p.poster} watched={p.watched} />
    <Details name={p.name} secondary={p.secondary} />
  </div>
);

export interface MediaItemWithThumbnailProps {
  thumbnail: string;
  name: string;
  secondary: any;
  watched: boolean;
  progress?: number;
  style?: JSX.CSSProperties;
  onClick: JSX.EventHandlerUnion<HTMLDivElement, MouseEvent>;
}

export const MediaItemWithThumbnail: Component<MediaItemWithThumbnailProps> = (
  p
) => (
  <div class={mediaItemClasses} style={p.style} onClick={p.onClick}>
    <div class="w-full flex items-center justify-center bg-[rgb(100,100,100)] rounded-lg relative aspect-video">
      <Show when={p.thumbnail} fallback={<ImageFallback />}>
        {(src) => (
          <Image class="w-full h-full object-cover rounded-lg" src={src} />
        )}
      </Show>
      <Show when={p.watched}>
        <div class="absolute inset-0 bg-black/30 rounded-lg">
          <CircleCheckIcon class="absolute top-0 right-0 m-4 text-white" />
        </div>
      </Show>
      <Show when={p.progress !== undefined}>
        <div class="absolute bottom-0 left-0 right-0 m-4 bg-white rounded-[2px]">
          <div
            class="h-[4px] bg-orange-400 rounded-[2px]"
            style={{ width: `calc(${p.progress} * 100%)` }}
          />
        </div>
      </Show>
    </div>
    <Details name={p.name} secondary={p.secondary} />
  </div>
);

const ImageFallback: Component = (p) => (
  <FileVideoIcon size={56} class="text-white" />
);

interface DetailsProps {
  name: string;
  secondary: string;
}

function Details(p: DetailsProps) {
  return (
    <div class="py-4">
      <p class="overflow-hidden text-ellipsis whitespace-nowrap text-[1.1rem] font-bold">
        {p.name}
      </p>
      <p class="overflow-hidden text-ellipsis whitespace-nowrap text-[0.9rem]">
        {p.secondary}
      </p>
    </div>
  );
}
