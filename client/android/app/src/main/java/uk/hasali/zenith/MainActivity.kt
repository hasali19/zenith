package uk.hasali.zenith

import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material.AlertDialog
import androidx.compose.material.LinearProgressIndicator
import androidx.compose.material.Text
import androidx.compose.material.TextButton
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.lifecycleScope
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.google.accompanist.insets.ProvideWindowInsets
import com.google.android.gms.cast.framework.CastContext
import io.ktor.client.*
import io.ktor.client.features.json.*
import io.ktor.client.features.json.serializer.*
import kotlinx.coroutines.launch
import uk.hasali.zenith.ui.AppTheme
import uk.hasali.zenith.ui.LocalZenithClient

class MainActivity : FragmentActivity() {
    private lateinit var client: HttpClient

    private var isUpdateAvailable by mutableStateOf(false)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        client = HttpClient {
            install(JsonFeature) {
                serializer = KotlinxSerializer(kotlinx.serialization.json.Json {
                    ignoreUnknownKeys = true
                })
            }
        }

        // Initialise cast context
        CastContext.getSharedInstance(this)

        setContent {
            App()
        }
    }

    override fun onResume() {
        super.onResume()

        lifecycleScope.launch {
            isUpdateAvailable = checkForUpdates()
        }
    }

    @Composable
    private fun App() {
        val zenithApiClient = remember { ZenithApiClient(client) }
        val nav = rememberNavController()

        AppTheme {
            ProvideWindowInsets {
                val entry by nav.currentBackStackEntryAsState()
                val route = entry?.destination?.route

                if (isUpdateAvailable && route?.startsWith("player") != true) {
                    UpdateDialog()
                }

                CompositionLocalProvider(
                    LocalZenithClient provides zenithApiClient,
                ) {
                    AppNavigation(nav = nav)
                }
            }
        }
    }

    @Composable
    private fun UpdateDialog() {
        val scope = rememberCoroutineScope()
        var isDownloading by remember { mutableStateOf(false) }
        var progress by remember { mutableStateOf(0f) }

        AlertDialog(
            title = { Text("Update") },
            text = {
                Column(modifier = Modifier.fillMaxWidth()) {
                    Text("An update is available")

                    if (isDownloading) {
                        Spacer(modifier = Modifier.height(16.dp))
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            LinearProgressIndicator(modifier = Modifier.weight(1f))
                            Spacer(modifier = Modifier.width(16.dp))
                            Text(text = "%.1f MiB".format(progress))
                        }
                    }
                }
            },
            confirmButton = {
                TextButton(
                    enabled = !isDownloading,
                    onClick = {
                        playClick()
                        isDownloading = true
                        scope.launch {
                            AppUpdater(application, client)
                                .downloadAndInstall { progress = it }
                        }
                    },
                ) {
                    Text("Install")
                }
            },
            onDismissRequest = {
                // Ignore
            }
        )
    }

    private suspend fun checkForUpdates(): Boolean {
        if (BuildConfig.DEBUG) {
            return false
        }

        val github = GitHubApiClient(client)
        val res = github.getActionsWorkflowRuns(5604606)
        val run = res.workflowRuns.firstOrNull {
            it.status == "completed" && it.conclusion == "success"
        }

        return run != null && run.headSha != BuildConfig.GIT_COMMIT_HASH
    }
}
