package uk.hasali.zenith.ui

fun twoDigitNumber(number: Int) = "$number".padStart(2, '0')

fun displayDuration(duration: Double) =
    if (duration <= 90 * 60) {
        "${(duration / 60).toInt()}m";
    } else {
        val hours = (duration / 3600).toInt()
        val minutes = ((duration % 3600) / 60).toInt();
        "${hours}h ${minutes}m";
    }