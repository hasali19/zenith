package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.size
import androidx.compose.material.Card
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import coil.compose.rememberImagePainter
import uk.hasali.zenith.playClick

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