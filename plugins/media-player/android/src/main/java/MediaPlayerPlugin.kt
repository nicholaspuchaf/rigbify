package com.rigbyfy.mediaplayer

import android.app.Activity
import android.Manifest
import android.content.ComponentName
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Build
import androidx.core.content.ContextCompat
import androidx.media3.common.MediaItem
import androidx.media3.common.MediaMetadata
import androidx.media3.common.Player
import androidx.media3.session.MediaController
import androidx.media3.session.SessionToken
import app.tauri.annotation.Command
import app.tauri.annotation.Permission
import app.tauri.annotation.PermissionCallback
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import com.google.common.util.concurrent.ListenableFuture

@TauriPlugin(permissions = [Permission(strings = [Manifest.permission.POST_NOTIFICATIONS], alias = "notifications")])
class MediaPlayerPlugin(private val activity: Activity) : Plugin(activity) {
    private var controllerFuture: ListenableFuture<MediaController>? = null
    private var notificationPermissionRequested = false

    @Command fun setQueue(invoke: Invoke) = withController(invoke) { controller ->
        val request = invoke.parseArgs(QueueRequest::class.java)
        if (request.tracks.isEmpty()) {
            throw IllegalArgumentException("A fila do player não pode estar vazia")
        }
        val items = request.tracks.map { track ->
            MediaItem.Builder()
                .setMediaId(track.id)
                .setUri(track.sourceUri)
                .setMediaMetadata(MediaMetadata.Builder()
                    .setTitle(track.title)
                    .setArtist(track.artist)
                    .setAlbumTitle(track.album)
                    .setArtworkUri(track.artworkUri?.let(Uri::parse))
                    .build())
                .build()
        }
        controller.setMediaItems(items, request.startIndex.coerceIn(0, (items.size - 1).coerceAtLeast(0)), request.positionMs.coerceAtLeast(0))
        controller.prepare()
        snapshot(controller)
    }

    @Command fun play(invoke: Invoke) {
        val granted = ContextCompat.checkSelfPermission(activity, Manifest.permission.POST_NOTIFICATIONS) == PackageManager.PERMISSION_GRANTED
        if (PlaybackStateMapper.needsNotificationPermission(Build.VERSION.SDK_INT, granted, notificationPermissionRequested)) {
            notificationPermissionRequested = true
            requestPermissionForAlias("notifications", invoke, "onNotificationPermissionResult")
            return
        }
        performPlay(invoke)
    }

    @PermissionCallback
    private fun onNotificationPermissionResult(invoke: Invoke) = performPlay(invoke)

    private fun performPlay(invoke: Invoke) = withController(invoke) { it.play(); snapshot(it) }
    @Command fun pause(invoke: Invoke) = withController(invoke) { it.pause(); snapshot(it) }
    @Command fun stop(invoke: Invoke) = withController(invoke) { it.stop(); snapshot(it) }
    @Command fun skipNext(invoke: Invoke) = withController(invoke) { it.seekToNextMediaItem(); snapshot(it) }
    @Command fun skipPrevious(invoke: Invoke) = withController(invoke) { it.seekToPreviousMediaItem(); snapshot(it) }
    @Command fun getState(invoke: Invoke) = withController(invoke, ::snapshot)
    @Command fun seekTo(invoke: Invoke) = withController(invoke) {
        it.seekTo(invoke.parseArgs(SeekRequest::class.java).positionMs.coerceAtLeast(0)); snapshot(it)
    }

    private fun withController(invoke: Invoke, operation: (MediaController) -> PlayerSnapshot) {
        val future = controllerFuture ?: MediaController.Builder(
            activity,
            SessionToken(activity, ComponentName(activity, PlaybackService::class.java))
        ).buildAsync().also { controllerFuture = it }
        future.addListener({
            try { invoke.resolveObject(operation(future.get())) }
            catch (error: Exception) { invoke.reject("Falha ao acessar o player nativo", "PLAYER_ERROR", error) }
        }, ContextCompat.getMainExecutor(activity))
    }

    private fun snapshot(player: Player) = PlayerSnapshot(
        status = PlaybackStateMapper.status(player.playbackState, player.playWhenReady, player.playerError != null),
        currentTrackId = player.currentMediaItem?.mediaId,
        positionMs = PlaybackStateMapper.safeTime(player.currentPosition),
        durationMs = PlaybackStateMapper.safeTime(player.duration),
        queueIndex = player.currentMediaItemIndex.coerceAtLeast(0),
        queueSize = player.mediaItemCount,
        isPlaying = player.isPlaying
    )

    override fun onDestroy() {
        controllerFuture?.let(MediaController::releaseFuture)
        controllerFuture = null
    }
}
