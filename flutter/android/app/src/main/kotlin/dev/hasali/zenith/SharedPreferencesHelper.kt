package dev.hasali.zenith

import android.content.Context

object SharedPreferencesHelper {
    private const val FILE_NAME = "flutter_downloader"
    private const val CALLBACK_DISPATCHER_HANDLE_KEY = "callback_dispatcher_handle"

    private fun Context.prefs() = getSharedPreferences(FILE_NAME, Context.MODE_PRIVATE)

    fun setCallbackDispatcherHandle(context: Context, handle: Long) {
        context.prefs().edit().putLong(CALLBACK_DISPATCHER_HANDLE_KEY, handle).apply()
    }

    fun getCallbackDispatcherHandle(context: Context): Long {
        return context.prefs().getLong(CALLBACK_DISPATCHER_HANDLE_KEY, -1)
    }
}
