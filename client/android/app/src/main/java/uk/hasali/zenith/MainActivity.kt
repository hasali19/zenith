package uk.hasali.zenith

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material.AlertDialog
import androidx.compose.material.LinearProgressIndicator
import androidx.compose.material.Text
import androidx.compose.material.TextButton
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import androidx.lifecycle.lifecycleScope
import com.google.accompanist.insets.ProvideWindowInsets
import com.zachklipp.compose.backstack.Backstack
import io.ktor.client.*
import io.ktor.client.features.json.*
import io.ktor.client.features.json.serializer.*
import kotlinx.coroutines.launch
import uk.hasali.zenith.ui.*

class MainActivity : ComponentActivity() {
    private val navigator: Navigator by viewModels()

    private lateinit var client: HttpClient

    private var isUpdateAvailable by mutableStateOf(false)

    @OptIn(ExperimentalAnimationApi::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        client = HttpClient() {
            install(JsonFeature) {
                serializer = KotlinxSerializer(kotlinx.serialization.json.Json {
                    ignoreUnknownKeys = true
                })
            }
        }

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

    override fun onBackPressed() {
        if (!navigator.pop()) {
            super.onBackPressed()
        }
    }

    @Composable
    private fun App() {
        val zenithApiClient = remember { ZenithApiClient(client) }

        AppTheme {
            ProvideWindowInsets {
                Backstack(backstack = navigator.stack) { screen ->
                    if (isUpdateAvailable && screen !is Screen.Player) {
                        UpdateDialog()
                    }

                    when (screen) {
                        is Screen.ImportQueue -> ImportQueueScreen(
                            client = zenithApiClient,
                            navigator = navigator,
                        )
                        is Screen.TranscodeQueue -> TranscodeQueueScreen(
                            client = zenithApiClient,
                            navigator = navigator,
                        )
                        is Screen.Main -> MainScreen(
                            client = zenithApiClient,
                            navigator = navigator,
                        )
                        is Screen.ShowDetails -> ShowDetailsScreen(
                            client = zenithApiClient,
                            navigator = navigator,
                            show = screen.show,
                        )
                        is Screen.SeasonDetails -> SeasonDetailsScreen(
                            client = zenithApiClient,
                            navigator = navigator,
                            show = screen.show,
                            season = screen.season,
                        )
                        is Screen.EpisodeDetails -> EpisodeDetailsScreen(
                            client = zenithApiClient,
                            navigator = navigator,
                            season = screen.season,
                            episode = screen.episode,
                        )
                        is Screen.Player -> PlayerScreen(
                            client = zenithApiClient,
                            navigator = navigator,
                            id = screen.id,
                        )
                    }
                }
            }
        }
    }

    @Composable
    private fun UpdateDialog() {
        val scope = rememberCoroutineScope()
        var isDownloading by remember { mutableStateOf(false) }

        AlertDialog(
            title = {
                Text("Update")
            },
            text = {
                Column(modifier = Modifier.fillMaxWidth()) {
                    Text("An update is available")

                    if (isDownloading) {
                        Spacer(modifier = Modifier.height(16.dp))
                        LinearProgressIndicator(modifier = Modifier.fillMaxWidth())
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
                                .downloadAndInstall()
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
