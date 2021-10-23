package uk.hasali.zenith.screens.library.seasondetails

import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season

data class SeasonDetailsViewState(
    val season: Season? = null,
    val episodes: List<Episode>? = null,
)
