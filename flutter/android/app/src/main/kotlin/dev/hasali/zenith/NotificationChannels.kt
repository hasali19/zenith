package dev.hasali.zenith

import android.content.Context
import androidx.core.app.NotificationChannelCompat
import androidx.core.app.NotificationManagerCompat

object NotificationChannels {

    const val Downloads = "downloads"

    fun createAll(context: Context) {
        createUpdateNotificationChannel(context)
    }

    private fun createUpdateNotificationChannel(context: Context) {
        val notificationManager = NotificationManagerCompat.from(context)

        val channel =
            NotificationChannelCompat.Builder(Downloads, NotificationManagerCompat.IMPORTANCE_DEFAULT)
                .setName("Downloads")
                .setShowBadge(true)
                .build()

        notificationManager.createNotificationChannel(channel)
    }
}
