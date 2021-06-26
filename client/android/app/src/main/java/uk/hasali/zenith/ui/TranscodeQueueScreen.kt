package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.TranscoderState
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.ZenithApiClient

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun TranscodeQueueScreen() {
    val client = LocalZenithClient.current
    val navigator = LocalNavigator.current

    val state by produceState<TranscoderState?>(null) {
        value = client.getTranscoderState()
    }

    Scaffold(topBar = { AppBar(navigator = navigator, title = "Transcode queue", menu = false) }) {
        if (state == null) {
            Box(modifier = Modifier.fillMaxSize()) {
                CircularProgressIndicator(modifier = Modifier.align(Alignment.Center))
            }
        } else {
            LazyColumn(contentPadding = PaddingValues(16.dp)) {
                item {
                    state?.current?.let { id ->
                        Text("Current", style = MaterialTheme.typography.subtitle2)
                        TranscodeQueueListItem(client = client, id = id)
                    }
                }

                item {
                    Spacer(modifier = Modifier.height(8.dp))
                }

                item {
                    Text(
                        "Queued (${state?.queue?.size})",
                        style = MaterialTheme.typography.subtitle2
                    )
                }

                items(state!!.queue.size) { i ->
                    TranscodeQueueListItem(client = client, id = state!!.queue[i])
                    Divider()
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun TranscodeQueueListItem(client: ZenithApiClient, id: Int) {
    val info by produceState<VideoInfo?>(initialValue = null, id) {
        value = client.getVideoInfo(id)
    }

    ListItem(text = { Text(info?.path ?: id.toString()) })
}
