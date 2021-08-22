package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListScope
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.google.accompanist.swiperefresh.SwipeRefresh
import com.google.accompanist.swiperefresh.rememberSwipeRefreshState

@Composable
fun SwipeRefreshLazyColumn(
    isRefreshing: Boolean,
    onRefresh: () -> Unit,
    isEmpty: Boolean = false,
    emptyContent: @Composable BoxScope.() -> Unit = {},
    content: LazyListScope.() -> Unit,
) {
    val state = rememberSwipeRefreshState(isRefreshing)

    SwipeRefresh(state = state, onRefresh = onRefresh, modifier = Modifier.fillMaxSize()) {
        if (isEmpty) {
            Box(
                content = emptyContent,
                modifier = Modifier
                    .fillMaxSize()
                    .verticalScroll(rememberScrollState()),
            )
        } else {
            LazyColumn(content = content)
        }
    }
}
