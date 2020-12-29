package uk.co.hasali.zenith

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.size
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
import uk.co.hasali.zenith.ui.ZenithTheme

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            ZenithApp()
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
fun ZenithApp() {
    var currentScreen: Screen by remember { mutableStateOf(Screen.Home) }
    ZenithTheme {
        TopLevelScreenScaffold(
            screens = listOf(Screen.Home, Screen.Movies, Screen.TvShows),
            currentScreen = currentScreen,
            onScreenChange = { currentScreen = it }
        ) {
            when (currentScreen) {
                is Screen.Home -> HomeScreen()
                is Screen.Movies -> MoviesScreen()
                is Screen.TvShows -> TvShowsScreen()
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
