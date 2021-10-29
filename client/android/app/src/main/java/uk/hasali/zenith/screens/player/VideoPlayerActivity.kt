package uk.hasali.zenith.screens.player

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.runtime.*
import androidx.core.view.WindowCompat
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import com.google.android.gms.cast.framework.CastContext
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.combineTransform
import kotlinx.coroutines.launch
import uk.hasali.zenith.ui.AppTheme
import uk.hasali.zenith.ui.rememberFlowWithLifecycle

@AndroidEntryPoint
class VideoPlayerActivity : ComponentActivity() {

    private val model: PlayerViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        // Initialise cast context
        CastContext.getSharedInstance(this)

        val navigateToExternalPlayer = { url: String ->
            finish()
            startActivity(Intent(Intent.ACTION_VIEW).apply {
                setDataAndType(Uri.parse(url), "video/*")
            })
        }

        setContent {
            val item by rememberFlowWithLifecycle(model.item)
                .collectAsState(null)

            AppTheme {
                item?.let {
                    VideoPlayer(item = it)
                }
            }
        }
    }

    @Composable
    private fun VideoPlayer(item: VideoItem) {
        var player by remember { mutableStateOf<VideoPlayer?>(null) }

        DisposableEffect(Unit) {
            LocalVideoPlayer(this@VideoPlayerActivity).let {
                player = it
                onDispose {
                    it.dispose()
                }
            }
        }

        DisposableEffect(player, item) {
            player.let {
                if (it != null) {
                    it.setItem(item)
                    onDispose {
                        it.stop()
                    }
                } else {
                    onDispose { }
                }
            }
        }

        LaunchedEffect(player, item) {
            player?.let { player ->
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

        FullScreen {
            KeepScreenOn {
                player?.let {
                    VideoPlayer(player = it)
                }
            }
        }
    }
}
