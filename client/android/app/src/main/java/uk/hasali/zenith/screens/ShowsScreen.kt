package uk.hasali.zenith.screens

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import uk.hasali.zenith.Show
import uk.hasali.zenith.ui.LocalZenithClient
import uk.hasali.zenith.ui.PosterGridListScreen

@Composable
fun ShowsScreen(onNavigateToShow: (Show) -> Unit) {
    val client = LocalZenithClient.current

    val shows by produceState<List<Show>?>(null) {
        value = client.getShows()
    }

    PosterGridListScreen(
        items = shows,
        poster = { it.poster },
        name = { it.name },
        date = { it.startDate },
        isWatched = { it.userData.unwatched == 0 },
        onClick = onNavigateToShow,
    )
}
