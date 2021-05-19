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
import io.ktor.client.*
import io.ktor.client.request.*
import uk.hasali.zenith.Navigator
import uk.hasali.zenith.TranscoderState
import uk.hasali.zenith.VideoInfo

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun TranscodeQueueScreen(client: HttpClient, navigator: Navigator) {
    val state by produceState<TranscoderState?>(initialValue = null) {
        value = client.get("https://zenith.hasali.uk/api/transcoder")
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
                    Text("Queued (${state?.queue?.size})", style = MaterialTheme.typography.subtitle2)
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
fun TranscodeQueueListItem(client: HttpClient, id: Int) {
    val info by produceState<VideoInfo?>(initialValue = null, id) {
        value = client.get("https://zenith.hasali.uk/api/videos/$id/info")
    }

    ListItem(text = { Text(info?.path ?: id.toString()) })
}
