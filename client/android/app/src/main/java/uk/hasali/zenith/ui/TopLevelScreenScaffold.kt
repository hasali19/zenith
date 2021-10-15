package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier

@Composable
fun TopLevelScreenScaffold(
    onNavigate: (String) -> Unit,
    content: @Composable () -> Unit,
) {
    Scaffold(topBar = { AppBar(onNavigate = onNavigate) }) { padding ->
        Box(modifier = Modifier.padding(padding)) {
            content()
        }
    }
}

@Composable
private fun AppBar(onNavigate: (String) -> Unit) {
    AppBar(title = "Zenith") {
        CastButton()
        AppBarOverflowMenu {
            item("Import queue") {
                onNavigate("import_queue")
            }

            item("Transcode queue") {
                onNavigate("transcode_queue")
            }

            item("About") {
                onNavigate("about")
            }
        }
    }
}
