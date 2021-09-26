package uk.hasali.zenith.ui

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.Spacer
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.statusBarsHeight
import com.google.accompanist.insets.statusBarsPadding
import uk.hasali.zenith.playClick

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
        Column {
            Spacer(modifier = Modifier.statusBarsHeight())
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

class AppBarOverflowMenuItemScope(private val onDismissMenu: () -> Unit) {
    private val items: MutableList<@Composable () -> Unit> = mutableListOf()

    fun item(text: String, onClick: () -> Unit) {
        items.add {
            val context = LocalContext.current

            DropdownMenuItem(onClick = {
                context.playClick()
                onDismissMenu()
                onClick()
            }) {
                Text(text)
            }
        }
    }

    @Composable
    fun Composable() {
        for (item in items) {
            item()
        }
    }
}

@Composable
fun AppBarOverflowMenu(items: AppBarOverflowMenuItemScope.() -> Unit) {
    val context = LocalContext.current
    var expanded by remember { mutableStateOf(false) }

    Box {
        IconButton(onClick = {
            context.playClick()
            expanded = true
        }) {
            Icon(Icons.Default.MoreVert, contentDescription = "More")
        }

        DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
            AppBarOverflowMenuItemScope { expanded = false }
                .also(items)
                .Composable()
        }
    }
}

@Composable
fun FadingAppBar(alpha: Float, onBackPressed: () -> Unit) {
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
                        tint = if (isSystemInDarkTheme()) {
                            MaterialTheme.colors.onSurface
                        } else {
                            MaterialTheme.colors.onPrimary
                        },
                    )
                }
            },
            backgroundColor = Color.Transparent,
            elevation = 0.dp,
            modifier = Modifier.statusBarsPadding(),
        )
    }
}
