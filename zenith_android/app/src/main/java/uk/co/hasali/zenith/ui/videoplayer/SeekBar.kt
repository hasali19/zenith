package uk.co.hasali.zenith.ui.videoplayer

import android.text.format.DateUtils
import android.view.ViewGroup
import android.widget.SeekBar
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.AmbientContext
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView

@Composable
fun SeekBar(
    position: Float,
    buffered: Float,
    max: Float,
    onSeekStart: () -> Unit = {},
    onSeekEnd: (Float) -> Unit = {},
) {
    var internalPosition by remember { mutableStateOf(position) }

    val context = AmbientContext.current
    val view = remember {
        SeekBar(context).apply {
            layoutParams = ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
            )
        }
    }

    DisposableEffect(max) {
        view.max = max.toInt()
        onDispose { }
    }

    DisposableEffect(position) {
        internalPosition = position
        onDispose { }
    }

    DisposableEffect(internalPosition) {
        view.progress = internalPosition.toInt()
        onDispose { }
    }

    DisposableEffect(buffered) {
        view.secondaryProgress = buffered.toInt()
        onDispose { }
    }

    DisposableEffect(onSeekStart, onSeekEnd) {
        view.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
            override fun onProgressChanged(seekBar: SeekBar, progress: Int, fromUser: Boolean) {
                if (fromUser) {
                    internalPosition = progress.toFloat()
                }
            }

            override fun onStartTrackingTouch(seekBar: SeekBar) {
                onSeekStart()
            }

            override fun onStopTrackingTouch(seekBar: SeekBar) {
                onSeekEnd(seekBar.progress.toFloat())
            }
        })

        onDispose { }
    }

    Column {
        AndroidView(viewBlock = { view }, modifier = Modifier.padding(top = 8.dp))
        Row {
            val modifier = Modifier
                .padding(horizontal = 16.dp)
                .weight(1f)

            TimeText(
                time = internalPosition,
                modifier = modifier,
                align = TextAlign.Start,
            )

            TimeText(
                time = max - internalPosition,
                modifier = modifier,
                align = TextAlign.End,
            )
        }
    }
}

@Composable
private fun TimeText(
    time: Float,
    modifier: Modifier = Modifier,
    align: TextAlign = TextAlign.Start,
) {
    Text(
        text = DateUtils.formatElapsedTime(time.toLong()),
        color = Color.White,
        textAlign = align,
        style = MaterialTheme.typography.caption,
        modifier = modifier,
    )
}
