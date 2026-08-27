import type { Track } from '../types/music';

export function mostPlayedTracks(tracks: Track[], limit = 4): Track[] {
  return [...tracks]
    .sort((left, right) => (right.playCount ?? 0) - (left.playCount ?? 0) || left.title.localeCompare(right.title, 'pt-BR'))
    .slice(0, limit);
}
