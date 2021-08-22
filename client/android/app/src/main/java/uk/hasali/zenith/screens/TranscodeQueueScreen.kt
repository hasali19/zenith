package uk.hasali.zenith.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.navigationBarsPadding
import uk.hasali.zenith.TranscoderState
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.CenteredLoadingIndicator
import uk.hasali.zenith.ui.LocalZenithClient

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun TranscodeQueueScreen(onNavigateUp: () -> Unit) {
    val client = LocalZenithClient.current

    val state by produceState<TranscoderState?>(null) {
        value = client.getTranscoderState()
    }

    TranscodeQueueScreen(
        client = client,
        state = state,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun TranscodeQueueScreen(
    client: ZenithApiClient,
    state: TranscoderState?,
    onNavigateUp: () -> Unit,
) {
    Scaffold(
        topBar = { AppBar(title = "Transcode queue", onBackPressed = onNavigateUp) },
        modifier = Modifier.navigationBarsPadding(),
    ) {
        when {
            state == null -> CenteredLoadingIndicator()

            state.current == null && state.queue.isEmpty() -> Box(modifier = Modifier.fillMaxSize()) {
                Text("Transcode queue is empty", modifier = Modifier.align(Alignment.Center))
            }

            else -> TranscodeQueueList(client = client, state = state)
        }
    }
}

@Composable
private fun TranscodeQueueList(client: ZenithApiClient, state: TranscoderState) {
    LazyColumn(contentPadding = PaddingValues(16.dp)) {
        if (state.current != null) {
            item {
                Text("Current", style = MaterialTheme.typography.subtitle2)
                TranscodeQueueListItem(client = client, id = state.current)
            }
        }

        if (state.queue.isNotEmpty()) {
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

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun TranscodeQueueListItem(client: ZenithApiClient, id: Int) {
    // TODO: This is not ideal - return this info directly from transcoder api
    val info by produceState<VideoInfo?>(initialValue = null, id) {
        value = client.getVideoInfo(id)
    }

    ListItem(text = { Text(info?.path ?: id.toString()) })
}
