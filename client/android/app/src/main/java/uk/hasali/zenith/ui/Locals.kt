package uk.hasali.zenith.ui

import androidx.compose.runtime.compositionLocalOf
import uk.hasali.zenith.ZenithApiClient

val LocalNavigator = compositionLocalOf<Navigator> { error("No navigator found") }
val LocalZenithClient = compositionLocalOf<ZenithApiClient> { error("No client found!") }
