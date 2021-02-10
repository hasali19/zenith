package uk.co.hasali.zenith.ui.moviedetails

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
import uk.co.hasali.zenith.api.Movie
import uk.co.hasali.zenith.ui.videoplayer.VideoPlayerActivity

class MovieDetailsActivity : AppCompatActivity() {

    private lateinit var movie: Movie

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        WindowCompat.setDecorFitsSystemWindows(window, false)

        val movieId = intent.getIntExtra("movie_id", -1)
        var movie: Movie? by mutableStateOf(null)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@MovieDetailsActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            movie = Fuel.get("$serverUrl/api/movies/$movieId")
                .awaitObject(gsonDeserializer())
        }

        setContent {
            ProvideWindowInsets {
                movie?.let { movie ->
                    MovieDetailsScreen(
                        movie = movie,
                        onPlay = { playMovie() },
                        onBackPressed = { finish() }
                    )
                }
            }
        }
    }

    private fun playMovie() {
        startActivity(
            Intent(this, VideoPlayerActivity::class.java).apply {
                putExtra(VideoPlayerActivity.EXTRA_ITEM_ID, movie.id)
            }
        )
    }
}
