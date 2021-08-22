package uk.hasali.zenith.screens.player

import android.content.res.ColorStateList
import android.text.format.DateUtils
import android.widget.SeekBar
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.graphics.BlendModeColorFilterCompat
import androidx.core.graphics.BlendModeCompat

@Composable
fun SeekBar(
    position: Long,
    duration: Long,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long) -> Unit,
    modifier: Modifier = Modifier,
) {
    val primaryColor = MaterialTheme.colors.primary.toArgb()
    var sliderPosition by remember { mutableStateOf(0L) }

    DisposableEffect(position) {
        sliderPosition = position
        onDispose { }
    }

    Row(modifier = modifier.padding(16.dp)) {
        TimeText(
            time = sliderPosition,
            align = TextAlign.Start,
        )

        AndroidView(
            modifier = Modifier.weight(1f),
            factory = {
                SeekBar(it).apply {
                    // Set progress and thumb colors from Compose Material theme
                    progressTintList = ColorStateList.valueOf(primaryColor)
                    thumb.colorFilter = BlendModeColorFilterCompat
                        .createBlendModeColorFilterCompat(primaryColor, BlendModeCompat.SRC_ATOP)

                    setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
                        override fun onProgressChanged(
                            seekBar: SeekBar?,
                            progress: Int,
                            fromUser: Boolean,
                        ) {
                            sliderPosition = progress.toLong()
                        }

                        override fun onStartTrackingTouch(seekBar: SeekBar?) {
                            onSeekStart()
                        }

                        override fun onStopTrackingTouch(seekBar: SeekBar?) {
                            onSeekEnd(progress.toLong())
                        }
                    })
                }
            },
            update = {
                it.max = duration.toInt()
                it.progress = position.toInt()
            }
        )

        TimeText(
            time = duration - sliderPosition,
            align = TextAlign.Start,
        )
    }
}

@Composable
private fun TimeText(
    time: Long,
    modifier: Modifier = Modifier,
    align: TextAlign = TextAlign.Start,
) {
    Text(
        text = DateUtils.formatElapsedTime(time),
        color = Color.White,
        textAlign = align,
        style = MaterialTheme.typography.caption,
        modifier = modifier,
    )
}
