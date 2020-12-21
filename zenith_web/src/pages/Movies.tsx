import { useState } from "react";
import styled from "styled-components";
import { Grid } from "../components/Grid";

import { useOnce } from "../hooks";

interface Movie {
  id: number;
  title: string;
  year: number | null;
  poster_url: string | null;
}

export function Movies() {
  const [movies, setMovies] = useState<Movie[]>([]);

  useOnce(() => {
    fetch("/api/movies")
      .then((res) => res.json())
      .then((data) => setMovies(data));
  });

  const items = movies.map((movie) => ({
    id: movie.id,
    poster: movie.poster_url
      ? `https://image.tmdb.org/t/p/w185${movie.poster_url}`
      : null,
    primary: movie.title,
    secondary: movie.year?.toString() || null,
  }));

  return (
    <Root>
      <Title>Movies</Title>
      <Grid items={items} />
    </Root>
  );
}

const Root = styled.div`
  padding: 16px;
`;

const Title = styled.h1`
  margin-bottom: 16px;
`;
