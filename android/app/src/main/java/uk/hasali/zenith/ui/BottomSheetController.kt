package uk.hasali.zenith.ui

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.height
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetState
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.Stable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch

interface BottomSheetContent {
    @Composable
    fun ContentWithScope(scope: BottomSheetContentScope) {
        scope.Content()
    }

    @Composable
    fun BottomSheetContentScope.Content()
}

@Stable
interface BottomSheetController {
    @OptIn(ExperimentalMaterialApi::class)
    val state: ModalBottomSheetState

    suspend fun show(content: BottomSheetContent)

    @Composable
    fun Content(columnScope: ColumnScope)
}

@Composable
@OptIn(ExperimentalMaterialApi::class)
fun rememberBottomSheetController(): BottomSheetController =
    BottomSheetControllerImpl(
        state = rememberModalBottomSheetState(ModalBottomSheetValue.Hidden)
    )

interface BottomSheetContentScope : ColumnScope {
    fun hide()
}

private class BottomSheetContentScopeImpl(
    columnScope: ColumnScope,
    private val onHide: () -> Unit,
) : ColumnScope by columnScope, BottomSheetContentScope {
    override fun hide() {
        onHide()
    }
}

@OptIn(ExperimentalMaterialApi::class)
private class BottomSheetControllerImpl(
    override val state: ModalBottomSheetState,
) : BottomSheetController {
    private val content = mutableStateOf<BottomSheetContent?>(null)

    override suspend fun show(content: BottomSheetContent) {
        this.content.value = content
        state.show()
    }

    private suspend fun hide() {
        state.hide()
        content.value = null
    }

    @Composable
    override fun Content(columnScope: ColumnScope) {
        val coroutineScope = rememberCoroutineScope()

        BackHandler(enabled = state.isVisible) {
            coroutineScope.launch {
                hide()
            }
        }

        content.value.let { content ->
            if (content != null) {
                content.ContentWithScope(BottomSheetContentScopeImpl(columnScope) {
                    coroutineScope.launch {
                        hide()
                    }
                })
            } else {
                Box(modifier = Modifier.height(1.dp))
            }
        }
    }
}
