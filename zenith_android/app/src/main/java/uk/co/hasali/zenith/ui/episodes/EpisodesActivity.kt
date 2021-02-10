package uk.co.hasali.zenith.ui.episodes

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.ui.platform.setContent
import androidx.core.view.WindowCompat
import androidx.lifecycle.lifecycleScope
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import dev.chrisbanes.accompanist.insets.ProvideWindowInsets
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.UserSettingsRepository
import uk.co.hasali.zenith.api.TvEpisode
import uk.co.hasali.zenith.api.TvSeason
import uk.co.hasali.zenith.api.TvShow
import uk.co.hasali.zenith.ui.videoplayer.VideoPlayerActivity

class EpisodesActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        WindowCompat.setDecorFitsSystemWindows(window, false)

        val seasonId = intent.getIntExtra("season_id", -1)

        var show: TvShow? by mutableStateOf(null)
        var season: TvSeason? by mutableStateOf(null)
        var episodes: List<TvEpisode> by mutableStateOf(listOf())

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@EpisodesActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            season = Fuel.get("$serverUrl/api/tv/seasons/$seasonId")
                .awaitObject(gsonDeserializer())

            show = Fuel.get("$serverUrl/api/tv/shows/${season!!.showId}")
                .awaitObject(gsonDeserializer())

            episodes = Fuel.get("$serverUrl/api/tv/seasons/$seasonId/episodes")
                .awaitObject(gsonDeserializer())
        }

        setContent {
            ProvideWindowInsets {
                if (show != null && season != null) {
                    EpisodesScreen(
                        show = show!!,
                        season = season!!,
                        episodes = episodes,
                        onEpisodeClick = { playEpisode(it) },
                        onBackPressed = { finish() },
                    )
                }
            }
        }
    }

    private fun playEpisode(episode: TvEpisode) {
        startActivity(
            Intent(this, VideoPlayerActivity::class.java).apply {
                putExtra(VideoPlayerActivity.EXTRA_ITEM_ID, episode.id)
            }
        )
    }
}
