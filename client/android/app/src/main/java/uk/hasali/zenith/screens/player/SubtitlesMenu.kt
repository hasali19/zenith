package uk.hasali.zenith.screens.player

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.playClick

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun SubtitlesMenu(
    subtitles: List<SubtitleTrack>,
    current: SubtitleTrack?,
    onSelectSubtitle: (SubtitleTrack?) -> Unit,
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
                SubtitleListItem(
                    primary = "None",
                    secondary = null,
                    selected = current == null,
                ) {
                    onSelectSubtitle(null)
                }
            }

            items(subtitles) {
                SubtitleListItem(
                    primary = it.language,
                    secondary = it.title,
                    selected = current == it,
                ) {
                    onSelectSubtitle(it)
                }
            }
        }
    }
}

@ExperimentalMaterialApi
@Composable
private fun SubtitleListItem(
    primary: String?,
    secondary: String?,
    selected: Boolean,
    onClick: () -> Unit
) {
    val context = LocalContext.current

    ListItem(
        secondaryText = if (secondary != null) ({ Text(secondary) }) else null,
        trailing = { if (selected) Icon(Icons.Default.Check, null) },
        modifier = Modifier.clickable {
            context.playClick()
            onClick()
        },
    ) {
        Text(primary ?: "Unknown")
    }
}
