package uk.hasali.zenith.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp

@Composable
fun MediaItemWithPoster(
    poster: String,
    primary: String,
    secondary: String,
    isWatched: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    @Composable
    fun Content() {
        Column(modifier = Modifier.padding(vertical = 4.dp)) {
            Text(
                primary,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.subtitle2
            )

            Text(
                secondary,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.caption
            )
        }
    }

    Column(modifier = modifier) {
        Poster(url = poster, overlay = { WatchedOverlay(visible = isWatched) }, onClick = onClick)
        Content()
    }
}

@Composable
private fun WatchedOverlay(visible: Boolean) {
    if (!visible) return

    Box(modifier = Modifier.fillMaxSize()) {
        Box(
            modifier = Modifier
                .align(Alignment.TopEnd)
                .padding(4.dp)
                .background(Color.Black.copy(alpha = 0.4f), shape = CircleShape)
                .padding(4.dp),
        ) {
            Icon(
                imageVector = Icons.Default.Check,
                contentDescription = "Watched",
                tint = Color.White,
                modifier = Modifier.size(16.dp),
            )
        }
    }
}
