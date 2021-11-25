package uk.hasali.zenith.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.flowWithLifecycle
import kotlinx.coroutines.flow.Flow

@Composable
fun <T> rememberFlowWithLifecycle(
    flow: Flow<T>,
    lifecycle: Lifecycle = LocalLifecycleOwner.current.lifecycle,
    minActiveState: Lifecycle.State = Lifecycle.State.STARTED
): Flow<T> = remember(flow, lifecycle) {
    flow.flowWithLifecycle(
        lifecycle = lifecycle,
        minActiveState = minActiveState
    )
}

fun twoDigitNumber(number: Int) = "$number".padStart(2, '0')

fun displayDuration(duration: Double) =
    if (duration <= 90 * 60) {
        "${(duration / 60).toInt()}m"
    } else {
        val hours = (duration / 3600).toInt()
        val minutes = ((duration % 3600) / 60).toInt()
        "${hours}h ${minutes}m"
    }
