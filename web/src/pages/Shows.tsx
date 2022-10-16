import { useNavigate } from "solid-app-router";
import { Component, createSignal, onMount } from "solid-js";
import { MediaItemGrid } from "../MediaItemGrid";
import preferences from "../preferences";

export const ShowsScreen: Component = () => {
  const navigate = useNavigate();
  const [shows, setShows] = createSignal<any[]>([]);

  onMount(() => {
    fetch(`${preferences.server}/api/tv/shows`)
      .then((res) => res.json())
      .then(setShows);
  });

  return (
    <MediaItemGrid
      items={shows().map((show) => ({
        id: show.id,
        name: show.name,
        date: show.start_date,
        poster: `${preferences.server}/api/items/${show.id}/images/poster`,
        watched: show.user_data.unwatched === 0,
      }))}
      onItemClick={(item) => navigate(`/shows/${item.id}`)}
    />
  );
};