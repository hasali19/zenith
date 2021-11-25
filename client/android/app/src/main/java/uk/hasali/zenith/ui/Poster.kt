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
import androidx.compose.ui.platform.LocalDensity
import coil.compose.rememberImagePainter

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun Poster(
    url: String?,
    modifier: Modifier = Modifier,
    overlay: (@Composable () -> Unit)? = null,
    onClick: (() -> Unit)? = null,
) {
    BoxWithConstraints(modifier = modifier) {
        val width = with(LocalDensity.current) {
            constraints.maxWidth.toDp()
        }

        Card(
            enabled = onClick != null,
            modifier = Modifier.size(width, width * (3f / 2f)),
            onClick = { onClick?.invoke() },
        ) {
            if (url != null)
                Image(
                    painter = rememberImagePainter(url, builder = { crossfade(true) }),
                    contentDescription = "Poster",
                    contentScale = ContentScale.Crop,
                    modifier = Modifier.fillMaxSize(),
                )

            overlay?.invoke()
        }
    }
}
