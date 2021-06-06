package uk.hasali.zenith.ui

import androidx.compose.foundation.ScrollState
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.saveable.Saver
import androidx.compose.runtime.saveable.listSaver
import androidx.compose.runtime.saveable.rememberSaveable

fun twoDigitNumber(number: Int) = "$number".padStart(2, '0')

fun displayDuration(duration: Double) =
    if (duration <= 90 * 60) {
        "${(duration / 60).toInt()}m";
    } else {
        val hours = (duration / 3600).toInt()
        val minutes = ((duration % 3600) / 60).toInt();
        "${hours}h ${minutes}m";
    }

@Composable
fun rememberSaveableScrollState(): ScrollState {
    val saver = Saver<ScrollState, Int>(
        save = { it.value },
        restore = { ScrollState(it) },
    )

    return rememberSaveable(saver = saver) { ScrollState(0) }
}

@Composable
fun rememberSaveableLazyListState(): LazyListState {
    val saver = listSaver<LazyListState, Int>(
        save = {
            listOf(
                it.firstVisibleItemIndex,
                it.firstVisibleItemScrollOffset,
            )
        },
        restore = {
            LazyListState(
                firstVisibleItemIndex = it[0],
                firstVisibleItemScrollOffset = it[1],
            )
        }
    )

    return rememberSaveable(saver = saver) { LazyListState() }
}
