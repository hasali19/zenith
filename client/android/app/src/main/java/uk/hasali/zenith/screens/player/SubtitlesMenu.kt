package uk.hasali.zenith.screens.player

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
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
    current: SubtitleStreamInfo?,
    onSelectSubtitle: (SubtitleStreamInfo?) -> Unit,
) {
    Column {
        Text(
            text = "Subtitles",
            style = MaterialTheme.typography.subtitle2,
            modifier = Modifier.padding(16.dp),
        )
        Divider()
        LazyColumn(
            modifier = Modifier
                .fillMaxWidth()
                .heightIn(max = 400.dp),
        ) {
            item {
                SubtitleListItem(title = "None", language = null, selected = current == null) {
                    onSelectSubtitle(null)
                }
            }

            items(subtitles) {
                val label = it.title
                    ?: it.language
                    ?: when (it) {
                        is SubtitleStreamInfo.Embedded -> "Track ${it.index}"
                        is SubtitleStreamInfo.External -> it.path
                    }

                SubtitleListItem(title = label, language = it.language, selected = current == it) {
                    onSelectSubtitle(it)
                }
            }
        }
    }
}

@ExperimentalMaterialApi
@Composable
private fun SubtitleListItem(title: String, language: String?, selected: Boolean, onClick: () -> Unit) {
    val context = LocalContext.current

    ListItem(
        secondaryText = if (language != null) ({ Text(language) }) else null,
        trailing = { if (selected) Icon(Icons.Default.Check, null) },
        modifier = Modifier.clickable {
            context.playClick()
            onClick()
        },
    ) {
        Text(title)
    }
}
