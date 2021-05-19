package uk.hasali.zenith.ui

import android.content.Context
import android.media.AudioManager
import android.view.SoundEffectConstants
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
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.google.accompanist.coil.rememberCoilPainter
import com.google.accompanist.insets.statusBarsHeight
import uk.hasali.zenith.Navigator
import uk.hasali.zenith.Screen

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
    var expanded by remember { mutableStateOf(false) }

    IconButton(onClick = { expanded = true }) {
        Icon(Icons.Default.MoreVert, contentDescription = "More")
    }

    DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
        DropdownMenuItem(onClick = {
            expanded = false
            navigator.push(Screen.TranscodeQueue)
        }) {
            Text("Transcode queue")
        }
    }
}

@Composable
fun MediaItemWithPoster(poster: String, primary: String, secondary: String, onClick: () -> Unit) {
    val context = LocalContext.current

    @Composable
    fun Poster(width: Dp, height: Dp) {
        Card {
            Image(
                painter = rememberCoilPainter(
                    poster,
                    fadeIn = true
                ),
                contentDescription = "Poster",
                modifier = Modifier
                    .size(width, height)
                    .clickable {
                        val audioManager =
                            context.getSystemService(Context.AUDIO_SERVICE) as AudioManager
                        audioManager.playSoundEffect(SoundEffectConstants.CLICK, 1.0f)
                        onClick()
                    }
            )
        }
    }

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

    BoxWithConstraints(modifier = Modifier.padding(4.dp)) {
        with(LocalDensity.current) {
            val width = constraints.maxWidth
            val height = width * 1.5

            Column {
                Poster(width = width.toDp(), height = height.toInt().toDp())
                Content()
            }
        }
    }
}
