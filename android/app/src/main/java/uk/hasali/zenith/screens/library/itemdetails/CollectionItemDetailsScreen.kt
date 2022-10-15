package uk.hasali.zenith.screens.library.itemdetails

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.lazy.LazyListScope
import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.unit.Dp
import kotlinx.coroutines.launch
import uk.hasali.zenith.ui.BottomSheetController
import uk.hasali.zenith.ui.CastButton
import uk.hasali.zenith.ui.Poster

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun CollectionItemDetailsScreen(
    title: String,
    backdrop: String?,
    poster: String?,
    headerContent: @Composable () -> Unit,
    overview: String?,
    isWatched: Boolean,
    bottomSheetController: BottomSheetController,
    onRefreshMetadata: () -> Unit,
    onNavigateUp: () -> Unit,
    content: LazyListScope.(width: Dp) -> Unit = {},
) {
    val scope = rememberCoroutineScope()

    ItemDetailsScreen(
        backdrop = backdrop,
        poster = { Poster(url = poster) },
        appBarActions = {
            CastButton()
            IconButton(onClick = {
                scope.launch {
                    bottomSheetController.show(
                        CollectionActionsSheetContent(
                            title = title,
                            onRefreshMetadata = onRefreshMetadata,
                        )
                    )
                }
            }) {
                Icon(Icons.Default.MoreVert, contentDescription = "More")
            }
        },
        headerContent = headerContent,
        overview = overview,
        isWatched = isWatched,
        onNavigateUp = onNavigateUp,
        content = content,
    )
}
