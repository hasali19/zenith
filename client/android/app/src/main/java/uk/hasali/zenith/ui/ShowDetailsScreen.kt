package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
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
import com.google.accompanist.insets.navigationBarsPadding
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show
import uk.hasali.zenith.ZenithApiClient

@Composable
fun ShowDetailsScreen(show: Show) {
    val client = LocalZenithClient.current
    val navigator = LocalNavigator.current

    val seasons by produceState(initialValue = emptyList<Season>()) {
        value = client.getSeasons(show.id)
    }

    Surface(
        modifier = Modifier
            .fillMaxSize()
            .navigationBarsPadding(),
    ) {
        BoxWithConstraints(modifier = Modifier.verticalScroll(rememberSaveableScrollState())) {
            Image(
                painter = rememberCoilPainter(request = show.backdrop, fadeIn = true),
                contentDescription = "Backdrop",
                modifier = Modifier.aspectRatio(16f / 9f)
            )

            val backdropHeight = with(LocalDensity.current) {
                (constraints.maxWidth * 9f / 16f).toDp()
            }

            Column(modifier = Modifier.padding(top = backdropHeight - 48.dp)) {
                HeaderSection(show = show)
                Spacer(modifier = Modifier.height(16.dp))
                OverviewSection(content = show.overview)
                Spacer(modifier = Modifier.height(16.dp))
                SeasonsSection(
                    show = show,
                    seasons = seasons,
                    onItemClick = { navigator.push(Screen.SeasonDetails(show, it)) },
                )
            }
        }
    }
}

@Composable
private fun HeaderSection(show: Show) {
    Row(modifier = Modifier.padding(horizontal = 16.dp)) {
        Poster(url = show.poster, modifier = Modifier.width(150.dp))
        Spacer(modifier = Modifier.width(16.dp))
        Column(modifier = Modifier.align(Alignment.CenterVertically)) {
            val year = Instant.fromEpochSeconds(show.startDate)
                .toLocalDateTime(TimeZone.UTC)
                .year

            Text(show.name, style = MaterialTheme.typography.h5)
            Text(year.toString(), style = MaterialTheme.typography.caption)
        }
    }
}

@Composable
private fun OverviewSection(content: String) {
    Text(
        text = "Overview",
        style = MaterialTheme.typography.h6,
        modifier = Modifier.padding(horizontal = 16.dp),
    )
    Spacer(modifier = Modifier.height(8.dp))
    Text(
        text = content,
        style = MaterialTheme.typography.body2,
        modifier = Modifier.padding(horizontal = 16.dp),
    )
}

@Composable
private fun SeasonsSection(show: Show, seasons: List<Season>, onItemClick: (Season) -> Unit) {
    Text(
        text = "Seasons",
        style = MaterialTheme.typography.h6,
        modifier = Modifier.padding(horizontal = 16.dp),
    )
    Spacer(modifier = Modifier.height(8.dp))
    SeasonsList(
        show = show,
        seasons = seasons,
        onItemClick = onItemClick,
    )
}

@Composable
private fun SeasonsList(show: Show, seasons: List<Season>, onItemClick: (Season) -> Unit) {
    LazyRow(contentPadding = PaddingValues(12.dp, 0.dp)) {
        items(seasons) { season ->
            MediaItemWithPoster(
                poster = season.poster,
                primary = season.name,
                secondary = show.name,
                onClick = { onItemClick(season) },
                modifier = Modifier
                    .width(120.dp)
                    .padding(4.dp)
            )
        }
    }
}
