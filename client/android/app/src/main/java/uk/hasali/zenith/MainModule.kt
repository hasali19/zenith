package uk.hasali.zenith

import com.jakewharton.retrofit2.converter.kotlinx.serialization.asConverterFactory
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.components.ActivityRetainedComponent
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import retrofit2.Retrofit
import retrofit2.create
import uk.hasali.zenith.api.ZenithMediaService

@Module
@InstallIn(ActivityRetainedComponent::class)
object MainModule {
    @OptIn(ExperimentalSerializationApi::class)
    @Provides
    fun provideGitHubService(): GitHubService {
        val mediaType = "application/json".toMediaType()
        val format = Json {
            ignoreUnknownKeys = true
        }

        return Retrofit.Builder()
            .baseUrl("https://api.github.com")
            .addConverterFactory(format.asConverterFactory(mediaType))
            .build()
            .create()
    }

    @OptIn(ExperimentalSerializationApi::class)
    @Provides
    fun provideZenithMediaService(preferences: Preferences): ZenithMediaService {
        val mediaType = "application/json".toMediaType()
        val format = Json {
            ignoreUnknownKeys = true
        }

        // TODO: Temporary hack
        val server = runBlocking { preferences.serverUrl.first() }

        return Retrofit.Builder()
            .baseUrl("$server/api/")
            .addConverterFactory(format.asConverterFactory(mediaType))
            .build()
            .create()
    }
}
