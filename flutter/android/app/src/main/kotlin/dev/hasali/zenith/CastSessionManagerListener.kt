package dev.hasali.zenith

import com.google.android.gms.cast.framework.CastSession
import com.google.android.gms.cast.framework.SessionManagerListener

open class CastSessionManagerListener : SessionManagerListener<CastSession> {
    override fun onSessionEnded(session: CastSession, error: Int) {
    }

    override fun onSessionEnding(session: CastSession) {
    }

    override fun onSessionResumeFailed(session: CastSession, error: Int) {
    }

    override fun onSessionResumed(session: CastSession, wasSuspended: Boolean) {
    }

    override fun onSessionResuming(session: CastSession, sessionId: String) {
    }

    override fun onSessionStartFailed(session: CastSession, error: Int) {
    }

    override fun onSessionStarted(session: CastSession, sessionId: String) {
    }

    override fun onSessionStarting(session: CastSession) {
    }

    override fun onSessionSuspended(session: CastSession, reason: Int) {
    }
}
