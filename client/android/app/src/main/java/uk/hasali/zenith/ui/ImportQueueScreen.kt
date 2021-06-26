package uk.hasali.zenith.ui

import android.widget.Toast
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowDropDown
import androidx.compose.material.icons.filled.Videocam
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
fun rememberImportQueue(client: ZenithApiClient): Pair<List<ImportQueueItem>, suspend () -> Unit> {
    var items by remember { mutableStateOf(emptyList<ImportQueueItem>()) }

    val refresh = suspend {
        items = client.getImportQueue()
    }

    LaunchedEffect(client) {
        refresh()
    }

    return Pair(items, refresh)
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun ImportQueueScreen() {
    val client = LocalZenithClient.current
    val navigator = LocalNavigator.current
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val (items, refresh) = rememberImportQueue(client)
    var selected: ImportQueueItem? by remember { mutableStateOf(null) }

    Scaffold(topBar = { AppBar(navigator = navigator, title = "Import queue", menu = false) }) {
        LazyColumn(state = rememberSaveableLazyListState()) {
            items(items) {
                ListItem(
                    icon = {
                        Icon(Icons.Default.Videocam, contentDescription = "Video")
                    },
                    secondaryText = {
                        Text(
                            text = it.path,
                            maxLines = 2,
                            overflow = TextOverflow.Ellipsis,
                            modifier = Modifier.padding(bottom = 12.dp),
                        )
                    },
                    modifier = Modifier.clickable {
                        context.playClick()
                        selected = it
                    },
                ) {
                    Text(text = it.name)
                }
            }
        }

        selected?.let {
            ImportDialog(
                client = client,
                item = it,
                onDismiss = {
                    selected = null
                    scope.launch { refresh() }
                },
            )
        }
    }
}

private enum class ItemType {
    Movie,
    Episode,
}

@Composable
private fun ImportDialog(client: ZenithApiClient, item: ImportQueueItem, onDismiss: () -> Unit) {
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
        val source = ImportSource.LocalImportSource(path = item.path)
        scope.launch {
            try {
                when (type) {
                    ItemType.Movie -> {
                        client.importMovie(
                            ImportMovieRequest(
                                source = source,
                                title = title,
                                year = year.toInt(),
                            )
                        )
                    }
                    ItemType.Episode -> {
                        val data = ImportEpisodeRequest(
                            source = ImportSource.LocalImportSource(path = item.path),
                            seasonNumber = season.toInt(),
                            episodeNumber = episode.toInt(),
                        )

                        show.let { show ->
                            if (show != null) {
                                client.importEpisode(showId = show.id, episode = data)
                            } else {
                                client.importShow(
                                    show = ImportShowRequest(
                                        name = showName,
                                        episodes = listOf(data),
                                    ),
                                )
                            }
                        }
                    }
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
                    Row(modifier = Modifier.weight(1f)) {
                        RadioButton(
                            selected = type == ItemType.Movie,
                            onClick = {
                                context.playClick()
                                type = ItemType.Movie
                            },
                        )
                        Text(
                            text = "Movie",
                            modifier = Modifier
                                .align(Alignment.CenterVertically)
                                .padding(horizontal = 4.dp),
                        )
                    }
                    Row(modifier = Modifier.weight(1f)) {
                        RadioButton(
                            selected = type == ItemType.Episode,
                            onClick = {
                                context.playClick()
                                type = ItemType.Episode
                            },
                        )
                        Text(
                            text = "Episode",
                            modifier = Modifier
                                .align(Alignment.CenterVertically)
                                .padding(horizontal = 4.dp),
                        )
                    }
                }

                Spacer(modifier = Modifier.height(8.dp))

                when (type) {
                    ItemType.Movie -> {
                        OutlinedTextField(
                            value = title,
                            onValueChange = { title = it },
                            label = { Text(text = "Title") },
                        )
                        Spacer(modifier = Modifier.height(8.dp))
                        OutlinedTextField(
                            value = year,
                            onValueChange = { year = it },
                            label = { Text(text = "Year") },
                            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        )
                    }
                    ItemType.Episode -> {
                        val shows by produceState(emptyList<Show?>()) {
                            value = listOf(null) + client.getShows()
                        }

                        ShowSelect(
                            values = shows,
                            text = { it?.name ?: "New Show" },
                            selected = show,
                            onChange = { show = it },
                        )
                        Spacer(modifier = Modifier.height(8.dp))
                        if (show == null) {
                            OutlinedTextField(
                                value = showName,
                                onValueChange = { showName = it },
                                label = { Text(text = "Show name") },
                            )
                            Spacer(modifier = Modifier.height(8.dp))
                        }
                        Row {
                            OutlinedTextField(
                                value = season,
                                onValueChange = { season = it },
                                label = { Text(text = "Season") },
                                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                                modifier = Modifier.weight(1f),
                            )
                            Spacer(modifier = Modifier.width(8.dp))
                            OutlinedTextField(
                                value = episode,
                                onValueChange = { episode = it },
                                label = { Text(text = "Episode") },
                                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                                modifier = Modifier.weight(1f),
                            )
                        }
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
private fun <T> ShowSelect(
    values: List<T>,
    selected: T,
    text: (T) -> String,
    onChange: (T) -> Unit
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
