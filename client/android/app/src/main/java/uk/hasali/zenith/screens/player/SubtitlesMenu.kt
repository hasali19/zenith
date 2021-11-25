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
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.LanguageCodes
import java.util.*

private data class SubtitleItem(
    val language: String,
    val title: String?,
    val track: SubtitleTrack,
)

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun SubtitlesMenu(
    subtitles: List<SubtitleTrack>,
    current: SubtitleTrack?,
    onSelectSubtitle: (SubtitleTrack?) -> Unit,
    modifier: Modifier = Modifier,
) {
    val items = remember(subtitles) {
        subtitles
            .map {
                var language = "Unknown"

                it.language?.let { code ->
                    // Locale requires alpha2 code for languages that have both
                    // so we need to convert it first if possible
                    val tag = LanguageCodes.getAlpha3(code) ?: code
                    val locale = Locale.forLanguageTag(tag)
                    language = locale.displayName
                }

                SubtitleItem(
                    language = language,
                    title = it.title,
                    track = it,
                )
            }
            .sortedWith(
                compareBy(
                    { it.language },
                    { it.title },
                    { it.track.id },
                ),
            )
    }

    Column(modifier = modifier) {
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

            items(items) {
                SubtitleListItem(
                    primary = it.language,
                    secondary = it.title,
                    selected = current == it.track,
                ) {
                    onSelectSubtitle(it.track)
                }
            }
        }
    }
}

@ExperimentalMaterialApi
@Composable
private fun SubtitleListItem(
    primary: String,
    secondary: String?,
    selected: Boolean,
    onClick: () -> Unit
) {
    ListItem(
        secondaryText = if (secondary != null) ({ Text(secondary) }) else null,
        trailing = { if (selected) Icon(Icons.Default.Check, null) },
        modifier = Modifier.clickable(onClick = onClick),
    ) {
        Text(primary)
    }
}
