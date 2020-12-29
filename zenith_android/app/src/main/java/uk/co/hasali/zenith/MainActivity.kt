package uk.co.hasali.zenith

import android.app.Application
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.res.loadVectorResource
import androidx.compose.ui.res.vectorResource
import androidx.compose.ui.unit.dp
import androidx.datastore.preferences.core.preferencesKey
import androidx.lifecycle.AndroidViewModel
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map
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
                        is Screen.Movies -> MoviesScreen()
                        is Screen.TvShows -> TvShowsScreen()
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
    ) {
        content()
    }
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

@Composable
fun MoviesScreen() {
    Box(modifier = Modifier.fillMaxSize()) {
        Image(
            imageVector = vectorResource(id = R.drawable.movie),
            modifier = Modifier.align(Alignment.Center).size(48.dp),
            colorFilter = ColorFilter.tint(Color.DarkGray)
        )
    }
}

@Composable
fun TvShowsScreen() {
    Box(modifier = Modifier.fillMaxSize()) {
        Image(
            imageVector = vectorResource(id = R.drawable.television),
            modifier = Modifier.align(Alignment.Center).size(48.dp),
            colorFilter = ColorFilter.tint(Color.DarkGray)
        )
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
