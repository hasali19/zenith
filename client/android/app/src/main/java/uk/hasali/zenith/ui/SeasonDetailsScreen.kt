package uk.hasali.zenith.ui

import android.content.Context
import android.media.AudioManager
import android.view.SoundEffectConstants
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.google.accompanist.coil.rememberCoilPainter
import io.ktor.client.*
import io.ktor.client.request.*
import uk.hasali.zenith.*

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun SeasonDetailsScreen(client: HttpClient, navigator: Navigator, season: Season) {
    val context = LocalContext.current
    val episodes by produceState(initialValue = emptyList<Episode>()) {
        value = client.get("https://zenith.hasali.uk/api/tv/seasons/${season.id}/episodes")
    }

    Scaffold(topBar = { AppBar(title = season.name, navigator = navigator) }) {
        LazyVerticalGrid(cells = GridCells.Adaptive(200.dp), contentPadding = PaddingValues(4.dp)) {
            items(episodes.size) { i ->
                val episode = episodes[i]

                BoxWithConstraints(modifier = Modifier.padding(4.dp)) {
                    with(LocalDensity.current) {
                        val width = constraints.maxWidth
                        val height = width * (9.0 / 16.0)

                        Column {
                            Card {
                                Image(
                                    painter = rememberCoilPainter(
                                        request = episode.thumbnail,
                                        fadeIn = true
                                    ),
                                    contentDescription = "Thumbnail",
                                    modifier = Modifier
                                        .size(
                                            width.toDp(),
                                            height
                                                .toInt()
                                                .toDp(),
                                        )
                                        .clickable {
                                            val audioManager =
                                                context.getSystemService(Context.AUDIO_SERVICE) as AudioManager
                                            audioManager.playSoundEffect(
                                                SoundEffectConstants.CLICK,
                                                1.0f
                                            )
                                            navigator.push(Screen.Player(episode.id))
                                        }
                                )

                                if (episode.isWatched) {
                                    Box(
                                        modifier = Modifier
                                            .size(
                                                width.toDp(),
                                                height
                                                    .toInt()
                                                    .toDp(),
                                            )
                                            .background(Color.Black.copy(alpha = 0.4f))
                                    ) {
                                        Icon(
                                            Icons.Default.Check,
                                            contentDescription = "Watched",
                                            modifier = Modifier.align(Alignment.Center),
                                        )
                                    }
                                }
                            }

                            Column(modifier = Modifier.padding(vertical = 4.dp)) {
                                val duration = if (episode.duration <= 90 * 60) {
                                    val minutes = (episode.duration / 60).toInt()
                                    "${minutes}m"
                                } else {
                                    val hours = (episode.duration / 3600).toInt()
                                    val minutes = ((episode.duration % 3600) / 60).toInt()
                                    "${hours}h ${minutes}m"

                                }

                                Text(
                                    episode.name,
                                    maxLines = 1,
                                    overflow = TextOverflow.Ellipsis,
                                    style = MaterialTheme.typography.subtitle2
                                )

                                Text(
                                    duration,
                                    maxLines = 1,
                                    overflow = TextOverflow.Ellipsis,
                                    color = Color.LightGray.copy(alpha = 0.8f),
                                    style = MaterialTheme.typography.caption
                                )

                                Text(
                                    episode.overview,
                                    maxLines = 3,
                                    overflow = TextOverflow.Ellipsis,
                                    style = MaterialTheme.typography.caption
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
