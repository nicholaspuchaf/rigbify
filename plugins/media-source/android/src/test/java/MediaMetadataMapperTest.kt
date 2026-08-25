package com.rigbyfy.mediasource

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class MediaMetadataMapperTest {
    @Test
    fun `uses normalized metadata title when available`() {
        assertEquals("Real title", MediaMetadataMapper.title("  Real title  ", "fallback.mp3"))
    }

    @Test
    fun `creates title from file name`() {
        assertEquals("midnight drive", MediaMetadataMapper.title(null, "midnight_drive.mp3"))
    }

    @Test
    fun `normalizes numeric metadata`() {
        assertEquals(4, MediaMetadataMapper.positiveInt("4/12"))
        assertNull(MediaMetadataMapper.positiveInt("0"))
        assertEquals(0, MediaMetadataMapper.duration("invalid"))
    }
}
