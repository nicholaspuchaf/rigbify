package com.rigbyfy.mediaplayer

import androidx.media3.common.Player
import org.junit.Assert.assertEquals
import org.junit.Test

class PlaybackStateMapperTest {
    @Test fun mapsReadyStates() {
        assertEquals("playing", PlaybackStateMapper.status(Player.STATE_READY, true, false))
        assertEquals("paused", PlaybackStateMapper.status(Player.STATE_READY, false, false))
    }

    @Test fun errorTakesPrecedence() {
        assertEquals("error", PlaybackStateMapper.status(Player.STATE_READY, true, true))
    }

    @Test fun negativeTimesAreNormalized() {
        assertEquals(0L, PlaybackStateMapper.safeTime(-9223372036854775807L))
    }

    @Test fun requestsNotificationPermissionOnlyWhenRequired() {
        assertEquals(true, PlaybackStateMapper.needsNotificationPermission(33, false, false))
        assertEquals(false, PlaybackStateMapper.needsNotificationPermission(32, false, false))
        assertEquals(false, PlaybackStateMapper.needsNotificationPermission(33, true, false))
        assertEquals(false, PlaybackStateMapper.needsNotificationPermission(33, false, true))
    }
}
