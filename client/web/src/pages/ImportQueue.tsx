import React, { FC, useEffect, useState } from "react";
import { css } from "@emotion/react";
import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  FormControl,
  Icon,
  InputLabel,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  MenuItem,
  Select,
  TextField,
  Typography,
} from "@material-ui/core";
import api, { TvShow } from "../api";

const ImportQueueScreen: FC<{}> = ({}) => {
  const [queue, setQueue] = useState<any[]>([]);
  const [selected, setSelected] = useState<any | null>(null);

  useEffect(() => {
    fetch("/api/import/queue")
      .then((res) => res.json())
      .then(setQueue);
  }, []);

  return (
    <div>
      <Typography
        variant="h4"
        css={(theme) =>
          css`
            margin: ${theme.spacing(2)};
          `
        }
      >
        Import Queue
      </Typography>
      <List>
        {queue.map((item) => (
          <ListItem key={item.path} button onClick={() => setSelected(item)}>
            <ListItemIcon>
              <Icon>videocam</Icon>
            </ListItemIcon>
            <ListItemText primary={item.name} secondary={item.path} />
          </ListItem>
        ))}
      </List>
      <ImportDialog item={selected} onClose={() => setSelected(null)} />
    </div>
  );
};

const ImportDialog: FC<{ item: any | null; onClose: () => void }> = ({
  item,
  onClose,
}) => {
  const [shows, setShows] = useState<TvShow[]>([]);
  const [type, setType] = useState("TvEpisode");
  const [show, setShow] = useState<number | "" | "new">("");
  const [name, setName] = useState("");
  const [season, setSeason] = useState("");
  const [episode, setEpisode] = useState("");

  let content = <React.Fragment></React.Fragment>;

  useEffect(() => {
    if (item) {
      api.tv.getShows().then(setShows);
    }
  }, [item]);

  if (item) {
    content = (
      <React.Fragment>
        <DialogContentText>{item?.name}</DialogContentText>
        <FormControl fullWidth margin="dense" variant="standard">
          <InputLabel>Type</InputLabel>
          <Select value={type} onChange={(e) => setType(e.target.value)}>
            <MenuItem value="Movie">Movie</MenuItem>
            <MenuItem value="TvEpisode">Episode</MenuItem>
          </Select>
        </FormControl>
        {type === "Movie" && (
          <Typography
            css={css`
              margin-top: 16px;
            `}
          >
            TODO 🙂
          </Typography>
        )}
        {type === "TvEpisode" && (
          <React.Fragment>
            <FormControl fullWidth margin="dense" variant="standard">
              <InputLabel>Show</InputLabel>
              <Select value={show} onChange={(e) => setShow(e.target.value)}>
                <MenuItem value="new">New</MenuItem>
                {shows.map((show) => (
                  <MenuItem key={show.id} value={show.id}>
                    {show.name}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
            {show === "new" && (
              <TextField
                fullWidth
                label="Show Name"
                margin="dense"
                variant="standard"
                value={name}
                onChange={(e) => setName(e.target.value)}
              />
            )}
            <div
              css={css`
                display: flex;
              `}
            >
              <TextField
                label="Season"
                type="number"
                margin="dense"
                variant="standard"
                value={season}
                onChange={(e) => setSeason(e.target.value)}
                css={(theme) =>
                  css`
                    flex: 1;
                    margin-right: ${theme.spacing(1)};
                  `
                }
              />
              <TextField
                label="Episode"
                type="number"
                margin="dense"
                variant="standard"
                value={episode}
                onChange={(e) => setEpisode(e.target.value)}
                css={(theme) =>
                  css`
                    flex: 1;
                    margin-left: ${theme.spacing(1)};
                  `
                }
              />
            </div>
          </React.Fragment>
        )}
      </React.Fragment>
    );
  }

  return (
    <Dialog open={!!item} fullWidth maxWidth="sm" onClose={onClose}>
      <DialogTitle>Import</DialogTitle>
      <DialogContent>{content}</DialogContent>
      <DialogActions>
        <Button onClick={onClose}>Cancel</Button>
        <Button
          onClick={async () => {
            if (!show || !name || !season || !episode) {
              return;
            }

            const episodeData = {
              source: {
                type: "Local",
                path: item.path,
              },
              season_number: parseInt(season),
              episode_number: parseInt(episode),
            };

            if (show === "new") {
              const res = await fetch(`/api/tv/shows`, {
                method: "POST",
                headers: {
                  "Content-Type": "application/json",
                },
                body: JSON.stringify({
                  name: name,
                  episodes: [episodeData],
                }),
              });

              if (res.status === 200) {
                onClose();
              } else {
                console.error(await res.text());
              }
            } else {
              const res = await fetch(`/api/tv/shows/${show}/episodes`, {
                method: "POST",
                headers: {
                  "Content-Type": "application/json",
                },
                body: JSON.stringify(episodeData),
              });

              if (res.status === 200) {
                onClose();
              } else {
                console.error(await res.text());
              }
            }
          }}
        >
          Import
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default ImportQueueScreen;
