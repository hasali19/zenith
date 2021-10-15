package uk.hasali.zenith.screens

import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import uk.hasali.zenith.Show
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.CastButton
import uk.hasali.zenith.ui.LocalZenithClient
import uk.hasali.zenith.ui.PosterGridListScreen

@Composable
fun ShowsScreen(onNavigateToShow: (Show) -> Unit, onNavigateUp: () -> Unit) {
    val client = LocalZenithClient.current

    val shows by produceState<List<Show>?>(null) {
        value = client.getShows()
    }

    Scaffold(
        topBar = {
            AppBar(title = "Shows", onBackPressed = onNavigateUp) {
                CastButton()
            }
        },
    ) {
        PosterGridListScreen(
            items = shows,
            poster = { it.poster },
            name = { it.name },
            date = { it.startDate },
            isWatched = { it.userData.unwatched == 0 },
            onClick = onNavigateToShow,
        )
    }
}
