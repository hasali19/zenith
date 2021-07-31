package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import coil.compose.rememberImagePainter
import com.google.accompanist.insets.statusBarsHeight
import com.google.accompanist.insets.statusBarsPadding
import uk.hasali.zenith.playClick

@Composable
fun AppBar(
    title: String? = null,
    menu: Boolean = true,
    backButton: Boolean = true,
) {
    val navigator = LocalNavigator.current
    val navigationIcon: (@Composable () -> Unit)? = if (!backButton) null else {
        {
            IconButton(onClick = { navigator.pop() }) {
                Icon(Icons.Default.ArrowBack, contentDescription = "Back")
            }
        }
    }

    Surface(color = MaterialTheme.colors.primarySurface, elevation = 4.dp) {
        Column {
            Spacer(modifier = Modifier.statusBarsHeight())
            TopAppBar(
                title = { Text(title ?: "") },
                navigationIcon = navigationIcon,
                backgroundColor = Color.Transparent,
                elevation = 0.dp,
                actions = {
                    CastButton()
                    if (menu) {
                        AppBarMenu(navigator = navigator)
                    }
                },
            )
        }
    }
}

@Composable
fun AppBarMenu(navigator: Navigator) {
    val context = LocalContext.current
    var expanded by remember { mutableStateOf(false) }

    Box {
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
                navigator.push(Screen.ImportQueue)
            }) {
                Text("Import queue")
            }

            DropdownMenuItem(onClick = {
                context.playClick()
                expanded = false
                navigator.push(Screen.TranscodeQueue)
            }) {
                Text("Transcode queue")
            }
        }
    }
}

@Composable
fun FadingAppBar(alpha: Float) {
    val navigator = LocalNavigator.current

    Box {
        Surface(
            content = {},
            color = MaterialTheme.colors.primarySurface,
            elevation = 4.dp,
            modifier = Modifier
                .matchParentSize()
                .graphicsLayer { this.alpha = minOf(alpha, 1f) },
        )
        TopAppBar(
            title = { },
            navigationIcon = {
                IconButton(onClick = { navigator.pop() }) {
                    Icon(
                        imageVector = Icons.Default.ArrowBack,
                        contentDescription = "Back",
                        tint = if (isSystemInDarkTheme()) {
                            MaterialTheme.colors.onSurface
                        } else {
                            MaterialTheme.colors.onPrimary
                        },
                    )
                }
            },
            backgroundColor = Color.Transparent,
            elevation = 0.dp,
            modifier = Modifier.statusBarsPadding(),
        )
    }
}

@Composable
fun Poster(url: String?, modifier: Modifier = Modifier, onClick: (() -> Unit)? = null) {
    val context = LocalContext.current

    BoxWithConstraints(modifier = modifier) {
        val width = with(LocalDensity.current) {
            constraints.maxWidth.toDp()
        }

        Card(modifier = Modifier.size(width, width * (3f / 2f))) {
            Image(
                painter = rememberImagePainter(url, builder = { crossfade(true) }),
                contentDescription = "Poster",
                contentScale = ContentScale.Crop,
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

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun Thumbnail(
    url: String?,
    modifier: Modifier = Modifier,
    overlay: (@Composable () -> Unit)? = null,
    onClick: (() -> Unit)? = null,
) {
    val context = LocalContext.current

    BoxWithConstraints(modifier = modifier) {
        val width = with(LocalDensity.current) {
            constraints.maxWidth.toDp()
        }

        Card(
            enabled = onClick != null,
            modifier = Modifier.size(width, width * (9f / 16f)),
            onClick = {
                context.playClick()
                onClick?.invoke()
            }
        ) {
            if (url != null)
                Image(
                    painter = rememberImagePainter(url, builder = { crossfade(true) }),
                    contentDescription = "Thumbnail",
                    contentScale = ContentScale.Crop,
                    modifier = Modifier.fillMaxSize(),
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
