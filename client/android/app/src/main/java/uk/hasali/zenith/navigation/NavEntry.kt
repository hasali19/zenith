package uk.hasali.zenith.navigation

import android.os.Parcelable
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.saveable.SaveableStateHolder
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.*
import androidx.lifecycle.viewmodel.compose.LocalViewModelStoreOwner

interface NavEntry<T> : LifecycleOwner, ViewModelStoreOwner {
    val screen: T

    fun setLifecycleState(state: Lifecycle.State)
    fun setParentLifecycleState(state: Lifecycle.State)
}

class DefaultNavEntry<T : Any>(
    override val screen: T,
    private val viewModelStoreProvider: ViewModelStoreProvider,
) : NavEntry<T> {
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

    fun clearViewModels() {
        viewModelStoreProvider.clear(this)
    }

    private fun updateLifecycle() {
        lifecycle.currentState = minOf(parentLifecycleState, lifecycleState)
    }

    override fun getLifecycle(): Lifecycle {
        return lifecycle
    }

    override fun getViewModelStore(): ViewModelStore {
        return viewModelStoreProvider.get(this)
    }

    override fun equals(other: Any?): Boolean {
        return other is DefaultNavEntry<*> && screen == other.screen
    }

    override fun hashCode(): Int {
        return screen.hashCode()
    }
}

@Composable
fun <T : Parcelable> NavEntry<T>.LocalsProvider(
    holder: SaveableStateHolder,
    content: @Composable () -> Unit
) {
    val navScreenProvider = NavScreenProvider(LocalNavScreenProvider.current, screen)

    CompositionLocalProvider(
        LocalLifecycleOwner provides this,
        LocalViewModelStoreOwner provides this,
        LocalNavScreenProvider provides navScreenProvider,
    ) {
        holder.SaveableStateProvider(screen) {
            content()
        }
    }
}
