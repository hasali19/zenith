package uk.hasali.zenith.screens.library.shows

import androidx.compose.material.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.lifecycle.Lifecycle
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.*

@Composable
fun ShowsScreen(
    model: ShowsViewModel = hiltViewModel(),
    onNavigateToItem: (id: Int) -> Unit,
    onNavigateUp: () -> Unit,
) {
    val shows by rememberFlowWithLifecycle(model.shows)
        .collectAsState(null)

    LifecycleEffect(Lifecycle.State.RESUMED) {
        model.refresh()
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
            year = { it.startYear() },
            isWatched = { it.userData.unwatched == 0 },
            onClick = { onNavigateToItem(it.id) },
        )
    }
}
