import { useParams } from "solid-app-router";
import { Component, onCleanup, onMount } from "solid-js";
import preferences from "../preferences";
import { setPlayerSrc } from "../VideoPlayerSurface";

export const PlayerScreen: Component = () => {
  const params = useParams();

  onMount(() => {
    setPlayerSrc(`${preferences.server}/api/videos/${params.id}`);
    onCleanup(() => {
      setPlayerSrc(null);
    });
  });

  return null;
};
