package com.rigbyfy.mediadownloader

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class MediaInfoMapperTest {
    @Test fun mapsMetadataAndFormats() {
        val info = MediaInfoMapper.fromJson("""{"id":"abc","title":"Song","uploader":"Artist","duration":12.5,"formats":[{"format_id":"140","ext":"m4a","acodec":"mp4a","vcodec":"none","abr":128,"filesize":1000}]}""", "https://youtu.be/abc")
        assertEquals("Song", info.title)
        assertEquals(12500L, info.durationMs)
        assertTrue(info.formats.single().audioOnly)
    }
}
