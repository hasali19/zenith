package uk.hasali.zenith.screens.player

import android.text.format.DateUtils
import androidx.annotation.OptIn
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.media3.common.util.UnstableApi
import androidx.media3.ui.DefaultTimeBar
import androidx.media3.ui.TimeBar

@OptIn(UnstableApi::class)
@Composable
fun SeekBar(
    position: Long,
    duration: Long,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long) -> Unit,
    modifier: Modifier = Modifier,
) {
    var sliderPosition by remember { mutableStateOf(0L) }

    DisposableEffect(position) {
        sliderPosition = position
        onDispose { }
    }

    // TODO: This padding is a temporary hack for rounded corners. Ideally this can be removed
    //       once we implement showing/hiding system UI along with controls
    Column(modifier = modifier.padding(bottom = 32.dp)) {
        AndroidView(
            modifier = Modifier.fillMaxWidth(),
            factory = {
                DefaultTimeBar(it).apply {
                    addListener(object : TimeBar.OnScrubListener {
                        override fun onScrubStart(timeBar: TimeBar, position: Long) {
                            onSeekStart()
                        }

                        override fun onScrubMove(timeBar: TimeBar, position: Long) {
                            sliderPosition = position
                        }

                        override fun onScrubStop(
                            timeBar: TimeBar,
                            position: Long,
                            canceled: Boolean
                        ) {
                            onSeekEnd(position)
                        }
                    })
                }
            },
            update = {
                it.setDuration(duration)
                it.setPosition(position)
            }
        )

        Row(
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier
                .padding(horizontal = 16.dp)
                .fillMaxWidth(),
        ) {
            Row {
                TimeText(
                    time = sliderPosition,
                    alpha = 1.0f,
                )

                Text(
                    text = "•",
                    color = Color.White.copy(alpha = 0.7f),
                    style = MaterialTheme.typography.caption,
                    modifier = Modifier.padding(horizontal = 4.dp),
                )

                TimeText(
                    time = duration - sliderPosition,
                    alpha = 0.7f,
                )
            }
        }
    }
}

@Composable
private fun TimeText(
    time: Long,
    alpha: Float,
) {
    Text(
        text = DateUtils.formatElapsedTime(time),
        color = Color.White.copy(alpha = alpha),
        style = MaterialTheme.typography.caption,
    )
}
