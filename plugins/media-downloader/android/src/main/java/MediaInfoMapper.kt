package com.rigbyfy.mediadownloader

import org.json.JSONObject

object MediaInfoMapper {
    fun fromJson(raw: String, requestedUrl: String): MediaInfo {
        val json = JSONObject(raw.trim().lineSequence().last { it.trim().startsWith("{") })
        val formatsJson = json.optJSONArray("formats")
        val formats = buildList {
            if (formatsJson != null) for (index in 0 until formatsJson.length()) {
                val item = formatsJson.optJSONObject(index) ?: continue
                val rawAudioCodec = item.rawText("acodec")
                val rawVideoCodec = item.rawText("vcodec")
                val audioCodec = rawAudioCodec?.takeUnless { it == "none" }
                val videoCodec = rawVideoCodec?.takeUnless { it == "none" }
                add(MediaFormat(
                    id = item.optString("format_id"),
                    container = item.text("ext"),
                    audioCodec = audioCodec,
                    videoCodec = videoCodec,
                    bitrate = item.number("tbr") ?: item.number("abr"),
                    filesize = item.longNumber("filesize") ?: item.longNumber("filesize_approx"),
                    resolution = item.text("resolution") ?: dimensions(item),
                    audioOnly = audioCodec != null && rawVideoCodec == "none",
                    videoOnly = videoCodec != null && rawAudioCodec == "none"
                ))
            }
        }
        val duration = json.number("duration")?.times(1000)?.toLong()
        return MediaInfo(
            id = json.optString("id"), url = requestedUrl,
            title = json.text("title") ?: "Mídia sem título",
            artist = json.text("artist") ?: json.text("creator") ?: json.text("uploader"),
            channel = json.text("channel") ?: json.text("uploader"),
            album = json.text("album"), thumbnail = json.text("thumbnail"),
            durationMs = duration, formats = formats
        )
    }

    private fun JSONObject.text(key: String): String? = optString(key).trim().takeIf { it.isNotEmpty() && it != "null" && it != "none" }
    private fun JSONObject.rawText(key: String): String? = optString(key).trim().takeIf { it.isNotEmpty() && it != "null" }
    private fun JSONObject.number(key: String): Double? = if (has(key) && !isNull(key)) optDouble(key).takeIf { !it.isNaN() } else null
    private fun JSONObject.longNumber(key: String): Long? = number(key)?.toLong()?.takeIf { it > 0 }
    private fun dimensions(item: JSONObject): String? {
        val width = item.optInt("width", 0); val height = item.optInt("height", 0)
        return if (width > 0 && height > 0) "${width}x${height}" else null
    }
}
