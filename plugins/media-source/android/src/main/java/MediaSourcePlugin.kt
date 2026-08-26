package com.rigbyfy.mediasource

import android.app.Activity
import android.content.Intent
import android.media.MediaMetadataRetriever
import android.net.Uri
import android.provider.OpenableColumns
import android.provider.DocumentsContract
import android.util.Log
import android.webkit.MimeTypeMap
import org.json.JSONObject
import androidx.activity.result.ActivityResult
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import java.io.File
import java.security.MessageDigest
import java.util.concurrent.Executors

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
    val genre: String?,
    val externalId: String? = null
)

data class BundlePlaylist(
    val id: String,
    val name: String,
    val trackKeys: List<String>
)

data class BundleImport(
    val tracks: List<SelectedTrack>,
    val playlists: List<BundlePlaylist>,
    val missingFiles: Int = 0
)

@TauriPlugin
class MediaSourcePlugin(private val activity: Activity) : Plugin(activity) {
    private val executor = Executors.newSingleThreadExecutor()

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

    @Command
    fun importBundle(invoke: Invoke) {
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE).apply {
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION)
        }
        startActivityForResult(invoke, intent, "onBundleDirectorySelected")
    }

    @ActivityCallback
    private fun onBundleDirectorySelected(invoke: Invoke, result: ActivityResult) {
        if (result.resultCode != Activity.RESULT_OK) {
            Log.w(TAG, "Seleção do bundle cancelada: resultCode=${result.resultCode}")
            invoke.reject("Seleção do bundle cancelada", "BUNDLE_SELECTION_CANCELLED")
            return
        }
        val treeUri = result.data?.data
        if (treeUri == null) {
            Log.e(TAG, "Seleção do bundle retornou RESULT_OK sem URI")
            invoke.reject("Pasta do bundle não foi recebida", "BUNDLE_IMPORT_ERROR")
            return
        }
        Log.i(TAG, "Pasta de bundle selecionada: $treeUri")
        executor.execute { readSelectedBundle(invoke, treeUri) }
    }

    private fun readSelectedBundle(invoke: Invoke, treeUri: Uri) {
        try {
            activity.contentResolver.takePersistableUriPermission(treeUri, Intent.FLAG_GRANT_READ_URI_PERMISSION)
            val manifestUri = bundleUri(treeUri, "manifest.json")
                ?: throw IllegalStateException("Selecione a pasta que contém manifest.json e a pasta audio")
            val manifestText = activity.contentResolver.openInputStream(manifestUri)
                ?.bufferedReader()
                ?.use { it.readText() }
                ?: throw IllegalStateException("Não foi possível ler manifest.json")
            val bundle = readBundle(treeUri, JSONObject(manifestText))
            Log.i(TAG, "Bundle selecionado: uri=$treeUri, faixas=${bundle.tracks.size}, ausentes=${bundle.missingFiles}")
            invoke.resolveObject(bundle)
        } catch (error: Exception) {
            Log.e(TAG, "Falha ao ler bundle selecionado", error)
            invoke.reject(error.message ?: "Não foi possível importar o bundle", "BUNDLE_IMPORT_ERROR", error)
        }
    }

    private fun readBundle(treeUri: Uri, manifest: JSONObject): BundleImport {
        val tracksObject = manifest.getJSONObject("tracks")
        var missingFiles = 0
        val tracks = buildList {
            tracksObject.keys().forEach { key ->
                val value = tracksObject.getJSONObject(key)
                if (value.isNull("file")) return@forEach
                val relativeFile = value.optString("file").trim()
                if (relativeFile.isEmpty()) return@forEach
                val fileUri = bundleUri(treeUri, relativeFile)
                if (fileUri == null) {
                    missingFiles += 1
                    return@forEach
                }
                val fileName = relativeFile.substringAfterLast('/')
                val extension = fileName.substringAfterLast('.', "").lowercase()
                add(
                    SelectedTrack(
                        sourceType = "local_document",
                        sourceUri = fileUri.toString(),
                        externalId = key,
                        title = value.optString("title", "Música desconhecida"),
                        fileName = fileName,
                        artist = value.optString("artist").takeIf { it.isNotBlank() },
                        album = value.optString("album").takeIf { it.isNotBlank() },
                        durationMs = value.optLong("duration_ms", 0),
                        artworkUri = null,
                        mimeType = MimeTypeMap.getSingleton().getMimeTypeFromExtension(extension),
                        fileSize = null,
                        trackNumber = null,
                        discNumber = null,
                        year = null,
                        genre = null
                    )
                )
            }
        }
        val playlists = buildList {
            val values = manifest.optJSONArray("playlists") ?: return@buildList
            for (index in 0 until values.length()) {
                val value = values.getJSONObject(index)
                val keys = value.optJSONArray("trackKeys") ?: continue
                add(BundlePlaylist(value.optString("id"), value.optString("name"), buildList {
                    for (keyIndex in 0 until keys.length()) add(keys.getString(keyIndex))
                }))
            }
        }
        return BundleImport(tracks, playlists, missingFiles)
    }

    private fun bundleUri(treeUri: Uri, relativePath: String): Uri? {
        val segments = relativePath.split('/').filter { it.isNotBlank() }
        if (segments.isEmpty() || segments.any { it == "." || it == ".." }) return null
        val rootDocumentId = DocumentsContract.getTreeDocumentId(treeUri)
        val documentId = "$rootDocumentId/${segments.joinToString("/")}"
        return DocumentsContract.buildDocumentUriUsingTree(treeUri, documentId)
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

    override fun onDestroy() {
        executor.shutdown()
    }

    private companion object {
        const val TAG = "RigbyfyMediaSource"
        const val MAX_ARTWORK_BYTES = 10 * 1024 * 1024
    }
}
