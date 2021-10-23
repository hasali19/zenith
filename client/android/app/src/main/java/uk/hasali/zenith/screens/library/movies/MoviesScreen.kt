package uk.hasali.zenith.screens.library.movies

import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import uk.hasali.zenith.Movie
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.CastButton
import uk.hasali.zenith.ui.PosterGridListScreen
import uk.hasali.zenith.ui.rememberFlowWithLifecycle

@Composable
fun MoviesScreen(
    model: MoviesViewModel = hiltViewModel(),
    onNavigateToMovie: (Movie) -> Unit,
    onNavigateUp: () -> Unit,
) {
    val movies by rememberFlowWithLifecycle(model.movies)
        .collectAsState(null)

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
            date = { it.releaseDate },
            isWatched = { it.userData.isWatched },
            onClick = onNavigateToMovie,
        )
    }
}
