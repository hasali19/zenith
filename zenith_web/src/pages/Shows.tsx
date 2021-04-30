import { useEffect, useState } from "react";
import { useHistory } from "react-router";

import api, { TvShow } from "../api";
import PosterMediaItem from "../components/PosterMediaItem";
import VirtualItemGrid from "../components/VirtualItemGrid";

export default function () {
  const history = useHistory();
  const [shows, setShows] = useState<TvShow[]>([]);

  useEffect(() => {
    api.tv.getShows().then(setShows);
  }, []);

  return (
    <div style={{ height: "100%" }}>
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
  );
}
