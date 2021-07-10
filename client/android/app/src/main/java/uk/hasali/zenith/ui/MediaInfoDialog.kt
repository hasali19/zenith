package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.material.AlertDialog
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.withStyle
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.playClick

@Composable
fun MediaInfoDialog(info: VideoInfo, onDismiss: () -> Unit) {
    val context = LocalContext.current

    @Composable
    fun Field(name: String, value: String) {
        Text(buildAnnotatedString {
            withStyle(style = MaterialTheme.typography.body2.toSpanStyle()) {
                withStyle(style = SpanStyle(fontWeight = FontWeight.Bold)) {
                    append(name)
                    append(':')
                }

                append(' ')
                append(value)
            }
        })
    }

    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Media Info") },
        text = {
            Column {
                Text("File", style = MaterialTheme.typography.subtitle2)
                Field("Path", info.path)
                Field("Format", info.format)
                Spacer(modifier = Modifier.height(8.dp))

                Text("Video", style = MaterialTheme.typography.subtitle2)
                Field("Codec", info.video.codec)
                Field("Profile", info.video.profile)
                Field("Resolution", "${info.video.width}x${info.video.height}")
                Spacer(modifier = Modifier.height(8.dp))

                Text("Audio", style = MaterialTheme.typography.subtitle2)
                Field("Codec", info.audio.codec)
                Spacer(modifier = Modifier.height(8.dp))
            }
        },
        confirmButton = {
            TextButton(onClick = {
                context.playClick()
                onDismiss()
            }) {
                Text("Close")
            }
        },
    )
}
