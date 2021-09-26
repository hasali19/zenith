package uk.hasali.zenith

import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.content.pm.PackageInstaller
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.features.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.IOException
import java.util.zip.ZipInputStream

class AppUpdater(private val context: Context, private val client: HttpClient) {
    suspend fun downloadAndInstall(onProgress: (Float) -> Unit = {}) {
        downloadAndInstall(
            url = "https://nightly.link/hasali19/zenith/workflows/android/master/zenith-apk.zip",
            onProgress = onProgress,
        )
    }

    private suspend fun downloadAndInstall(url: String, onProgress: (Float) -> Unit) {
        val response: HttpResponse = client.get(url) {
            onDownload { bytesSentTotal, _ ->
                // Report progress in MiB
                onProgress(bytesSentTotal.toFloat() / 1024f / 1024f)
            }
        }

        val zip = ZipInputStream(ByteArrayInputStream(response.receive()))
        val content = ByteArrayOutputStream()

        zip.nextEntry
        zip.copyTo(content)

        var session: PackageInstaller.Session? = null
        try {
            val installer = context.packageManager.packageInstaller
            val params =
                PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL)

            withContext(Dispatchers.IO) {
                val sessionId = installer.createSession(params)

                session = installer.openSession(sessionId)
                session?.let { session ->
                    session.openWrite("package", 0, -1).use { output ->
                        ByteArrayInputStream(content.toByteArray()).copyTo(output)
                        session.fsync(output)
                    }
                }
            }

            val intent = Intent(context, InstallReceiver::class.java)
            val pendingIntent = PendingIntent.getBroadcast(
                context,
                3439,
                intent,
                PendingIntent.FLAG_UPDATE_CURRENT
            )
            val receiver = pendingIntent.intentSender

            session?.commit(receiver)
            session?.close()
        } catch (e: IOException) {
            throw RuntimeException("Couldn't install package", e)
        } catch (e: RuntimeException) {
            session?.abandon()
            throw e
        }
    }
}
