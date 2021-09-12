package uk.hasali.zenith.ui

import android.widget.Toast
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowDropDown
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.DialogProperties
import kotlinx.coroutines.launch
import uk.hasali.zenith.*

private class EpisodeData(
    show: Show?,
    showName: String,
    season: String,
    episode: String,
) {
    val show = mutableStateOf(show)
    val showName = mutableStateOf(showName)
    val season = mutableStateOf(season)
    val episode = mutableStateOf(episode)
}

private class MovieData(
    title: String,
    year: String,
) {
    val title = mutableStateOf(title)
    val year = mutableStateOf(year)
}

@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun ImportItemDialog(
    item: ImportQueueItem,
    onDismiss: () -> Unit,
) {
    val client = LocalZenithClient.current
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val shows by produceState(emptyList<Show>()) {
        value = client.getShows()
    }

    var type by remember(item) {
        mutableStateOf(
            when (item.info) {
                is ImportQueueItemInfo.Movie -> ItemType.Movie
                else -> ItemType.Episode
            }
        )
    }

    val movie = remember(item) {
        if (item.info is ImportQueueItemInfo.Movie) {
            MovieData(
                title = item.info.title,
                year = item.info.year?.toString() ?: "",
            )
        } else {
            MovieData(
                title = "",
                year = "",
            )
        }
    }

    val episode = remember(item, shows) {
        if (item.info is ImportQueueItemInfo.Episode) {
            EpisodeData(
                show = shows.find { it.name.equals(item.info.name, ignoreCase = true) },
                showName = item.info.name,
                season = item.info.season.toString(),
                episode = item.info.episode.toString(),
            )
        } else {
            EpisodeData(
                show = null,
                showName = "",
                season = "",
                episode = "",
            )
        }
    }

    fun onImportClick() {
        scope.launch {
            try {
                val source = ImportSource.LocalImportSource(path = item.path)
                when (type) {
                    ItemType.Movie -> client.importMovie(source, movie.title.value, movie.year.value)
                    ItemType.Episode -> client.importEpisode(
                        source,
                        episode.show.value,
                        episode.showName.value,
                        episode.season.value,
                        episode.episode.value,
                    )
                }

                onDismiss()
            } catch (e: Exception) {
                Toast.makeText(context, e.message, Toast.LENGTH_SHORT)
                    .show()
            }
        }
    }

    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text(text = "Import") },
        text = {
            Column(modifier = Modifier.fillMaxWidth()) {
                Text(text = item.name)
                Spacer(modifier = Modifier.height(16.dp))
                Row {
                    ItemTypeRadioOption(label = "Movie", selected = type == ItemType.Movie) {
                        type = ItemType.Movie
                    }

                    ItemTypeRadioOption(label = "Episode", selected = type == ItemType.Episode) {
                        type = ItemType.Episode
                    }
                }

                Spacer(modifier = Modifier.height(8.dp))

                when (type) {
                    ItemType.Movie -> ImportMovieForm(
                        title = movie.title.value,
                        year = movie.year.value,
                        onTitleChange = { movie.title.value = it },
                        onYearChange = { movie.year.value = it },
                    )
                    ItemType.Episode -> {
                        ImportEpisodeForm(
                            shows = listOf(null) + shows,
                            show = episode.show.value,
                            showName = episode.showName.value,
                            season = episode.season.value,
                            episode = episode.episode.value,
                            onShowChange = { episode.show.value = it },
                            onShowNameChange = { episode.showName.value = it },
                            onSeasonChange = { episode.season.value = it },
                            onEpisodeChange = { episode.episode.value = it },
                        )
                    }
                }
            }
        },
        confirmButton = {
            TextButton(onClick = {
                context.playClick()
                onImportClick()
            }) {
                Text("Import")
            }
        },
        dismissButton = {
            TextButton(onClick = {
                context.playClick()
                onDismiss()
            }) {
                Text("Cancel")
            }
        },
        properties = DialogProperties(usePlatformDefaultWidth = false),
        modifier = Modifier.padding(32.dp),
    )
}

@Composable
private fun RowScope.ItemTypeRadioOption(label: String, selected: Boolean, onClick: () -> Unit) {
    val context = LocalContext.current

    Row(modifier = Modifier.weight(1f)) {
        RadioButton(
            selected = selected,
            onClick = {
                context.playClick()
                onClick()
            },
        )
        Text(
            text = label,
            modifier = Modifier
                .align(Alignment.CenterVertically)
                .padding(horizontal = 4.dp),
        )
    }
}

@Composable
private fun ImportMovieForm(
    title: String,
    year: String,
    onTitleChange: (String) -> Unit,
    onYearChange: (String) -> Unit,
) {
    OutlinedTextField(
        value = title,
        onValueChange = onTitleChange,
        label = { Text(text = "Title") },
    )

    Spacer(modifier = Modifier.height(8.dp))

    OutlinedTextField(
        value = year,
        onValueChange = onYearChange,
        label = { Text(text = "Year") },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
    )
}

@Composable
private fun ImportEpisodeForm(
    shows: List<Show?>,
    show: Show?,
    showName: String,
    season: String,
    episode: String,
    onShowChange: (Show?) -> Unit,
    onShowNameChange: (String) -> Unit,
    onSeasonChange: (String) -> Unit,
    onEpisodeChange: (String) -> Unit,
) {
    ShowSelectMenu(
        values = shows,
        text = { it?.name ?: "New Show" },
        selected = show,
        onChange = onShowChange,
    )

    Spacer(modifier = Modifier.height(8.dp))

    if (show == null) {
        OutlinedTextField(
            value = showName,
            onValueChange = onShowNameChange,
            label = { Text(text = "Show name") },
            modifier = Modifier.padding(bottom = 8.dp),
        )
    }

    Row {
        OutlinedTextField(
            value = season,
            onValueChange = onSeasonChange,
            label = { Text(text = "Season") },
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier.weight(1f),
        )

        Spacer(modifier = Modifier.width(8.dp))

        OutlinedTextField(
            value = episode,
            onValueChange = onEpisodeChange,
            label = { Text(text = "Episode") },
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier.weight(1f),
        )
    }
}

@Composable
private fun <T> ShowSelectMenu(
    values: List<T>,
    selected: T,
    text: (T) -> String,
    onChange: (T) -> Unit,
) {
    val context = LocalContext.current
    var expanded by remember { mutableStateOf(false) }

    Box {
        BoxWithConstraints {
            val width = maxWidth

            Column {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.clickable {
                        context.playClick()
                        expanded = true
                    },
                ) {
                    Text(
                        text = text(selected),
                        modifier = Modifier
                            .weight(1f)
                            .padding(horizontal = 8.dp, vertical = 16.dp),
                    )

                    Icon(Icons.Default.ArrowDropDown, contentDescription = "Expand")
                }

                DropdownMenu(
                    expanded = expanded,
                    onDismissRequest = { expanded = false },
                    modifier = Modifier.width(width)
                ) {
                    for (value in values) {
                        DropdownMenuItem(onClick = {
                            context.playClick()
                            expanded = false
                            onChange(value)
                        }) {
                            Text(text = text(value), maxLines = 1, overflow = TextOverflow.Ellipsis)
                        }
                    }
                }
            }
        }
    }
}

private enum class ItemType {
    Movie,
    Episode,
}

private suspend fun ZenithApiClient.importMovie(source: ImportSource, title: String, year: String) {
    importMovie(
        ImportMovieRequest(
            source = source,
            title = title,
            year = year.toInt(),
        )
    )
}

private suspend fun ZenithApiClient.importEpisode(
    source: ImportSource,
    show: Show?,
    showName: String,
    season: String,
    episode: String,
) {
    val data = ImportEpisodeRequest(
        source = source,
        seasonNumber = season.toInt(),
        episodeNumber = episode.toInt(),
    )

    if (show != null) {
        importEpisode(showId = show.id, episode = data)
    } else {
        importShow(
            ImportShowRequest(
                name = showName,
                episodes = listOf(data),
            ),
        )
    }
}
