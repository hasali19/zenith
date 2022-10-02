package uk.hasali.zenith.screens.library.itemdetails

import androidx.compose.foundation.layout.Column
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import uk.hasali.zenith.api.Movie
import uk.hasali.zenith.ui.BottomSheetController
import uk.hasali.zenith.ui.formatDuration

@Composable
fun MovieDetailsScreen(
    movie: Movie,
    bottomSheetController: BottomSheetController,
    onSetWatched: (Boolean) -> Unit,
    onPlay: (Double?) -> Unit,
    onTranscode: () -> Unit,
    onRefreshMetadata: () -> Unit,
    onImportSubtitle: (String, ByteArray) -> Unit,
    onNavigateUp: () -> Unit,
) {
    VideoItemDetailsScreen(
        name = movie.title,
        backdrop = movie.backdrop,
        poster = movie.poster,
        overview = movie.overview,
        headerContent = { HeaderContent(movie = movie) },
        video = movie,
        bottomSheetController = bottomSheetController,
        onSetWatched = onSetWatched,
        onPlay = onPlay,
        onConvertVideo = onTranscode,
        onRefreshMetadata = onRefreshMetadata,
        onImportSubtitle = onImportSubtitle,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun HeaderContent(movie: Movie) {
    val duration = formatDuration(movie.videoInfo.duration)
    val year = movie.releaseYear()

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
