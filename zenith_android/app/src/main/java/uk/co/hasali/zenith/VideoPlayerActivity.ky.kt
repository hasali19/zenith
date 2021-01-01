package uk.co.hasali.zenith

import android.os.Build
import android.os.Bundle
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import com.google.android.exoplayer2.MediaItem
import com.google.android.exoplayer2.SimpleExoPlayer
import com.google.android.exoplayer2.ui.PlayerView
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch

class VideoPlayerActivity : AppCompatActivity() {

    private var player: SimpleExoPlayer? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_video_player)

        val streamId = intent.getIntExtra("stream_id", -1)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@VideoPlayerActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            val playerView = findViewById<PlayerView>(R.id.player_view)

            player = SimpleExoPlayer.Builder(this@VideoPlayerActivity)
                .build()
                .apply {
                    setMediaItem(MediaItem.fromUri("$serverUrl/api/stream/$streamId"))
                    prepare()
                    play()
                }

            playerView.player = player
        }
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        if (hasFocus) {
            hideSystemUi()
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        player?.release()
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
