package uk.hasali.zenith.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.google.accompanist.insets.navigationBarsPadding
import uk.hasali.zenith.BuildConfig
import uk.hasali.zenith.ui.AppBar

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun AboutScreen(onNavigateUp: () -> Unit) {
    Scaffold(
        topBar = { AppBar(title = "About", onBackPressed = onNavigateUp) },
        modifier = Modifier.navigationBarsPadding(),
    ) {
        Column(modifier = Modifier.verticalScroll(rememberScrollState())) {
            ListItem(secondaryText = { Text(BuildConfig.APPLICATION_ID) }) {
                Text("App ID")
            }
            Divider()
            ListItem(secondaryText = { Text(BuildConfig.BUILD_TYPE) }) {
                Text("Build type")
            }
            Divider()
            ListItem(secondaryText = { Text("${BuildConfig.VERSION_NAME}-${BuildConfig.VERSION_CODE}") }) {
                Text("Version")
            }
            Divider()
            ListItem(secondaryText = { Text(BuildConfig.GIT_COMMIT_HASH) }) {
                Text("Revision")
            }
            Divider()
        }
    }
}
