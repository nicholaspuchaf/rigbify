package com.rigbyfy.mediadownloader

import android.app.Activity
import android.net.Uri
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import com.yausername.ffmpeg.FFmpeg
import com.yausername.youtubedl_android.YoutubeDL
import com.yausername.youtubedl_android.YoutubeDLRequest
import java.io.File
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.CountDownLatch
import java.util.concurrent.Executors

@TauriPlugin
class MediaDownloaderPlugin(private val activity: Activity) : Plugin(activity) {
    private val executor = Executors.newCachedThreadPool()
    private val tasks = ConcurrentHashMap<String, DownloadStatus>()
    private val requests = ConcurrentHashMap<String, NativeDownloadRequest>()
    private val initialized = CountDownLatch(1)
    @Volatile private var initializationError: Exception? = null

    init {
        executor.execute {
            try {
                YoutubeDL.getInstance().init(activity.application)
                FFmpeg.getInstance().init(activity.application)
            } catch (error: Exception) {
                initializationError = error
            } finally {
                initialized.countDown()
            }
        }
    }

    @Command
    fun getMediaInfo(invoke: Invoke) {
        val request = invoke.parseArgs(MediaInfoRequest::class.java)
        if (!isAllowedUrl(request.url)) {
            invoke.reject("Use uma URL HTTPS válida do YouTube ou YouTube Music", "INVALID_URL")
            return
        }
        executor.execute {
            try {
                ensureInitialized()
                val processId = "info-${System.nanoTime()}"
                val command = YoutubeDLRequest(request.url).apply {
                    addOption("--dump-single-json")
                    addOption("--skip-download")
                    addOption("--no-playlist")
                    addOption("--no-warnings")
                    // YouTube can force SABR for the default client, which
                    // leaves formats without a downloadable URL and causes 403s.
                    addOption("--extractor-args", "youtube:player_client=web_embedded")
                }
                val response = YoutubeDL.getInstance().execute(command, processId) { _, _, _ -> }
                invoke.resolveObject(MediaInfoMapper.fromJson(response.out, request.url))
            } catch (error: Exception) {
                val detail = error.message?.trim()?.takeIf { it.isNotEmpty() } ?: error.javaClass.simpleName
                Log.e(TAG, "Falha ao consultar mídia", error)
                invoke.reject("Não foi possível consultar esta mídia: $detail", "MEDIA_INFO_ERROR", error)
            }
        }
    }

    @Command
    fun startDownload(invoke: Invoke) {
        val request = invoke.parseArgs(NativeDownloadRequest::class.java)
        val validation = validate(request)
        if (validation != null) {
            invoke.reject(validation, "INVALID_DOWNLOAD")
            return
        }
        if (tasks.containsKey(request.downloadId)) {
            invoke.reject("Este download já existe", "DUPLICATE_DOWNLOAD")
            return
        }
        requests[request.downloadId] = request
        val queued = status(request, "download_started", message = "Preparando download")
        publish(queued)
        invoke.resolveObject(queued)
        executor.execute { executeDownload(request) }
    }

    @Command
    fun cancelDownload(invoke: Invoke) {
        val id = invoke.parseArgs(DownloadIdRequest::class.java).downloadId
        val current = tasks[id]
        if (current == null) {
            invoke.reject("Download não encontrado", "DOWNLOAD_NOT_FOUND")
            return
        }
        try { YoutubeDL.getInstance().destroyProcessById(id) } catch (_: Exception) { }
        cleanup(id)
        val cancelled = current.copy(status = "cancelled", message = "Download cancelado")
        publish(cancelled)
        invoke.resolveObject(cancelled)
    }

    @Command
    fun getDownloadStatus(invoke: Invoke) {
        val id = invoke.parseArgs(DownloadIdRequest::class.java).downloadId
        val current = tasks[id]
        if (current == null) invoke.reject("Download não encontrado", "DOWNLOAD_NOT_FOUND")
        else invoke.resolveObject(current)
    }

    private fun executeDownload(request: NativeDownloadRequest) {
        try {
            ensureInitialized()
            val finalDirectory = File(activity.filesDir, "media/downloads").apply { mkdirs() }
            File(activity.cacheDir, "media-downloads/${request.downloadId}").apply { mkdirs() }
            val command = buildRequest(request, finalDirectory)
            val response = YoutubeDL.getInstance().execute(command, request.downloadId) { progress, eta, line ->
                val phase = if (line.contains("Merger") || line.contains("ExtractAudio") || line.contains("Metadata") || line.contains("EmbedThumbnail")) "post_processing" else "download_progress"
                publish(status(request, phase, progress.toDouble(), eta.takeIf { it >= 0 }, line.takeLast(240)))
            }
            if (tasks[request.downloadId]?.status == "cancelled") return
            publish(status(request, "post_processing", 100.0, message = "Finalizando arquivo"))
            val file = findOutput(response.out, finalDirectory, request.options.sourceId)
                ?: throw IllegalStateException("O arquivo final não foi localizado")
            val complete = status(request, "completed", 100.0, message = "Download concluído").copy(
                sourceUri = file.toURI().toString(), fileSize = file.length()
            )
            publish(complete)
            cleanup(request.downloadId, keepFinal = true)
        } catch (error: Exception) {
            if (tasks[request.downloadId]?.status == "cancelled") return
            cleanup(request.downloadId)
            publish(status(request, "failed", message = error.message ?: "Falha no download"))
        }
    }

    private fun buildRequest(request: NativeDownloadRequest, output: File): YoutubeDLRequest {
        val options = request.options
        val command = YoutubeDLRequest(options.source.url)
        command.addOption("--no-playlist")
        command.addOption("--newline")
        command.addOption("--no-mtime")
        // Prefer a client that still exposes direct media URLs when YouTube
        // forces SABR on the default client.
        command.addOption("--extractor-args", "youtube:player_client=web_embedded")
        command.addOption("--paths", "temp:${File(activity.cacheDir, "media-downloads/${request.downloadId}").absolutePath}")
        command.addOption("-o", "${output.absolutePath}/%(title).120B [%(id)s].%(ext)s")
        command.addOption("--print", "after_move:__RIGBYFY_FILE__%(filepath)s")
        command.addOption("--add-metadata")
        val selected = options.formatId?.takeIf { it.matches(Regex("[A-Za-z0-9._-]{1,64}")) }
        when (options.outputFormat) {
            "mp3", "m4a", "opus" -> {
                command.addOption("-f", selected ?: "bestaudio/best")
                command.addOption("--extract-audio")
                command.addOption("--audio-format", options.outputFormat)
                command.addOption("--embed-thumbnail")
            }
            "mp4" -> {
                val height = options.quality?.toIntOrNull()
                val format = selected?.let { "$it+bestaudio/best" } ?: if (height != null) "bestvideo[height<=$height]+bestaudio/best[height<=$height]" else "bestvideo+bestaudio/best"
                command.addOption("-f", format)
                command.addOption("--merge-output-format", "mp4")
                command.addOption("--embed-thumbnail")
            }
            else -> command.addOption("-f", selected ?: if (options.mediaKind == "video") "bestvideo+bestaudio/best" else "bestaudio/best")
        }
        return command
    }

    private fun status(request: NativeDownloadRequest, state: String, progress: Double = 0.0, eta: Long? = null, message: String? = null) = DownloadStatus(
        jobId = request.jobId, downloadId = request.downloadId, status = state,
        progress = progress.coerceIn(0.0, 100.0), eta = eta, message = message,
        title = request.options.title, artist = request.options.artist, album = request.options.album,
        thumbnail = request.options.thumbnail, durationMs = request.options.durationMs,
        sourceUrl = request.options.source.url, sourceId = request.options.sourceId
    )

    private fun publish(value: DownloadStatus) {
        tasks[value.downloadId] = value
        triggerObject("download-progress", value)
    }

    private fun validate(request: NativeDownloadRequest): String? {
        if (!request.jobId.matches(Regex("[A-Za-z0-9-]{1,80}")) || !request.downloadId.matches(Regex("[A-Za-z0-9-]{1,80}"))) return "Identificador de download inválido"
        if (!isAllowedUrl(request.options.source.url)) return "URL inválida"
        if (request.options.source.kind != "single") return "Playlists serão habilitadas em uma etapa futura"
        if (request.options.outputFormat !in setOf("original", "mp3", "m4a", "opus", "mp4")) return "Formato de saída inválido"
        if (request.options.mediaKind !in setOf("audio", "video")) return "Tipo de mídia inválido"
        if (request.options.quality != null && request.options.quality !in setOf("best", "2160", "1440", "1080", "720", "480", "360")) return "Qualidade inválida"
        if (request.options.formatId != null && !request.options.formatId!!.matches(Regex("[A-Za-z0-9._-]{1,64}"))) return "Formato inválido"
        return null
    }

    private fun isAllowedUrl(value: String): Boolean {
        val uri = runCatching { Uri.parse(value.trim()) }.getOrNull() ?: return false
        val host = uri.host?.lowercase() ?: return false
        return uri.scheme == "https" && (host == "youtu.be" || host == "youtube.com" || host.endsWith(".youtube.com"))
    }

    private fun findOutput(output: String, directory: File, sourceId: String): File? {
        val marked = output.lineSequence().lastOrNull { it.startsWith("__RIGBYFY_FILE__") }?.removePrefix("__RIGBYFY_FILE__")?.trim()?.let(::File)
        if (marked?.isFile == true) return marked
        return directory.listFiles()?.filter { it.isFile && !it.name.endsWith(".part") && (sourceId.isBlank() || it.name.contains(sourceId)) }?.maxByOrNull { it.lastModified() }
    }

    private fun ensureInitialized() {
        initialized.await()
        initializationError?.let { error ->
            val detail = generateSequence(error as Throwable?) { it.cause }
                .mapNotNull { it.message?.trim()?.takeIf(String::isNotEmpty) }
                .joinToString("; ")
                .ifBlank { error.javaClass.simpleName }
            Log.e(TAG, "Inicialização do yt-dlp/FFmpeg falhou: $detail", error)
            throw IllegalStateException("Falha ao inicializar yt-dlp/FFmpeg: $detail", error)
        }
    }

    private fun cleanup(id: String, keepFinal: Boolean = false) {
        File(activity.cacheDir, "media-downloads/$id").deleteRecursively()
        if (!keepFinal) File(activity.filesDir, "media/downloads").listFiles()?.filter { it.name.endsWith(".part") || it.name.endsWith(".ytdl") }?.forEach { it.delete() }
        requests.remove(id)
    }

    private companion object { const val TAG = "RigbyfyDownloader" }
}
