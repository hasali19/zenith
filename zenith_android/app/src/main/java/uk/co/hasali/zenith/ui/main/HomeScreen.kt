package uk.co.hasali.zenith.ui.main

import android.content.Intent
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.AmbientContext
import androidx.compose.ui.unit.dp
import com.github.kittinunf.fuel.Fuel
import com.github.kittinunf.fuel.coroutines.awaitObject
import com.github.kittinunf.fuel.gson.gsonDeserializer
import uk.co.hasali.zenith.api.Movie
import uk.co.hasali.zenith.api.TvShow
import uk.co.hasali.zenith.ui.PosterCard
import uk.co.hasali.zenith.ui.moviedetails.MovieDetailsActivity
import uk.co.hasali.zenith.ui.showdetails.ShowDetailsActivity

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
            Intent(context, ShowDetailsActivity::class.java).apply {
                putExtra("show_id", showId)
            }
        )
    }

    LaunchedEffect(serverUrl) {
        movies = Fuel.get("$serverUrl/api/movies/recent")
            .awaitObject(gsonDeserializer())

        shows = Fuel.get("$serverUrl/api/tv/shows/recent")
            .awaitObject(gsonDeserializer())
    }

    Column(modifier = Modifier.verticalScroll(rememberScrollState())) {
        Column(modifier = Modifier.padding(4.dp)) {
            if (movies.isNotEmpty()) {
                Row(modifier = Modifier.padding(8.dp)) {
                    Text("Recently Added Movies")
                }

                LazyRow(contentPadding = PaddingValues(4.dp)) {
                    items(movies) { movie ->
                        PosterCard(
                            posterUrl = movie.poster,
                            primaryText = movie.title,
                            secondaryText = movie.releaseYear?.toString(),
                            modifier = Modifier
                                .padding(4.dp)
                                .preferredWidth(110.dp)
                                .clickable {
                                    onMovieClick(movie.id)
                                }
                        )
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
                        PosterCard(
                            posterUrl = show.poster,
                            primaryText = show.name,
                            secondaryText = show.startYear?.toString(),
                            count = show.unwatchedEpisodes,
                            modifier = Modifier
                                .padding(4.dp)
                                .preferredWidth(110.dp)
                                .clickable {
                                    onShowClick(show.id)
                                }
                        )
                    }
                }
            }
        }
    }
}
