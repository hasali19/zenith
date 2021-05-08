package uk.hasali.zenith

import android.app.Activity
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.content.pm.PackageInstaller
import android.media.AudioManager
import android.os.Bundle
import android.view.SoundEffectConstants
import android.view.WindowManager
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import androidx.lifecycle.ViewModel
import com.google.accompanist.coil.rememberCoilPainter
import com.google.accompanist.insets.ProvideWindowInsets
import com.google.accompanist.insets.statusBarsHeight
import com.google.android.exoplayer2.ExoPlayer
import com.google.android.exoplayer2.MediaItem
import com.google.android.exoplayer2.Player
import com.google.android.exoplayer2.SimpleExoPlayer
import com.google.android.exoplayer2.ui.PlayerView
import com.zachklipp.compose.backstack.Backstack
import io.ktor.client.*
import io.ktor.client.features.json.*
import io.ktor.client.features.json.serializer.*
import io.ktor.client.request.*
import kotlinx.coroutines.*
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.IOException
import java.util.zip.ZipInputStream

@Serializable
data class Show(
    val id: Int,
    val name: String,
    val poster: String,
    val backdrop: String,
    val overview: String,
    @SerialName("start_date") val startDate: Long,
)

@Serializable
data class Season(
    val id: Int,
    val name: String,
    val poster: String,
)

@Serializable
data class Episode(
    val id: Int,
    val name: String,
    val overview: String,
    val thumbnail: String,
    val duration: Double,
    @SerialName("is_watched") val isWatched: Boolean,
)

@Serializable
data class StreamInfo(val duration: Double, val position: Double?)

@Serializable
data class VideoInfo(val path: String)

@Serializable
data class TranscoderState(val current: Int?, val queue: List<Int>)

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

                ProvideWindowInsets {
                    Backstack(backstack = navigator.stack) { screen ->
                        when (screen) {
                            is Screen.TranscodeQueue -> TranscodeQueueScreen(
                                client = client,
                                navigator = navigator,
                            )
                            is Screen.Shows -> ShowsScreen(client = client, navigator = navigator)
                            is Screen.ShowDetails -> ShowDetailsScreen(
                                client = client,
                                navigator = navigator,
                                show = screen.show,
                            )
                            is Screen.SeasonDetails -> SeasonDetailsScreen(
                                client = client,
                                navigator = navigator,
                                show = screen.show,
                                season = screen.season,
                            )
                            is Screen.Player -> PlayerScreen(
                                client = client,
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
            val entry = zip.nextEntry
            val content = ByteArrayOutputStream()

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

@Composable
fun AppTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colors = if (isSystemInDarkTheme()) darkColors() else lightColors(),
    ) {
        content()
    }
}

@Composable
fun AppBar(navigator: Navigator, title: String = "Zenith", menu: Boolean = true) {
    Surface(color = MaterialTheme.colors.primarySurface, elevation = 4.dp) {
        Column {
            Spacer(modifier = Modifier.statusBarsHeight())
            TopAppBar(
                title = { Text(title) },
                backgroundColor = Color.Transparent,
                elevation = 0.dp,
                actions = {
                    if (menu) {
                        AppBarMenu(navigator = navigator)
                    }
                }
            )
        }
    }
}

@Composable
fun AppBarMenu(navigator: Navigator) {
    var expanded by remember { mutableStateOf(false) }

    IconButton(onClick = { expanded = true }) {
        Icon(Icons.Default.MoreVert, contentDescription = "More")
    }

    DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
        DropdownMenuItem(onClick = {
            expanded = false
            navigator.push(Screen.TranscodeQueue)
        }) {
            Text("Transcode queue")
        }
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun TranscodeQueueScreen(client: HttpClient, navigator: Navigator) {
    val state by produceState<TranscoderState?>(initialValue = null) {
        value = client.get("https://zenith.hasali.uk/api/transcoder")
    }

    Scaffold(topBar = { AppBar(navigator = navigator, title = "Transcode queue", menu = false) }) {
        if (state == null) {
            Box(modifier = Modifier.fillMaxSize()) {
                CircularProgressIndicator(modifier = Modifier.align(Alignment.Center))
            }
        } else {
            LazyColumn(contentPadding = PaddingValues(16.dp)) {
                item {
                    state?.current?.let { id ->
                        Text("Current", style = MaterialTheme.typography.subtitle2)
                        TranscodeQueueListItem(client = client, id = id)
                    }
                }

                item {
                    Spacer(modifier = Modifier.height(8.dp))
                }

                item {
                    Text("Queued (${state?.queue?.size})", style = MaterialTheme.typography.subtitle2)
                }

                items(state!!.queue.size ?: 0) { i ->
                    TranscodeQueueListItem(client = client, id = state!!.queue[i])
                    Divider()
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun TranscodeQueueListItem(client: HttpClient, id: Int) {
    val info by produceState<VideoInfo?>(initialValue = null, id) {
        value = client.get("https://zenith.hasali.uk/api/videos/$id/info")
    }

    ListItem(text = { Text(info?.path ?: id.toString()) })
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun ShowsScreen(client: HttpClient, navigator: Navigator) {
    val shows by produceState(initialValue = emptyList<Show>()) {
        value = client.get("https://zenith.hasali.uk/api/tv/shows")
    }

    Scaffold(topBar = { AppBar(navigator = navigator) }) {
        LazyVerticalGrid(
            cells = GridCells.Adaptive(120.dp),
            contentPadding = PaddingValues(4.dp),
        ) {
            items(shows.size) { i ->
                val show = shows[i]
                val year = Instant.fromEpochSeconds(show.startDate)
                    .toLocalDateTime(TimeZone.UTC)
                    .year

                MediaItemWithPoster(
                    poster = show.poster,
                    primary = show.name,
                    secondary = year.toString(),
                    onClick = { navigator.push(Screen.ShowDetails(shows[i])) },
                )
            }
        }
    }
}

@Composable
fun ShowDetailsScreen(client: HttpClient, navigator: Navigator, show: Show) {
    val seasons by produceState(initialValue = emptyList<Season>()) {
        value = client.get("https://zenith.hasali.uk/api/tv/shows/${show.id}/seasons")
    }

    Surface(modifier = Modifier.fillMaxSize()) {
        BoxWithConstraints(modifier = Modifier.verticalScroll(rememberScrollState())) {
            Image(
                painter = rememberCoilPainter(request = show.backdrop, fadeIn = true),
                contentDescription = "Backdrop",
                modifier = Modifier.aspectRatio(16f / 9f)
            )

            val backdropHeight = with(LocalDensity.current) {
                (constraints.maxWidth * 9f / 16f).toDp()
            }

            Column(modifier = Modifier.padding(top = backdropHeight - 48.dp)) {
                Row(modifier = Modifier.padding(horizontal = 16.dp)) {
                    Card {
                        Image(
                            painter = rememberCoilPainter(request = show.poster, fadeIn = true),
                            contentDescription = "Poster",
                            modifier = Modifier
                                .width(140.dp)
                                .aspectRatio(2f / 3f),
                        )
                    }

                    Spacer(modifier = Modifier.width(16.dp))

                    Column(modifier = Modifier.align(Alignment.CenterVertically)) {
                        val year = Instant.fromEpochSeconds(show.startDate)
                            .toLocalDateTime(TimeZone.UTC)
                            .year

                        Text(show.name, style = MaterialTheme.typography.h5)
                        Text(year.toString(), style = MaterialTheme.typography.caption)
                    }
                }

                Spacer(modifier = Modifier.height(16.dp))

                Text(
                    show.overview,
                    style = MaterialTheme.typography.body2,
                    modifier = Modifier.padding(horizontal = 16.dp),
                )

                Spacer(modifier = Modifier.height(16.dp))

                LazyRow(contentPadding = PaddingValues(12.dp, 0.dp)) {
                    items(seasons.size) { i ->
                        val season = seasons[i]
                        Box(modifier = Modifier.width(120.dp)) {
                            MediaItemWithPoster(
                                poster = season.poster,
                                primary = season.name,
                                secondary = show.name,
                                onClick = {
                                    navigator.push(Screen.SeasonDetails(show, season))
                                }
                            )
                        }
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun SeasonDetailsScreen(client: HttpClient, navigator: Navigator, show: Show, season: Season) {
    val context = LocalContext.current
    val episodes by produceState(initialValue = emptyList<Episode>()) {
        value = client.get("https://zenith.hasali.uk/api/tv/seasons/${season.id}/episodes")
    }

    Scaffold(topBar = { AppBar(title = season.name, navigator = navigator) }) {
        LazyVerticalGrid(cells = GridCells.Adaptive(200.dp), contentPadding = PaddingValues(4.dp)) {
            items(episodes.size) { i ->
                val episode = episodes[i]

                BoxWithConstraints(modifier = Modifier.padding(4.dp)) {
                    with(LocalDensity.current) {
                        val width = constraints.maxWidth
                        val height = width * (9.0 / 16.0)

                        Column {
                            Card {
                                Image(
                                    painter = rememberCoilPainter(
                                        request = episode.thumbnail,
                                        fadeIn = true
                                    ),
                                    contentDescription = "Thumbnail",
                                    modifier = Modifier
                                        .size(
                                            width.toDp(),
                                            height
                                                .toInt()
                                                .toDp(),
                                        )
                                        .clickable {
                                            val audioManager =
                                                context.getSystemService(Context.AUDIO_SERVICE) as AudioManager
                                            audioManager.playSoundEffect(
                                                SoundEffectConstants.CLICK,
                                                1.0f
                                            )
                                            navigator.push(Screen.Player(episode.id))
                                        }
                                )

                                if (episode.isWatched) {
                                    Box(
                                        modifier = Modifier
                                            .size(
                                                width.toDp(),
                                                height
                                                    .toInt()
                                                    .toDp(),
                                            )
                                            .background(Color.Black.copy(alpha = 0.4f))
                                    ) {
                                        Icon(
                                            Icons.Default.Check,
                                            contentDescription = "Watched",
                                            modifier = Modifier.align(Alignment.Center),
                                        )
                                    }
                                }
                            }

                            Column(modifier = Modifier.padding(vertical = 4.dp)) {
                                val duration = if (episode.duration <= 90 * 60) {
                                    val minutes = (episode.duration / 60).toInt()
                                    "${minutes}m"
                                } else {
                                    val hours = (episode.duration / 3600).toInt()
                                    val minutes = ((episode.duration % 3600) / 60).toInt()
                                    "${hours}h ${minutes}m"

                                }

                                Text(
                                    episode.name,
                                    maxLines = 1,
                                    overflow = TextOverflow.Ellipsis,
                                    style = MaterialTheme.typography.subtitle2
                                )

                                Text(
                                    duration,
                                    maxLines = 1,
                                    overflow = TextOverflow.Ellipsis,
                                    color = Color.LightGray.copy(alpha = 0.8f),
                                    style = MaterialTheme.typography.caption
                                )

                                Text(
                                    episode.overview,
                                    maxLines = 3,
                                    overflow = TextOverflow.Ellipsis,
                                    style = MaterialTheme.typography.caption
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

@Composable
fun PlayerScreen(client: HttpClient, navigator: Navigator, id: Int) {
    val info by produceState<StreamInfo?>(initialValue = null, id) {
        value = client.get("https://zenith.hasali.uk/api/stream/$id/info")
    }

    KeepScreenOn {
        FullScreen {
            if (info != null) {
                VideoPlayer(
                    id = id,
                    client = client,
                    startPosition = info!!.position?.toInt() ?: 0,
                    onVideoEnded = { navigator.pop() },
                )
            }
        }
    }
}

@Composable
fun KeepScreenOn(content: @Composable() () -> Unit) {
    val activity = LocalContext.current as? Activity
    val window = activity?.window

    if (window != null) {
        DisposableEffect(Unit) {
            window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)

            onDispose {
                window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
            }
        }
    }

    content()
}

@Composable
fun FullScreen(content: @Composable() () -> Unit) {
    val activity = LocalContext.current as? Activity
    val window = activity?.window

    if (window != null) {
        DisposableEffect(Unit) {
            val controller = WindowCompat.getInsetsController(window, window.decorView)
            if (controller != null) {
                controller.hide(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
                controller.systemBarsBehavior =
                    WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
            }

            onDispose {
                controller?.show(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
            }
        }
    }

    content()
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
fun VideoPlayer(id: Int, client: HttpClient, startPosition: Int, onVideoEnded: () -> Unit) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val player = remember {
        SimpleExoPlayer.Builder(context)
            .build()
            .also { player ->
                player.addListener(object : Player.EventListener {
                    override fun onPlaybackStateChanged(state: Int) {
                        if (state == ExoPlayer.STATE_ENDED) {
                            onVideoEnded()
                        }
                    }
                })

                scope.launch {
                    while (true) {
                        if (player.playWhenReady) {
                            val position = player.currentPosition / 1000
                            launch {
                                client.post("https://zenith.hasali.uk/api/progress/$id?position=$position")
                            }
                        }

                        delay(2000)
                    }
                }
            }
    }

    DisposableEffect(id) {
        player.setMediaItem(MediaItem.fromUri("https://zenith.hasali.uk/api/videos/$id"))
        player.prepare()
        player.seekTo(startPosition.toLong() * 1000)
        player.play()

        onDispose {
            player.stop()
        }
    }

    DisposableEffect(Unit) {
        onDispose {
            player.release()
        }
    }

    Surface(
        color = Color.Black,
        modifier = Modifier.fillMaxSize(),
    ) {
        AndroidView(
            modifier = Modifier.fillMaxSize(),
            factory = { context -> PlayerView(context) },
            update = { playerView -> playerView.player = player },
        )
    }
}

@Composable
fun MediaItemWithPoster(poster: String, primary: String, secondary: String, onClick: () -> Unit) {
    val context = LocalContext.current

    @Composable
    fun Poster(width: Dp, height: Dp) {
        Card {
            Image(
                painter = rememberCoilPainter(
                    poster,
                    fadeIn = true
                ),
                contentDescription = "Poster",
                modifier = Modifier
                    .size(width, height)
                    .clickable {
                        val audioManager =
                            context.getSystemService(Context.AUDIO_SERVICE) as AudioManager
                        audioManager.playSoundEffect(SoundEffectConstants.CLICK, 1.0f)
                        onClick()
                    }
            )
        }
    }

    @Composable
    fun Content() {
        Column(modifier = Modifier.padding(vertical = 4.dp)) {
            Text(
                primary,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.subtitle2
            )

            Text(
                secondary,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.caption
            )
        }
    }

    BoxWithConstraints(modifier = Modifier.padding(4.dp)) {
        with(LocalDensity.current) {
            val width = constraints.maxWidth
            val height = width * 1.5

            Column {
                Poster(width = width.toDp(), height = height.toInt().toDp())
                Content()
            }
        }
    }
}
