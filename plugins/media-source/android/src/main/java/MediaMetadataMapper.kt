package com.rigbyfy.mediasource

internal object MediaMetadataMapper {
    fun clean(value: String?): String? = value?.trim()?.takeIf { it.isNotEmpty() }

    fun title(title: String?, fileName: String?): String {
        clean(title)?.let { return it }
        val withoutExtension = clean(fileName)?.substringBeforeLast('.', fileName ?: "")
        return withoutExtension
            ?.replace('_', ' ')
            ?.replace('-', ' ')
            ?.trim()
            ?.replace(Regex("\\s+"), " ")
            ?.takeIf { it.isNotEmpty() }
            ?: "Música desconhecida"
    }

    fun positiveInt(value: String?): Int? = clean(value)?.substringBefore('/')?.toIntOrNull()?.takeIf { it > 0 }

    fun duration(value: String?): Long = value?.toLongOrNull()?.coerceAtLeast(0) ?: 0
}
