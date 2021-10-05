package uk.hasali.zenith.ui

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Checkbox
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlin.math.floor

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun <T> PosterGridListScreen(
    items: List<T>?,
    poster: (T) -> String?,
    name: (T) -> String,
    date: (T) -> Long?,
    isWatched: (T) -> Boolean = { false },
    onClick: (T) -> Unit,
) {
    when (items) {
        null -> CenteredLoadingIndicator()
        // else -> LazyVerticalGrid(
        //     cells = GridCells.Adaptive(120.dp),
        //     contentPadding = PaddingValues(4.dp),
        // ) {
        //     item {
        //         Row {
        //             Text("Show watched")
        //             Checkbox(checked = true, onCheckedChange = {})
        //         }
        //     }
        //     items(items.size) { i ->
        //         val item = items[i]
        //         val dateVal = date(item)
        //         val year = if (dateVal == null) null else
        //             Instant.fromEpochSeconds(dateVal)
        //                 .toLocalDateTime(TimeZone.UTC)
        //                 .year
        //
        //         MediaItemWithPoster(
        //             poster = poster(item),
        //             primary = name(item),
        //             secondary = year?.toString() ?: "",
        //             isWatched = isWatched(item),
        //             onClick = { onClick(item) },
        //             modifier = Modifier
        //                 .fillMaxWidth()
        //                 .padding(4.dp),
        //         )
        //     }
        // }
        else -> BoxWithConstraints {
            val columns = floor(maxWidth / 120.dp).toInt()

            LazyColumn(contentPadding = PaddingValues(4.dp)) {
                item {
                    Row(
                        horizontalArrangement = Arrangement.End,
                        verticalAlignment = Alignment.CenterVertically,
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(4.dp),
                    ) {
                        Text("Show watched", style = MaterialTheme.typography.body2)
                        Checkbox(checked = true, onCheckedChange = {})
                    }
                }
                items(items.chunked(columns)) { row ->
                    Row {
                        for (item in row) {
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
                                    .weight(1f)
                                    .padding(4.dp),
                            )
                        }
                    }
                }
            }
        }
    }
}
