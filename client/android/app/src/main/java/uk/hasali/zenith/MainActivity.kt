package uk.hasali.zenith

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.layout.*
import androidx.compose.material.AlertDialog
import androidx.compose.material.LinearProgressIndicator
import androidx.compose.material.Text
import androidx.compose.material.TextButton
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveableStateHolder
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import androidx.lifecycle.lifecycleScope
import com.google.accompanist.insets.ProvideWindowInsets
import io.ktor.client.*
import io.ktor.client.features.json.*
import io.ktor.client.features.json.serializer.*
import kotlinx.coroutines.launch
import soup.compose.material.motion.MaterialMotion
import soup.compose.material.motion.materialFadeThrough
import uk.hasali.zenith.ui.*

class MainActivity : ComponentActivity() {
    private var navigator: Navigator? = null

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
        if (navigator?.pop() != true) {
            super.onBackPressed()
        }
    }

    @Composable
    private fun App() {
        val zenithApiClient = remember { ZenithApiClient(client) }
        val saveableStateHolder = rememberSaveableStateHolder()

        val navigator = remember { Navigator(saveableStateHolder) }
            .also { this.navigator = it }

        AppTheme {
            ProvideWindowInsets {
                if (isUpdateAvailable && navigator.currentScreen !is Screen.Player) {
                    UpdateDialog()
                }

                CompositionLocalProvider(
                    LocalNavigator provides navigator,
                    LocalZenithClient provides zenithApiClient,
                ) {
                    MaterialMotion(
                        targetState = navigator.currentScreen,
                        motionSpec = materialFadeThrough(),
                    ) { screen ->
                        navigator.SaveableStateProvider(screen) {
                            Screen(screen)
                        }
                    }
                }
            }
        }
    }

    @Composable
    private fun Screen(screen: Screen) {
        when (screen) {
            is Screen.ImportQueue -> ImportQueueScreen()
            is Screen.TranscodeQueue -> TranscodeQueueScreen()
            is Screen.Main -> MainScreen()
            is Screen.ShowDetails -> ShowDetailsScreen(show = screen.show)
            is Screen.SeasonDetails -> SeasonDetailsScreen(
                show = screen.show,
                season = screen.season,
            )
            is Screen.EpisodeDetails -> EpisodeDetailsScreen(
                season = screen.season,
                episode = screen.episode,
            )
            is Screen.Player -> PlayerScreen(id = screen.id)
        }
    }

    @Composable
    private fun UpdateDialog() {
        val scope = rememberCoroutineScope()
        var isDownloading by remember { mutableStateOf(false) }
        var progress by remember { mutableStateOf(0f) }

        AlertDialog(
            title = {
                Text("Update")
            },
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
