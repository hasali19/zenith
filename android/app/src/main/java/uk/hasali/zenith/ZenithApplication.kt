package uk.hasali.zenith

import android.app.Application
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class ZenithApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        LanguageCodes.init(this)
    }
}
