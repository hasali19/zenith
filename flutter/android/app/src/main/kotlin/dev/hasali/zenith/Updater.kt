package dev.hasali.zenith

import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.content.pm.PackageInstaller
import android.os.Build
import java.io.BufferedInputStream
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.IOException
import java.net.URL

class AppUpdater(private val context: Context) {
    fun downloadAndInstall(url: String, onProgress: (Float) -> Unit) {
        val result = ByteArrayOutputStream()
        val stream = BufferedInputStream(URL(url).openStream())
        val buffer = ByteArray(8192)
        var total = 0f
        while (true) {
            val read = stream.read(buffer)
            if (read == -1) break
            result.write(buffer, 0, read)
            total += read.toFloat()
            onProgress(total / 1024f / 1024f)
        }

        var session: PackageInstaller.Session? = null
        try {
            val installer = context.packageManager.packageInstaller
            val params =
                PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL)

            val sessionId = installer.createSession(params)

            session = installer.openSession(sessionId)
            session.openWrite("package", 0, -1).use { output ->
                ByteArrayInputStream(result.toByteArray()).copyTo(output)
                session.fsync(output)
            }

            var flags = PendingIntent.FLAG_UPDATE_CURRENT
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                flags = flags or PendingIntent.FLAG_MUTABLE
            }

            val intent = Intent(context, InstallReceiver::class.java)
            val pendingIntent = PendingIntent.getBroadcast(context, 3439, intent, flags)
            val receiver = pendingIntent.intentSender

            session.commit(receiver)
            session.close()
        } catch (e: IOException) {
            throw RuntimeException("Couldn't install package", e)
        } catch (e: RuntimeException) {
            session?.abandon()
            throw e
        }
    }
}
