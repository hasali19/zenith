export type ItemId = string | number;

export interface Movie {
  id: number;
  title: string;
  release_date: number | null;
  overview: string | null;
  poster: string | null;
  backdrop: string | null;
  video_info: VideoInfo;
  user_data: VideoUserData;
}

export interface Show {
  id: number;
  name: string;
  start_date: number | null;
  end_date: number | null;
  overview: string | null;
  poster: string | null;
  backdrop: string | null;
  unwatched_episodes: number;
}

export interface Season {
  id: number;
  show_id: number;
  season_number: number;
  name: string | null;
  overview: string | null;
  poster: string | null;
  backdrop: string | null;
  user_data: CollectionUserData;
}

export interface Episode {
  id: number;
  show_id: number;
  season_id: number;
  season_number: number;
  episode_number: number;
  name: string | null;
  air_date: number | null;
  overview: string | null;
  thumbnail: string | null;
  video_info: VideoInfo;
  user_data: VideoUserData;
}

export interface VideoInfo {
  path: string;
  duration: number;
  subtitles: Subtitle[];
}

export interface Subtitle {
  id: number;
  video_id: number;
  stream_index: number | null;
  path: string | null;
  title: string | null;
  language: string | null;
}

export interface VideoUserData {
  is_watched: boolean;
  position: number;
}

export interface CollectionUserData {
  unwatched: number;
}

export type MediaItem =
  | (Movie & { type: "movie" })
  | (Show & { type: "show" })
  | (Season & { type: "season" })
  | (Episode & { type: "episode" });

export default {
  items: {
    async getItem(id: ItemId): Promise<MediaItem> {
      const res = await fetch(`/api/items/${id}`);
      return await res.json();
    },
  },

  movies: {
    async getMovies(): Promise<Movie[]> {
      const res = await fetch(`/api/movies`);
      return await res.json();
    },

    async getMovie(id: ItemId): Promise<Movie> {
      const res = await fetch(`/api/movies/${id}`);
      return await res.json();
    },

    async getRecent(): Promise<Movie[]> {
      const res = await fetch(`/api/movies/recent`);
      return await res.json();
    },
  },

  tv: {
    async getShows(): Promise<Show[]> {
      const res = await fetch(`/api/tv/shows`);
      return await res.json();
    },

    async getShow(id: ItemId): Promise<Show> {
      const res = await fetch(`/api/tv/shows/${id}`);
      return await res.json();
    },

    async getRecentShows(): Promise<Show[]> {
      const res = await fetch(`/api/tv/shows/recent`);
      return await res.json();
    },

    async getSeasons(showId: ItemId): Promise<Season[]> {
      const res = await fetch(`/api/tv/shows/${showId}/seasons`);
      return await res.json();
    },

    async getSeason(id: ItemId): Promise<Season> {
      const res = await fetch(`/api/tv/seasons/${id}`);
      return await res.json();
    },

    async getEpisodes(seasonId: ItemId): Promise<Episode[]> {
      const res = await fetch(`/api/tv/seasons/${seasonId}/episodes`);
      return await res.json();
    },

    async getEpisode(id: ItemId): Promise<Episode> {
      const res = await fetch(`/api/tv/episodes/${id}`);
      return await res.json();
    },
  },

  videos: {
    getUrl(id: ItemId) {
      return `/api/videos/${id}`;
    },
  },

  metadata: {
    async refresh(id: ItemId) {
      await fetch(`/api/metadata/${id}/refresh`, { method: "POST" });
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
