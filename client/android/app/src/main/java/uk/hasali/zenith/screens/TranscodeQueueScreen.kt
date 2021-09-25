package uk.hasali.zenith.screens

import android.widget.Toast
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.navigationBarsPadding
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.collect
import uk.hasali.zenith.TranscoderEvent
import uk.hasali.zenith.TranscoderJob
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.ZenithApiClient
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.LocalZenithClient

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun TranscodeQueueScreen(onNavigateUp: () -> Unit) {
    val context = LocalContext.current
    val client = LocalZenithClient.current

    val queue = remember { mutableStateListOf<TranscoderJob>() }

    fun showError(message: String) {
        Toast.makeText(context, message, Toast.LENGTH_SHORT)
            .show()
    }

    LaunchedEffect(Unit) {
        while (true) {
            try {
                client.getTranscoderEvents()
                    .collect {
                        when (it) {
                            is TranscoderEvent.InitialState -> {
                                queue.clear()
                                queue.addAll(it.queue)
                            }

                            is TranscoderEvent.Queued -> queue.add(it.toJob())

                            is TranscoderEvent.Started -> queue[0] = it.toJob()

                            is TranscoderEvent.Progress -> queue[0] = it.toJob()

                            is TranscoderEvent.Success -> queue.removeAt(0)

                            is TranscoderEvent.Failure -> {
                                queue.removeAt(0)
                                showError("Transcoding failed (id: ${it.id})")
                            }
                        }
                    }
            } catch (t: Throwable) {
                showError(t.message ?: "Disconnected from server")
                delay(1000)
            }
        }
    }

    TranscodeQueueScreen(
        client = client,
        queue = queue,
        onNavigateUp = onNavigateUp,
    )
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun TranscodeQueueScreen(
    client: ZenithApiClient,
    queue: List<TranscoderJob>,
    onNavigateUp: () -> Unit,
) {
    Scaffold(
        topBar = { AppBar(title = "Transcode queue", onBackPressed = onNavigateUp) },
        modifier = Modifier.navigationBarsPadding(),
    ) {
        if (queue.isEmpty()) {
            Box(modifier = Modifier.fillMaxSize()) {
                Text(
                    text = "Transcode queue is empty",
                    modifier = Modifier.align(Alignment.Center),
                )
            }
        } else {
            LazyColumn(contentPadding = PaddingValues(16.dp)) {
                items(queue.filterIsInstance<TranscoderJob.Processing>()) {
                    Text("Processing", style = MaterialTheme.typography.subtitle2)
                    TranscodeQueueListItem(client = client, id = it.id, progress = it.progress)
                }

                if (queue.size > 1) {
                    item {
                        Spacer(modifier = Modifier.height(8.dp))
                        Text("Queued", style = MaterialTheme.typography.subtitle2)
                    }
                }

                items(queue.filterIsInstance<TranscoderJob.Queued>()) {
                    TranscodeQueueListItem(client = client, id = it.id)
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun TranscodeQueueListItem(client: ZenithApiClient, id: Int, progress: Double? = null) {
    // TODO: This is not ideal - return this info directly from transcoder api
    val info by produceState<VideoInfo?>(initialValue = null, id) {
        value = client.getVideoInfo(id)
    }

    ListItem {
        if (progress != null) {
            Column {
                Text(
                    info?.path ?: id.toString(),
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis,
                )
                Spacer(modifier = Modifier.height(8.dp))
                LinearProgressIndicator(
                    progress = progress.toFloat(),
                    modifier = Modifier.fillMaxWidth(),
                )
            }
        } else {
            Text(info?.path ?: id.toString())
        }
    }
}
