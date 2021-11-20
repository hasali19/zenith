package uk.hasali.zenith

import androidx.compose.runtime.compositionLocalOf
import kotlinx.coroutines.flow.StateFlow

interface PictureInPictureController {
    val isInPictureInPictureMode: StateFlow<Boolean>
    fun setEnterOnUserLeaveHint(value: Boolean)
}

val LocalPictureInPictureController = compositionLocalOf<PictureInPictureController> {
    error("No PictureInPictureController found")
}
