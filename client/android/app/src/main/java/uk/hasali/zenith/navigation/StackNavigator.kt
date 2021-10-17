package uk.hasali.zenith.navigation

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

class StackNavigator<T : Any>(
    private val lifecycleOwner: LifecycleOwner,
    private val viewModelStoreProvider: ViewModelStoreProvider,
    stack: List<T>,
) {
    constructor(
        lifecycleOwner: LifecycleOwner,
        viewModelStoreProvider: ViewModelStoreProvider,
        initial: T,
    ) : this(lifecycleOwner, viewModelStoreProvider, listOf(initial))

    private val _entering = mutableListOf<DefaultNavEntry<T>>()
    private val _suspending = mutableListOf<DefaultNavEntry<T>>()
    private val _exiting = mutableListOf<DefaultNavEntry<T>>()

    private val _stack = SnapshotStateList<DefaultNavEntry<T>>().apply {
        for (item in stack) {
            add(
                DefaultNavEntry(item, viewModelStoreProvider).apply {
                    setLifecycleState(Lifecycle.State.CREATED)
                    setParentLifecycleState(lifecycleOwner.lifecycle.currentState)
                }
            )
        }

        // Ensure that the topmost entry gets put into RESUMED
        // when it enters the screen
        _entering.add(last())
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
        val from = _stack.last()
        val to = DefaultNavEntry(screen, viewModelStoreProvider).apply {
            setLifecycleState(Lifecycle.State.CREATED)
            setParentLifecycleState(lifecycleOwner.lifecycle.currentState)
        }

        _stack.add(to)
        _entering.add(to)
        _suspending.add(from)
    }

    fun pop() {
        if (_stack.size <= 1) {
            return
        }

        _stack.removeLast().also {
            _exiting.add(it)
        }

        _entering.add(_stack.last())
    }

    fun popAll() {
        if (_stack.size <= 1) {
            return
        }

        _stack.last()
        while (stack.size > 1) {
            _exiting.add(_stack.removeLast())
        }

        _entering.add(_stack.last())
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
            save = { v -> v.stack.map { it.screen }.toList() },
            restore = { StackNavigator(lifecycleOwner, viewModelStoreProvider, it) },
        ),
    ) {
        StackNavigator(lifecycleOwner, viewModelStoreProvider, initial)
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
