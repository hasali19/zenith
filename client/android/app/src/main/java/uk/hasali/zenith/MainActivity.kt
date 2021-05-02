package uk.hasali.zenith

import android.content.Context
import android.media.AudioManager
import android.os.Bundle
import android.view.SoundEffectConstants
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.GridCells
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

sealed class Screen {
    object Shows : Screen()
    data class ShowDetails(val show: Show) : Screen()
}

class Navigator {
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
    private val navigator = Navigator()

    @OptIn(ExperimentalAnimationApi::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        setContent {
            AppTheme {
                ProvideWindowInsets {
                    Backstack(backstack = navigator.stack) { screen ->
                        when (screen) {
                            is Screen.Shows -> ShowsScreen(navigator = navigator)
                            is Screen.ShowDetails -> ShowDetailsScreen(show = screen.show)
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
fun AppBar() {
    Surface(color = MaterialTheme.colors.primarySurface, elevation = 4.dp) {
        Column {
            Spacer(modifier = Modifier.statusBarsHeight())
            TopAppBar(
                title = { Text("Zenith") },
                backgroundColor = Color.Transparent,
                elevation = 0.dp,
            )
        }
    }
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun ShowsScreen(navigator: Navigator) {
    val client = remember {
        HttpClient() {
            install(JsonFeature) {
                serializer = KotlinxSerializer(kotlinx.serialization.json.Json {
                    ignoreUnknownKeys = true
                })
            }
        }
    }

    val shows by produceState(initialValue = emptyList<Show>()) {
        value = client.get("https://zenith.hasali.uk/api/tv/shows")
    }

    Scaffold(topBar = { AppBar() }) {
        LazyVerticalGrid(
            cells = GridCells.Adaptive(120.dp),
            contentPadding = PaddingValues(4.dp),
        ) {
            items(shows.size) { i ->
                ShowGridItem(
                    show = shows[i],
                    onClick = { navigator.push(Screen.ShowDetails(shows[i])) },
                )
            }
        }
    }
}

@Composable
fun ShowDetailsScreen(show: Show) {
    Surface(modifier = Modifier.fillMaxSize()) {
        Column {
            Image(
                painter = rememberCoilPainter(request = show.backdrop, fadeIn = true),
                contentDescription = "Backdrop",
                modifier = Modifier.aspectRatio(16f / 9f)
            )

            Column(modifier = Modifier.offset(y = (-48).dp)) {
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
            }
        }
    }
}

@Composable
fun ShowGridItem(show: Show, onClick: () -> Unit) {
    val context = LocalContext.current

    @Composable
    fun Poster(width: Dp, height: Dp) {
        Card {
            Image(
                painter = rememberCoilPainter(
                    show.poster,
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
        val year = Instant.fromEpochSeconds(show.startDate)
            .toLocalDateTime(TimeZone.UTC)
            .year

        Column(modifier = Modifier.padding(vertical = 4.dp)) {
            Text(
                show.name,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                style = MaterialTheme.typography.subtitle2
            )

            Text(
                year.toString(),
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
