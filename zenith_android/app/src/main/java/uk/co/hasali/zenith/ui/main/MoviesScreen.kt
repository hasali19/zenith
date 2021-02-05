package uk.co.hasali.zenith.ui.main

import android.content.Intent
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.GridCells
import androidx.compose.foundation.lazy.LazyVerticalGrid
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.AmbientContext
import androidx.compose.ui.unit.dp
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import uk.co.hasali.zenith.api.Movie
import uk.co.hasali.zenith.ui.PosterCard
import uk.co.hasali.zenith.ui.moviedetails.MovieDetailsActivity

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
        movies = Fuel.get("$serverUrl/api/movies")
            .awaitObject(gsonDeserializer())
    }

    LazyVerticalGrid(cells = GridCells.Adaptive(128.dp), contentPadding = PaddingValues(4.dp)) {
        items(movies) { movie ->
            PosterCard(
                posterUrl = movie.poster,
                primaryText = movie.title,
                secondaryText = movie.releaseYear?.toString(),
                modifier = Modifier
                    .padding(4.dp)
                    .fillMaxWidth()
                    .clickable {
                        onItemClick(movie.id)
                    }
            )
        }
    }
}
