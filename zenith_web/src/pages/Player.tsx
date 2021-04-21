import { LinearProgress } from "@material-ui/core";
import { css } from "@material-ui/styled-engine";
import { useEffect, useState } from "react";
import { useParams } from "react-router";
import api, { StreamInfo } from "../api";

export default function Player() {
  const params = useParams<any>();
  const [info, setInfo] = useState<StreamInfo | null>(null);

  useEffect(() => {
    api.stream.getInfo(params.id).then(setInfo);
  }, []);

  if (!info) {
    return <LinearProgress variant="indeterminate" />;
  }

  return (
    <div
      css={css`
        width: 100%;
        height: 100%;
      `}
    >
      <video
        src={api.stream.getTranscodeUrl(params.id)}
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
