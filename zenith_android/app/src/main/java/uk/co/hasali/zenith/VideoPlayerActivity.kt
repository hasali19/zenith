package uk.co.hasali.zenith

import android.content.Context
import android.os.Build
import android.os.Bundle
import android.support.v4.media.session.MediaSessionCompat
import android.view.*
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Settings
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.gesture.tapGestureFilter
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.ComposeView
import androidx.compose.ui.res.vectorResource
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import androidx.work.*
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.core.awaitUnit
import com.github.kittinunf.fuel.coroutines.awaitUnit
import com.google.android.exoplayer2.*
import com.google.android.exoplayer2.ext.mediasession.MediaSessionConnector
import com.google.android.exoplayer2.ui.AspectRatioFrameLayout
import com.google.android.exoplayer2.util.MimeTypes
import com.google.android.exoplayer2.video.VideoListener
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch

class VideoPlayerActivity : AppCompatActivity() {

    enum class PlaybackState {
        PLAYING,
        PAUSED,
    }

    private var streamId: Int? = null
    private var serverUrl: String? = null

    private var player: SimpleExoPlayer? = null
    private var session: MediaSessionCompat? = null
    private var connector: MediaSessionConnector? = null

    @OptIn(ExperimentalMaterialApi::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_video_player)

        streamId = intent.getIntExtra("stream_id", -1)

        val aspectRatioLayout: AspectRatioFrameLayout = findViewById(R.id.aspect_ratio_layout)
        val surfaceView: SurfaceView = findViewById(R.id.surface_view)
        val composeView: ComposeView = findViewById(R.id.compose_view)

        var showControls by mutableStateOf(false)
        var playbackState by mutableStateOf(PlaybackState.PLAYING)
        var playbackPosition by mutableStateOf(0L)
        var duration by mutableStateOf(0L)
        var buffering by mutableStateOf(false)
        var useTranscoding by mutableStateOf(false)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@VideoPlayerActivity)
            val settings = settingsRepo.settings.first()

            serverUrl = settings.serverUrl!!

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
                            playbackState = if (playWhenReady) {
                                PlaybackState.PLAYING
                            } else {
                                PlaybackState.PAUSED
                            }
                        }

                        override fun onPlaybackStateChanged(state: Int) {
                            buffering = state == ExoPlayer.STATE_BUFFERING
                            when (state) {
                                ExoPlayer.STATE_READY -> duration = this@apply.duration
                                ExoPlayer.STATE_ENDED -> finish()
                                else -> {
                                }
                            }
                        }
                    })

                    // TODO: Add option to play transcoded stream
                    val item = MediaItem.Builder()
                        .setUri("$serverUrl/api/stream/$streamId/original")
                        .build()

                    setMediaItem(item)

                    prepare()
                    play()
                }

            launch {
                while (player.let { it != null && it.playbackState != Player.STATE_ENDED }) {
                    if (player?.playWhenReady == true) {
                        playbackPosition = player?.currentPosition ?: 0
                    }
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
            val optionsState = rememberModalBottomSheetState(ModalBottomSheetValue.Hidden)

            ModalBottomSheetLayout(
                sheetState = optionsState,
                sheetContent = {
                    ListItem(
                        // modifier = Modifier.clickable { useTranscoding = !useTranscoding },
                        text = { Text("Transcoding") },
                        trailing = {
                            Switch(
                                checked = useTranscoding,
                                onCheckedChange = { checked ->
                                    if (checked) {
                                        player?.let { player ->
                                            val pos = player.currentPosition

                                            player.setMediaItem(
                                                MediaItem.Builder()
                                                    .setUri("$serverUrl/api/stream/$streamId/hls")
                                                    .setMimeType(MimeTypes.APPLICATION_M3U8)
                                                    .build()
                                            )

                                            player.seekTo(pos)
                                        }
                                    } else {
                                        lifecycleScope.launch {
                                            player?.let { player ->
                                                val pos = player.currentPosition

                                                player.stop()

                                                Fuel.post("$serverUrl/api/stream/$streamId/hls/stop")
                                                    .awaitUnit(scope = Dispatchers.IO)

                                                player.setMediaItem(
                                                    MediaItem.Builder()
                                                        .setUri("$serverUrl/api/stream/$streamId/original")
                                                        .build()
                                                )

                                                player.prepare()
                                                player.play()

                                                player.seekTo(pos)
                                            }
                                        }
                                    }

                                    useTranscoding = checked
                                }
                            )
                        }
                    )
                }
            ) {
                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .tapGestureFilter { showControls = !showControls }
                ) {
                    if (buffering) {
                        CircularProgressIndicator(modifier = Modifier.align(Alignment.Center))
                    }

                    if (showControls) {
                        Column(
                            modifier = Modifier
                                .align(Alignment.BottomCenter)
                                .background(Color(0f, 0f, 0f, 0.5f))
                                .padding(8.dp)
                                .tapGestureFilter { /* Intercept tap */ }
                        ) {
                            SeekBar(
                                position = position,
                                max = duration.toFloat() / 1000,
                                onSeekStart = { player?.playWhenReady = false },
                                onSeekEnd = { pos ->
                                    player?.seekTo((pos * 1000).toLong())
                                    player?.playWhenReady = true
                                }
                            )

                            ConstraintLayout(modifier = Modifier.fillMaxWidth()) {
                                val (center, right) = createRefs()

                                FloatingActionButton(
                                    modifier = Modifier.constrainAs(center) {
                                        top.linkTo(parent.top)
                                        bottom.linkTo(parent.bottom)
                                        start.linkTo(parent.start)
                                        end.linkTo(parent.end)
                                    },
                                    onClick = {
                                        player?.let { it.playWhenReady = !it.playWhenReady }
                                    }
                                ) {
                                    Icon(
                                        vectorResource(
                                            id = when (playbackState) {
                                                PlaybackState.PAUSED -> R.drawable.play
                                                PlaybackState.PLAYING -> R.drawable.pause
                                            }
                                        )
                                    )
                                }

                                IconButton(
                                    onClick = { optionsState.show() },
                                    modifier = Modifier
                                        .constrainAs(right) {
                                            top.linkTo(parent.top)
                                            bottom.linkTo(parent.bottom)
                                            end.linkTo(parent.end)
                                        }
                                ) {
                                    Icon(
                                        imageVector = Icons.Default.Settings,
                                        tint = Color.White
                                    )
                                }
                            }
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

    class CleanupWorker(context: Context, params: WorkerParameters) :
        CoroutineWorker(context, params) {
        override suspend fun doWork(): Result {
            val uri = inputData.getString("URI") ?: return Result.failure()

            Fuel.post(uri)
                .awaitUnit()

            return Result.success()
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

        if (serverUrl != null && streamId != null) {
            WorkManager
                .getInstance(this)
                .enqueue(
                    OneTimeWorkRequestBuilder<CleanupWorker>()
                        .setInputData(workDataOf("URI" to "$serverUrl/api/stream/$streamId/hls/stop"))
                        .build()
                )
        }
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
