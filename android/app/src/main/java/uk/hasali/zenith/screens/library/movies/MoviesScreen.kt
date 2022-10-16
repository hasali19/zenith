package uk.hasali.zenith.screens.library.movies

import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.lifecycle.Lifecycle
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.*

@Composable
fun MoviesScreen(
    model: MoviesViewModel = hiltViewModel(),
    onNavigateToItem: (id: Int) -> Unit,
    onNavigateUp: () -> Unit,
) {
    val movies by rememberFlowWithLifecycle(model.movies)
        .collectAsState(null)

    LifecycleEffect(Lifecycle.State.RESUMED) {
        model.refresh()
    }

    Scaffold(
        topBar = {
            AppBar(title = "Movies", onBackPressed = onNavigateUp) {
                CastButton()
            }
        },
    ) {
        PosterGridListScreen(
            items = movies,
            poster = { it.poster },
            name = { it.title },
            year = { it.releaseYear() },
            isWatched = { it.userData.isWatched },
            onClick = { onNavigateToItem(it.id) },
        )
    }
}