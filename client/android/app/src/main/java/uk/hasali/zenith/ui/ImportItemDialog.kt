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
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import uk.hasali.zenith.*

@Composable
fun ImportItemDialog(
    item: ImportQueueItem,
    onDismiss: () -> Unit,
) {
    val client = LocalZenithClient.current
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    var type by remember { mutableStateOf(ItemType.Episode) }

    var show: Show? by remember { mutableStateOf(null) }
    var showName by remember { mutableStateOf("") }
    var season by remember { mutableStateOf("") }
    var episode by remember { mutableStateOf("") }

    var title by remember { mutableStateOf("") }
    var year by remember { mutableStateOf("") }

    fun onImportClick() {
        scope.launch {
            try {
                val source = ImportSource.LocalImportSource(path = item.path)
                when (type) {
                    ItemType.Movie -> client.importMovie(source, title, year)
                    ItemType.Episode -> client.importEpisode(
                        source,
                        show,
                        showName,
                        season,
                        episode,
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
                        title = title,
                        year = year,
                        onTitleChange = { title = it },
                        onYearChange = { year = it },
                    )
                    ItemType.Episode -> {
                        val shows by produceState(emptyList<Show?>()) {
                            value = listOf(null) + client.getShows()
                        }

                        ImportEpisodeForm(
                            shows = shows,
                            show = show,
                            showName = showName,
                            season = season,
                            episode = episode,
                            onShowChange = { show = it },
                            onShowNameChange = { showName = it },
                            onSeasonChange = { season = it },
                            onEpisodeChange = { episode = it },
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
