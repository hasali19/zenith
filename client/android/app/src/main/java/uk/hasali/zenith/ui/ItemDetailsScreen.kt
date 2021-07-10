package uk.hasali.zenith.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.dp
import com.google.accompanist.coil.rememberCoilPainter
import com.google.accompanist.insets.navigationBarsPadding

@Composable
fun ItemDetailsScreen(
    backdrop: String?,
    poster: String?,
    headerContent: @Composable () -> Unit,
    actionsRow: (@Composable () -> Unit)? = null,
    overview: String? = null,
    isWatched: Boolean = false,
    content: @Composable () -> Unit = {},
) {
    Surface(
        modifier = Modifier
            .fillMaxSize()
            .navigationBarsPadding(),
    ) {
        BoxWithConstraints(modifier = Modifier.verticalScroll(rememberScrollState())) {
            Box(modifier = Modifier.aspectRatio(16f / 9f)) {
                Image(
                    painter = rememberCoilPainter(request = backdrop, fadeIn = true),
                    contentDescription = "Backdrop",
                    contentScale = ContentScale.Crop,
                    modifier = Modifier.fillMaxWidth(),
                )

                if (isWatched) {
                    Box(
                        modifier = Modifier
                            .fillMaxSize()
                            .background(Color.Black.copy(alpha = 0.4f))
                    ) {
                        Icon(
                            imageVector = Icons.Default.Check,
                            contentDescription = "Watched",
                            modifier = Modifier
                                .size(32.dp)
                                .align(Alignment.Center),
                            tint = Color.White,
                        )
                    }
                }
            }

            val backdropHeight = with(LocalDensity.current) {
                (constraints.maxWidth * 9f / 16f).toDp()
            }

            Column(modifier = Modifier.padding(top = backdropHeight - 48.dp, bottom = 16.dp)) {
                Column(modifier = Modifier.padding(horizontal = 16.dp)) {
                    HeaderSection(poster = poster, content = headerContent)
                    Spacer(modifier = Modifier.height(16.dp))

                    if (actionsRow != null) {
                        actionsRow()
                        Spacer(modifier = Modifier.height(16.dp))
                    }

                    if (overview != null) {
                        OverviewSection(content = overview)
                        Spacer(modifier = Modifier.height(16.dp))
                    }
                }

                content()
            }
        }
    }
}

@Composable
private fun HeaderSection(poster: String?, content: @Composable () -> Unit) {
    Row {
        Poster(url = poster, modifier = Modifier.width(150.dp))
        Spacer(modifier = Modifier.width(16.dp))
        Box(modifier = Modifier.align(Alignment.CenterVertically)) {
            content()
        }
    }
}

@Composable
private fun OverviewSection(content: String) {
    Text(
        text = "Overview",
        style = MaterialTheme.typography.subtitle2,
        color = if (MaterialTheme.colors.isLight) Color.Black else Color.White,
    )
    Spacer(modifier = Modifier.height(8.dp))
    Text(text = content, style = MaterialTheme.typography.body2)
}
