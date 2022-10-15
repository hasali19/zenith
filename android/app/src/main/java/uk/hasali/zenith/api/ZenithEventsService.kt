package uk.hasali.zenith.api

import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.channels.SendChannel
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.channels.onFailure
import kotlinx.coroutines.channels.trySendBlocking
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.flow.first
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.Response
import okhttp3.sse.EventSource
import okhttp3.sse.EventSourceListener
import okhttp3.sse.EventSources
import uk.hasali.zenith.Preferences
import java.util.concurrent.TimeUnit
import javax.inject.Inject

class ZenithEventsService @Inject constructor(private val preferences: Preferences) {
    private val sseClient = EventSourceClient()

    suspend fun getTranscoderEvents(): Flow<TranscoderEvent> {
        val server = preferences.serverUrl.first()
        val url = "$server/api/transcoder/events"
        return sseClient.get(url)
    }

    private class EventSourceClient {
        private val client = OkHttpClient.Builder()
            .readTimeout(0, TimeUnit.SECONDS)
            .build()

        private val eventSourceFactory = EventSources.createFactory(client)

        private class Listener<T>(
            private val deserializer: DeserializationStrategy<T>,
            private val channel: SendChannel<T>
        ) : EventSourceListener() {
            override fun onEvent(eventSource: EventSource, id: String?, type: String?, data: String) {
                channel.trySendBlocking(Json.decodeFromString(deserializer, data))
                    .onFailure { channel.close(it) }
            }

            override fun onClosed(eventSource: EventSource) {
                channel.close()
            }

            override fun onFailure(
                eventSource: EventSource,
                t: Throwable?,
                response: Response?
            ) {
                channel.close(t)
            }
        }

        @OptIn(ExperimentalCoroutinesApi::class)
        inline fun <reified T> get(url: String) = callbackFlow<T> {
            val req = Request.Builder()
                .url(url)
                .build()

            val source = eventSourceFactory.newEventSource(req, Listener(serializer(), channel))

            awaitClose {
                source.cancel()
            }
        }
    }
}
