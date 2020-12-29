package uk.co.hasali.zenith

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.layout.WithConstraints
import androidx.compose.ui.platform.AmbientContext
import androidx.compose.ui.platform.AmbientDensity
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.res.loadVectorResource
import androidx.compose.ui.res.vectorResource
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import com.google.gson.annotations.SerializedName
import dev.chrisbanes.accompanist.coil.CoilImage
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.ui.ZenithTheme

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            ZenithApp(settingsRepo = UserSettingsRepository.getInstance(this))
        }
    }
}

sealed class Screen(val name: String, val icon: @Composable () -> Unit) {
    object Home : Screen("Home", { Icon(Icons.Filled.Home) })

    object Movies : Screen("Movies", {
        loadVectorResource(id = R.drawable.movie).resource.resource?.let { Icon(it) }
    })

    object TvShows : Screen("TV Shows", {
        loadVectorResource(id = R.drawable.television).resource.resource?.let { Icon(it) }
    })

    object SelectServer : Screen("Select Server", {})
}

@Composable
fun ZenithApp(settingsRepo: UserSettingsRepository) {
    val scope = rememberCoroutineScope()

    val settings by settingsRepo.settings.collectAsState(initial = null)
    if (settings == null) {
        return
    }

    val serverUrl = settings!!.serverUrl
    var currentScreen: Screen by remember { mutableStateOf(Screen.Home) }
    val screen = if (serverUrl == null) Screen.SelectServer else currentScreen

    ZenithTheme {
        when (screen) {
            is Screen.SelectServer -> SelectServerScreen(onSave = {
                scope.launch {
                    settingsRepo.setServerUrl(it)
                }
            })

            else -> {
                TopLevelScreenScaffold(
                    screens = listOf(Screen.Home, Screen.Movies, Screen.TvShows),
                    currentScreen = currentScreen,
                    onScreenChange = { currentScreen = it }
                ) {
                    when (currentScreen) {
                        is Screen.Home -> HomeScreen()
                        is Screen.Movies -> MoviesScreen(serverUrl!!)
                        is Screen.TvShows -> TvShowsScreen(serverUrl!!)
                        else -> throw IllegalStateException()
                    }
                }
            }
        }
    }
}

@Composable
fun TopLevelScreenScaffold(
    screens: List<Screen>,
    currentScreen: Screen,
    onScreenChange: (Screen) -> Unit,
    content: @Composable () -> Unit
) {
    Scaffold(
        topBar = {
            TopAppBar(title = { Text(text = "Zenith") })
        },
        bodyContent = {
            Box(modifier = Modifier.padding(it)) {
                content()
            }
        },
        bottomBar = {
            BottomNavigation {
                screens.forEach { screen ->
                    BottomNavigationItem(
                        icon = screen.icon,
                        label = { Text(screen.name) },
                        selected = currentScreen.name == screen.name,
                        onClick = { if (currentScreen.name != screen.name) onScreenChange(screen) }
                    )
                }
            }
        }
    )
}

@Composable
fun HomeScreen() {
    Box(modifier = Modifier.fillMaxSize()) {
        Image(
            imageVector = Icons.Default.Home,
            modifier = Modifier.align(Alignment.Center).size(48.dp),
            colorFilter = ColorFilter.tint(Color.DarkGray)
        )
    }
}

data class Movie(
    val id: Int,
    val title: String,
    val year: Int?,
    @SerializedName("poster_url")
    val posterUrl: String?,
)

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun MoviesScreen(serverUrl: String) {
    var movies: List<Movie> by remember { mutableStateOf(emptyList()) }

    LaunchedEffect(serverUrl) {
        movies = Fuel.get("$serverUrl/api/movies")
            .awaitObject(gsonDeserializer())
    }

    LazyVerticalGrid(cells = GridCells.Adaptive(128.dp), contentPadding = PaddingValues(4.dp)) {
        items(movies) { movie ->
            Card(modifier = Modifier.padding(4.dp).fillMaxWidth()) {
                Column {
                    WithConstraints {
                        val height = with(AmbientDensity.current) {
                            constraints.maxWidth.toDp() * (3f / 2f)
                        }

                        Box(modifier = Modifier.fillMaxWidth().preferredHeight(height)) {
                            movie.posterUrl?.let { url ->
                                CoilImage(data = url, modifier = Modifier.fillMaxWidth())
                            }
                        }
                    }

                    Column(modifier = Modifier.padding(8.dp)) {
                        Text(
                            text = movie.title,
                            style = MaterialTheme.typography.body2,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis
                        )

                        Text(
                            text = movie.year?.toString() ?: "",
                            style = MaterialTheme.typography.caption,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis
                        )
                    }
                }
            }
        }
    }
}

data class TvShow(
    val id: Int,
    val name: String,
    @SerializedName("poster_url")
    val posterUrl: String?,
)

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun TvShowsScreen(serverUrl: String) {
    var shows: List<TvShow> by remember { mutableStateOf(emptyList()) }

    LaunchedEffect(serverUrl) {
        shows = Fuel.get("$serverUrl/api/tv_shows")
            .awaitObject(gsonDeserializer())
    }

    LazyVerticalGrid(cells = GridCells.Adaptive(128.dp), contentPadding = PaddingValues(4.dp)) {
        items(shows) { show ->
            Card(modifier = Modifier.padding(4.dp).fillMaxWidth()) {
                Column {
                    WithConstraints {
                        val height = with(AmbientDensity.current) {
                            constraints.maxWidth.toDp() * (3f / 2f)
                        }

                        Box(modifier = Modifier.fillMaxWidth().preferredHeight(height)) {
                            show.posterUrl?.let { url ->
                                CoilImage(data = url, modifier = Modifier.fillMaxWidth())
                            }
                        }
                    }

                    Column(modifier = Modifier.padding(8.dp)) {
                        Text(
                            text = show.name,
                            style = MaterialTheme.typography.body2,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis
                        )
                    }
                }
            }
        }
    }
}

@Composable
fun SelectServerScreen(onSave: (String) -> Unit) {
    var url by remember { mutableStateOf("") }

    Scaffold(
        topBar = {
            TopAppBar(title = { Text(text = "Select Server") })
        }
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            TextField(
                value = url,
                onValueChange = { url = it },
                label = { Text("Server address") },
                modifier = Modifier.fillMaxWidth(),
            )

            Spacer(modifier = Modifier.height(16.dp))

            Button(
                onClick = { onSave(url) },
                modifier = Modifier.align(Alignment.End)
            ) {
                Text(text = "Save")
            }
        }
    }
}
