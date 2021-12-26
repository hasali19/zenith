package uk.hasali.zenith.screens.player

import androidx.lifecycle.ViewModel
import dagger.hilt.android.lifecycle.HiltViewModel
import uk.hasali.zenith.media.MediaSessionManager
import javax.inject.Inject

@HiltViewModel
class VideoPlayerViewModel @Inject constructor(
    private val mediaSessionManager: MediaSessionManager,
) : ViewModel() {
    val session get() = mediaSessionManager.current

    fun stop() {
        mediaSessionManager.stop()
    }
}
