package com.rigbyfy.mediadownloader

import app.tauri.annotation.InvokeArg

@InvokeArg class MediaInfoRequest { var url: String = "" }
@InvokeArg class YtdlpUpdateRequest { var channel: String = "stable" }
data class YtdlpVersion(val version: String, val versionName: String)
data class YtdlpUpdateResult(val status: String, val version: String, val versionName: String)
@InvokeArg class DownloadSource { var kind: String = "single"; var url: String = "" }
@InvokeArg class DownloadOptions {
    var source: DownloadSource = DownloadSource()
    var outputFormat: String = "original"
    var mediaKind: String = "audio"
    var quality: String? = null
    var formatId: String? = null
    var title: String = ""
    var artist: String? = null
    var album: String? = null
    var thumbnail: String? = null
    var durationMs: Long? = null
    var sourceId: String = ""
}
@InvokeArg class NativeDownloadRequest {
    var jobId: String = ""
    var downloadId: String = ""
    var options: DownloadOptions = DownloadOptions()
}
@InvokeArg class DownloadIdRequest { var downloadId: String = "" }

data class MediaFormat(
    val id: String,
    val container: String?,
    val audioCodec: String?,
    val videoCodec: String?,
    val bitrate: Double?,
    val filesize: Long?,
    val resolution: String?,
    val audioOnly: Boolean,
    val videoOnly: Boolean
)
data class MediaInfo(
    val id: String,
    val url: String,
    val title: String,
    val artist: String?,
    val channel: String?,
    val album: String?,
    val thumbnail: String?,
    val durationMs: Long?,
    val formats: List<MediaFormat>
)
data class DownloadStatus(
    val jobId: String,
    val downloadId: String,
    val status: String,
    val progress: Double = 0.0,
    val downloadedBytes: Long = 0,
    val totalBytes: Long? = null,
    val speed: Double? = null,
    val eta: Long? = null,
    val message: String? = null,
    val sourceUri: String? = null,
    val fileSize: Long? = null,
    val title: String? = null,
    val artist: String? = null,
    val album: String? = null,
    val thumbnail: String? = null,
    val durationMs: Long? = null,
    val sourceUrl: String? = null,
    val sourceId: String? = null
)
