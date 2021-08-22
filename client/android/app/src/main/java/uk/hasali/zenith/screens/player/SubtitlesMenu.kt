package uk.hasali.zenith.screens.player

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.SubtitleStreamInfo
import uk.hasali.zenith.playClick

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun SubtitlesMenu(
    subtitles: List<SubtitleStreamInfo>,
    onSelectItem: (SubtitleStreamInfo?) -> Unit,
    onDismiss: () -> Unit,
) {
    val context = LocalContext.current

    AlertDialog(
        onDismissRequest = onDismiss,
        text = {
            Column {
                Text("Subtitles")
                Spacer(modifier = Modifier.height(16.dp))
                LazyColumn(
                    modifier = Modifier
                        .fillMaxWidth()
                        .heightIn(max = 400.dp),
                ) {
                    item {
                        ListItem(
                            text = { Text("None") },
                            modifier = Modifier.clickable {
                                context.playClick()
                                onSelectItem(null)
                                onDismiss()
                            },
                        )
                    }

                    items(subtitles) {
                        ListItem(
                            text = {
                                val label = it.title
                                    ?: it.language
                                    ?: when (it) {
                                        is SubtitleStreamInfo.Embedded -> "Track ${it.index}"
                                        is SubtitleStreamInfo.External -> it.path
                                    }

                                Text(label)
                            },
                            modifier = Modifier.clickable {
                                context.playClick()
                                onSelectItem(it)
                                onDismiss()
                            },
                        )
                    }
                }
            }
        },
        confirmButton = {
            TextButton(
                onClick = {
                    context.playClick()
                    onDismiss()
                },
            ) {
                Text("Close")
            }
        },
    )
}
