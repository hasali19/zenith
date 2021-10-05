package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.padding
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Movie
import androidx.compose.material.icons.filled.Tv
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavBackStackEntry
import com.google.accompanist.insets.navigationBarsPadding
import uk.hasali.zenith.playClick

@Composable
fun TopLevelScreenScaffold(
    entry: NavBackStackEntry?,
    onNavigate: (String) -> Unit,
    content: @Composable () -> Unit,
) {
    Scaffold(
        modifier = Modifier.navigationBarsPadding(),
        topBar = { AppBar(onNavigate = onNavigate) },
        bottomBar = { BottomNavigation(entry = entry, onNavigate = onNavigate) },
    ) { padding ->
        Box(modifier = Modifier.padding(padding)) {
            content()
        }
    }
}

@Composable
private fun AppBar(onNavigate: (String) -> Unit) {
    AppBar(title = "Zenith") {
        CastButton()
        AppBarOverflowMenu {
            item("Import queue") {
                onNavigate("import_queue")
            }

            item("Transcode queue") {
                onNavigate("transcode_queue")
            }

            item("About") {
                onNavigate("about")
            }
        }
    }
}

@Composable
private fun BottomNavigation(entry: NavBackStackEntry?, onNavigate: (String) -> Unit) {
    val context = LocalContext.current
    val currentRoute = entry?.destination?.route

    @Composable
    fun RowScope.NavigationItem(name: String, icon: ImageVector, route: String) {
        BottomNavigationItem(
            selected = currentRoute == route,
            icon = { Icon(icon, contentDescription = name) },
            label = { Text(name) },
            onClick = {
                context.playClick()
                if (currentRoute != route) {
                    onNavigate(route)
                }
            },
        )
    }

    BottomNavigation {
        NavigationItem(name = "Home", icon = Icons.Default.Home, route = "main/home")
        NavigationItem(name = "Movies", icon = Icons.Default.Movie, route = "main/movies")
        NavigationItem(name = "Shows", icon = Icons.Default.Tv, route = "main/shows")
    }
}
