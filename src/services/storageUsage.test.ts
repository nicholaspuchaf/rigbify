import { describe, expect, it } from 'vitest';
import { calculateStorageUsage, formatBytes } from './storageUsage';
import type { LibraryTrack } from '../types/library';

const track = (fileSize: number | null): LibraryTrack => ({
  id: crypto.randomUUID(), sourceType: 'local_document', sourceUri: 'content://audio', sourcePageUrl: null, externalId: null,
  title: 'Faixa', artist: null, album: null, durationMs: 1, artworkUri: null, mimeType: null, fileSize,
  trackNumber: null, discNumber: null, year: null, genre: null, addedAt: 0, updatedAt: 0, lastPlayedAt: null,
  playCount: 0, favorite: false, availability: 'available'
});

describe('storage usage', () => {
  it('sums only known, non-negative music sizes', () => {
    expect(calculateStorageUsage([track(1_024), track(null), track(-1)]) ).toEqual({
      trackCount: 3, knownSizeTrackCount: 1, unknownSizeTrackCount: 2, musicBytes: 1_024
    });
  });

  it('formats bytes for the settings summary', () => {
    expect(formatBytes(0)).toBe('0 B');
    expect(formatBytes(1_572_864)).toBe('1,5 MB');
  });
});
