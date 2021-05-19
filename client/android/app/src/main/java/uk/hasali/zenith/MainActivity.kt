package uk.hasali.zenith

import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.content.pm.PackageInstaller
import android.media.AudioManager
import android.os.Bundle
import android.view.SoundEffectConstants
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
import androidx.lifecycle.ViewModel
import com.google.accompanist.insets.ProvideWindowInsets
import com.zachklipp.compose.backstack.Backstack
import io.ktor.client.*
import io.ktor.client.features.json.*
import io.ktor.client.features.json.serializer.*
import io.ktor.client.request.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uk.hasali.zenith.ui.*
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.IOException
import java.util.zip.ZipInputStream

sealed class Screen {
    object TranscodeQueue : Screen()
    object Shows : Screen()
    data class ShowDetails(val show: Show) : Screen()
    data class SeasonDetails(val show: Show, val season: Season) : Screen()
    data class Player(val id: Int) : Screen()
}

class Navigator : ViewModel() {
    var stack by mutableStateOf(listOf<Screen>(Screen.Shows))

    fun push(screen: Screen) {
        stack = stack + screen
    }

    fun pop(): Boolean {
        return if (stack.size > 1) {
            stack = stack.dropLast(1)
            true
        } else {
            false
        }
    }
}

class MainActivity : ComponentActivity() {
    private val navigator: Navigator by viewModels()

    private lateinit var client: HttpClient

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
            var showUpdateDialog by remember { mutableStateOf(false) }

            LaunchedEffect(Unit) {
                if (checkForUpdates()) {
                    showUpdateDialog = true
                }
            }

            AppTheme {
                if (showUpdateDialog) {
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
                                    val audioManager =
                                        getSystemService(Context.AUDIO_SERVICE) as AudioManager
                                    audioManager.playSoundEffect(SoundEffectConstants.CLICK, 1.0f)
                                    isDownloading = true
                                    installApk("https://nightly.link/hasali19/zenith/workflows/android/android/zenith-apk.zip")
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

                val zenithApiClient = ZenithApiClient(client)

                ProvideWindowInsets {
                    Backstack(backstack = navigator.stack) { screen ->
                        when (screen) {
                            is Screen.TranscodeQueue -> TranscodeQueueScreen(
                                client = zenithApiClient,
                                navigator = navigator,
                            )
                            is Screen.Shows -> ShowsScreen(
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
                                season = screen.season,
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
    }

    override fun onBackPressed() {
        if (!navigator.pop()) {
            super.onBackPressed()
        }
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

    private fun installApk(url: String) {
        CoroutineScope(Dispatchers.Main).launch {
            val response: ByteArray = client.get(url)
            val zip = ZipInputStream(ByteArrayInputStream(response))
            val content = ByteArrayOutputStream()

            zip.nextEntry
            zip.copyTo(content)

            var session: PackageInstaller.Session? = null
            try {
                val installer = packageManager.packageInstaller
                val params =
                    PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL)

                withContext(Dispatchers.IO) {
                    val sessionId = installer.createSession(params)

                    session = installer.openSession(sessionId)
                    session?.let { session ->
                        session.openWrite("package", 0, -1).use { output ->
                            ByteArrayInputStream(content.toByteArray()).copyTo(output)
                            session.fsync(output)
                        }
                    }
                }

                val intent = Intent(application, InstallReceiver::class.java)
                val pendingIntent = PendingIntent.getBroadcast(
                    application,
                    3439,
                    intent,
                    PendingIntent.FLAG_UPDATE_CURRENT
                )
                val receiver = pendingIntent.intentSender

                session?.commit(receiver)
                session?.close()
            } catch (e: IOException) {
                throw RuntimeException("Couldn't install package", e)
            } catch (e: RuntimeException) {
                session?.abandon()
                throw e
            }
        }
    }
}
