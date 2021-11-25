package uk.hasali.zenith.screens.library.seasondetails

import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show

data class SeasonDetailsViewState(
    val show: Show? = null,
    val season: Season? = null,
    val episodes: List<Episode>? = null,
)
