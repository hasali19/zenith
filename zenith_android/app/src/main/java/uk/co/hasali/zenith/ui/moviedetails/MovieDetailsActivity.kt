package uk.co.hasali.zenith.ui.moviedetails

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
import uk.co.hasali.zenith.api.Movie

class MovieDetailsActivity : AppCompatActivity() {

    private lateinit var movie: Movie

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val movieId = intent.getIntExtra("movie_id", -1)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@MovieDetailsActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            movie = Fuel.get("$serverUrl/api/movies/$movieId")
                .awaitObject(gsonDeserializer())

            setContent {
                MovieDetailsScreen(
                    movie = movie,
                    onPlay = { playMovie() },
                    onBackPressed = { finish() }
                )
            }
        }
    }

    private fun playMovie() {
        startActivity(
            Intent(this, VideoPlayerActivity::class.java).apply {
                putExtra("stream_id", movie.id)
            }
        )
    }
}
