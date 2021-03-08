type ItemId = string | number

export interface MovieJson {
  id: number
  title: string
  release_date: number
  overview: string | null
  poster: string | null
  backdrop: string | null
  duration: number
}

export class Movie implements MovieJson {
  public id: number
  public title: string
  public release_date: number
  public overview: string | null
  public poster: string | null
  public backdrop: string | null
  public duration: number

  constructor(json: MovieJson) {
    this.id = json.id
    this.title = json.title
    this.release_date = json.release_date // eslint-disable-line
    this.overview = json.overview
    this.poster = json.poster
    this.backdrop = json.backdrop
    this.duration = json.duration
  }

  public releaseYear() {
    return new Date(this.release_date * 1000).getUTCFullYear()
  }
}

export interface TvShowJson {
  id: number
  name: string
  start_date: number | null
  end_date: number | null
  overview: string | null
  poster: string | null
  backdrop: string | null
  unwatched_episodes: number
}

export class TvShow implements TvShowJson {
  public id: number
  public name: string
  public start_date: number | null
  public end_date: number | null
  public overview: string | null
  public poster: string | null
  public backdrop: string | null
  public unwatched_episodes: number

  constructor(json: TvShowJson) {
    this.id = json.id
    this.name = json.name
    this.start_date = json.start_date // eslint-disable-line
    this.end_date = json.end_date // eslint-disable-line
    this.overview = json.overview
    this.poster = json.poster
    this.backdrop = json.backdrop
    this.unwatched_episodes = json.unwatched_episodes // eslint-disable-line
  }

  public startYear() {
    if (this.start_date) {
      return new Date(this.start_date * 1000).getUTCFullYear()
    } else {
      return null
    }
  }
}

export interface StreamInfo {
  duration: number
}

export default {
  movies: {
    async getRecent() {
      const res = await fetch(`/api/movies/recent`)
      const movies = await res.json()
      return movies.map((m: MovieJson) => new Movie(m))
    },
  },

  tv: {
    async getRecentShows() {
      const res = await fetch(`/api/tv/shows/recent`)
      const shows = await res.json()
      return shows.map((s: TvShowJson) => new TvShow(s))
    },
  },

  stream: {
    async getInfo(id: ItemId) {
      const res = await fetch(`/api/stream/${id}/info`)
      const info = await res.json()
      return info as StreamInfo
    },

    getTranscodeUrl(id: ItemId, start = 0) {
      return `/api/stream/${id}/transcode?start=${start}`
    },
  },
}
