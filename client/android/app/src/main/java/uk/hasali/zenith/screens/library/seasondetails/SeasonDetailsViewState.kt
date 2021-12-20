package uk.hasali.zenith.screens.library.seasondetails

import uk.hasali.zenith.api.Episode
import uk.hasali.zenith.api.Season
import uk.hasali.zenith.api.Show

data class SeasonDetailsViewState(
    val show: Show? = null,
    val season: Season? = null,
    val episodes: List<Episode>? = null,
)
