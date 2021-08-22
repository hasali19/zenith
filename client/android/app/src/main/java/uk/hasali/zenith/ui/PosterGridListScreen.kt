package uk.hasali.zenith.ui

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun <T> PosterGridListScreen(
    items: List<T>?,
    poster: (T) -> String,
    name: (T) -> String,
    date: (T) -> Long?,
    isWatched: (T) -> Boolean = { false },
    onClick: (T) -> Unit,
) {
    when (items) {
        null -> CenteredLoadingIndicator()
        else -> LazyVerticalGrid(
            cells = GridCells.Adaptive(120.dp),
            contentPadding = PaddingValues(4.dp),
        ) {
            items(items.size) { i ->
                val item = items[i]
                val dateVal = date(item)
                val year = if (dateVal == null) null else
                    Instant.fromEpochSeconds(dateVal)
                        .toLocalDateTime(TimeZone.UTC)
                        .year

                MediaItemWithPoster(
                    poster = poster(item),
                    primary = name(item),
                    secondary = year?.toString() ?: "",
                    isWatched = isWatched(item),
                    onClick = { onClick(item) },
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(4.dp),
                )
            }
        }
    }
}
