package uk.co.hasali.zenith

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import dev.chrisbanes.accompanist.coil.CoilImage
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.api.TvEpisode
import uk.co.hasali.zenith.api.TvSeason
import uk.co.hasali.zenith.api.TvShow
import uk.co.hasali.zenith.ui.ZenithTheme

class TvEpisodesActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val seasonId = intent.getIntExtra("season_id", -1)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@TvEpisodesActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl!!

            val season: TvSeason = Fuel.get("$serverUrl/api/tv/seasons/$seasonId")
                .awaitObject(gsonDeserializer())

            val show: TvShow = Fuel.get("$serverUrl/api/tv/shows/${season.showId}")
                .awaitObject(gsonDeserializer())

            val episodes: List<TvEpisode> = Fuel.get("$serverUrl/api/tv/seasons/$seasonId/episodes")
                .awaitObject(gsonDeserializer())

            setContent {
                fun onItemClick(streamId: Int) {
                    startActivity(
                        Intent(this@TvEpisodesActivity, VideoPlayerActivity::class.java).apply {
                            putExtra("stream_id", streamId)
                        }
                    )
                }

                ZenithTheme {
                    Scaffold(topBar = {
                        TopAppBar(
                            title = {
                                Column {
                                    Text(text = show.name)
                                    Text(
                                        text = season.name ?: "Season ${season.seasonNumber}",
                                        style = MaterialTheme.typography.caption
                                    )
                                }
                            }, navigationIcon = {
                                IconButton(onClick = { finish() }) {
                                    Icon(
                                        Icons.Default.ArrowBack
                                    )
                                }
                            }
                        )
                    }) {
                        val itemHeight = 96
                        val itemWidth = itemHeight * (16f / 9f)

                        LazyColumn {
                            items(episodes) { episode ->
                                Box(modifier = Modifier.clickable { onItemClick(episode.id) }) {
                                    Row(
                                        modifier = Modifier
                                            .preferredHeight(itemHeight.dp)
                                            .padding(8.dp)
                                    ) {
                                        Box(modifier = Modifier.preferredWidth(itemWidth.dp)) {
                                            if (episode.thumbnail != null) {
                                                CoilImage(
                                                    data = episode.thumbnail,
                                                    modifier = Modifier.align(
                                                        Alignment.Center
                                                    )
                                                )
                                            }
                                        }

                                        val name = episode.name ?: String.format(
                                            "S%02dE%02d",
                                            season.seasonNumber,
                                            episode.episodeNumber
                                        )

                                        Column(modifier = Modifier.align(Alignment.CenterVertically)) {
                                            Text(
                                                text = "${episode.episodeNumber} - $name",
                                                style = MaterialTheme.typography.body2,
                                                overflow = TextOverflow.Ellipsis,
                                                maxLines = 1,
                                            )
                                            Text(
                                                text = episode.overview ?: "",
                                                style = MaterialTheme.typography.caption,
                                                overflow = TextOverflow.Ellipsis,
                                                maxLines = 4,
                                                color = Color.Gray,
                                            )
                                        }
                                    }
                                }

                                Divider()
                            }
                        }
                    }
                }
            }
        }
    }
}
