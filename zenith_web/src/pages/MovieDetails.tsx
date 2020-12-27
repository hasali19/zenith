import { useEffect, useState } from "react";
import { useHistory, useParams } from "react-router";
import styled from "styled-components";

interface Movie {
  id: number;
  title: string;
  year: number | null;
  overview: string | null;
  poster_url: string | null;
  backdrop_url: string | null;
  stream_id: number;
}

export function MovieDetails() {
  const history = useHistory();
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

  return (
    <Root>
      <Background src={movie.backdrop_url} />
      <BackgroundOverlay />
      <Foreground>
        <MobileBackdrop src={movie.backdrop_url!!} />
        <Main>
          <Poster src={movie.poster_url!!} />
          <Details>
            <Title>{movie.title}</Title>
            <Year>{movie.year}</Year>
            <Overview>{movie.overview}</Overview>
            <div>
              <button
                onClick={() => history.push(`/player/${movie.stream_id}`)}
                style={{ width: 60, height: 60 }}
              >
                Play {"|>"}
              </button>
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
  width: 250px;
  border-radius: 4px;

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
