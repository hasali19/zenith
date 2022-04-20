import { useNavigate } from "solid-app-router";
import { Component, createSignal, onMount } from "solid-js";
import { MediaItemGrid } from "../MediaItemGrid";
import preferences from "../preferences";

export const MoviesScreen: Component = () => {
  const navigate = useNavigate();
  const [movies, setMovies] = createSignal<any[]>([]);

  onMount(() => {
    fetch(`${preferences.server}/api/movies`)
      .then((res) => res.json())
      .then(setMovies);
  });

  return (
    <MediaItemGrid
      items={movies().map((movie) => ({
        id: movie.id,
        name: movie.title,
        date: movie.release_date,
        poster: movie.poster,
        watched: movie.user_data.is_watched,
      }))}
      onItemClick={(item) => navigate(`/movies/${item.id}`)}
    />
  );
};
