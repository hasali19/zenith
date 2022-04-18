package uk.hasali.zenith

import android.content.Intent
import android.content.res.Configuration
import android.os.Build
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
import androidx.core.app.NotificationChannelCompat
import androidx.core.app.NotificationManagerCompat
import androidx.core.view.WindowCompat
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import com.google.accompanist.insets.ProvideWindowInsets
import com.google.android.gms.cast.framework.CastContext
import dagger.Lazy
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch
import okhttp3.OkHttpClient
import uk.hasali.zenith.api.ZenithMediaService
import uk.hasali.zenith.media.MediaSessionManager
import uk.hasali.zenith.ui.AppTheme
import uk.hasali.zenith.ui.LocalZenithClient
import javax.inject.Inject

@AndroidEntryPoint
class MainActivity : FragmentActivity() {
    @Inject
    lateinit var preferences: Preferences

    @Inject
    lateinit var github: GitHubService

    @Inject
    lateinit var serverUrlProvider: ServerUrlProvider

    @Inject
    lateinit var zenith: Lazy<ZenithMediaService>

    @Inject
    lateinit var mediaSessionManager: MediaSessionManager

    @Inject
    lateinit var mediaProgressReporter: Lazy<MediaProgressReporter>

    private val httpClient = OkHttpClient()

    private var availableUpdate by mutableStateOf<AvailableUpdate?>(null)

    private val pictureInPictureController = object : PictureInPictureController {
        var shouldEnterOnUserLeaveHint = false

        override val isInPictureInPictureMode = MutableStateFlow(
            Build.VERSION.SDK_INT >= Build.VERSION_CODES.N && this@MainActivity.isInPictureInPictureMode
        )

        override fun setEnterOnUserLeaveHint(value: Boolean) {
            shouldEnterOnUserLeaveHint = value
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        // Initialise cast context
        CastContext.getSharedInstance(this)

        // Create notification channel for media notifications
        NotificationManagerCompat.from(this)
            .createNotificationChannel(
                NotificationChannelCompat.Builder("media", NotificationManagerCompat.IMPORTANCE_LOW)
                    .setName("Media")
                    .build(),
            )

        mediaSessionManager.init()

        lifecycleScope.launch {
            val server = preferences.serverUrl.first()
            if (server == null) {
                startSelectServerActivity()
            } else {
                serverUrlProvider.url = server
                startMediaProgressReporter()
                setContent {
                    App()
                }
            }
        }
    }

    private fun startSelectServerActivity() {
        val intent = Intent(this, SelectServerActivity::class.java)
        startActivity(intent)
        finish()
    }

    private fun startMediaProgressReporter() {
        val reporter = mediaProgressReporter.get()
        lifecycleScope.launch {
            repeatOnLifecycle(Lifecycle.State.RESUMED) {
                reporter.run()
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()

        // Dispose current session if it is local
        val player = mediaSessionManager.getCurrentPlayer()
        if (player?.isLocal == true) {
            mediaSessionManager.endCurrentSession()
        }

        mediaSessionManager.dispose()
    }

    override fun onResume() {
        super.onResume()

        lifecycleScope.launch {
            availableUpdate = checkForUpdates()
        }
    }

    override fun onPause() {
        super.onPause()

        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.N || !isInPictureInPictureMode) {
            // Pause local playback
            val player = mediaSessionManager.getCurrentPlayer()
            if (player != null && player.isLocal) {
                player.setPlayWhenReady(false)
            }
        }
    }

    override fun onStop() {
        super.onStop()

        // Pause local playback
        val player = mediaSessionManager.getCurrentPlayer()
        if (player != null && player.isLocal) {
            player.setPlayWhenReady(false)
        }
    }

    override fun onPictureInPictureModeChanged(
        isInPictureInPictureMode: Boolean,
        newConfig: Configuration?
    ) {
        super.onPictureInPictureModeChanged(isInPictureInPictureMode, newConfig)
        pictureInPictureController.isInPictureInPictureMode.value = isInPictureInPictureMode
    }

    override fun onUserLeaveHint() {
        if (pictureInPictureController.shouldEnterOnUserLeaveHint) {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
                enterPictureInPictureMode()
            }
        }
    }

    @Composable
    private fun App() {
        AppTheme {
            ProvideWindowInsets {
                availableUpdate?.let {
                    UpdateDialog(availableUpdate = it)
                }

                CompositionLocalProvider(
                    LocalPictureInPictureController provides pictureInPictureController,
                    LocalZenithClient provides zenith.get(),
                ) {
                    AppNavigation(
                        mediaSessionManager = mediaSessionManager,
                        onLaunchSelectServer = this::startSelectServerActivity,
                    )
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
                        isDownloading = true
                        scope.launch {
                            AppUpdater(application, httpClient)
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

        val res = github.getActionsWorkflowRuns(5604606)
        val run = res.workflowRuns.firstOrNull {
            it.status == "completed" && it.conclusion == "success"
        }

        val installedCommit = getString(R.string.GIT_COMMIT_HASH)
        if (run == null || run.headBranch != "master" || run.headSha == installedCommit) {
            return null
        }

        return AvailableUpdate(
            installed = installedCommit,
            latest = run.headSha,
        )
    }
}
