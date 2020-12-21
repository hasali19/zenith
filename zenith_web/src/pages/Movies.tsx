import { useState } from "react";
import { useHistory } from "react-router";
import { useOnce } from "../hooks";
import styled from "styled-components";

import { Grid } from "../components/Grid";

interface Movie {
  id: number;
  title: string;
  year: number | null;
  poster_url: string | null;
}

export function Movies() {
  const history = useHistory();
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
      <Grid
        items={items}
        onItemClick={(item) => history.push("/movies/" + item.id)}
      />
    </Root>
  );
}

const Root = styled.div`
  padding: 16px;
`;

const Title = styled.h1`
  margin-bottom: 16px;
`;
