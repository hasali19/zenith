package dev.hasali.zenith

import android.Manifest
import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.ContentValues
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_DATA_SYNC
import android.net.Uri
import android.os.Build
import android.provider.MediaStore
import android.util.Log
import android.webkit.MimeTypeMap
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import androidx.work.CoroutineWorker
import androidx.work.ForegroundInfo
import androidx.work.WorkManager
import androidx.work.WorkerParameters
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.NonCancellable
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.OutputStream
import java.net.HttpURLConnection
import java.net.URL
import java.util.UUID

class DownloadWorker(appContext: Context, params: WorkerParameters) :
    CoroutineWorker(appContext, params) {

    class NotificationActionBroadcastReceiver : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            val id = UUID.fromString(intent!!.getStringExtra("id"))
            Log.i("DownloadWorker", "Cancelling work: $id")
            WorkManager.getInstance(context!!).cancelWorkById(id)
        }
    }

    companion object {
        fun showStartingNotification(context: Context, id: UUID, filename: String) {
            if (context.checkPermission(Manifest.permission.POST_NOTIFICATIONS)) {
                val intent =
                    Intent(context, NotificationActionBroadcastReceiver::class.java).apply {
                        putExtra("id", id.toString())
                    }

                val broadcast = PendingIntent.getBroadcast(
                    context,
                    0,
                    intent,
                    PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_MUTABLE
                )

                NotificationManagerCompat.from(context).notify(
                    id.hashCode(),
                    NotificationCompat.Builder(context, NotificationChannels.DOWNLOADS)
                        .setSmallIcon(R.drawable.ic_notification_small)
                        .setContentTitle(filename)
                        .setContentText("Starting download...")
                        .setOngoing(true)
                        .setProgress(0, 0, true)
                        .addAction(
                            android.R.drawable.ic_menu_close_clear_cancel,
                            "Cancel",
                            broadcast
                        )
                        .build()
                )
            }
        }
    }

    override suspend fun doWork(): Result {
        val uri = inputData.getString("uri")!!
        val cookies = inputData.getString("cookies")
        val filename = inputData.getString("filename")!!

        var success = false
        var outputUri: Uri? = null

        coroutineScope {
            var totalBytes = 0L
            var downloadedBytes = 0L

            val notifier = launch {
                while (true) {
                    if (totalBytes > 0 && downloadedBytes > 0) {
                        updateProgressNotification(filename, downloadedBytes, totalBytes)
                    }
                    delay(1000)
                }
            }

            try {
                outputUri = download(uri, cookies, filename) { d, t ->
                    downloadedBytes = d
                    totalBytes = t
                }
                success = true
            } catch (e: Exception) {
                Log.e("DownloadWorker", e.stackTraceToString())
            } finally {
                notifier.cancel()
            }

            if (!isStopped) {
                showCompletedNotification(filename, success, totalBytes)
            }

            withContext(Dispatchers.Main + NonCancellable) {
                val engine = BackgroundFlutterEngine.getInstance(applicationContext)
                engine.downloaderChannel.invokeMethodWithResult(
                    "onDownloadResult", mapOf(
                        "id" to id.toString(),
                        "success" to success,
                        "uri" to outputUri?.toString(),
                    )
                )
            }
        }

        return if (success) {
            Result.success()
        } else {
            Result.failure()
        }
    }

    private suspend fun download(
        url: String,
        cookies: String?,
        filename: String,
        onProgress: (downloaded: Long, total: Long) -> Unit
    ): Uri = withContext(Dispatchers.IO) {
        withOutputStream(filename) { outputStream ->
            val connection = (URL(url).openConnection() as HttpURLConnection).apply {
                requestMethod = "GET"
                setRequestProperty("Cookie", cookies)
            }

            var downloaded = 0L
            val length = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
                connection.contentLengthLong
            } else {
                connection.getHeaderField("Content-Length")?.toLong() ?: -1
            }

            val inputStream = connection.inputStream.buffered()
            for (it in inputStream.iterator().asSequence().chunked(8192)) {
                outputStream.write(it.toByteArray())
                downloaded += it.size
                onProgress(downloaded, length)
                if (isStopped) {
                    throw CancellationException()
                }
            }
        }
    }

    private inline fun withOutputStream(
        filename: String,
        block: (outputStream: OutputStream) -> Unit
    ): Uri {
        val extension = MimeTypeMap.getFileExtensionFromUrl(filename)
        val mimeType = MimeTypeMap.getSingleton().getMimeTypeFromExtension(extension)
        val resolver = applicationContext.contentResolver

        val videoCollection = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            MediaStore.Video.Media.getContentUri(MediaStore.VOLUME_EXTERNAL_PRIMARY)
        } else {
            MediaStore.Video.Media.EXTERNAL_CONTENT_URI
        }

        val contentValues = ContentValues().apply {
            put(MediaStore.Video.Media.DISPLAY_NAME, filename)
            put(
                MediaStore.MediaColumns.MIME_TYPE,
                mimeType
            )
            put(MediaStore.MediaColumns.IS_PENDING, 1)
        }

        val uri = resolver.insert(videoCollection, contentValues)
            ?: throw Exception("Failed to add file to MediaStore")

        try {
            resolver.openOutputStream(uri)!!.use(block)
        } catch (e: Exception) {
            Log.w("DownloadWorker", "Download failed, deleting output file")
            resolver.delete(uri, null, null)
            throw e
        }

        contentValues.clear()
        contentValues.put(MediaStore.Downloads.IS_PENDING, 0)
        resolver.update(uri, contentValues, null, null)

        return uri
    }

    private suspend fun updateProgressNotification(
        filename: String,
        downloaded: Long,
        length: Long
    ) {
        val intent =
            Intent(applicationContext, NotificationActionBroadcastReceiver::class.java).apply {
                putExtra("id", id.toString())
            }

        val broadcast = PendingIntent.getBroadcast(
            applicationContext,
            0,
            intent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_MUTABLE
        )

        val pc = (downloaded.toDouble() / length.toDouble()) * 100.0
        val notification =
            NotificationCompat.Builder(applicationContext, NotificationChannels.DOWNLOADS)
                .setSmallIcon(R.drawable.ic_notification_small)
                .setContentTitle(filename)
                .setContentText("${downloaded.formatAsSize()}/${length.formatAsSize()}")
                .setOngoing(true)
                .setProgress(100, pc.toInt(), false)
                .addAction(
                    android.R.drawable.ic_menu_close_clear_cancel,
                    "Cancel",
                    broadcast
                )
                .build()

        val foregroundInfo = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            ForegroundInfo(id.hashCode(), notification, FOREGROUND_SERVICE_TYPE_DATA_SYNC)
        } else {
            ForegroundInfo(id.hashCode(), notification)
        }

        setForeground(foregroundInfo)
    }

    private fun showCompletedNotification(filename: String, success: Boolean, length: Long) {
        if (applicationContext.checkPermission(Manifest.permission.POST_NOTIFICATIONS)) {
            val notification =
                NotificationCompat.Builder(applicationContext, NotificationChannels.DOWNLOADS)
                    .setSmallIcon(R.drawable.ic_notification_small)
                    .setContentTitle(filename)
                    .setContentText(if (success) "Download complete · ${length.formatAsSize()}" else "Download failed")
                    .build()

            NotificationManagerCompat.from(applicationContext)
                .notify(
                    UUID.randomUUID().hashCode(),
                    NotificationCompat.Builder(applicationContext, notification)
                        .build()
                )
        }
    }
}
