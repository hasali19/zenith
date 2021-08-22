package uk.hasali.zenith.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
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
import uk.hasali.zenith.*
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.CenteredLoadingIndicator
import uk.hasali.zenith.ui.ImportItemDialog
import uk.hasali.zenith.ui.LocalZenithClient

@Composable
fun ImportQueueScreen(onNavigateUp: () -> Unit) {
    val client = LocalZenithClient.current
    val scope = rememberCoroutineScope()

    var items by remember { mutableStateOf<List<ImportQueueItem>?>(null) }

    LaunchedEffect(Unit) {
        items = client.getImportQueue()
    }

    ImportQueueScreen(
        items = items,
        onRefreshItems = {
            scope.launch {
                items = client.getImportQueue()
            }
        },
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun ImportQueueScreen(
    items: List<ImportQueueItem>?,
    onRefreshItems: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    var selected: ImportQueueItem? by remember { mutableStateOf(null) }

    Scaffold(
        topBar = { AppBar(title = "Import queue", onBackPressed = onNavigateUp) },
        modifier = Modifier.navigationBarsPadding(),
    ) {
        when {
            items == null -> CenteredLoadingIndicator()

            items.isEmpty() -> Row(
                modifier = Modifier.fillMaxSize(),
                horizontalArrangement = Arrangement.Center,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Icon(Icons.Default.DoneAll, contentDescription = "All done")
                Spacer(modifier = Modifier.width(8.dp))
                Text("All done 🙂")
            }

            else -> ImportQueueList(
                items = items,
                onSelectItem = { selected = it },
            )
        }

        selected?.let {
            ImportItemDialog(item = it) {
                selected = null
                onRefreshItems()
            }
        }
    }
}

@Composable
private fun ImportQueueList(items: List<ImportQueueItem>, onSelectItem: (ImportQueueItem) -> Unit) {
    LazyColumn {
        items(items) {
            ImportQueueListItem(item = it) {
                onSelectItem(it)
            }
        }
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
