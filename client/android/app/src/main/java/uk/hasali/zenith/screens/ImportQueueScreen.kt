package uk.hasali.zenith.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.items
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.DoneAll
import androidx.compose.material.icons.filled.Videocam
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.navigationBarsPadding
import kotlinx.coroutines.launch
import uk.hasali.zenith.ImportQueueItem
import uk.hasali.zenith.playClick
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.ImportItemDialog
import uk.hasali.zenith.ui.LocalZenithClient
import uk.hasali.zenith.ui.SwipeRefreshLazyColumn

@Composable
fun ImportQueueScreen(onNavigateUp: () -> Unit) {
    val client = LocalZenithClient.current
    val scope = rememberCoroutineScope()

    var isRefreshing by remember { mutableStateOf(false) }
    var items by remember { mutableStateOf<List<ImportQueueItem>?>(null) }

    suspend fun refresh() {
        isRefreshing = true
        items = client.getImportQueue()
        isRefreshing = false
    }

    LaunchedEffect(Unit) {
        refresh()
    }

    ImportQueueScreen(
        items = items,
        isRefreshing = isRefreshing,
        onRefresh = { scope.launch { refresh() } },
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun ImportQueueScreen(
    items: List<ImportQueueItem>?,
    isRefreshing: Boolean,
    onRefresh: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    var selected: ImportQueueItem? by remember { mutableStateOf(null) }

    Scaffold(
        topBar = { AppBar(title = "Import queue", onBackPressed = onNavigateUp) },
        modifier = Modifier.navigationBarsPadding(),
    ) {
        SwipeRefreshLazyColumn(
            isRefreshing = isRefreshing,
            isEmpty = items?.isEmpty() == true,
            onRefresh = onRefresh,
            emptyContent = { ImportQueueEmpty() },
        ) {
            if (items != null) {
                items(items) {
                    ImportQueueListItem(item = it) {
                        selected = it
                    }
                }
            }
        }

        selected?.let {
            ImportItemDialog(item = it) {
                selected = null
                onRefresh()
            }
        }
    }
}

@Composable
private fun BoxScope.ImportQueueEmpty() {
    Row(horizontalArrangement = Arrangement.Center, modifier = Modifier.align(Alignment.Center)) {
        Icon(Icons.Default.DoneAll, contentDescription = "All done")
        Spacer(modifier = Modifier.width(8.dp))
        Text("All done 🙂")
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun ImportQueueListItem(item: ImportQueueItem, onClick: () -> Unit) {
    val context = LocalContext.current

    ListItem(
        icon = { Icon(Icons.Default.Videocam, contentDescription = "Video") },
        secondaryText = {
            Text(
                text = item.path,
                maxLines = 2,
                overflow = TextOverflow.Ellipsis,
                modifier = Modifier.padding(bottom = 12.dp),
            )
        },
        modifier = Modifier.clickable {
            context.playClick()
            onClick()
        },
    ) {
        Text(text = item.name)
    }
}
