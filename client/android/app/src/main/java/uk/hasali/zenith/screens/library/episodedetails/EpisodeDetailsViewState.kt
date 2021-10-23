package uk.hasali.zenith.screens.library.episodedetails

import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show

data class EpisodeDetailsViewState(
    val show: Show? = null,
    val season: Season? = null,
    val episode: Episode? = null,
)
