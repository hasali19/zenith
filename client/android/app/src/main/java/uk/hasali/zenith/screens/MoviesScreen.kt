package uk.hasali.zenith.screens

import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import uk.hasali.zenith.Movie
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.CastButton
import uk.hasali.zenith.ui.LocalZenithClient
import uk.hasali.zenith.ui.PosterGridListScreen

@Composable
fun MoviesScreen(onNavigateToMovie: (Movie) -> Unit, onNavigateUp: () -> Unit) {
    val client = LocalZenithClient.current

    val movies by produceState<List<Movie>?>(null) {
        value = client.getMovies()
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
            date = { it.releaseDate },
            isWatched = { it.userData.isWatched },
            onClick = onNavigateToMovie,
        )
    }
}
