package uk.co.hasali.zenith.ui.videoplayer

import android.os.Build
import android.os.Bundle
import android.support.v4.media.session.MediaSessionCompat
import android.view.*
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.lifecycle.lifecycleScope
import androidx.work.*
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import com.google.android.exoplayer2.*
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.R
import uk.co.hasali.zenith.UserSettingsRepository
import uk.co.hasali.zenith.databinding.ActivityVideoPlayerBinding

private data class StreamInfo(val duration: Float)

private const val MEDIA_SESSION_TAG = "ZenithMediaSession"

class VideoPlayerActivity : AppCompatActivity() {

    companion object {
        const val EXTRA_ITEM_ID = "item_id"
    }

    private lateinit var view: ActivityVideoPlayerBinding

    private lateinit var player: Player
    private lateinit var session: MediaSessionCompat

    private var itemId = -1
    private var playbackState by mutableStateOf(PlayState.PLAYING)
    private var playbackPosition by mutableStateOf(0f)
    private var bufferedPosition by mutableStateOf(0f)
    private var duration by mutableStateOf(0f)
    private var buffering by mutableStateOf(false)
    private var serverUrl: String? by mutableStateOf(null)

    private var progressUpdateJob: Job? = null

    @OptIn(ExperimentalMaterialApi::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        view = ActivityVideoPlayerBinding.inflate(layoutInflater)
        setContentView(view.root)

        itemId = intent.getIntExtra(EXTRA_ITEM_ID, -1)
        if (itemId == -1) {
            val message = getString(R.string.player_invalid_item_id)
            Toast.makeText(this, message, Toast.LENGTH_SHORT).show()
            finish()
        }

        session = MediaSessionCompat(this, MEDIA_SESSION_TAG)
        player = Player(this, view.surfaceView, session).apply {
            onVideoSizeChanged = { _, _, aspectRatio ->
                view.aspectRatioLayout.setAspectRatio(aspectRatio)
            }

            onVideoBufferingChanged = {
                buffering = it
            }

            onVideoPlaybackStateChanged = {
                playbackState = it
                when (it) {
                    PlayState.PAUSED -> progressUpdateJob?.cancel()
                    PlayState.PLAYING -> {
                        progressUpdateJob?.cancel()
                        progressUpdateJob = launchProgressUpdate()
                    }
                }
            }

            onVideoEnded = {
                finish()
            }
        }

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@VideoPlayerActivity)
            val settings = settingsRepo.settings.first()

            serverUrl = settings.serverUrl!!

            val info: StreamInfo = Fuel.get("$serverUrl/api/stream/$itemId/info")
                .awaitObject(gsonDeserializer())

            duration = info.duration

            player.setVideoItem(object : VideoItem {
                override fun getUrlForPosition(position: Float): String {
                    return "$serverUrl/api/stream/$itemId/transcode?start=${position.toLong()}"
                }
            })

            player.play()
            session.isActive = true
        }

        view.composeView.setContent {
            ControlsOverlay(
                buffering = buffering,
                position = playbackPosition,
                buffered = bufferedPosition,
                duration = duration,
                state = playbackState,
                onPlayPause = { player.state = player.state.toggle() },
                onSeekStart = { player.pause() },
                onSeekTo = {
                    player.seekTo(it)
                    player.play()
                },
            )
        }
    }

    private fun launchProgressUpdate() = lifecycleScope.launch {
        while (isActive && !player.isEnded) {
            playbackPosition = player.position
            bufferedPosition = player.bufferedPosition
            delay(1000)
        }
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        if (hasFocus) {
            hideSystemUi()
        }
    }

    override fun onPause() {
        super.onPause()
        player.pause()
    }

    override fun onResume() {
        super.onResume()
        player.play()
    }

    override fun onDestroy() {
        super.onDestroy()
        player.release()
        session.release()
    }

    private fun hideSystemUi() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R) {
            @Suppress("DEPRECATION")
            window.decorView.systemUiVisibility = (View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                    or View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                    or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                    or View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                    or View.SYSTEM_UI_FLAG_FULLSCREEN)
        } else {
            TODO()
        }
    }
}
