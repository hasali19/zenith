package uk.hasali.zenith.ui

import androidx.compose.runtime.compositionLocalOf
import uk.hasali.zenith.api.ZenithMediaService

val LocalZenithClient = compositionLocalOf<ZenithMediaService> { error("No client found!") }
