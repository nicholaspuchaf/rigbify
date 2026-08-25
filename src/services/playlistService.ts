import { invoke } from '@tauri-apps/api/core';
import type { Playlist, PlaylistDetails, PlaylistSortMode, PlaylistUpdate, SortDirection } from '../types/playlist';

type Invoke = <T>(command: string, args?: Record<string, unknown>) => Promise<T>;

export function createPlaylistService(call: Invoke = invoke) {
  return {
    create: (name: string) => call<Playlist>('create_playlist', { name }),
    list: () => call<Playlist[]>('list_playlists'),
    get: (id: string) => call<PlaylistDetails>('get_playlist', { id }),
    update: (id: string, input: PlaylistUpdate) => call<Playlist>('update_playlist', { id, input }),
    delete: (id: string) => call<void>('delete_playlist', { id }),
    addTracks: (id: string, trackIds: string[]) => call<PlaylistDetails>('add_tracks_to_playlist', { id, trackIds }),
    replaceOrder: (id: string, trackIds: string[], expectedVersion: number) =>
      call<PlaylistDetails>('replace_playlist_order', { id, input: { trackIds, expectedVersion } }),
    setSort: (id: string, mode: PlaylistSortMode, direction: SortDirection = 'ascending') =>
      call<PlaylistDetails>('set_playlist_sort', { id, mode, direction })
  };
}

export const playlistService = createPlaylistService();
