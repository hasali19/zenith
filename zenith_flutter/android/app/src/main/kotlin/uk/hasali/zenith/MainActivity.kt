package uk.hasali.zenith

import android.app.PendingIntent
import android.content.Intent
import android.content.pm.PackageInstaller
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.IOException
import java.util.zip.ZipInputStream

private const val CHANNEL = "zenith.hasali.uk/updater"

class MainActivity : FlutterActivity() {
    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        MethodChannel(flutterEngine.dartExecutor.binaryMessenger, CHANNEL).setMethodCallHandler { call, result ->
            when (call.method) {
                "installApk" -> installApk(call.argument("url")!!)
                else -> result.notImplemented()
            }
        }
    }

    private fun installApk(url: String) {
        CoroutineScope(Dispatchers.Main).launch {
            val client = HttpClient()
            val response: ByteArray = client.request(url)
            val zip = ZipInputStream(ByteArrayInputStream(response))
            val entry = zip.nextEntry
            val content = ByteArrayOutputStream()

            zip.copyTo(content)

            var session: PackageInstaller.Session? = null
            try {
                val installer = packageManager.packageInstaller
                val params = PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL)

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

                val intent = Intent(application, InstallReceiver::class.java)
                val pendingIntent = PendingIntent.getBroadcast(application, 3439, intent, PendingIntent.FLAG_UPDATE_CURRENT)
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
}
