package uk.hasali.zenith

import android.content.Context
import android.media.AudioManager
import android.os.Bundle
import android.view.SoundEffectConstants
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import androidx.lifecycle.ViewModel
import com.google.accompanist.coil.rememberCoilPainter
import com.google.accompanist.insets.ProvideWindowInsets
import com.google.accompanist.insets.statusBarsHeight
import com.zachklipp.compose.backstack.Backstack
import io.ktor.client.*
import io.ktor.client.features.json.*
import io.ktor.client.features.json.serializer.*
import io.ktor.client.request.*
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

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
)

sealed class Screen {
    object Shows : Screen()
    data class ShowDetails(val show: Show) : Screen()
    data class SeasonDetails(val show: Show, val season: Season) : Screen()
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

    @OptIn(ExperimentalAnimationApi::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        setContent {
            val client = remember {
                HttpClient() {
                    install(JsonFeature) {
                        serializer = KotlinxSerializer(kotlinx.serialization.json.Json {
                            ignoreUnknownKeys = true
                        })
                    }
                }
            }

            AppTheme {
                ProvideWindowInsets {
                    Backstack(backstack = navigator.stack) { screen ->
                        when (screen) {
                            is Screen.Shows -> ShowsScreen(client = client, navigator = navigator)
                            is Screen.ShowDetails -> ShowDetailsScreen(
                                client = client,
                                navigator = navigator,
                                show = screen.show,
                            )
                            is Screen.SeasonDetails -> SeasonDetailsScreen(
                                client = client,
                                show = screen.show,
                                season = screen.season,
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
fun AppBar(title: String = "Zenith") {
    Surface(color = MaterialTheme.colors.primarySurface, elevation = 4.dp) {
        Column {
            Spacer(modifier = Modifier.statusBarsHeight())
            TopAppBar(
                title = { Text(title) },
                backgroundColor = Color.Transparent,
                elevation = 0.dp,
            )
        }
    }
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun ShowsScreen(client: HttpClient, navigator: Navigator) {
    val shows by produceState(initialValue = emptyList<Show>()) {
        value = client.get("https://zenith.hasali.uk/api/tv/shows")
    }

    Scaffold(topBar = { AppBar() }) {
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
fun SeasonDetailsScreen(client: HttpClient, show: Show, season: Season) {
    val context = LocalContext.current
    val episodes by produceState(initialValue = emptyList<Episode>()) {
        value = client.get("https://zenith.hasali.uk/api/tv/seasons/${season.id}/episodes")
    }

    Scaffold(topBar = { AppBar(title = season.name) }) {
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
                                        }
                                )
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
