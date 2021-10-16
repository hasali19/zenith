package uk.hasali.zenith.navigation

import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.saveable.SaveableStateHolder
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.LifecycleRegistry

interface NavEntry<T> : LifecycleOwner {
    val screen: T

    fun setLifecycleState(state: Lifecycle.State)
    fun setParentLifecycleState(state: Lifecycle.State)
}

class DefaultNavEntry<T>(override val screen: T) : NavEntry<T> {
    private var parentLifecycleState = Lifecycle.State.INITIALIZED
    private var lifecycleState = Lifecycle.State.INITIALIZED
    private val lifecycle = LifecycleRegistry(this)

    override fun setLifecycleState(state: Lifecycle.State) {
        lifecycleState = state
        updateLifecycle()
    }

    override fun setParentLifecycleState(state: Lifecycle.State) {
        parentLifecycleState = state
        updateLifecycle()
    }

    private fun updateLifecycle() {
        lifecycle.currentState = minOf(parentLifecycleState, lifecycleState)
    }

    override fun getLifecycle(): Lifecycle {
        return lifecycle
    }
}

@Composable
fun <T: Any> NavEntry<T>.LocalsProvider(holder: SaveableStateHolder, content: @Composable () -> Unit) {
    CompositionLocalProvider(LocalLifecycleOwner provides this) {
        holder.SaveableStateProvider(screen) {
            content()
        }
    }
}
