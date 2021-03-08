type ItemId = string | number

export interface StreamInfo {
  duration: number
}

export default {
  async getStreamInfo(id: ItemId) {
    const res = await fetch(`/api/stream/${id}/info`)
    const info = await res.json()
    return info as StreamInfo
  },

  getTranscodeStreamUrl(id: ItemId, start = 0) {
    return `/api/stream/${id}/transcode?start=${start}`
  },
}
