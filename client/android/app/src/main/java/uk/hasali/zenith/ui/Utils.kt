package uk.hasali.zenith.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.LifecycleOwner

@Composable
fun LifecycleObserver(onEvent: (owner: LifecycleOwner, event: Lifecycle.Event) -> Unit) {
    val owner = LocalLifecycleOwner.current
    val observer = remember(onEvent) {
        LifecycleEventObserver(onEvent)
    }

    DisposableEffect(owner, observer) {
        val lifecycle = owner.lifecycle
        lifecycle.addObserver(observer)
        onDispose {
            lifecycle.removeObserver(observer)
        }
    }
}

fun twoDigitNumber(number: Int) = "$number".padStart(2, '0')

fun displayDuration(duration: Double) =
    if (duration <= 90 * 60) {
        "${(duration / 60).toInt()}m";
    } else {
        val hours = (duration / 3600).toInt()
        val minutes = ((duration % 3600) / 60).toInt();
        "${hours}h ${minutes}m";
    }
