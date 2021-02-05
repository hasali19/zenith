package uk.co.hasali.zenith

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.AmbientDensity
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import dev.chrisbanes.accompanist.coil.CoilImage
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.api.Movie
import uk.co.hasali.zenith.ui.ZenithTheme

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
                MovieDetailsScreen()
            }
        }
    }

    @Composable
    fun MovieDetailsScreen() {
        fun onPlayButtonClick() {
            startActivity(
                Intent(this@MovieDetailsActivity, VideoPlayerActivity::class.java).apply {
                    putExtra("stream_id", movie.id)
                }
            )
        }

        val scrollState = rememberScrollState()

        ZenithTheme {
            Surface(color = MaterialTheme.colors.background) {
                Box {
                    ConstraintLayout(
                        modifier = Modifier
                            .fillMaxSize()
                            .verticalScroll(state = scrollState)
                    ) {
                        val (backdrop, fab, content) = createRefs()

                        BoxWithConstraints(
                            modifier = Modifier.constrainAs(backdrop) {
                                top.linkTo(parent.top)
                                start.linkTo(parent.start)
                                end.linkTo(parent.end)
                            }
                        ) {
                            val height = with(AmbientDensity.current) {
                                constraints.maxWidth.toDp() * (9f / 16f)
                            }

                            Box(
                                modifier = Modifier.preferredHeight(height)
                            ) {
                                movie.backdrop?.let { url ->
                                    CoilImage(data = url, contentDescription = null)
                                }
                            }
                        }

                        FloatingActionButton(
                            onClick = { onPlayButtonClick() },
                            modifier = Modifier
                                .padding(32.dp)
                                .constrainAs(fab) {
                                    end.linkTo(parent.end)
                                    centerAround(backdrop.bottom)
                                },
                        ) {
                            Icon(Icons.Default.PlayArrow, "Play")
                        }

                        Column(
                            modifier = Modifier
                                .padding(horizontal = 16.dp, vertical = 32.dp)
                                .constrainAs(content) {
                                    top.linkTo(backdrop.bottom)
                                    start.linkTo(parent.start)
                                    end.linkTo(parent.end)
                                }
                        ) {
                            Column {
                                Text(
                                    text = movie.title,
                                    style = MaterialTheme.typography.h6
                                )

                                Row {
                                    Text(
                                        text = movie.releaseYear?.toString().orEmpty(),
                                        style = MaterialTheme.typography.body2
                                    )

                                    Text(
                                        text = "\u2022",
                                        style = MaterialTheme.typography.body2,
                                        modifier = Modifier.padding(horizontal = 8.dp),
                                    )

                                    Text(
                                        text = formatDuration(movie.duration),
                                        style = MaterialTheme.typography.body2
                                    )
                                }
                            }

                            Spacer(modifier = Modifier.preferredHeight(16.dp))
                            Text(text = movie.overview ?: "")
                        }
                    }

                    TopAppBar(
                        title = { /* No title */ },
                        backgroundColor = Color.Transparent,
                        elevation = 0.dp,
                        navigationIcon = {
                            IconButton(onClick = { finish() }) {
                                Icon(Icons.Default.ArrowBack, "Back")
                            }
                        },
                    )
                }
            }
        }
    }
}

private fun formatDuration(duration: Double): String {
    val value = duration.toLong()
    return if (value <= 90 * 60) {
        "${value / 60}m"
    } else {
        val hours = value / 3600
        val minutes = (value % 3600) / 60
        "${hours}h ${minutes}m"
    }
}
