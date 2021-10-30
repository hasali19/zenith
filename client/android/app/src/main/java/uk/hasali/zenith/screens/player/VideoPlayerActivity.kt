package uk.hasali.zenith.screens.player

import android.os.Bundle
import androidx.activity.compose.LocalOnBackPressedDispatcherOwner
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.runtime.*
import androidx.core.view.WindowCompat
import androidx.fragment.app.FragmentActivity
import com.google.accompanist.insets.ProvideWindowInsets
import com.google.android.gms.cast.framework.CastContext
import com.google.android.gms.cast.framework.CastSession
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.flow.collectLatest
import uk.hasali.zenith.ui.AppTheme
import uk.hasali.zenith.ui.rememberFlowWithLifecycle

@AndroidEntryPoint
class VideoPlayerActivity : FragmentActivity() {

    private val model: PlayerViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        val castContext = CastContext.getSharedInstance(this)
        val castSessionManager = castContext.sessionManager
        val castSession by mutableStateOf(castSessionManager.currentCastSession)

        setContent {
            val item by rememberFlowWithLifecycle(model.item)
                .collectAsState(null)

            AppTheme {
                ProvideWindowInsets {
                    item?.let {
                        VideoPlayer(item = it, castSession = castSession)
                    }
                }
            }
        }
    }

    @Composable
    private fun VideoPlayer(item: VideoItem, castSession: CastSession?) {
        var player by remember { mutableStateOf<VideoPlayer?>(null) }

        DisposableEffect(castSession) {
            castSession.let { session ->
                val playerImpl = when (session) {
                    null -> LocalVideoPlayer(this@VideoPlayerActivity)
                    else -> RemoteVideoPlayer(this@VideoPlayerActivity, session)
                }

                player = playerImpl
                onDispose {
                    playerImpl.dispose()
                }
            }
        }

        LaunchedEffect(player, item) {
            player?.let { player ->
                player.setItem(item)
                player.isPlaying.collectLatest { isPlaying ->
                    if (isPlaying) {
                        player.pollPosition(5000)
                            .collect {
                                model.updateProgress(it)
                            }
                    }
                }
            }
        }

        if (castSession == null) {
            KeepScreenOn()
            FullScreen()
        }

        val onBackPressedDispatcher = LocalOnBackPressedDispatcherOwner.current
            ?.onBackPressedDispatcher

        player?.let {
            VideoPlayer(
                player = it,
                autoHideControls = castSession == null,
                onClosePressed = {
                    it.stop()
                    onBackPressedDispatcher?.onBackPressed()
                },
            )
        }
    }
}
