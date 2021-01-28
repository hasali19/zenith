package uk.co.hasali.zenith

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.animation.Crossfade
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.ScrollableColumn
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.WithConstraints
import androidx.compose.ui.platform.AmbientContext
import androidx.compose.ui.platform.AmbientDensity
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.res.loadVectorResource
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import com.google.gson.annotations.SerializedName
import dev.chrisbanes.accompanist.coil.CoilImage
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.ui.ZenithTheme

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        lifecycleScope.launch {
            val settingsRepo = UserSettingsRepository.getInstance(this@MainActivity)
            val settings = settingsRepo.settings.first()
            val serverUrl = settings.serverUrl

            if (serverUrl == null) {
                // Server url has not been initialised, go to setup screen
                startActivity(Intent(this@MainActivity, SetupActivity::class.java))
                finish()
            } else {
                setContent {
                    ZenithApp(
                        serverUrl = serverUrl,
                        onLaunchSetup = {
                            startActivity(Intent(this@MainActivity, SetupActivity::class.java))
                            finish()
                        }
                    )
                }
            }
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
}

@Composable
fun ZenithApp(serverUrl: String, onLaunchSetup: () -> Unit = {}) {
    var screen: Screen by remember { mutableStateOf(Screen.Home) }

    ZenithTheme {
        TopLevelScreenScaffold(
            screens = listOf(Screen.Home, Screen.Movies, Screen.TvShows),
            currentScreen = screen,
            onScreenChange = { screen = it },
            onLaunchSetup = onLaunchSetup,
        ) {
            Crossfade(current = screen) { screen ->
                when (screen) {
                    is Screen.Home -> HomeScreen(serverUrl)
                    is Screen.Movies -> MoviesScreen(serverUrl)
                    is Screen.TvShows -> TvShowsScreen(serverUrl)
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
    onLaunchSetup: () -> Unit,
    content: @Composable () -> Unit,
) {
    var showMenu by remember { mutableStateOf(false) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(text = "Zenith") },
                actions = {
                    DropdownMenu(
                        toggle = {
                            IconButton(onClick = { showMenu = true }) {
                                Icon(imageVector = Icons.Default.MoreVert)
                            }
                        },
                        expanded = showMenu,
                        onDismissRequest = { showMenu = false }
                    ) {
                        DropdownMenuItem(onClick = onLaunchSetup) {
                            Text("Change server")
                        }
                    }
                }
            )
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

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun HomeScreen(serverUrl: String) {
    val context = AmbientContext.current

    var movies: List<Movie> by remember { mutableStateOf(emptyList()) }
    var shows: List<TvShow> by remember { mutableStateOf(emptyList()) }

    fun onMovieClick(movieId: Int) {
        context.startActivity(
            Intent(context, MovieDetailsActivity::class.java).apply {
                putExtra("movie_id", movieId)
            }
        )
    }

    fun onShowClick(showId: Int) {
        context.startActivity(
            Intent(context, TvShowDetailsActivity::class.java).apply {
                putExtra("show_id", showId)
            }
        )
    }

    LaunchedEffect(serverUrl) {
        movies =
            Fuel.get("$serverUrl/api/items?item_type=movie&sort_by[0]=added_at&watch_status=never_watched&limit=10")
                .awaitObject(gsonDeserializer())

        shows =
            Fuel.get("$serverUrl/api/items?item_type=tv_show&sort_by[0]=added_at&watch_status=never_watched&limit=10")
                .awaitObject(gsonDeserializer())
    }

    ScrollableColumn {
        Column(modifier = Modifier.padding(4.dp)) {
            if (movies.isNotEmpty()) {
                Row(modifier = Modifier.padding(8.dp)) {
                    Text("Recently Added Movies")
                }

                LazyRow(contentPadding = PaddingValues(4.dp)) {
                    items(movies) { movie ->
                        Card(
                            modifier = Modifier
                                .padding(4.dp)
                                .preferredWidth(110.dp)
                                .clickable {
                                    onMovieClick(movie.id)
                                }
                        ) {
                            Column {
                                WithConstraints {
                                    val height = with(AmbientDensity.current) {
                                        constraints.maxWidth.toDp() * (3f / 2f)
                                    }

                                    Box(
                                        modifier = Modifier
                                            .fillMaxWidth()
                                            .preferredHeight(height)
                                    ) {
                                        movie.posterUrl?.let { url ->
                                            CoilImage(data = url,
                                                modifier = Modifier.fillMaxWidth())
                                        }
                                    }
                                }

                                Column(modifier = Modifier.padding(8.dp)) {
                                    Text(
                                        text = movie.name,
                                        style = MaterialTheme.typography.body2,
                                        maxLines = 1,
                                        overflow = TextOverflow.Ellipsis
                                    )

                                    Text(
                                        text = movie.releaseYear?.toString() ?: "",
                                        style = MaterialTheme.typography.caption,
                                        maxLines = 1,
                                        overflow = TextOverflow.Ellipsis
                                    )
                                }
                            }
                        }
                    }
                }

                Spacer(modifier = Modifier.preferredHeight(8.dp))
            }

            if (shows.isNotEmpty()) {
                Row(modifier = Modifier.padding(8.dp)) {
                    Text("Recently Added TV")
                }

                LazyRow(contentPadding = PaddingValues(4.dp)) {
                    items(shows) { show ->
                        Card(
                            modifier = Modifier
                                .padding(4.dp)
                                .preferredWidth(110.dp)
                                .clickable {
                                    onShowClick(show.id)
                                }
                        ) {
                            Column {
                                WithConstraints {
                                    val height = with(AmbientDensity.current) {
                                        constraints.maxWidth.toDp() * (3f / 2f)
                                    }

                                    Box(
                                        modifier = Modifier
                                            .fillMaxWidth()
                                            .preferredHeight(height)
                                    ) {
                                        show.posterUrl?.let { url ->
                                            CoilImage(data = url,
                                                modifier = Modifier.fillMaxWidth())
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

                                    Text(
                                        text = show.startYear?.toString() ?: "",
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
        }
    }
}

data class Movie(
    val id: Int,
    val name: String,
    @SerializedName("release_year")
    val releaseYear: Int?,
    @SerializedName("poster_url")
    val posterUrl: String?,
)

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun MoviesScreen(serverUrl: String) {
    val context = AmbientContext.current
    var movies: List<Movie> by remember { mutableStateOf(emptyList()) }

    fun onItemClick(movieId: Int) {
        context.startActivity(
            Intent(context, MovieDetailsActivity::class.java).apply {
                putExtra("movie_id", movieId)
            }
        )
    }

    LaunchedEffect(serverUrl) {
        movies = Fuel.get("$serverUrl/api/items?item_type=movie")
            .awaitObject(gsonDeserializer())
    }

    LazyVerticalGrid(cells = GridCells.Adaptive(128.dp), contentPadding = PaddingValues(4.dp)) {
        items(movies) { movie ->
            Card(
                modifier = Modifier
                    .padding(4.dp)
                    .fillMaxWidth()
                    .clickable {
                        onItemClick(movie.id)
                    }
            ) {
                Column {
                    WithConstraints {
                        val height = with(AmbientDensity.current) {
                            constraints.maxWidth.toDp() * (3f / 2f)
                        }

                        Box(
                            modifier = Modifier
                                .fillMaxWidth()
                                .preferredHeight(height)
                        ) {
                            movie.posterUrl?.let { url ->
                                CoilImage(data = url, modifier = Modifier.fillMaxWidth())
                            }
                        }
                    }

                    Column(modifier = Modifier.padding(8.dp)) {
                        Text(
                            text = movie.name,
                            style = MaterialTheme.typography.body2,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis
                        )

                        Text(
                            text = movie.releaseYear?.toString() ?: "",
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
    @SerializedName("start_year")
    val startYear: Int?,
)

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun TvShowsScreen(serverUrl: String) {
    val context = AmbientContext.current
    var shows: List<TvShow> by remember { mutableStateOf(emptyList()) }

    fun onItemClick(showId: Int) {
        context.startActivity(
            Intent(context, TvShowDetailsActivity::class.java).apply {
                putExtra("show_id", showId)
            }
        )
    }

    LaunchedEffect(serverUrl) {
        shows = Fuel.get("$serverUrl/api/items?item_type=tv_show")
            .awaitObject(gsonDeserializer())
    }

    LazyVerticalGrid(cells = GridCells.Adaptive(128.dp), contentPadding = PaddingValues(4.dp)) {
        items(shows) { show ->
            Card(
                modifier = Modifier
                    .padding(4.dp)
                    .fillMaxWidth()
                    .clickable {
                        onItemClick(show.id)
                    }
            ) {
                Column {
                    WithConstraints {
                        val height = with(AmbientDensity.current) {
                            constraints.maxWidth.toDp() * (3f / 2f)
                        }

                        Box(
                            modifier = Modifier
                                .fillMaxWidth()
                                .preferredHeight(height)
                        ) {
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

                        Text(
                            text = show.startYear?.toString() ?: "",
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
