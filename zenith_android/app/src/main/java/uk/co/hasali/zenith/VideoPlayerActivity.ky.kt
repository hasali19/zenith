package uk.co.hasali.zenith

import android.os.Build
import android.os.Bundle
import android.support.v4.media.session.MediaSessionCompat
import android.support.v4.media.session.PlaybackStateCompat
import android.view.*
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.ComposeView
import androidx.compose.ui.res.vectorResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import com.google.android.exoplayer2.*
import com.google.android.exoplayer2.ext.mediasession.MediaSessionConnector
import com.google.android.exoplayer2.ui.AspectRatioFrameLayout
import com.google.android.exoplayer2.video.VideoListener
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch

class VideoPlayerActivity : AppCompatActivity() {

    data class StreamInfo(val duration: Float)

    enum class PlaybackState {
        PLAYING,
        PAUSED,
    }

    private var player: SimpleExoPlayer? = null
    private var session: MediaSessionCompat? = null
    private var connector: MediaSessionConnector? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_video_player)

        val streamId = intent.getIntExtra("stream_id", -1)

        val aspectRatioLayout: AspectRatioFrameLayout = findViewById(R.id.aspect_ratio_layout)
        val surfaceView: SurfaceView = findViewById(R.id.surface_view)
        val composeView: ComposeView = findViewById(R.id.compose_view)

        var playbackState by mutableStateOf(PlaybackState.PLAYING)
        var playbackPosition by mutableStateOf(0L)
        var duration by mutableStateOf(0L)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@VideoPlayerActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            val info: StreamInfo = Fuel.get("$serverUrl/api/stream/$streamId/info")
                .awaitObject(gsonDeserializer())

            duration = info.duration.toLong()

            player = SimpleExoPlayer.Builder(this@VideoPlayerActivity)
                .build()
                .apply {
                    setVideoSurfaceView(surfaceView)

                    addVideoListener(object : VideoListener {
                        override fun onVideoSizeChanged(
                            width: Int,
                            height: Int,
                            unappliedRotationDegrees: Int,
                            pixelWidthHeightRatio: Float
                        ) {
                            // Set the aspect ratio for the SurfaceView
                            val aspectRatio =
                                if (width == 0 || height == 0) 1f
                                else (width * pixelWidthHeightRatio) / height

                            aspectRatioLayout.setAspectRatio(aspectRatio)
                        }
                    })

                    addListener(object : Player.EventListener {
                        override fun onPlayWhenReadyChanged(playWhenReady: Boolean, reason: Int) {
                            if (playWhenReady) {
                                playbackState = PlaybackState.PLAYING
                            } else {
                                playbackState = PlaybackState.PAUSED
                            }
                        }
                    })

                    setMediaItem(MediaItem.fromUri("$serverUrl/api/stream/$streamId"))

                    prepare()
                    play()
                }

            launch {
                while (player.let { it != null && it.playbackState != Player.STATE_ENDED }) {
                    playbackPosition = player?.currentPosition ?: 0
                    delay(1000)
                }
            }

            val session = MediaSessionCompat(this@VideoPlayerActivity, "ZenithMediaSession").apply {
                isActive = true
                session = this
            }

            connector = MediaSessionConnector(session).apply {
                setPlayer(player)
                setControlDispatcher(object : DefaultControlDispatcher() {})
            }
        }

        composeView.setContent {
            val position = playbackPosition.toFloat() / 1000
            val range = 0f..duration.toFloat()

            Box(modifier = Modifier.fillMaxSize()) {
                Column(
                    modifier = Modifier.align(Alignment.BottomCenter)
                        .background(Color(0f, 0f, 0f, 0.5f))
                        .padding(16.dp)
                ) {
                    Slider(position, {}, valueRange = range)
                    Row(
                        horizontalArrangement = Arrangement.Center,
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        FloatingActionButton(onClick = {
                            player?.let { it.playWhenReady = !it.playWhenReady }
                        }) {
                            Icon(
                                vectorResource(
                                    id = when (playbackState) {
                                        PlaybackState.PAUSED -> R.drawable.play
                                        PlaybackState.PLAYING -> R.drawable.pause
                                    }
                                )
                            )
                        }
                    }
                }
            }
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
        player?.pause()
    }

    override fun onResume() {
        super.onResume()
        player?.play()
    }

    override fun onDestroy() {
        super.onDestroy()
        player?.release()
        session?.release()
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

    private fun showSystemUi() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.R) {
            @Suppress("DEPRECATION")
            window.decorView.systemUiVisibility = (View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                    or View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                    or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN)
        } else {
            TODO()
        }
    }
}
