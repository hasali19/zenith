package uk.hasali.zenith.ui

import androidx.compose.runtime.Composable
import androidx.compose.ui.viewinterop.AndroidView
import androidx.mediarouter.app.MediaRouteButton
import androidx.mediarouter.media.MediaControlIntent
import androidx.mediarouter.media.MediaRouteSelector
import com.google.android.gms.cast.framework.CastButtonFactory

@Composable
fun CastButton() {
    AndroidView(
        factory = {
            MediaRouteButton(it).apply {
                routeSelector = MediaRouteSelector.Builder()
                    .addControlCategory(MediaControlIntent.CATEGORY_REMOTE_PLAYBACK)
                    .build()

                CastButtonFactory.setUpMediaRouteButton(it, this)
            }
        }
    )
}
