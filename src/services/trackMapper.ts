import type { LibraryTrack } from '../types/library';
import type { Track } from '../types/music';

export function toUiTrack(track: LibraryTrack): Track {
  return {
    id: track.id,
    title: track.title,
    artist: track.artist ?? 'Artista desconhecido',
    album: track.album ?? 'Álbum desconhecido',
    duration: Math.round(track.durationMs / 1_000),
    hue: stableHue(track.id),
    favorite: track.favorite,
    playCount: track.playCount,
    sourceUri: track.sourceUri,
    artworkUri: track.artworkUri ?? undefined
  };
}

export function stableHue(value: string): number {
  let hash = 0;
  for (const character of value) hash = (hash * 31 + character.charCodeAt(0)) | 0;
  return Math.abs(hash) % 360;
}
