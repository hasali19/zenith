import { useParams } from "react-router";
import { css } from "@material-ui/styled-engine";

import api from "../api";

export default function Player() {
  const params = useParams<any>();
  return (
    <div
      css={css`
        width: 100%;
        height: 100%;
      `}
    >
      <video
        src={api.videos.getVideoUrl(params.id)}
        controls
        autoPlay
        css={css`
          width: 100%;
          height: 100%;
        `}
      />
    </div>
  );
}
