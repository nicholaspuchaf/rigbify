package com.rigbyfy.mediaplayer

import androidx.media3.common.Player
import app.tauri.annotation.InvokeArg

@InvokeArg
class QueueTrack {
    var id: String = ""
    var sourceUri: String = ""
    var title: String = ""
    var artist: String? = null
    var album: String? = null
    var artworkUri: String? = null
}

@InvokeArg
class QueueRequest {
    var tracks: List<QueueTrack> = emptyList()
    var startIndex: Int = 0
    var positionMs: Long = 0
}

@InvokeArg
class SeekRequest {
    var positionMs: Long = 0
}

data class PlayerSnapshot(
    val status: String,
    val currentTrackId: String?,
    val positionMs: Long,
    val durationMs: Long,
    val queueIndex: Int,
    val queueSize: Int,
    val isPlaying: Boolean
)

object PlaybackStateMapper {
    fun status(playbackState: Int, playWhenReady: Boolean, hasError: Boolean): String = when {
        hasError -> "error"
        playbackState == Player.STATE_BUFFERING -> "loading"
        playbackState == Player.STATE_READY && playWhenReady -> "playing"
        playbackState == Player.STATE_READY -> "paused"
        playbackState == Player.STATE_ENDED -> "stopped"
        else -> "idle"
    }

    fun safeTime(value: Long): Long = value.coerceAtLeast(0)

    fun needsNotificationPermission(sdk: Int, granted: Boolean, alreadyRequested: Boolean): Boolean =
        sdk >= 33 && !granted && !alreadyRequested
}
