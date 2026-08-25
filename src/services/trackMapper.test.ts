import { describe, expect, it } from 'vitest';
import type { LibraryTrack } from '../types/library';
import { stableHue, toUiTrack } from './trackMapper';

const persistedTrack: LibraryTrack = {
  id: 'track-1', sourceType: 'local_document', sourceUri: 'content://audio/1', title: 'Song',
  artist: null, album: null, durationMs: 90_600, artworkUri: null, mimeType: 'audio/mpeg',
  fileSize: 100, trackNumber: null, discNumber: null, year: null, genre: null, addedAt: 1,
  updatedAt: 1, lastPlayedAt: null, playCount: 0, favorite: false, availability: 'available'
};

describe('toUiTrack', () => {
  it('maps persisted metadata and supplies readable fallbacks', () => {
    const track = toUiTrack(persistedTrack);
    expect(track.artist).toBe('Artista desconhecido');
    expect(track.album).toBe('Álbum desconhecido');
    expect(track.duration).toBe(91);
  });

  it('creates a stable artwork hue from the id', () => {
    expect(stableHue('same')).toBe(stableHue('same'));
    expect(stableHue('same')).toBeGreaterThanOrEqual(0);
    expect(stableHue('same')).toBeLessThan(360);
  });
});
