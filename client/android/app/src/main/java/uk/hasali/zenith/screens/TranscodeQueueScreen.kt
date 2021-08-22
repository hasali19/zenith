package uk.hasali.zenith.screens

import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.navigationBarsPadding
import kotlinx.coroutines.launch
import uk.hasali.zenith.TranscoderState
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.LocalZenithClient
import uk.hasali.zenith.ui.SwipeRefreshLazyColumn

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun TranscodeQueueScreen(onNavigateUp: () -> Unit) {
    val scope = rememberCoroutineScope()
    val client = LocalZenithClient.current

    var isRefreshing by remember { mutableStateOf(false) }
    var state by remember { mutableStateOf<TranscoderState?>(null) }

    suspend fun refresh() {
        isRefreshing = true
        state = client.getTranscoderState()
        isRefreshing = false
    }

    LaunchedEffect(Unit) {
        refresh()
    }

    TranscodeQueueScreen(
        client = client,
        state = state,
        isRefreshing = isRefreshing,
        onRefresh = { scope.launch { refresh() } },
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun TranscodeQueueScreen(
    client: ZenithApiClient,
    state: TranscoderState?,
    isRefreshing: Boolean,
    onRefresh: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    Scaffold(
        topBar = { AppBar(title = "Transcode queue", onBackPressed = onNavigateUp) },
        modifier = Modifier.navigationBarsPadding(),
    ) {
        SwipeRefreshLazyColumn(
            isRefreshing = isRefreshing,
            isEmpty = state?.current == null && state?.queue?.isEmpty() == true,
            contentPadding = PaddingValues(16.dp),
            onRefresh = onRefresh,
            emptyContent = {
                Text(
                    text = "Transcode queue is empty",
                    modifier = Modifier.align(Alignment.Center),
                )
            }
        ) {
            if (state?.current != null) {
                item {
                    Text("Current", style = MaterialTheme.typography.subtitle2)
                    TranscodeQueueListItem(client = client, id = state.current)
                }
            }

            if (state?.queue?.isNotEmpty() == true) {
                if (state.current != null) {
                    item {
                        Spacer(modifier = Modifier.height(8.dp))
                    }
                }

                item {
                    Text(
                        text = "Queued (${state.queue.size})",
                        style = MaterialTheme.typography.subtitle2,
                    )
                }

                items(state.queue) { id ->
                    TranscodeQueueListItem(client = client, id = id)
                    Divider()
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun TranscodeQueueListItem(client: ZenithApiClient, id: Int) {
    // TODO: This is not ideal - return this info directly from transcoder api
    val info by produceState<VideoInfo?>(initialValue = null, id) {
        value = client.getVideoInfo(id)
    }

    ListItem(text = { Text(info?.path ?: id.toString()) })
}
