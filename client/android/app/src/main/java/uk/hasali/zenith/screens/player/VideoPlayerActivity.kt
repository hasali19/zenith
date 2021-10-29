package uk.hasali.zenith.screens.player

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.core.view.WindowCompat
import com.google.android.gms.cast.framework.CastContext
import dagger.hilt.android.AndroidEntryPoint
import uk.hasali.zenith.ui.AppTheme

@AndroidEntryPoint
class VideoPlayerActivity : ComponentActivity() {

    private val model: PlayerViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        // Initialise cast context
        CastContext.getSharedInstance(this)

        val navigateToExternalPlayer = { url: String ->
            finish()
            startActivity(Intent(Intent.ACTION_VIEW).apply {
                setDataAndType(Uri.parse(url), "video/*")
            })
        }

        setContent {
            AppTheme {
                PlayerScreen(
                    model = model,
                    onLaunchExternal = navigateToExternalPlayer,
                    onNavigateUp = { finish() },
                )
            }
        }
    }
}
