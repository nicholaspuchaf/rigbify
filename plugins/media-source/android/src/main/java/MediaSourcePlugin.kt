package com.rigbyfy.mediasource

import android.app.Activity
import android.content.Intent
import android.media.MediaMetadataRetriever
import android.net.Uri
import android.provider.OpenableColumns
import androidx.activity.result.ActivityResult
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import java.io.File
import java.security.MessageDigest

data class SelectedTrack(
    val sourceType: String = "local_document",
    val sourceUri: String,
    val title: String,
    val fileName: String?,
    val artist: String?,
    val album: String?,
    val durationMs: Long,
    val artworkUri: String?,
    val mimeType: String?,
    val fileSize: Long?,
    val trackNumber: Int?,
    val discNumber: Int?,
    val year: Int?,
    val genre: String?
)

@TauriPlugin
class MediaSourcePlugin(private val activity: Activity) : Plugin(activity) {
    @Command
    fun pickAudio(invoke: Invoke) {
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "audio/*"
            putExtra(Intent.EXTRA_ALLOW_MULTIPLE, true)
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION)
        }
        startActivityForResult(invoke, intent, "onAudioPicked")
    }

    @ActivityCallback
    private fun onAudioPicked(invoke: Invoke, result: ActivityResult) {
        if (result.resultCode != Activity.RESULT_OK) {
            invoke.resolveObject(emptyList<SelectedTrack>())
            return
        }

        val data = result.data
        val uris = buildList {
            data?.data?.let(::add)
            data?.clipData?.let { clip ->
                for (index in 0 until clip.itemCount) add(clip.getItemAt(index).uri)
            }
        }.distinct()

        try {
            val flags = data?.flags?.and(
                Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_GRANT_WRITE_URI_PERMISSION
            ) ?: Intent.FLAG_GRANT_READ_URI_PERMISSION
            uris.forEach { uri ->
                runCatching { activity.contentResolver.takePersistableUriPermission(uri, flags) }
            }
            invoke.resolveObject(uris.map(::readMetadata))
        } catch (error: Exception) {
            invoke.reject("Não foi possível ler as músicas selecionadas", "METADATA_ERROR", error)
        }
    }

    private fun readMetadata(uri: Uri): SelectedTrack {
        val resolver = activity.contentResolver
        var fileName: String? = null
        var fileSize: Long? = null
        resolver.query(uri, arrayOf(OpenableColumns.DISPLAY_NAME, OpenableColumns.SIZE), null, null, null)?.use { cursor ->
            if (cursor.moveToFirst()) {
                fileName = cursor.getString(cursor.getColumnIndexOrThrow(OpenableColumns.DISPLAY_NAME))
                val sizeIndex = cursor.getColumnIndex(OpenableColumns.SIZE)
                if (sizeIndex >= 0 && !cursor.isNull(sizeIndex)) fileSize = cursor.getLong(sizeIndex)
            }
        }

        val retriever = MediaMetadataRetriever()
        return try {
            retriever.setDataSource(activity, uri)
            val rawTitle = retriever.extractMetadata(MediaMetadataRetriever.METADATA_KEY_TITLE)
            SelectedTrack(
                sourceUri = uri.toString(),
                title = MediaMetadataMapper.title(rawTitle, fileName),
                fileName = fileName,
                artist = MediaMetadataMapper.clean(retriever.extractMetadata(MediaMetadataRetriever.METADATA_KEY_ARTIST)),
                album = MediaMetadataMapper.clean(retriever.extractMetadata(MediaMetadataRetriever.METADATA_KEY_ALBUM)),
                durationMs = MediaMetadataMapper.duration(retriever.extractMetadata(MediaMetadataRetriever.METADATA_KEY_DURATION)),
                artworkUri = saveArtwork(uri, retriever.embeddedPicture),
                mimeType = resolver.getType(uri),
                fileSize = fileSize,
                trackNumber = MediaMetadataMapper.positiveInt(retriever.extractMetadata(MediaMetadataRetriever.METADATA_KEY_CD_TRACK_NUMBER)),
                discNumber = MediaMetadataMapper.positiveInt(retriever.extractMetadata(MediaMetadataRetriever.METADATA_KEY_DISC_NUMBER)),
                year = retriever.extractMetadata(MediaMetadataRetriever.METADATA_KEY_YEAR)?.toIntOrNull(),
                genre = MediaMetadataMapper.clean(retriever.extractMetadata(MediaMetadataRetriever.METADATA_KEY_GENRE))
            )
        } finally {
            retriever.release()
        }
    }

    private fun saveArtwork(uri: Uri, bytes: ByteArray?): String? {
        if (bytes == null || bytes.isEmpty() || bytes.size > MAX_ARTWORK_BYTES) return null
        val directory = File(activity.cacheDir, "artwork").apply { mkdirs() }
        val digest = MessageDigest.getInstance("SHA-256").digest(uri.toString().toByteArray())
        val name = digest.take(12).joinToString("") { "%02x".format(it) }
        return File(directory, "$name.image").apply { writeBytes(bytes) }.toURI().toString()
    }

    private companion object {
        const val MAX_ARTWORK_BYTES = 10 * 1024 * 1024
    }
}
