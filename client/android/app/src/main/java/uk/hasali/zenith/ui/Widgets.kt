package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.google.accompanist.coil.rememberCoilPainter
import com.google.accompanist.insets.statusBarsHeight
import uk.hasali.zenith.playClick

@Composable
fun AppBar(navigator: Navigator, title: String = "Zenith", menu: Boolean = true) {
    Surface(color = MaterialTheme.colors.primarySurface, elevation = 4.dp) {
        Column {
            Spacer(modifier = Modifier.statusBarsHeight())
            TopAppBar(
                title = { Text(title) },
                backgroundColor = Color.Transparent,
                elevation = 0.dp,
                actions = {
                    if (menu) {
                        AppBarMenu(navigator = navigator)
                    }
                }
            )
        }
    }
}

@Composable
fun AppBarMenu(navigator: Navigator) {
    val context = LocalContext.current
    var expanded by remember { mutableStateOf(false) }

    IconButton(onClick = {
        context.playClick()
        expanded = true
    }) {
        Icon(Icons.Default.MoreVert, contentDescription = "More")
    }

    DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
        DropdownMenuItem(onClick = {
            context.playClick()
            expanded = false
            navigator.push(Screen.TranscodeQueue)
        }) {
            Text("Transcode queue")
        }
    }
}

@Composable
fun Poster(url: String, modifier: Modifier = Modifier, onClick: (() -> Unit)? = null) {
    val context = LocalContext.current

    BoxWithConstraints(modifier = modifier) {
        val width = with(LocalDensity.current) {
            constraints.maxWidth.toDp()
        }

        Card(modifier = Modifier.size(width, width * (3f / 2f))) {
            Image(
                painter = rememberCoilPainter(request = url, fadeIn = true),
                contentDescription = "Poster",
                modifier = modifier
                    .fillMaxSize()
                    .clickable(enabled = onClick != null) {
                        context.playClick()
                        onClick?.invoke()
                    },
            )
        }
    }
}

@Composable
fun Thumbnail(
    url: String,
    modifier: Modifier = Modifier,
    overlay: (@Composable () -> Unit)? = null,
    onClick: (() -> Unit)? = null,
) {
    val context = LocalContext.current

    BoxWithConstraints(modifier = modifier) {
        val width = with(LocalDensity.current) {
            constraints.maxWidth.toDp()
        }

        Card(modifier = Modifier.size(width, width * (9f / 16f))) {
            Image(
                painter = rememberCoilPainter(request = url, fadeIn = true),
                contentDescription = "Thumbnail",
                modifier = Modifier
                    .fillMaxSize()
                    .clickable(enabled = onClick != null) {
                        context.playClick()
                        onClick?.invoke()
                    },
            )

            overlay?.invoke()
        }
    }
}

@Composable
fun MediaItemWithPoster(
    poster: String,
    primary: String,
    secondary: String,
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
        Poster(url = poster, onClick = onClick)
        Content()
    }
}
