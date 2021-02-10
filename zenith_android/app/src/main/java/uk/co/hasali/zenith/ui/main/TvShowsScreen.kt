package uk.co.hasali.zenith.ui.main

import android.content.Intent
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.preferredWidth
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
import uk.co.hasali.zenith.api.TvShow
import uk.co.hasali.zenith.ui.PosterCard
import uk.co.hasali.zenith.ui.showdetails.ShowDetailsActivity

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun TvShowsScreen(serverUrl: String) {
    val context = AmbientContext.current
    var shows: List<TvShow> by remember { mutableStateOf(emptyList()) }

    fun onItemClick(showId: Int) {
        context.startActivity(
            Intent(context, ShowDetailsActivity::class.java).apply {
                putExtra("show_id", showId)
            }
        )
    }

    LaunchedEffect(serverUrl) {
        shows = Fuel.get("$serverUrl/api/tv/shows")
            .awaitObject(gsonDeserializer())
    }

    LazyVerticalGrid(cells = GridCells.Adaptive(128.dp), contentPadding = PaddingValues(4.dp)) {
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
                        onItemClick(show.id)
                    }
            )
        }
    }
}
