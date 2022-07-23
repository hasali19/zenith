package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun <T> PosterGridListScreen(
    items: List<T>?,
    poster: (T) -> String?,
    name: (T) -> String,
    year: (T) -> Int?,
    isWatched: (T) -> Boolean = { false },
    onClick: (T) -> Unit,
) {
    when (items) {
        null -> CenteredLoadingIndicator()
        else -> LazyVerticalGrid(
            columns = GridCells.Adaptive(120.dp),
            contentPadding = PaddingValues(4.dp),
        ) {
            items(items.size) { i ->
                val item = items[i]

                MediaItemWithPoster(
                    poster = poster(item),
                    primary = name(item),
                    secondary = year(item)?.toString() ?: "",
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
