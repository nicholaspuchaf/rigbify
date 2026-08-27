import { describe, expect, it } from 'vitest';
import { albumCollections, artistCollections, sortPlaylists } from './libraryCollections';
import type { Track } from '../types/music';

const tracks: Track[] = [
  { id: '1', title: 'Um', artist: 'Álvaro', album: 'Café', duration: 1, hue: 1 },
  { id: '2', title: 'Dois', artist: 'Álvaro', album: 'Café', duration: 1, hue: 2 },
  { id: '3', title: 'Três', artist: '', album: '', duration: 1, hue: 3 }
];

describe('library collections', () => {
  it('groups albums and artists, including missing metadata', () => {
    expect(albumCollections(tracks).map(({ title, trackIds }) => [title, trackIds])).toEqual([
      ['Álbum desconhecido', ['3']], ['Café', ['1', '2']]
    ]);
    expect(artistCollections(tracks).map(({ title, trackIds }) => [title, trackIds])).toEqual([
      ['Álvaro', ['1', '2']], ['Artista desconhecido', ['3']]
    ]);
  });

  it('sorts playlists deterministically for every supported preference', () => {
    const playlists = [
      { name: 'Zebra', createdAt: 1, updatedAt: 3 },
      { name: 'Álbum', createdAt: 3, updatedAt: 2 },
      { name: 'Café', createdAt: 2, updatedAt: 3 }
    ];
    expect(sortPlaylists(playlists, 'recent').map(({ name }) => name)).toEqual(['Café', 'Zebra', 'Álbum']);
    expect(sortPlaylists(playlists, 'name-ascending').map(({ name }) => name)).toEqual(['Álbum', 'Café', 'Zebra']);
    expect(sortPlaylists(playlists, 'name-descending').map(({ name }) => name)).toEqual(['Zebra', 'Café', 'Álbum']);
  });
});
