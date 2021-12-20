package uk.hasali.zenith.api

import okhttp3.MultipartBody
import retrofit2.http.*

interface ZenithMediaService {
    @GET("movies")
    suspend fun getMovies(): List<Movie>

    @GET("movies/recent")
    suspend fun getRecentMovies(): List<Movie>

    @GET("movies/{id}")
    suspend fun getMovie(@Path("id") id: Int): Movie

    @GET("tv/shows")
    suspend fun getShows(): List<Show>

    @GET("tv/shows/recent")
    suspend fun getRecentShows(): List<Show>

    @GET("tv/shows/{id}")
    suspend fun getShow(@Path("id") id: Int): Show

    @GET("tv/shows/{id}/seasons")
    suspend fun getSeasons(@Path("id") showId: Int): List<Season>

    @GET("tv/seasons/{id}")
    suspend fun getSeason(@Path("id") id: Int): Season

    @GET("tv/seasons/{id}/episodes")
    suspend fun getEpisodes(@Path("id") seasonId: Int): List<Episode>

    @GET("tv/episodes/{id}")
    suspend fun getEpisode(@Path("id") id: Int): Episode

    @GET("items/{id}")
    suspend fun getItem(@Path("id") id: Int): MediaItem

    @PATCH("items/{id}/user_data")
    suspend fun updateUserData(@Path("id") id: Int, @Body data: VideoUserDataPatch)

    @POST("metadata/refresh")
    suspend fun refreshMetadata(@Path("id") id: Int)

    @POST("transcoder")
    suspend fun startTranscode(@Query("video_id") videoId: Int)

    @POST("movies")
    suspend fun importMovie(@Body data: ImportMovieRequest)

    @POST("tv/shows")
    suspend fun importShow(@Body data: ImportShowRequest)

    @POST("tv/shows/{id}/episodes")
    suspend fun importEpisode(@Path("id") showId: Int, @Body data: ImportEpisodeRequest)

    @Multipart
    @POST("import/subtitle")
    suspend fun importSubtitle(
        @Part("data") data: ImportSubtitleRequest,
        @Part file: MultipartBody.Part,
    )

    @GET("progress/{id}")
    suspend fun updateProgress(@Path("id") videoId: Int, @Query("position") position: Long)

    @GET("import/queue")
    suspend fun getImportQueue(): List<ImportQueueItem>
}
