type ItemId = string | number;

export interface MovieJson {
  id: number;
  title: string;
  release_date: number;
  overview: string | null;
  poster: string | null;
  backdrop: string | null;
  duration: number;
}

export class Movie implements MovieJson {
  public id: number;
  public title: string;
  public release_date: number;
  public overview: string | null;
  public poster: string | null;
  public backdrop: string | null;
  public duration: number;

  constructor(json: MovieJson) {
    this.id = json.id;
    this.title = json.title;
    this.release_date = json.release_date; // eslint-disable-line
    this.overview = json.overview;
    this.poster = json.poster;
    this.backdrop = json.backdrop;
    this.duration = json.duration;
  }

  public releaseYear() {
    return new Date(this.release_date * 1000).getUTCFullYear();
  }
}

export interface TvShowJson {
  id: number;
  name: string;
  start_date: number | null;
  end_date: number | null;
  overview: string | null;
  poster: string | null;
  backdrop: string | null;
  unwatched_episodes: number;
}

export class TvShow implements TvShowJson {
  public id: number;
  public name: string;
  public start_date: number | null;
  public end_date: number | null;
  public overview: string | null;
  public poster: string | null;
  public backdrop: string | null;
  public unwatched_episodes: number;

  constructor(json: TvShowJson) {
    this.id = json.id;
    this.name = json.name;
    this.start_date = json.start_date; // eslint-disable-line
    this.end_date = json.end_date; // eslint-disable-line
    this.overview = json.overview;
    this.poster = json.poster;
    this.backdrop = json.backdrop;
    this.unwatched_episodes = json.unwatched_episodes; // eslint-disable-line
  }

  public startYear() {
    if (this.start_date) {
      return new Date(this.start_date * 1000).getUTCFullYear();
    } else {
      return null;
    }
  }
}

export interface TvEpisodeJson {
  id: number;
  show_id: number;
  season_id: number;
  episode_number: number;
  name: string | null;
  air_date: number | null;
  overview: string | null;
  thumbnail: string | null;
  duration: number;
  is_watched: boolean;
}

export class TvEpisode implements TvEpisodeJson {
  public id: number;
  public show_id: number;
  public season_id: number;
  public episode_number: number;
  public name: string | null;
  public air_date: number | null;
  public overview: string | null;
  public thumbnail: string | null;
  public duration: number;
  public is_watched: boolean;

  constructor(json: TvEpisodeJson) {
    this.id = json.id;
    this.show_id = json.show_id; // eslint-disable-line
    this.season_id = json.season_id; // eslint-disable-line
    this.episode_number = json.episode_number; // eslint-disable-line
    this.name = json.name;
    this.air_date = json.air_date; // eslint-disable-line
    this.overview = json.overview;
    this.thumbnail = json.thumbnail;
    this.duration = json.duration;
    this.is_watched = json.is_watched; // eslint-disable-line
  }
}

export interface TvSeasonJson {
  id: number;
  show_id: number;
  season_number: number;
  name: string | null;
  overview: string | null;
  poster: string | null;
  backdrop: string | null;
}

export class TvSeason {
  public id: number;
  public show_id: number;
  public season_number: number;
  public name: string | null;
  public overview: string | null;
  public poster: string | null;
  public backdrop: string | null;

  constructor(json: TvSeasonJson) {
    this.id = json.id;
    this.show_id = json.show_id; // eslint-disable-line
    this.season_number = json.season_number; // eslint-disable-line
    this.name = json.name;
    this.overview = json.overview;
    this.poster = json.poster;
    this.backdrop = json.backdrop;
  }
}

export interface StreamInfo {
  duration: number;
  position: number | null;
  subtitles: SubtitleInfo[];
}

export interface SubtitleInfo {
  index: number;
  language: string | null;
  title: string | null;
}

export default {
  movies: {
    async getMovies() {
      const res = await fetch(`/api/movies`);
      const movies = await res.json();
      return movies.map((m: MovieJson) => new Movie(m));
    },

    async getMovie(id: ItemId) {
      const res = await fetch(`/api/movies/${id}`);
      const show = await res.json();
      return new Movie(show);
    },

    async getRecent() {
      const res = await fetch(`/api/movies/recent`);
      const movies = await res.json();
      return movies.map((m: MovieJson) => new Movie(m));
    },
  },

  tv: {
    async getShows() {
      const res = await fetch(`/api/tv/shows`);
      const shows = await res.json();
      return shows.map((m: TvShowJson) => new TvShow(m));
    },

    async getShow(id: ItemId) {
      const res = await fetch(`/api/tv/shows/${id}`);
      const show = await res.json();
      return new TvShow(show);
    },

    async getRecentShows() {
      const res = await fetch(`/api/tv/shows/recent`);
      const shows = await res.json();
      return shows.map((s: TvShowJson) => new TvShow(s));
    },

    async getSeasons(showId: ItemId) {
      const res = await fetch(`/api/tv/shows/${showId}/seasons`);
      const seasons = await res.json();
      return seasons.map((s: TvSeasonJson) => new TvSeason(s));
    },

    async getSeason(id: ItemId) {
      const res = await fetch(`/api/tv/seasons/${id}`);
      const season = await res.json();
      return new TvSeason(season);
    },

    async getEpisode(id: ItemId) {
      const res = await fetch(`/api/tv/episodes/${id}`);
      const episode = await res.json();
      return new TvEpisode(episode);
    },
  },

  metadata: {
    async refresh(id: ItemId) {
      await fetch(`/api/metadata/${id}/refresh`, { method: "POST" });
    },
  },

  stream: {
    async getInfo(id: ItemId) {
      const res = await fetch(`/api/stream/${id}/info`);
      const info = await res.json();
      return info as StreamInfo;
    },

    getTranscodeUrl(id: ItemId, start = 0) {
      return `/api/stream/${id}/transcode?start=${start}`;
    },
  },

  progress: {
    async update(id: ItemId, position: number) {
      await fetch(`/api/progress/${id}?position=${position}`, {
        method: "POST",
      });
    },
  },
};
