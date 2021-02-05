package uk.co.hasali.zenith.ui.main

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.animation.Crossfade
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.runtime.savedinstancestate.Saver
import androidx.compose.runtime.savedinstancestate.savedInstanceState
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.viewinterop.viewModel
import androidx.lifecycle.ViewModel
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.*
import uk.co.hasali.zenith.ui.ZenithTheme
import uk.co.hasali.zenith.ui.setup.SetupActivity
import java.util.*

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
                    MainScreen(
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

class NavigationViewModel : ViewModel() {
    var currentScreen: Screen by mutableStateOf(Screen.Home)
}

@Composable
fun MainScreen(serverUrl: String, onLaunchSetup: () -> Unit = {}) {
    val nav: NavigationViewModel = viewModel()

    ZenithTheme {
        TopLevelScreenScaffold(
            screens = listOf(Screen.Home, Screen.Movies, Screen.TvShows),
            currentScreen = nav.currentScreen,
            onScreenChange = { nav.currentScreen = it },
            onLaunchSetup = onLaunchSetup,
        ) {
            Crossfade(current = nav.currentScreen) { screen ->
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
    Scaffold(
        topBar = { MainAppBar(onLaunchSetup) },
        bottomBar = { MainBottomNavigation(screens, currentScreen, onScreenChange) }
    ) {
        Box(modifier = Modifier.padding(it)) {
            content()
        }
    }
}

@Composable
fun MainAppBar(onLaunchSetup: () -> Unit) {
    var showMenu by remember { mutableStateOf(false) }

    TopAppBar(
        title = { Text(text = "Zenith") },
        actions = {
            DropdownMenu(
                toggle = {
                    IconButton(onClick = { showMenu = true }) {
                        Icon(imageVector = Icons.Default.MoreVert, "More")
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
}

@Composable
fun MainBottomNavigation(
    screens: List<Screen>,
    currentScreen: Screen,
    onScreenChange: (Screen) -> Unit,
) {
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
