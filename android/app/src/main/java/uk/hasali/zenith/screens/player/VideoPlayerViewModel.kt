package uk.hasali.zenith.screens.player

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.lifecycle.ViewModel
import dagger.hilt.android.lifecycle.HiltViewModel
import uk.hasali.zenith.media.MediaSessionManager
import uk.hasali.zenith.media.VideoPlayer
import javax.inject.Inject

@HiltViewModel
class VideoPlayerViewModel @Inject constructor(
    private val mediaSessionManager: MediaSessionManager,
) : ViewModel() {
    private val _player = mutableStateOf(mediaSessionManager.getCurrentPlayer())
    val player by _player

    private val mediaSessionManagerListener = object : MediaSessionManager.Listener {
        override fun onPlayerChanged() {
            _player.value = mediaSessionManager.getCurrentPlayer()
        }
    }

    init {
        mediaSessionManager.addListener(mediaSessionManagerListener)
    }

    override fun onCleared() {
        mediaSessionManager.removeListener(mediaSessionManagerListener)
    }

    fun stop() {
        mediaSessionManager.endCurrentSession()
    }
}
