package uk.hasali.zenith.screens.player

import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.material.Text
import androidx.compose.material.TopAppBar
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.ClosedCaption
import androidx.compose.material.icons.filled.Launch
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.statusBarsPadding
import uk.hasali.zenith.playClick

@Composable
fun AppBar(
    title: String,
    onBackPressed: () -> Unit,
    onShowSubtitlesMenu: () -> Unit,
    onLaunchExternal: (() -> Unit)? = null,
    onClosePlayer: (() -> Unit)? = null,
) {
    val context = LocalContext.current

    TopAppBar(
        navigationIcon = {
            IconButton(onClick = {
                context.playClick()
                onBackPressed()
            }) {
                Icon(Icons.Default.ArrowBack, contentDescription = "Back")
            }
        },
        title = {
            Text(
                text = title,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
            )
        },
        backgroundColor = Color.Transparent,
        elevation = 0.dp,
        actions = {
            IconButton(
                onClick = {
                    context.playClick()
                    onShowSubtitlesMenu()
                },
            ) {
                Icon(Icons.Default.ClosedCaption, contentDescription = "Captions")
            }

            if (onLaunchExternal != null) {
                IconButton(onClick = {
                    context.playClick()
                    onLaunchExternal()
                }) {
                    Icon(Icons.Default.Launch, contentDescription = "Launch external")
                }
            }

            if (onClosePlayer != null) {
                IconButton(onClick = {
                    context.playClick()
                    onClosePlayer()
                }) {
                    Icon(Icons.Default.Close, contentDescription = "Close")
                }
            }
        },
        modifier = Modifier.statusBarsPadding(),
    )
}
