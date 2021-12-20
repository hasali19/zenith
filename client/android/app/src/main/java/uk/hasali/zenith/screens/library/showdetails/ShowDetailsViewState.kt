package uk.hasali.zenith.screens.library.showdetails

import uk.hasali.zenith.api.Season
import uk.hasali.zenith.api.Show

data class ShowDetailsViewState(
    val show: Show? = null,
    val seasons: List<Season>? = null,
)
