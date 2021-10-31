package uk.hasali.zenith.screens.player

import android.app.Activity
import android.view.WindowManager
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.ui.input.pointer.AwaitPointerEventScope
import androidx.compose.ui.input.pointer.PointerEvent
import androidx.compose.ui.input.pointer.PointerEventType
import androidx.compose.ui.platform.LocalContext
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

@Composable
fun KeepScreenOn() {
    val activity = LocalContext.current as? Activity
    val window = activity?.window

    if (window != null) {
        DisposableEffect(Unit) {
            window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)

            onDispose {
                window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
            }
        }
    }
}

@Composable
fun FullScreen() {
    val activity = LocalContext.current as? Activity
    val window = activity?.window

    if (window != null) {
        DisposableEffect(Unit) {
            val controller = WindowCompat.getInsetsController(window, window.decorView)

            if (controller != null) {
                controller.hide(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
                controller.systemBarsBehavior =
                    WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
            }

            onDispose {
                controller?.show(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
            }
        }
    }
}

suspend fun AwaitPointerEventScope.awaitPointerEvent(type: PointerEventType): PointerEvent {
    while (true) {
        val e = awaitPointerEvent()
        if (e.type == type) {
            return e
        }
    }
}
