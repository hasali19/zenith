package uk.hasali.zenith

import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class ServerUrlProvider @Inject constructor() {
    var url: String? = null
}
