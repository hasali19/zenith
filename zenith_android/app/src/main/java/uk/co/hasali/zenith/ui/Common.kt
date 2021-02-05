package uk.co.hasali.zenith.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material.Card
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.AmbientDensity
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import dev.chrisbanes.accompanist.coil.CoilImage

@Composable
fun PosterCard(
    posterUrl: String?,
    primaryText: String,
    secondaryText: String?,
    modifier: Modifier = Modifier,
) {
    Card(modifier = modifier) {
        Column {
            BoxWithConstraints {
                val height = with(AmbientDensity.current) {
                    constraints.maxWidth.toDp() * (3f / 2f)
                }

                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .preferredHeight(height)
                ) {
                    posterUrl?.let { url ->
                        CoilImage(
                            data = url,
                            contentDescription = null,
                            fadeIn = true,
                            modifier = Modifier.fillMaxWidth(),
                        )
                    }
                }
            }

            Column(modifier = Modifier.padding(8.dp)) {
                Text(
                    text = primaryText,
                    style = MaterialTheme.typography.body2,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )

                Text(
                    text = secondaryText.orEmpty(),
                    style = MaterialTheme.typography.caption,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
            }
        }
    }
}

@Composable
fun Backdrop(url: String?, modifier: Modifier = Modifier) {
    BoxWithConstraints(modifier = modifier) {
        val height = with(AmbientDensity.current) {
            constraints.maxWidth.toDp() * (9f / 16f)
        }

        Box(modifier = Modifier.preferredHeight(height)) {
            url?.let { url ->
                CoilImage(data = url, contentDescription = null, fadeIn = true)
            }
        }
    }
}
