package dev.hasali.zenith

import android.content.Context
import androidx.core.app.NotificationChannelCompat
import androidx.core.app.NotificationManagerCompat

object NotificationChannels {

    const val DOWNLOADS = "downloads"
    const val MEDIA = "media"

    fun createAll(context: Context) {
        createUpdateNotificationChannel(context)
        createMediaNotificationChannel(context)
    }

    private fun createUpdateNotificationChannel(context: Context) {
        val notificationManager = NotificationManagerCompat.from(context)

        val channel =
            NotificationChannelCompat.Builder(DOWNLOADS, NotificationManagerCompat.IMPORTANCE_LOW)
                .setName("Downloads")
                .setShowBadge(true)
                .build()

        notificationManager.createNotificationChannel(channel)
    }

    private fun createMediaNotificationChannel(context: Context) {
        val notificationManager = NotificationManagerCompat.from(context)

        val channel = NotificationChannelCompat.Builder(MEDIA, NotificationManagerCompat.IMPORTANCE_HIGH)
            .setName("Media")
            .setImportance(NotificationManagerCompat.IMPORTANCE_LOW)
            .build()

        notificationManager.createNotificationChannel(channel)
    }
}
