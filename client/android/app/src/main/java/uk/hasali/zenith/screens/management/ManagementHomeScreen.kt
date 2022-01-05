package uk.hasali.zenith.screens.management

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Speed
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import uk.hasali.zenith.ui.AppBar

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun ManagementHomeScreen(onNavigateToTranscodeQueue: () -> Unit) {
    Scaffold(topBar = { AppBar("Manage Server") }) {
        Column(modifier = Modifier.verticalScroll(rememberScrollState())) {
            ListItem(
                icon = { Icon(Icons.Default.Speed, null) },
                modifier = Modifier.clickable(onClick = onNavigateToTranscodeQueue),
            ) {
                Text("Transcode Queue")
            }
        }
    }
}
