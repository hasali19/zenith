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
  Toolbar,
  Typography,
} from "@material-ui/core";

import api, { TvShow } from "../api";
import AppBar from "../AppBar";

const ImportQueueScreen: FC<{}> = ({}) => {
  const [queue, setQueue] = useState<any[] | null>(null);
  const [selected, setSelected] = useState<any | null>(null);

  useEffect(() => {
    fetch("/api/import/queue")
      .then((res) => res.json())
      .then(setQueue);
  }, []);

  return (
    <div
      css={css`
        height: 100%;
        display: flex;
        flex-direction: column;
      `}
    >
      <AppBar title="Import Queue" />
      <Toolbar />
      {queue && queue.length === 0 && (
        <div
          css={css`
            flex: 1;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100%;
          `}
        >
          <Icon>done_all</Icon>
          <Typography variant="body2">All done 🙂</Typography>
        </div>
      )}
      {queue && queue.length > 0 && (
        <div
          css={css`
            height: 100%;
            overflow-y: auto;
          `}
        >
          <List>
            {queue?.map((item) => (
              <ListItem
                key={item.path}
                button
                onClick={() => setSelected(item)}
              >
                <ListItemIcon>
                  <Icon>videocam</Icon>
                </ListItemIcon>
                <ListItemText primary={item.name} secondary={item.path} />
              </ListItem>
            ))}
          </List>
        </div>
      )}
      <ImportDialog item={selected} onClose={() => setSelected(null)} />
    </div>
  );
};

const ImportDialog: FC<{ item: any | null; onClose: () => void }> = ({
  item,
  onClose,
}) => {
  const [shows, setShows] = useState<TvShow[]>([]);
  const [type, setType] = useState<"Movie" | "TvEpisode">("TvEpisode");
  const [show, setShow] = useState<number | "" | "new">("");
  const [name, setName] = useState("");
  const [season, setSeason] = useState("");
  const [episode, setEpisode] = useState("");
  const [title, setTitle] = useState("");
  const [year, setYear] = useState("");

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
          <React.Fragment>
            <TextField
              fullWidth
              label="Title"
              margin="dense"
              variant="standard"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
            />
            <TextField
              fullWidth
              label="Year"
              margin="dense"
              variant="standard"
              value={year}
              onChange={(e) => setYear(e.target.value)}
            />
          </React.Fragment>
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
            if (type === "Movie") {
              if (!title || !year) {
                return;
              }

              const yearNum = parseInt(year);
              // Probably not watching movies older than 1900, and won't be watching movies from
              // the future.
              // This will need to be updated once time travel becomes a thing.
              if (yearNum < 1900 || yearNum > new Date().getFullYear() + 1) {
                return;
              }

              const res = await fetch(`/api/movies`, {
                method: "POST",
                headers: {
                  "Content-Type": "application/json",
                },
                body: JSON.stringify({
                  source: {
                    type: "Local",
                    path: item.path,
                  },
                  title,
                  year: yearNum,
                }),
              });

              if (res.status === 200) {
                onClose();
              } else {
                console.error(await res.text());
              }
            } else {
              if (!show || (show === "new" && !name) || !season || !episode) {
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
