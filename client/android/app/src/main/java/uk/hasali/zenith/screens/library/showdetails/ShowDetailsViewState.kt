package uk.hasali.zenith.screens.library.showdetails

import uk.hasali.zenith.Season
import uk.hasali.zenith.Show

data class ShowDetailsViewState(
    val show: Show? = null,
    val seasons: List<Season>? = null,
)
