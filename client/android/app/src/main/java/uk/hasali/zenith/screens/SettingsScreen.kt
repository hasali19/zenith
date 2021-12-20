package uk.hasali.zenith.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.lifecycle.ViewModel
import dagger.hilt.android.lifecycle.HiltViewModel
import uk.hasali.zenith.BuildConfig
import uk.hasali.zenith.Preferences
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.AppBar
import javax.inject.Inject

@HiltViewModel
class SettingsViewModel @Inject constructor(preferences: Preferences) : ViewModel() {
    val server = preferences.serverUrl
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun SettingsScreen(model: SettingsViewModel = hiltViewModel(), onLaunchSelectServer: () -> Unit) {
    val server by model.server.collectAsState("")

    Scaffold(topBar = { AppBar(title = "Settings") }) {
        Column(modifier = Modifier.verticalScroll(rememberScrollState())) {
            ListItem(
                secondaryText = { Text(server.orEmpty()) },
                modifier = Modifier.clickable(onClick = onLaunchSelectServer),
            ) {
                Text("Server")
            }

            Divider()

            ListItem(secondaryText = { Text(BuildConfig.APPLICATION_ID) }) {
                Text("App ID")
            }

            ListItem(secondaryText = { Text(BuildConfig.BUILD_TYPE) }) {
                Text("Build type")
            }

            ListItem(secondaryText = { Text("${BuildConfig.VERSION_NAME}-${BuildConfig.VERSION_CODE}") }) {
                Text("Version")
            }

            ListItem(secondaryText = { Text(BuildConfig.GIT_COMMIT_HASH) }) {
                Text("Revision")
            }
        }
    }
}
