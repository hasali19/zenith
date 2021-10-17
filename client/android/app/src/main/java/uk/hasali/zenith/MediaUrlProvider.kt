package uk.hasali.zenith

import javax.inject.Inject

class MediaUrlProvider @Inject constructor() {
    fun getVideoUrl(server: String, id: Int) = "$server/api/videos/$id"
    fun getSubtitleUrl(server: String, id: Int) = "$server/api/subtitles/$id"
}
