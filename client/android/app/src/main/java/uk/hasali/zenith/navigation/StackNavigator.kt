package uk.hasali.zenith.navigation

import android.os.Parcelable
import androidx.activity.compose.BackHandler
import androidx.compose.animation.*
import androidx.compose.animation.core.tween
import androidx.compose.animation.core.updateTransition
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.Saver
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.saveable.rememberSaveableStateHolder
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.LifecycleOwner

interface StackNavigatorEntry<T> : NavEntry<T> {
    val level: Int
}

class StackNavigator<T>(private val lifecycleOwner: LifecycleOwner, stack: List<T>) {
    constructor(lifecycleOwner: LifecycleOwner, initial: T) : this(lifecycleOwner, listOf(initial))

    private class Entry<T> private constructor(
        override val level: Int,
        private val entry: DefaultNavEntry<T>,
    ) : StackNavigatorEntry<T>, NavEntry<T> by entry {
        constructor(level: Int, screen: T) : this(level, DefaultNavEntry(screen))
    }

    private val _entering = mutableListOf<Entry<T>>()
    private val _suspending = mutableListOf<Entry<T>>()
    private val _exiting = mutableListOf<Entry<T>>()

    private val _stack = SnapshotStateList<Entry<T>>().apply {
        for ((index, item) in stack.withIndex()) {
            add(
                Entry(index, item).apply {
                    setLifecycleState(Lifecycle.State.CREATED)
                    setParentLifecycleState(lifecycleOwner.lifecycle.currentState)
                }
            )
        }

        // Ensure that the topmost entry gets put into RESUMED
        // when it enters the screen
        _entering.add(last())
    }

    val stack: List<StackNavigatorEntry<T>> get() = _stack

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

        _exiting.forEach { it.setLifecycleState(Lifecycle.State.DESTROYED) }
        _exiting.clear()
    }

    fun push(screen: T) {
        val from = _stack.last()
        val to = Entry(_stack.size, screen).apply {
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
    return rememberSaveable(
        saver = Saver(
            save = { v -> v.stack.map { it.screen }.toList() },
            restore = { StackNavigator(lifecycleOwner, it) },
        ),
    ) {
        StackNavigator(lifecycleOwner, initial)
    }
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
fun <T : Parcelable> StackNavigator<T>.ContentHost(content: @Composable (T) -> Unit) {
    val current = stack.last()
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
