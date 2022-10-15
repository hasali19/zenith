package uk.hasali.zenith.screens.library.itemdetails

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.padding
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.ui.BottomSheetContent
import uk.hasali.zenith.ui.BottomSheetContentScope

data class CollectionActionsSheetContent(
    val title: String,
    val onRefreshMetadata: () -> Unit,
) : BottomSheetContent {
    @OptIn(ExperimentalMaterialApi::class)
    @Composable
    override fun BottomSheetContentScope.Content() {
        Text(
            text = title,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
            style = MaterialTheme.typography.subtitle2,
            modifier = Modifier.padding(16.dp),
        )

        Divider()

        ListItem(modifier = Modifier.clickable {
            hide()
            onRefreshMetadata()
        }) {
            Text("Refresh metadata")
        }
    }
}
