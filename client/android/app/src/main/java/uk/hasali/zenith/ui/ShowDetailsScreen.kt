package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Card
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.dp
import com.google.accompanist.coil.rememberCoilPainter
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.*

@Composable
fun ShowDetailsScreen(client: ZenithApiClient, navigator: Navigator, show: Show) {
    val seasons by produceState(initialValue = emptyList<Season>()) {
        value = client.getSeasons(show.id)
    }

    Surface(modifier = Modifier.fillMaxSize()) {
        BoxWithConstraints(modifier = Modifier.verticalScroll(rememberScrollState())) {
            Image(
                painter = rememberCoilPainter(request = show.backdrop, fadeIn = true),
                contentDescription = "Backdrop",
                modifier = Modifier.aspectRatio(16f / 9f)
            )

            val backdropHeight = with(LocalDensity.current) {
                (constraints.maxWidth * 9f / 16f).toDp()
            }

            Column(modifier = Modifier.padding(top = backdropHeight - 48.dp)) {
                Row(modifier = Modifier.padding(horizontal = 16.dp)) {
                    Card {
                        Image(
                            painter = rememberCoilPainter(request = show.poster, fadeIn = true),
                            contentDescription = "Poster",
                            modifier = Modifier
                                .width(140.dp)
                                .aspectRatio(2f / 3f),
                        )
                    }

                    Spacer(modifier = Modifier.width(16.dp))

                    Column(modifier = Modifier.align(Alignment.CenterVertically)) {
                        val year = Instant.fromEpochSeconds(show.startDate)
                            .toLocalDateTime(TimeZone.UTC)
                            .year

                        Text(show.name, style = MaterialTheme.typography.h5)
                        Text(year.toString(), style = MaterialTheme.typography.caption)
                    }
                }

                Spacer(modifier = Modifier.height(16.dp))

                Text(
                    show.overview,
                    style = MaterialTheme.typography.body2,
                    modifier = Modifier.padding(horizontal = 16.dp),
                )

                Spacer(modifier = Modifier.height(16.dp))

                LazyRow(contentPadding = PaddingValues(12.dp, 0.dp)) {
                    items(seasons.size) { i ->
                        val season = seasons[i]
                        Box(modifier = Modifier.width(120.dp)) {
                            MediaItemWithPoster(
                                poster = season.poster,
                                primary = season.name,
                                secondary = show.name,
                                onClick = {
                                    navigator.push(Screen.SeasonDetails(show, season))
                                }
                            )
                        }
                    }
                }
            }
        }
    }
}
