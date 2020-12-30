package uk.co.hasali.zenith

import android.os.Bundle
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

    override fun onDestroy() {
        super.onDestroy()
        player?.release()
    }
}
