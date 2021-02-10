package uk.co.hasali.zenith.ui.moviedetails

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import dev.chrisbanes.accompanist.insets.AmbientWindowInsets
import dev.chrisbanes.accompanist.insets.statusBarsPadding
import uk.co.hasali.zenith.api.Movie
import uk.co.hasali.zenith.ui.Backdrop
import uk.co.hasali.zenith.ui.ZenithTheme

@Composable
fun MovieDetailsScreen(movie: Movie, onPlay: () -> Unit, onBackPressed: () -> Unit) {
    ZenithTheme {
        Surface(color = MaterialTheme.colors.background) {
            ScreenContent(movie, onPlay)
            Box(modifier = Modifier.statusBarsPadding()) {
                AppBar(onBackPressed)
            }
        }
    }
}

@Composable
fun AppBar(onBackPressed: () -> Unit) {
    TopAppBar(
        title = { /* No title */ },
        backgroundColor = Color.Transparent,
        elevation = 0.dp,
        navigationIcon = {
            IconButton(onClick = { onBackPressed() }) {
                Icon(Icons.Default.ArrowBack, "Back")
            }
        },
    )
}

@Composable
fun ScreenContent(movie: Movie, onPlay: () -> Unit) {
    ConstraintLayout(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
    ) {
        val (backdrop, fab, content) = createRefs()

        Backdrop(
            url = movie.backdrop,
            modifier = Modifier.constrainAs(backdrop) {
                top.linkTo(parent.top)
                start.linkTo(parent.start)
                end.linkTo(parent.end)
            }
        )

        FloatingActionButton(
            onClick = { onPlay() },
            modifier = Modifier
                .padding(32.dp)
                .constrainAs(fab) {
                    end.linkTo(parent.end)
                    centerAround(backdrop.bottom)
                },
        ) {
            Icon(Icons.Default.PlayArrow, "Play")
        }

        Column(
            modifier = Modifier
                .padding(horizontal = 16.dp, vertical = 32.dp)
                .constrainAs(content) {
                    top.linkTo(backdrop.bottom)
                    start.linkTo(parent.start)
                    end.linkTo(parent.end)
                }
        ) {
            HeaderContent(movie = movie)
            Spacer(modifier = Modifier.preferredHeight(16.dp))
            Text(text = movie.overview.orEmpty())
        }
    }
}

@Composable
fun HeaderContent(movie: Movie) {
    Column {
        Text(
            text = movie.title,
            style = MaterialTheme.typography.h6
        )

        Row {
            Text(
                text = movie.releaseYear?.toString().orEmpty(),
                style = MaterialTheme.typography.body2
            )

            Text(
                text = "\u2022",
                style = MaterialTheme.typography.body2,
                modifier = Modifier.padding(horizontal = 8.dp),
            )

            Text(
                text = formatDuration(movie.duration),
                style = MaterialTheme.typography.body2
            )
        }
    }
}

private fun formatDuration(duration: Double): String {
    val value = duration.toLong()
    return if (value <= 90 * 60) {
        "${value / 60}m"
    } else {
        val hours = value / 3600
        val minutes = (value % 3600) / 60
        "${hours}h ${minutes}m"
    }
}
