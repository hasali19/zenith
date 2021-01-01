package uk.co.hasali.zenith

import android.os.Build
import android.os.Bundle
import android.support.v4.media.session.MediaSessionCompat
import android.view.SurfaceView
import android.view.View
import android.view.ViewGroup
import android.widget.FrameLayout
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.viewinterop.AndroidView
import androidx.lifecycle.lifecycleScope
import com.google.android.exoplayer2.*
import com.google.android.exoplayer2.ext.mediasession.MediaSessionConnector
import com.google.android.exoplayer2.video.VideoListener
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch

class VideoPlayerActivity : AppCompatActivity() {

    private var player: SimpleExoPlayer? = null
    private var session: MediaSessionCompat? = null
    private var connector: MediaSessionConnector? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val streamId = intent.getIntExtra("stream_id", -1)

        var aspectRatio by mutableStateOf(16f / 9f)
        val surfaceView = SurfaceView(this).apply {
            layoutParams = ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT,
            )
        }

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@VideoPlayerActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

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
                            aspectRatio =
                                if (width == 0 || height == 0) 1f
                                else (width * pixelWidthHeightRatio) / height
                        }
                    })

                    setMediaItem(MediaItem.fromUri("$serverUrl/api/stream/$streamId"))

                    prepare()
                    play()
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

        setContent {
            Box(modifier = Modifier.fillMaxSize().background(Color.Black)) {
                AndroidView(
                    viewBlock = { FrameLayout(it).apply { addView(surfaceView) } },
                    modifier = Modifier.aspectRatio(aspectRatio).align(Alignment.Center),
                )
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
