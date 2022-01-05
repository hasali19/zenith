package uk.hasali.zenith

import android.util.Log
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.channels.trySendBlocking
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.flow.collectLatest
import uk.hasali.zenith.api.ZenithMediaService
import uk.hasali.zenith.media.MediaSessionManager
import uk.hasali.zenith.media.pollPosition
import javax.inject.Inject

class MediaProgressReporter @Inject constructor(
    private val zenith: ZenithMediaService,
    private val mediaSessionManager: MediaSessionManager,
) {
    private val playerFlow = callbackFlow {
        send(mediaSessionManager.getCurrentPlayer())

        val listener = object : MediaSessionManager.Listener {
            override fun onPlayerChanged() {
                trySendBlocking(mediaSessionManager.getCurrentPlayer())
            }
        }

        mediaSessionManager.addListener(listener)
        awaitClose {
            mediaSessionManager.removeListener(listener)
        }
    }

    suspend fun run() {
        if (!BuildConfig.DEBUG) {
            playerFlow.collectLatest { player ->
                if (player != null && player.isLocal) {
                    player.currentItem.collectLatest { item ->
                        if (item != null) {
                            player.playWhenReady.collectLatest { playWhenReady ->
                                reportPosition(item.id, player.position)
                                if (playWhenReady) {
                                    player.pollPosition(delayMs = 5000).collectLatest { position ->
                                        reportPosition(item.id, position)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    private suspend fun reportPosition(id: Int, position: Long) {
        try {
            zenith.updateProgress(id, position / 1000)
        } catch (t: Throwable) {
            Log.e("MediaProgressReporter", "Failed to report media position: ${t.message}")
        }
    }
}
