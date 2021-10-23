package uk.hasali.zenith.screens.library.shows

import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import uk.hasali.zenith.Show
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.AppBar
import uk.hasali.zenith.ui.CastButton
import uk.hasali.zenith.ui.PosterGridListScreen
import uk.hasali.zenith.ui.rememberFlowWithLifecycle

@Composable
fun ShowsScreen(
    model: ShowsViewModel = hiltViewModel(),
    onNavigateToShow: (Show) -> Unit,
    onNavigateUp: () -> Unit,
) {
    val shows by rememberFlowWithLifecycle(model.shows)
        .collectAsState(null)

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
