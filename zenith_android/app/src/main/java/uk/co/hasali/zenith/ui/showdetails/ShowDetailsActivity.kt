package uk.co.hasali.zenith.ui.showdetails

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
import uk.co.hasali.zenith.api.TvSeason
import uk.co.hasali.zenith.api.TvShow
import uk.co.hasali.zenith.ui.episodes.EpisodesActivity

class ShowDetailsActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        WindowCompat.setDecorFitsSystemWindows(window, false)

        val showId = intent.getIntExtra("show_id", -1)

        var show: TvShow? by mutableStateOf(null)
        var seasons: List<TvSeason> by mutableStateOf(listOf())

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@ShowDetailsActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            show = Fuel.get("$serverUrl/api/tv/shows/$showId")
                .awaitObject(gsonDeserializer())

            seasons = Fuel.get("$serverUrl/api/tv/shows/$showId/seasons")
                .awaitObject(gsonDeserializer())
        }

        setContent {
            ProvideWindowInsets {
                show?.let { show ->
                    TvShowDetailsScreen(
                        show = show,
                        seasons = seasons,
                        onSeasonClick = { navigateToSeason(it) },
                        onBackPressed = { finish() }
                    )
                }
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
