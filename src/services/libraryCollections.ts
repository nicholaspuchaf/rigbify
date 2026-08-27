import type { Collection, Track } from '../types/music';

export type PlaylistListSort = 'recent' | 'name-ascending' | 'name-descending';

const unknownAlbum = 'Álbum desconhecido';
const unknownArtist = 'Artista desconhecido';
const collator = new Intl.Collator('pt-BR', { sensitivity: 'base', numeric: true });

function groupTracks(tracks: Track[], type: 'album' | 'artist'): Collection[] {
  const groups = new Map<string, Track[]>();
  for (const track of tracks) {
    const title = type === 'album' ? track.album || unknownAlbum : track.artist || unknownArtist;
    const key = title.trim() || (type === 'album' ? unknownAlbum : unknownArtist);
    groups.set(key, [...(groups.get(key) ?? []), track]);
  }
  return [...groups.entries()]
    .map(([title, groupedTracks]) => ({
      id: `${type}:${title.normalize('NFKC').toLocaleLowerCase('pt-BR')}`,
      title,
      subtitle: type === 'album'
        ? `${groupedTracks[0]?.artist || unknownArtist} · ${groupedTracks.length} músicas`
        : `${groupedTracks.length} músicas`,
      hue: groupedTracks[0]?.hue ?? 0,
      type,
      trackIds: groupedTracks.map((track) => track.id)
    }))
    .sort((left, right) => collator.compare(left.title, right.title));
}

export function albumCollections(tracks: Track[]): Collection[] {
  return groupTracks(tracks, 'album');
}

export function artistCollections(tracks: Track[]): Collection[] {
  return groupTracks(tracks, 'artist');
}

export function sortPlaylists<T extends { name: string; createdAt: number; updatedAt: number }>(
  playlists: T[],
  sort: PlaylistListSort
): T[] {
  return [...playlists].sort((left, right) => {
    if (sort === 'name-ascending') return collator.compare(left.name, right.name) || right.updatedAt - left.updatedAt;
    if (sort === 'name-descending') return collator.compare(right.name, left.name) || right.updatedAt - left.updatedAt;
    return right.updatedAt - left.updatedAt || right.createdAt - left.createdAt || collator.compare(left.name, right.name);
  });
}
