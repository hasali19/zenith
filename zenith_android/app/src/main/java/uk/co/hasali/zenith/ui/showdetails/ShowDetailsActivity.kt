package uk.co.hasali.zenith.ui.showdetails

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
import uk.co.hasali.zenith.ui.episodes.EpisodesActivity
import uk.co.hasali.zenith.UserSettingsRepository
import uk.co.hasali.zenith.api.TvSeason
import uk.co.hasali.zenith.api.TvShow

class ShowDetailsActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val showId = intent.getIntExtra("show_id", -1)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@ShowDetailsActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            val show: TvShow = Fuel.get("$serverUrl/api/tv/shows/$showId")
                .awaitObject(gsonDeserializer())

            val seasons: List<TvSeason> = Fuel.get("$serverUrl/api/tv/shows/$showId/seasons")
                .awaitObject(gsonDeserializer())

            setContent {
                TvShowDetailsScreen(
                    show = show,
                    seasons = seasons,
                    onSeasonClick = { navigateToSeason(it) },
                    onBackPressed = { finish() }
                )
            }
        }
    }

    private fun navigateToSeason(season: TvSeason) {
        startActivity(
            Intent(this, EpisodesActivity::class.java).apply {
                putExtra("season_id", season.id)
            }
        )
    }
}
