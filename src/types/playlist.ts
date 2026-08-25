import type { LibraryTrack } from './library';

export type PlaylistSortMode = 'custom' | 'title' | 'artist' | 'album' | 'recently_added';
export type SortDirection = 'ascending' | 'descending';

export type Playlist = {
  id: string;
  name: string;
  description: string | null;
  artworkUri: string | null;
  sortMode: PlaylistSortMode;
  sortDirection: SortDirection;
  createdAt: number;
  updatedAt: number;
  version: number;
  trackCount: number;
  durationMs: number;
};

export type PlaylistDetails = Playlist & { tracks: LibraryTrack[] };

export type PlaylistUpdate = {
  name: string;
  description?: string;
  artworkUri?: string;
  expectedVersion: number;
};
