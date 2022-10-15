package uk.hasali.zenith.screens.management

import android.widget.Toast
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.flowWithLifecycle
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.delay
import uk.hasali.zenith.api.*
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.LocalZenithClient
import javax.inject.Inject

@HiltViewModel
class TranscodeQueueViewModel @Inject constructor(
    private val events: ZenithEventsService,
) : ViewModel() {
    suspend fun getEvents() = events.getTranscoderEvents()
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun TranscodeQueueScreen(
    model: TranscodeQueueViewModel = hiltViewModel(),
    onNavigateUp: () -> Unit,
) {
    val context = LocalContext.current
    val client = LocalZenithClient.current
    val lifecycleOwner = LocalLifecycleOwner.current

    val queue = remember { mutableStateListOf<TranscoderJob>() }

    fun showError(message: String) {
        Toast.makeText(context, message, Toast.LENGTH_SHORT)
            .show()
    }

    LaunchedEffect(Unit) {
        while (true) {
            try {
                model.getEvents()
                    .flowWithLifecycle(lifecycleOwner.lifecycle)
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

                            is TranscoderEvent.Error -> {
                                queue.removeAt(0)
                                showError("Transcoding failed (id: ${it.id})")
                            }
                        }
                    }
            } catch (t: Throwable) {
                if (t is CancellationException) {
                    throw t
                }

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
    client: ZenithMediaService,
    queue: List<TranscoderJob>,
    onNavigateUp: () -> Unit,
) {
    Scaffold(
        topBar = { AppBar(title = "Transcode queue", onBackPressed = onNavigateUp) },
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
private fun TranscodeQueueListItem(client: ZenithMediaService, id: Int, progress: Double? = null) {
    val item by produceState<MediaItem?>(initialValue = null, id) {
        value = client.getItem(id)
    }

    val label = item.let {
        when (it) {
            is Movie -> it.videoInfo.path
            is Episode -> it.videoInfo.path
            else -> id.toString()
        }
    }

    ListItem {
        if (progress != null) {
            Column {
                Text(label, maxLines = 1, overflow = TextOverflow.Ellipsis)
                Spacer(modifier = Modifier.height(8.dp))
                LinearProgressIndicator(
                    progress = progress.toFloat(),
                    modifier = Modifier.fillMaxWidth(),
                )
            }
        } else {
            Text(label)
        }
    }
}
