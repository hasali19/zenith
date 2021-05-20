import { useEffect, useState } from "react";
import { useHistory } from "react-router";
import { css } from "@emotion/react";
import { Toolbar } from "@material-ui/core";

import api, { TvShow } from "../api";
import AppBar from "../AppBar";
import PosterMediaItem from "../components/PosterMediaItem";
import VirtualItemGrid from "../components/VirtualItemGrid";

export default function () {
  const history = useHistory();
  const [shows, setShows] = useState<TvShow[]>([]);

  useEffect(() => {
    api.tv.getShows().then(setShows);
  }, []);

  return (
    <div
      css={css`
        height: 100%;
        display: flex;
        flex-direction: column;
      `}
    >
      <AppBar />
      <Toolbar />
      <div
        css={css`
          min-height: 0;
          flex: 1;
        `}
      >
        <VirtualItemGrid count={shows.length}>
          {(i, style) => {
            const show = shows[i];
            return (
              <PosterMediaItem
                key={show.id}
                poster={show.poster || undefined}
                primary={show.name}
                secondary={show.startYear()?.toString()}
                style={style}
                onClick={() => history.push(`/shows/${show.id}`)}
              />
            );
          }}
        </VirtualItemGrid>
      </div>
    </div>
  );
}
