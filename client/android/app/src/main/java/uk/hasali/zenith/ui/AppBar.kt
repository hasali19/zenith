package uk.hasali.zenith.ui

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.unit.dp

@Composable
fun AppBar(
    title: String? = null,
    onBackPressed: (() -> Unit)? = null,
    actions: @Composable RowScope.() -> Unit = {},
) {
    val navigationIcon: (@Composable () -> Unit)? = if (onBackPressed == null) null else {
        {
            IconButton(onClick = onBackPressed) {
                Icon(Icons.Default.ArrowBack, contentDescription = "Back")
            }
        }
    }

    Surface(color = MaterialTheme.colors.primarySurface, elevation = 4.dp) {
        Column(modifier = Modifier.statusBarsPadding()) {
            TopAppBar(
                title = { Text(title ?: "") },
                navigationIcon = navigationIcon,
                backgroundColor = Color.Transparent,
                elevation = 0.dp,
                actions = actions,
            )
        }
    }
}

@Composable
fun FadingAppBar(
    alpha: Float,
    onBackPressed: () -> Unit,
    actions: @Composable RowScope.() -> Unit = {},
) {
    Box {
        // Apply the alpha to a surface below the actual appbar
        Surface(
            content = {},
            color = MaterialTheme.colors.primarySurface,
            elevation = 4.dp,
            modifier = Modifier
                .matchParentSize()
                .graphicsLayer { this.alpha = minOf(alpha, 1f) },
        )

        // Draw transparent appbar on top so that the content is always visible
        TopAppBar(
            title = { },
            navigationIcon = {
                IconButton(onClick = onBackPressed) {
                    Icon(
                        imageVector = Icons.Default.ArrowBack,
                        contentDescription = "Back",
                    )
                }
            },
            backgroundColor = Color.Transparent,
            contentColor = if (isSystemInDarkTheme()) {
                MaterialTheme.colors.onSurface
            } else {
                MaterialTheme.colors.onPrimary
            },
            elevation = 0.dp,
            actions = {
                CompositionLocalProvider(LocalContentAlpha provides ContentAlpha.high) {
                    actions()
                }
            },
            modifier = Modifier.statusBarsPadding(),
        )
    }
}
