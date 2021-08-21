package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.size
import androidx.compose.material.Card
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import coil.compose.rememberImagePainter
import uk.hasali.zenith.playClick

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
