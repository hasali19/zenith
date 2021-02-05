package uk.co.hasali.zenith.ui.episodes

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.ui.platform.setContent
import androidx.lifecycle.lifecycleScope
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.UserSettingsRepository
import uk.co.hasali.zenith.ui.videoplayer.VideoPlayerActivity
import uk.co.hasali.zenith.api.TvEpisode
import uk.co.hasali.zenith.api.TvSeason
import uk.co.hasali.zenith.api.TvShow

class EpisodesActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val seasonId = intent.getIntExtra("season_id", -1)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@EpisodesActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            val season: TvSeason = Fuel.get("$serverUrl/api/tv/seasons/$seasonId")
                .awaitObject(gsonDeserializer())

            val show: TvShow = Fuel.get("$serverUrl/api/tv/shows/${season.showId}")
                .awaitObject(gsonDeserializer())

            val episodes: List<TvEpisode> = Fuel.get("$serverUrl/api/tv/seasons/$seasonId/episodes")
                .awaitObject(gsonDeserializer())

            setContent {
                EpisodesScreen(
                    show = show,
                    season = season,
                    episodes = episodes,
                    onEpisodeClick = { playEpisode(it) },
                    onBackPressed = { finish() },
                )
            }
        }
    }

    private fun playEpisode(episode: TvEpisode) {
        startActivity(
            Intent(this, VideoPlayerActivity::class.java).apply {
                putExtra("stream_id", episode.id)
            }
        )
    }
}
