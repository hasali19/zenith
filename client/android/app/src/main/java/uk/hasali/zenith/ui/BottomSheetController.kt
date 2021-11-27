package uk.hasali.zenith.ui

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.height
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetState
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch

interface BottomSheetContent {
    @Composable
    fun ColumnContent(scope: ColumnScope) {
        scope.Content()
    }

    @Composable
    fun ColumnScope.Content()
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

@OptIn(ExperimentalMaterialApi::class)
private class BottomSheetControllerImpl(override val state: ModalBottomSheetState) : BottomSheetController {
    private val content = mutableStateOf<BottomSheetContent?>(null)

    override suspend fun show(content: BottomSheetContent) {
        this.content.value = content
        state.show()
    }

    @Composable
    override fun Content(columnScope: ColumnScope) {
        val coroutineScope = rememberCoroutineScope()

        BackHandler(enabled = state.isVisible) {
            coroutineScope.launch {
                state.hide()
                content.value = null
            }
        }

        content.value.let { content ->
            if (content != null) {
                content.ColumnContent(columnScope)
            } else {
                Box(modifier = Modifier.height(1.dp))
            }
        }
    }
}
