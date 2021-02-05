package uk.co.hasali.zenith.ui.main

import androidx.compose.material.Icon
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.loadVectorResource
import uk.co.hasali.zenith.R

sealed class Screen(val name: String, val icon: @Composable () -> Unit) {
    object Home : Screen("Home", {
        Icon(Icons.Filled.Home, "Home")
    })

    object Movies : Screen("Movies", {
        loadVectorResource(id = R.drawable.movie).resource.resource?.let {
            Icon(it, "Movies")
        }
    })

    object TvShows : Screen("TV Shows", {
        loadVectorResource(id = R.drawable.television).resource.resource?.let {
            Icon(it, "TV Shows")
        }
    })
}
