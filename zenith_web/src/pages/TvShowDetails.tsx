import { useEffect, useState } from "react";
import { useParams } from "react-router";
import styled from "styled-components";

interface TvShow {
  id: number;
  name: string;
  overview: string | null;
  poster_url: string | null;
  backdrop_url: string | null;
  episodes: TvEpisode[];
}

interface TvEpisode {
  id: number;
  season: number;
  episode: number;
  overview: string | null;
  image_url: string | null;
}

export function TvShowDetails() {
  const params = useParams<{ id: string }>();
  const [show, setShow] = useState<TvShow | null>(null);

  useEffect(() => {
    fetch("/api/tv_shows/" + params.id)
      .then((res) => res.json())
      .then((data) => setShow(data));
  }, [params.id]);

  if (!show) {
    return null;
  }

  const backdrop = "https://image.tmdb.org/t/p/w1280" + show.backdrop_url;
  const poster = "https://image.tmdb.org/t/p/w185" + show.poster_url;

  return (
    <Root>
      <Background src={backdrop} />
      <BackgroundOverlay />
      <Foreground>
        <MobileBackdrop src={backdrop} />
        <Main>
          <Poster src={poster} />
          <Details>
            <Title>{show.name}</Title>
            <Overview>{show.overview}</Overview>
            <div>
              <button style={{ width: 60, height: 60 }}>Play {"|>"}</button>
            </div>
          </Details>
        </Main>
        <div style={{ padding: "0px 4% 4% 4%" }}>
          <h2 style={{ margin: "16px 0px" }}>Episodes</h2>
          {show.episodes.map((episode) => (
            <Episode key={episode.id}>
              <EpisodeImage
                src={`https://image.tmdb.org/t/p/w300${episode.image_url}`}
              />
              <EpisodeDetails>
                <h3>
                  Season {episode.season} Episode {episode.episode}
                </h3>
                <div>{episode.overview}</div>
              </EpisodeDetails>
            </Episode>
          ))}
        </div>
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

const Overview = styled.p`
  margin: 32px 0px;
`;

const Episode = styled.div`
  display: flex;
  margin: 32px 0px;

  @media (min-width: 600px) {
    align-items: center;
  }

  @media (max-width: 599px) {
    flex-direction: column;
  }
`;

const EpisodeImage = styled.img`
  width: 16vw;

  @media (max-width: 599px) {
    width: 36vw;
  }
`;

const EpisodeDetails = styled.div`
  flex: 1;

  @media (min-width: 600px) {
    padding: 16px;
  }

  @media (max-width: 599px) {
    margin-top: 16px;
  }
`;
