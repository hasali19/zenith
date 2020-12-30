package uk.co.hasali.zenith

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.WithConstraints
import androidx.compose.ui.platform.AmbientDensity
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import com.google.gson.annotations.SerializedName
import dev.chrisbanes.accompanist.coil.CoilImage
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.ui.ZenithTheme

data class TvShowDetails(
    val id: Int,
    val name: String,
    val overview: String?,
    @SerializedName("poster_url")
    val posterUrl: String?,
    @SerializedName("backdrop_url")
    val backdropUrl: String?,
    val episodes: List<TvShowEpisode>,
    @SerializedName("stream_id")
    val streamId: Int,
)

data class TvShowEpisode(
    val id: Int,
    val season: Int,
    val episode: Int,
    val name: String?,
    val overview: String?,
    @SerializedName("thumbnail_url")
    val thumbnailUrl: String?,
    @SerializedName("stream_id")
    val streamId: Int,
)

class TvShowDetailsActivity : AppCompatActivity() {

    private lateinit var show: TvShowDetails

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val showId = intent.getIntExtra("show_id", -1)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@TvShowDetailsActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            show = Fuel.get("$serverUrl/api/tv_shows/$showId")
                .awaitObject(gsonDeserializer())

            setContent {
                TvShowDetailsScreen()
            }
        }
    }

    @Composable
    fun TvShowDetailsScreen() {
        fun onEpisodeClick(streamId: Int) {
            startActivity(
                Intent(this@TvShowDetailsActivity, VideoPlayerActivity::class.java).apply {
                    putExtra("stream_id", streamId)
                }
            )
        }

        ZenithTheme {
            Surface(color = MaterialTheme.colors.background) {
                Box {
                    LazyColumn {
                        item {
                            WithConstraints {
                                val height = with(AmbientDensity.current) {
                                    constraints.maxWidth.toDp() * (9f / 16f)
                                }

                                Box(
                                    modifier = Modifier.preferredHeight(height)
                                ) {
                                    CoilImage(data = show.backdropUrl!!)
                                }

                                TopAppBar(
                                    title = { /* No title */ },
                                    backgroundColor = Color.Transparent,
                                    elevation = 0.dp,
                                    navigationIcon = {
                                        IconButton(onClick = { finish() }) {
                                            Icon(
                                                Icons.Default.ArrowBack
                                            )
                                        }
                                    },
                                )
                            }

                            Column(modifier = Modifier.padding(16.dp)) {
                                Column {
                                    Text(
                                        text = show.name,
                                        style = MaterialTheme.typography.h5
                                    )
                                }

                                Spacer(modifier = Modifier.preferredHeight(16.dp))
                                Text(text = show.overview ?: "")
                                Spacer(modifier = Modifier.preferredHeight(16.dp))

                                Text(text = "Episodes", style = MaterialTheme.typography.h6)
                            }
                        }

                        items(show.episodes) { episode ->
                            Card(
                                modifier = Modifier.fillMaxWidth()
                                    .wrapContentHeight()
                                    .padding(horizontal = 16.dp, vertical = 8.dp)
                                    .clickable { onEpisodeClick(episode.streamId) }
                            ) {
                                Row {
                                    Box(
                                        modifier = Modifier.preferredSize(160.dp, 90.dp)
                                            .background(Color.Black)
                                    ) {
                                        CoilImage(
                                            data = episode.thumbnailUrl!!,
                                            modifier = Modifier.fillMaxSize()
                                        )
                                    }
                                    Column(modifier = Modifier.padding(8.dp)) {
                                        Text(
                                            text = String.format(
                                                "S%02dE%02d",
                                                episode.season,
                                                episode.episode
                                            )
                                        )
                                        Text(
                                            text = episode.overview ?: "Unknown",
                                            style = MaterialTheme.typography.caption,
                                            maxLines = 2,
                                            overflow = TextOverflow.Ellipsis
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
