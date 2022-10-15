package uk.hasali.zenith.navigation

import android.os.Bundle
import android.os.Parcelable
import androidx.activity.compose.BackHandler
import androidx.compose.animation.*
import androidx.compose.animation.core.tween
import androidx.compose.animation.core.updateTransition
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.saveable.Saver
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.saveable.rememberSaveableStateHolder
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.*
import androidx.lifecycle.viewmodel.compose.LocalViewModelStoreOwner
import uk.hasali.zenith.parcelable
import uk.hasali.zenith.parcelableArrayList

class StackNavigator<T : Parcelable> constructor(
    private val lifecycleOwner: LifecycleOwner,
    private val viewModelStoreProvider: ViewModelStoreProvider,
    private val savedState: Bundle?,
) {
    private val _entering = mutableListOf<DefaultNavEntry<T>>()
    private val _suspending = mutableListOf<DefaultNavEntry<T>>()
    private val _exiting = mutableListOf<DefaultNavEntry<T>>()

    private val _stack = SnapshotStateList<DefaultNavEntry<T>>().apply {
        if (savedState != null) {
            // Restore backstack from saved state
            savedState.parcelableArrayList("backstack", Bundle::class.java)?.forEach {
                @Suppress("UNCHECKED_CAST")
                add(createEntry(it.parcelable("screen", Parcelable::class.java)!! as T, it.getBundle("state")))
            }

            // Ensure that the topmost entry gets RESUMED
            // when it enters the screen
            lastOrNull()?.let {
                _entering.add(it)
            }
        }
    }

    val stack: List<NavEntry<T>> get() = _stack

    init {
        lifecycleOwner.lifecycle.addObserver(LifecycleEventObserver { _, event ->
            for (entry in _stack) {
                entry.setParentLifecycleState(event.targetState)
            }
        })
    }

    fun beginEnterTransition() {
        _entering.forEach { it.setLifecycleState(Lifecycle.State.RESUMED) }
        _entering.clear()
    }

    fun endExitTransition() {
        _suspending.forEach { it.setLifecycleState(Lifecycle.State.CREATED) }
        _suspending.clear()

        _exiting.forEach {
            it.setLifecycleState(Lifecycle.State.DESTROYED)
            it.clearViewModels()
        }
        _exiting.clear()
    }

    fun push(screen: T) {
        val from = _stack.lastOrNull()
        val to = createEntry(screen)

        _stack.add(to)
        _entering.add(to)

        if (from != null) {
            _suspending.add(from)
        }
    }

    fun pop() {
        if (_stack.size <= 1) {
            return
        }

        _exiting.add(_stack.removeLast())
        _entering.add(_stack.last())
    }

    fun popAll() {
        if (_stack.size <= 1) {
            return
        }

        while (stack.size > 1) {
            _exiting.add(_stack.removeLast())
        }

        _entering.add(_stack.last())
    }

    fun saveState(): Bundle {
        val state = Bundle()
        val stack = ArrayList<Bundle>(_stack.size)

        for (item in _stack) {
            stack.add(Bundle().apply {
                putParcelable("screen", item.screen)
                putBundle("state", item.saveState())
            })
        }

        state.putParcelableArrayList("backstack", stack)

        return state
    }

    private fun createEntry(screen: T, savedState: Bundle? = null): DefaultNavEntry<T> {
        return DefaultNavEntry(screen, viewModelStoreProvider, savedState).apply {
            setLifecycleState(Lifecycle.State.CREATED)
            setParentLifecycleState(lifecycleOwner.lifecycle.currentState)
        }
    }
}

@Composable
fun <T : Parcelable> rememberStackNavigator(initial: T): StackNavigator<T> {
    val lifecycleOwner = LocalLifecycleOwner.current
    val viewModelStoreOwner = requireNotNull(LocalViewModelStoreOwner.current)
    val viewModelStoreProvider = ViewModelProvider(viewModelStoreOwner)
        .get<ViewModelStoreProvider>()

    return rememberSaveable(
        saver = Saver(
            save = { nav -> nav.saveState() },
            restore = { StackNavigator(lifecycleOwner, viewModelStoreProvider, it) },
        ),
    ) {
        StackNavigator<T>(lifecycleOwner, viewModelStoreProvider, null).apply {
            push(initial)
        }
    }
}

private data class CurrentNavEntry<T>(private val entry: NavEntry<T>, val level: Int) :
    NavEntry<T> by entry

@OptIn(ExperimentalAnimationApi::class)
@Composable
fun <T : Parcelable> StackNavigator<T>.ContentHost(content: @Composable (T) -> Unit) {
    val current = CurrentNavEntry(stack.last(), stack.size - 1)
    val holder = rememberSaveableStateHolder()
    val transition = updateTransition(current, "screen")

    BackHandler(stack.size > 1) {
        pop()
    }

    transition.AnimatedContent(
        transitionSpec = {
            if (targetState.level > initialState.level) {
                fadeIn(tween()) + scaleIn(tween(), initialScale = 0.8f) with
                        fadeOut(tween()) + scaleOut(tween(), targetScale = 1.1f)
            } else {
                fadeIn(tween()) + scaleIn(tween(), initialScale = 1.1f) with
                        fadeOut(tween()) + scaleOut(tween(), targetScale = 0.8f)
            }.apply {
                targetContentZIndex = targetState.level.toFloat()
            }
        },
    ) { entry ->
        DisposableEffect(entry) {
            beginEnterTransition()

            entry.lifecycle.addObserver(object : LifecycleEventObserver {
                override fun onStateChanged(source: LifecycleOwner, event: Lifecycle.Event) {
                    if (event.targetState == Lifecycle.State.DESTROYED) {
                        holder.removeState(entry.screen)
                        entry.lifecycle.removeObserver(this)
                    }
                }
            })

            onDispose {
                endExitTransition()
            }
        }

        entry.LocalsProvider(holder) {
            content(entry.screen)
        }
    }
}
