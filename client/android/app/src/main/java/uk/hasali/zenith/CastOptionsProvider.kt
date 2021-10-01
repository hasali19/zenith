package uk.hasali.zenith

import android.content.Context
import com.google.android.gms.cast.CastMediaControlIntent
import com.google.android.gms.cast.framework.CastOptions
import com.google.android.gms.cast.framework.OptionsProvider
import com.google.android.gms.cast.framework.SessionProvider

@Suppress("unused")
class CastOptionsProvider : OptionsProvider {
    override fun getCastOptions(context: Context): CastOptions =
        CastOptions.Builder()
            .setReceiverApplicationId("97F381C3")
            .build()

    override fun getAdditionalSessionProviders(context: Context): MutableList<SessionProvider>? =
        null
}
