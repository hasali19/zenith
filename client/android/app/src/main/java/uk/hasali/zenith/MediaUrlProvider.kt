package uk.hasali.zenith

import javax.inject.Inject

class MediaUrlProvider @Inject constructor(
    private val serverUrlProvider: ServerUrlProvider,
) {
    private val server
        get() = serverUrlProvider.url

    fun getVideoUrl(id: Int) = "$server/api/videos/$id"
    fun getSubtitleUrl(id: Int) = "$server/api/subtitles/$id"
}
