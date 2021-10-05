package uk.hasali.zenith

import android.content.Intent
import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.withStyle
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
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.launch
import uk.hasali.zenith.ui.AppTheme
import uk.hasali.zenith.ui.LocalZenithClient

class MainActivity : FragmentActivity() {
    private lateinit var preferences: Preferences
    private lateinit var client: HttpClient

    private var availableUpdate by mutableStateOf<AvailableUpdate?>(null)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        preferences = Preferences(this)

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
            availableUpdate = checkForUpdates()
        }
    }

    @Composable
    private fun App() {
        var loading by remember { mutableStateOf(true) }
        var serverUrl by remember { mutableStateOf<String?>(null) }

        LaunchedEffect(preferences) {
            preferences.serverUrl
                .collect {
                    loading = false
                    serverUrl = it
                }
        }

        if (!loading) {
            App(serverUrl)
        }
    }

    @Composable
    private fun App(serverUrl: String?) {
        when (serverUrl) {
            null -> LaunchedEffect(Unit) {
                val intent = Intent(
                    this@MainActivity,
                    SelectServerActivity::class.java,
                )
                startActivity(intent)
                finish()
            }

            else -> {
                val zenithApiClient = remember { ZenithApiClient(client, serverUrl) }
                val nav = rememberNavController()

                AppTheme {
                    ProvideWindowInsets {
                        val entry by nav.currentBackStackEntryAsState()
                        val route = entry?.destination?.route

                        availableUpdate?.let {
                            if (route?.startsWith("player") != true) {
                                UpdateDialog(availableUpdate = it)
                            }
                        }

                        CompositionLocalProvider(
                            LocalZenithClient provides zenithApiClient,
                        ) {
                            AppNavigation(nav = nav)
                        }
                    }
                }
            }
        }
    }

    @Composable
    private fun UpdateDialog(availableUpdate: AvailableUpdate) {
        val scope = rememberCoroutineScope()
        var isDownloading by remember { mutableStateOf(false) }
        var progress by remember { mutableStateOf(0f) }

        AlertDialog(
            title = { Text("Update") },
            text = {
                Column(modifier = Modifier.fillMaxWidth()) {
                    Text("An update is available")
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(
                        buildAnnotatedString {
                            withStyle(MaterialTheme.typography.subtitle2.toSpanStyle()) {
                                append("Installed: ")
                            }

                            append(availableUpdate.installed)
                        }
                    )
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(
                        buildAnnotatedString {
                            withStyle(MaterialTheme.typography.subtitle2.toSpanStyle()) {
                                append("Latest: ")
                            }

                            append(availableUpdate.latest)
                        }
                    )

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

    data class AvailableUpdate(val installed: String, val latest: String)

    private suspend fun checkForUpdates(): AvailableUpdate? {
        if (BuildConfig.DEBUG) {
            return null
        }

        val github = GitHubApiClient(client)
        val res = github.getActionsWorkflowRuns(5604606)
        val run = res.workflowRuns.firstOrNull {
            it.status == "completed" && it.conclusion == "success"
        }

        if (run == null || run.headSha == BuildConfig.GIT_COMMIT_HASH) {
            return null
        }

        return AvailableUpdate(
            installed = BuildConfig.GIT_COMMIT_HASH,
            latest = run.headSha,
        )
    }
}
