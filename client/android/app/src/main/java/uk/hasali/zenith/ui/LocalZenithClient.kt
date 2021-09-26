package uk.hasali.zenith.ui

import androidx.compose.runtime.compositionLocalOf
import androidx.navigation.NavController
import uk.hasali.zenith.ZenithApiClient

val LocalZenithClient = compositionLocalOf<ZenithApiClient> { error("No client found!") }
