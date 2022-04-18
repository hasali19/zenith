package uk.hasali.zenith

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.pm.PackageInstaller

private const val TAG = "AppInstaller"

class InstallReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        val status = intent.getIntExtra(PackageInstaller.EXTRA_STATUS, -123 /* -1 is used by STATUS_PENDING_USER_ACTION */)
        when (status) {
            PackageInstaller.STATUS_PENDING_USER_ACTION -> {
                val activityIntent = intent.getParcelableExtra<Intent>(Intent.EXTRA_INTENT)
                if (activityIntent != null) {
                    context.startActivity(activityIntent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK))
                }
            }
        }
    }
}
