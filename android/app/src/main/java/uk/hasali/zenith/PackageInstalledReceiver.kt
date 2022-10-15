package uk.hasali.zenith

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent


class PackageInstalledReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context?, intent: Intent?) {
        val pm = context!!.packageManager
        val launchIntent = pm.getLaunchIntentForPackage(context.packageName)
        context.startActivity(launchIntent)
    }
}
