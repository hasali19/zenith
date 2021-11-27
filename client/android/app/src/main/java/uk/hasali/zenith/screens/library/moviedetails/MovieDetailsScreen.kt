package uk.hasali.zenith.screens.library.moviedetails

import androidx.compose.foundation.layout.Column
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Movie
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.*

@Composable
fun MovieDetailsScreen(
    model: MovieDetailsViewModel = hiltViewModel(),
    bottomSheetController: BottomSheetController,
    onPlay: (position: Double?) -> Unit,
    onNavigateUp: () -> Unit
) {
    val movie by rememberFlowWithLifecycle(model.movie)
        .collectAsState(null)

    MovieDetailsScreen(
        movie = movie,
        bottomSheetController = bottomSheetController,
        onSetWatched = model::setWatched,
        onPlay = onPlay,
        onTranscode = model::startTranscode,
        onRefreshMetadata = model::refreshMetadata,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun MovieDetailsScreen(
    movie: Movie?,
    bottomSheetController: BottomSheetController,
    onSetWatched: (Boolean) -> Unit,
    onPlay: (Double?) -> Unit,
    onTranscode: () -> Unit,
    onRefreshMetadata: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    when (movie) {
        null -> CenteredLoadingIndicator()
        else -> VideoItemDetailsScreen(
            name = movie.title,
            backdrop = movie.backdrop,
            poster = movie.poster,
            overview = movie.overview,
            headerContent = { HeaderContent(movie = movie) },
            info = movie.videoInfo,
            userData = movie.userData,
            bottomSheetController = bottomSheetController,
            onSetWatched = onSetWatched,
            onPlay = onPlay,
            onConvertVideo = onTranscode,
            onRefreshMetadata = onRefreshMetadata,
            onNavigateUp = onNavigateUp,
        )
    }
}

@Composable
private fun HeaderContent(movie: Movie) {
    val duration = displayDuration(movie.videoInfo.duration)
    val year = movie.releaseDate?.let {
        Instant.fromEpochSeconds(it)
            .toLocalDateTime(TimeZone.UTC)
            .year
    }

    val secondary = if (year != null) {
        "$year - $duration"
    } else {
        duration
    }

    Column {
        Text(movie.title, style = MaterialTheme.typography.h6)
        Text(secondary, style = MaterialTheme.typography.caption)
    }
}
