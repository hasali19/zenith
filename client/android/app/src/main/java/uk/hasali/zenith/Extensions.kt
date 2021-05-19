package uk.hasali.zenith

import android.content.Context
import android.media.AudioManager
import android.view.SoundEffectConstants

fun Context.playClick() {
    val audioManager = getSystemService(Context.AUDIO_SERVICE) as AudioManager
    audioManager.playSoundEffect(SoundEffectConstants.CLICK, 0.5f)
}