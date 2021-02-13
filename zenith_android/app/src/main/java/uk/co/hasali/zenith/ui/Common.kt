package uk.co.hasali.zenith.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.Card
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import dev.chrisbanes.accompanist.coil.CoilImage

@Composable
fun PosterCard(
    posterUrl: String?,
    primaryText: String,
    secondaryText: String?,
    modifier: Modifier = Modifier,
    count: Int = 0,
) {
    Card(modifier = modifier) {
        Box {
            Column {
                BoxWithConstraints {
                    val height = with(LocalDensity.current) {
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

            if (count > 0) {
                CountBadge(
                    count = count,
                    modifier = Modifier
                        .align(Alignment.TopEnd)
                        .padding(2.dp)
                )
            }
        }
    }
}

@Composable
fun Backdrop(url: String?, modifier: Modifier = Modifier) {
    BoxWithConstraints(modifier = modifier) {
        val height = with(LocalDensity.current) {
            constraints.maxWidth.toDp() * (9f / 16f)
        }

        Box(modifier = Modifier.preferredHeight(height)) {
            url?.let { url ->
                CoilImage(data = url, contentDescription = null, fadeIn = true)
            }
        }
    }
}

@Composable
fun CountBadge(count: Int, modifier: Modifier = Modifier) {
    Box(
        contentAlignment = Alignment.Center,
        modifier = modifier
            .clip(CircleShape)
            .background(MaterialTheme.colors.secondary)
            .padding(2.dp)
            .preferredHeight(20.dp)
            .preferredWidthIn(min = 20.dp)
    ) {
        Text(
            text = count.toString(),
            style = MaterialTheme.typography.caption,
            color = MaterialTheme.colors.onSecondary,
        )
    }
}
