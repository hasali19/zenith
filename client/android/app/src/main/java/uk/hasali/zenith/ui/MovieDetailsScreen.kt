package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.Column
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Movie

@Composable
fun MovieDetailsScreen(movie: Movie) {
    val navigator = LocalNavigator.current

    VideoItemDetailsScreen(id = movie.id,
        backdrop = movie.backdrop,
        poster = movie.poster,
        overview = movie.overview,
        isWatched = false /* TODO */,
        headerContent = { HeaderContent(movie = movie) },
        onPlay = { replay ->
            navigator.push(Screen.Player(movie.id, movie.title, replay))
        }
    )
}

@Composable
private fun HeaderContent(movie: Movie) {
    Column {
        val year = Instant.fromEpochSeconds(movie.releaseDate)
            .toLocalDateTime(TimeZone.UTC)
            .year

        Text(movie.title, style = MaterialTheme.typography.h6)
        Text(year.toString(), style = MaterialTheme.typography.caption)
    }
}
