package uk.co.hasali.zenith

import android.view.ViewGroup
import android.widget.SeekBar
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.onCommit
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.AmbientContext
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView

@Composable
fun SeekBar(
    position: Float,
    max: Float,
    onSeekStart: () -> Unit = {},
    onSeekEnd: (Float) -> Unit = {}
) {
    val context = AmbientContext.current
    val view = remember {
        SeekBar(context).apply {
            layoutParams = ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
            )
        }
    }

    onCommit(max) {
        view.max = max.toInt()
    }

    onCommit(position) {
        view.progress = position.toInt()
    }

    onCommit(onSeekStart, onSeekEnd) {
        view.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
            override fun onProgressChanged(seekBar: SeekBar, progress: Int, fromUser: Boolean) {}

            override fun onStartTrackingTouch(seekBar: SeekBar) {
                onSeekStart()
            }

            override fun onStopTrackingTouch(seekBar: SeekBar) {
                onSeekEnd(seekBar.progress.toFloat())
            }
        })
    }

    AndroidView(viewBlock = { view }, modifier = Modifier.padding(vertical = 8.dp))
}
