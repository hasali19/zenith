import { Component } from "solid-js";
import { PlayIcon } from "./icons";

export interface PlayButtonProps {
  resume?: boolean;
  onClick: () => void;
}

export const PlayButton: Component<PlayButtonProps> = (p) => {
  return (
    <button
      class="px-6 py-3 flex items-center justify-center bg-[#ff7447] leading-4 font-[inherit] font-bold text-white rounded-[20px] cursor-pointer transition duration-200 hover:bg-[#ff8c67] active:scale-[0.9]"
      onClick={p.onClick}
    >
      <PlayIcon size={16} class="mr-4" />
      <span>{p.resume ? "Resume" : "Play"}</span>
    </button>
  );
};
