import { useEffect, useState } from "react";
import { useParams } from "react-router";
import styled from "styled-components";

interface Movie {
  id: number;
  title: string;
  year: number | null;
  overview: string | null;
  poster_url: string | null;
  backdrop_url: string | null;
}

export function MovieDetails() {
  const params = useParams<{ id: string }>();
  const [movie, setMovie] = useState<Movie | null>(null);

  useEffect(() => {
    fetch("/api/movies/" + params.id)
      .then((res) => res.json())
      .then((data) => setMovie(data));
  }, [params.id]);

  if (!movie) {
    return null;
  }

  const backdrop = "https://image.tmdb.org/t/p/w1280" + movie.backdrop_url;
  const poster = "https://image.tmdb.org/t/p/w185" + movie.poster_url;

  return (
    <Root>
      <Background src={backdrop} />
      <BackgroundOverlay />
      <Foreground>
        <MobileBackdrop src={backdrop} />
        <Main>
          <Poster src={poster} />
          <Details>
            <Title>{movie.title}</Title>
            <Year>{movie.year}</Year>
            <Overview>{movie.overview}</Overview>
            <div>
              <button style={{ width: 60, height: 60 }}>Play {"|>"}</button>
            </div>
          </Details>
        </Main>
      </Foreground>
    </Root>
  );
}

const Root = styled.div`
  height: 100%;
  position: relative;
`;

const Background = styled.div<{ src: string | null }>`
  position: absolute;
  width: 100%;
  height: 100%;
  background-image: url(${(props) => (props.src ? props.src : "")});
  background-size: cover;
  background-repeat: no-repeat;
  background-position: center;

  @media (max-width: 600px) {
    display: none;
  }
`;

const BackgroundOverlay = styled.div`
  position: absolute;
  width: 100%;
  height: 100%;
  background-color: #000000a0;

  @media (max-width: 600px) {
    display: none;
  }
`;

const Foreground = styled.div`
  position: absolute;
  width: 100%;
  height: 100%;
  overflow-y: auto;
`;

const MobileBackdrop = styled.div<{ src: string }>`
  width: 100%;
  height: 40%;
  background-image: url(${(props) => props.src});
  background-size: cover;
  background-position: center;

  @media (min-width: 600px) {
    display: none;
  }
`;

const Main = styled.div`
  @media (min-width: 600px) {
    display: flex;
    padding: 4%;
  }
`;

const Poster = styled.img`
  @media (max-width: 599px) {
    display: none;
  }
`;

const Details = styled.div`
  max-width: 700px;
  padding: 0px 32px;

  @media (max-width: 599px) {
    padding: 16px;
  }
`;

const Title = styled.h1`
  margin: 8px 0px;
`;

const Year = styled.h3`
  margin: 8px 0px;
`;

const Overview = styled.p`
  margin: 32px 0px;
`;
