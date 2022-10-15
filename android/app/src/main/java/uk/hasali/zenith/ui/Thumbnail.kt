package uk.hasali.zenith.ui

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
import coil.compose.AsyncImage
import coil.request.ImageRequest

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun Thumbnail(
    url: String?,
    modifier: Modifier = Modifier,
    overlay: (@Composable () -> Unit)? = null,
    onClick: (() -> Unit)? = null,
) {
    val content: @Composable () -> Unit = {
        if (url != null)
            AsyncImage(
                model = ImageRequest.Builder(LocalContext.current)
                    .data(url)
                    .crossfade(true)
                    .build(),
                contentDescription = "Thumbnail",
                contentScale = ContentScale.Crop,
                modifier = Modifier.fillMaxSize(),
            )

        overlay?.invoke()
    }

    BoxWithConstraints(modifier = modifier) {
        val width = with(LocalDensity.current) {
            constraints.maxWidth.toDp()
        }

        val cardModifier = Modifier.size(width, width * (9f / 16f))

        if (onClick != null) {
            Card(
                onClick = onClick,
                modifier = cardModifier,
                content = content,
            )
        } else {
            Card(
                modifier = cardModifier,
                content = content,
            )
        }
    }
}
