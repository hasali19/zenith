package uk.hasali.zenith

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.pm.PackageInstaller
import android.util.Log
import android.widget.Toast

private const val TAG = "AppInstaller"

class InstallReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        when (intent.getIntExtra(PackageInstaller.EXTRA_STATUS, -1)) {
            PackageInstaller.STATUS_PENDING_USER_ACTION -> {
                val activityIntent =
                    intent.getParcelableExtra<Intent>(Intent.EXTRA_INTENT)
                context.startActivity(activityIntent?.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK))
            }

//            PackageInstaller.STATUS_SUCCESS ->
//                Toast.makeText(context, "Install succeeded!", Toast.LENGTH_SHORT).show()
//
//            else -> {
//                val msg = intent.getStringExtra(PackageInstaller.EXTRA_STATUS_MESSAGE)
//                Log.e(TAG, "received $status and $msg")
//            }
        }
    }
}

